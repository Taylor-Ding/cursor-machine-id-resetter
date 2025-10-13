use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde_json::Value as JsonValue;
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug)]
pub struct ModifyResult {
    pub files_processed: usize,
    pub keys_updated: usize,
    pub keys_deleted: usize,
}

pub struct TelemetryModifier {
    telemetry_keys: Vec<String>,
    session_keys: Vec<String>,
}

impl TelemetryModifier {
    pub fn new(telemetry_keys: Vec<String>, session_keys: Vec<String>) -> Self {
        Self {
            telemetry_keys,
            session_keys,
        }
    }

    /// 处理单个 SQLite 数据库
    pub fn process_sqlite_database(&self, db_path: &Path) -> Result<ModifyResult> {
        let new_machine_id = Uuid::new_v4().to_string();
        let new_session_id = Uuid::new_v4().to_string();

        // 尝试多种连接方式
        let connection_strings = vec![
            format!("file:{}?_journal=WAL&_timeout=5000", db_path.display()),
            format!("file:{}?mode=rw", db_path.display()),
            db_path.to_string_lossy().to_string(),
        ];

        let mut last_error = None;

        for conn_str in connection_strings {
            match self.try_modify_database(&conn_str, &new_machine_id, &new_session_id) {
                Ok(result) => {
                    log::info!(
                        "成功修改数据库 {:?}: {} 个键更新, {} 个键删除",
                        db_path.file_name().unwrap_or_default(),
                        result.keys_updated,
                        result.keys_deleted
                    );
                    return Ok(result);
                }
                Err(e) => {
                    log::debug!("连接失败 {}: {}", conn_str, e);
                    last_error = Some(e);
                    continue;
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("无法连接到数据库: {:?}", db_path)))
    }

    fn try_modify_database(
        &self,
        conn_str: &str,
        machine_id: &str,
        session_id: &str,
    ) -> Result<ModifyResult> {
        let conn = Connection::open(conn_str).context("打开数据库连接失败")?;

        // 测试连接
        conn.execute("SELECT 1", [])
            .context("测试数据库连接失败")?;

        // 查找所有相关表
        let tables = self.find_item_tables(&conn)?;

        if tables.is_empty() {
            log::debug!("数据库中未找到 key-value 表结构");
            return Ok(ModifyResult {
                files_processed: 1,
                keys_updated: 0,
                keys_deleted: 0,
            });
        }

        // 开始事务
        let tx = conn.unchecked_transaction()?;

        let mut total_updated = 0;
        let mut total_deleted = 0;

        for (table_name, key_col, value_col) in tables {
            // 更新 telemetry keys
            for key in &self.telemetry_keys {
                let value = if key.to_lowercase().contains("session") {
                    session_id
                } else {
                    machine_id
                };

                let sql = format!(
                    "UPDATE \"{}\" SET \"{}\" = ?1 WHERE \"{}\" = ?2",
                    table_name, value_col, key_col
                );

                match tx.execute(&sql, params![value, key]) {
                    Ok(affected) if affected > 0 => {
                        total_updated += affected;
                        log::debug!("更新 {}.{} = {}", table_name, key, value);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        log::warn!("更新 {}.{} 失败: {}", table_name, key, e);
                    }
                }
            }

            // 删除 session keys
            for key in &self.session_keys {
                let sql = format!("DELETE FROM \"{}\" WHERE \"{}\" = ?1", table_name, key_col);

                match tx.execute(&sql, params![key]) {
                    Ok(affected) if affected > 0 => {
                        total_deleted += affected;
                        log::debug!("删除 {}.{}", table_name, key);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        log::warn!("删除 {}.{} 失败: {}", table_name, key, e);
                    }
                }
            }
        }

        // 提交事务
        tx.commit().context("提交事务失败")?;

        // VACUUM 优化
        if total_updated > 0 || total_deleted > 0 {
            if let Err(e) = conn.execute("VACUUM", []) {
                log::warn!("VACUUM 失败: {}", e);
            } else {
                log::info!("数据库优化完成");
            }
        }

        Ok(ModifyResult {
            files_processed: 1,
            keys_updated: total_updated,
            keys_deleted: total_deleted,
        })
    }

    /// 查找包含 key-value 结构的表
    fn find_item_tables(&self, conn: &Connection) -> Result<Vec<(String, String, String)>> {
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")?;

        let table_names: Vec<String> = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(Result::ok)
            .filter(|name: &String| !name.starts_with("sqlite_"))
            .collect();

        let mut result = Vec::new();

        // 优先查找 ItemTable
        if table_names.contains(&"ItemTable".to_string()) {
            if let Some(cols) = self.analyze_table_structure(conn, "ItemTable")? {
                log::debug!("找到 ItemTable，列: {} -> {}", cols.0, cols.1);
                result.push(("ItemTable".to_string(), cols.0, cols.1));
            }
        }

        // 查找其他可能的表
        for table in &table_names {
            if table == "ItemTable" {
                continue;
            }
            if let Some(cols) = self.analyze_table_structure(conn, table)? {
                log::debug!("找到表 {}，列: {} -> {}", table, cols.0, cols.1);
                result.push((table.clone(), cols.0, cols.1));
            }
        }

        Ok(result)
    }

    /// 分析表结构，查找 key 和 value 列
    fn analyze_table_structure(
        &self,
        conn: &Connection,
        table: &str,
    ) -> Result<Option<(String, String)>> {
        let sql = format!("PRAGMA table_info(\"{}\")", table);
        let mut stmt = conn.prepare(&sql)?;

        let columns: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(1))?
            .filter_map(Result::ok)
            .collect();

        // 精确匹配
        if columns.contains(&"key".to_string()) && columns.contains(&"value".to_string()) {
            return Ok(Some(("key".to_string(), "value".to_string())));
        }

        // 模糊匹配
        let mut key_col = None;
        let mut value_col = None;

        for col in &columns {
            let lower = col.to_lowercase();
            if key_col.is_none()
                && (lower.contains("key") || lower.contains("name") || lower == "id")
            {
                key_col = Some(col.clone());
            }
            if value_col.is_none()
                && (lower.contains("value")
                    || lower.contains("data")
                    || lower.contains("content"))
            {
                value_col = Some(col.clone());
            }
        }

        match (key_col, value_col) {
            (Some(k), Some(v)) => Ok(Some((k, v))),
            _ => Ok(None),
        }
    }

