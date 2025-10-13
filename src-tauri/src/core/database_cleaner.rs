use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::path::Path;

#[derive(Debug)]
pub struct CleanResult {
    pub databases_processed: usize,
    pub records_cleaned: usize,
}

pub struct DatabaseCleaner {
    keywords: Vec<String>,
    cache_patterns: Vec<String>,
}

impl DatabaseCleaner {
    pub fn new(keywords: Vec<String>, cache_patterns: Vec<String>) -> Self {
        Self {
            keywords,
            cache_patterns,
        }
    }

    pub fn clean_database(&self, db_path: &Path) -> Result<CleanResult> {
        let conn = Connection::open(db_path).context("打开数据库失败")?;

        let tables = self.get_user_tables(&conn)?;
        let tx = conn.unchecked_transaction()?;

        let mut cleaned_records = 0;

        // 1. 完全清空缓存表
        for table in &tables {
            if self.is_cache_table(table) {
                let sql = format!("DELETE FROM \"{}\"", table);
                match tx.execute(&sql, []) {
                    Ok(affected) if affected > 0 => {
                        cleaned_records += affected;
                        log::info!("清空缓存表 {}: {} 条记录", table, affected);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        log::warn!("清空表 {} 失败: {}", table, e);
                    }
                }
            }
        }

        // 2. 按关键词删除记录
        for table in &tables {
            if self.is_cache_table(table) {
                continue; // 已经完全清空
            }

            let columns = self.get_table_columns(&tx, table)?;

            for keyword in &self.keywords {
                for column in &columns {
                    let sql = format!(
                        "DELETE FROM \"{}\" WHERE \"{}\" LIKE ?1",
                        table, column
                    );
                    let pattern = format!("%{}%", keyword);

                    match tx.execute(&sql, params![pattern]) {
                        Ok(affected) if affected > 0 => {
                            cleaned_records += affected;
                            log::debug!(
                                "删除 {}.{} 包含 '{}': {} 条",
                                table,
                                column,
                                keyword,
                                affected
                            );
                        }
                        Ok(_) => {}
                        Err(e) => {
                            log::warn!("删除 {}.{} 失败: {}", table, column, e);
                        }
                    }
                }
            }
        }

        // 3. 重置用户相关列
        for table in &tables {
            let columns = self.get_table_columns(&tx, table)?;

            for column in columns {
                if self.is_user_column(&column) {
                    // 先尝试设置为 NULL
                    let sql = format!(
                        "UPDATE \"{}\" SET \"{}\" = NULL WHERE \"{}\" IS NOT NULL",
                        table, column, column
                    );

                    match tx.execute(&sql, []) {
                        Ok(affected) if affected > 0 => {
                            cleaned_records += affected;
                            log::debug!("重置用户列 {}.{}: {} 条", table, column, affected);
                        }
                        Ok(_) => {}
                        Err(_) => {
                            // 如果设置 NULL 失败，尝试设置空字符串
                            let sql = format!(
                                "UPDATE \"{}\" SET \"{}\" = '' WHERE \"{}\" != ''",
                                table, column, column
                            );
                            if let Ok(affected) = tx.execute(&sql, []) {
                                if affected > 0 {
                                    cleaned_records += affected;
                                    log::debug!("重置用户列 {}.{}: {} 条", table, column, affected);
                                }
                            }
                        }
                    }
                }
            }
        }

        tx.commit().context("提交事务失败")?;

        // VACUUM
        if cleaned_records > 0 {
            if let Err(e) = conn.execute("VACUUM", []) {
                log::warn!("VACUUM 失败: {}", e);
            } else {
                log::info!("数据库优化完成");
            }
        }

        log::info!(
            "数据库清理完成 {:?}: {} 条记录",
            db_path.file_name().unwrap_or_default(),
            cleaned_records
        );

        Ok(CleanResult {
            databases_processed: 1,
            records_cleaned: cleaned_records,
        })
    }

    fn get_user_tables(&self, conn: &Connection) -> Result<Vec<String>> {
        let mut stmt = conn.prepare(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
        )?;

        let tables = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(Result::ok)
            .collect();

        Ok(tables)
    }

    fn get_table_columns(&self, conn: &Connection, table: &str) -> Result<Vec<String>> {
        let sql = format!("PRAGMA table_info(\"{}\")", table);
        let mut stmt = conn.prepare(&sql)?;

        let columns = stmt
            .query_map([], |row| row.get::<_, String>(1))?
            .filter_map(Result::ok)
            .collect();

        Ok(columns)
    }

    fn is_cache_table(&self, table_name: &str) -> bool {
        let lower = table_name.to_lowercase();
        self.cache_patterns
            .iter()
            .any(|pattern| lower.contains(pattern))
    }

    fn is_user_column(&self, column_name: &str) -> bool {
        let lower = column_name.to_lowercase();
        matches!(
            lower.as_str(),
            "user_id" | "account_id" | "email" | "username" | "userid" | "accountid"
        )
    }
}