    /// 处理 JSON 文件
    pub fn process_json_file(&self, json_path: &Path) -> Result<ModifyResult> {
        let content = fs::read_to_string(json_path).context("读取JSON文件失败")?;

        if content.trim().is_empty() {
            return Ok(ModifyResult {
                files_processed: 1,
                keys_updated: 0,
                keys_deleted: 0,
            });
        }

        let mut data: JsonValue = serde_json::from_str(&content).context("解析JSON失败")?;

        let new_machine_id = Uuid::new_v4().to_string();
        let new_session_id = Uuid::new_v4().to_string();

        let (updated, deleted) =
            self.process_json_recursive(&mut data, &new_machine_id, &new_session_id);

        if updated > 0 || deleted > 0 {
            let formatted = serde_json::to_string_pretty(&data)?;
            fs::write(json_path, formatted)?;

            log::info!(
                "成功修改JSON {:?}: {} 个键更新, {} 个键删除",
                json_path.file_name().unwrap_or_default(),
                updated,
                deleted
            );

            Ok(ModifyResult {
                files_processed: 1,
                keys_updated: updated,
                keys_deleted: deleted,
            })
        } else {
            Ok(ModifyResult {
                files_processed: 1,
                keys_updated: 0,
                keys_deleted: 0,
            })
        }
    }

    fn process_json_recursive(
        &self,
        data: &mut JsonValue,
        machine_id: &str,
        session_id: &str,
    ) -> (usize, usize) {
        let mut updated = 0;
        let mut deleted = 0;

        if let Some(obj) = data.as_object_mut() {
            // 更新 telemetry keys
            for key in &self.telemetry_keys {
                if let Some(value) = obj.get_mut(key) {
                    *value = if key.to_lowercase().contains("session") {
                        JsonValue::String(session_id.to_string())
                    } else {
                        JsonValue::String(machine_id.to_string())
                    };
                    updated += 1;
                }
            }

            // 删除 session keys
            for key in &self.session_keys {
                if obj.remove(key).is_some() {
                    deleted += 1;
                }
            }

            // 递归处理嵌套对象
            for (_, v) in obj.iter_mut() {
                let (u, d) = self.process_json_recursive(v, machine_id, session_id);
                updated += u;
                deleted += d;
            }
        } else if let Some(arr) = data.as_array_mut() {
            for item in arr {
                let (u, d) = self.process_json_recursive(item, machine_id, session_id);
                updated += u;
                deleted += d;
            }
        }

        (updated, deleted)
    }
}
