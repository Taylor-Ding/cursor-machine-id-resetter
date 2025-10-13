use crate::core::{
    backup::BackupManager, cache_cleaner::{CacheCleaner, CacheCleanResult},
    database_cleaner::{CleanResult, DatabaseCleaner}, ide_config::IDEConfig,
    telemetry_modifier::{ModifyResult, TelemetryModifier},
};
use crate::utils::logger::{emit_log, LogLevel};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use sysinfo::System;
use tauri::AppHandle;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct ResetResult {
    pub telemetry_result: ModifyResult,
    pub database_result: CleanResult,
    pub cache_result: CacheCleanResult,
}

pub struct IDEResetter {
    config: IDEConfig,
    telemetry_modifier: TelemetryModifier,
    database_cleaner: DatabaseCleaner,
    cache_cleaner: CacheCleaner,
    backup_manager: BackupManager,
}

impl IDEResetter {
    /// 创建 Cursor 重置器
    pub fn new_cursor() -> Result<Self> {
        let config = IDEConfig::cursor();
        Self::new_with_config(config)
    }

    /// 创建 Windsurf 重置器
    pub fn new_windsurf() -> Result<Self> {
        let config = IDEConfig::windsurf();
        Self::new_with_config(config)
    }

    pub fn new_with_config(config: IDEConfig) -> Result<Self> {
        let backup_manager = BackupManager::new()
            .map_err(|e| anyhow::anyhow!("初始化备份管理器失败: {}", e))?;
        
        Ok(Self {
            telemetry_modifier: TelemetryModifier::new(
                config.telemetry_keys.clone(),
                config.session_keys.clone(),
            ),
            database_cleaner: DatabaseCleaner::new(
                config.database_keywords.clone(),
                config.cache_table_patterns.clone(),
            ),
            cache_cleaner: CacheCleaner::new(config.cache_directories.clone()),
            backup_manager,
            config,
        })
    }

    /// 检查 IDE 是否正在运行
    pub fn is_running(&self) -> bool {
        let mut system = System::new_all();
        system.refresh_all();

        for process_name in &self.config.process_names {
            for (_pid, process) in system.processes() {
                let proc_name = process.name().to_string_lossy().to_lowercase();
                let target_name = process_name.to_lowercase();

                if proc_name.contains(&target_name) || target_name.contains(&proc_name) {
                    return true;
                }
            }
        }

        false
    }

    /// 获取 IDE 数据路径
    pub fn get_data_path(&self) -> Result<PathBuf> {
        self.config
            .get_existing_data_path()
            .ok_or_else(|| anyhow::anyhow!("{} 未安装或数据目录不存在", self.config.display_name))
    }

    /// 检查 IDE 是否已安装
    pub fn is_installed(&self) -> bool {
        self.config.is_installed()
    }

    /// 获取当前机器ID
    pub fn get_current_machine_id(&self) -> Option<String> {
        let data_path = self.get_data_path().ok()?;
        let storage_path = data_path.join("User").join("globalStorage").join("storage.json");
        
        if storage_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&storage_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    // 尝试多个可能的键
                    return json.get("telemetry.devDeviceId")
                        .or_else(|| json.get("telemetry.machineId"))
                        .or_else(|| json.get("machineId"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                }
            }
        }
        None
    }

    /// 获取IDE版本
    pub fn get_ide_version(&self) -> Option<String> {
        let data_path = self.get_data_path().ok()?;
        
        // 尝试多个可能的位置
        let possible_paths = vec![
            data_path.parent()?.join("package.json"),
            data_path.join("package.json"),
        ];
        
        for package_json in possible_paths {
            if package_json.exists() {
                if let Ok(content) = std::fs::read_to_string(&package_json) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(version) = json.get("version").and_then(|v| v.as_str()) {
                            return Some(version.to_string());
                        }
                    }
                }
            }
        }
        
        Some("Latest".to_string())
    }

    /// 执行完整重置流程
    pub fn reset(&mut self, app: &AppHandle) -> Result<ResetResult> {
        emit_log(
            app,
            LogLevel::Info,
            format!("========== 开始 {} 重置流程 ==========", self.config.display_name),
        );

        // 1. 检查进程
        if self.is_running() {
            let msg = format!("{} 正在运行，请先关闭", self.config.display_name);
            emit_log(app, LogLevel::Error, &msg);
            return Err(anyhow::anyhow!(msg));
        }

        // 2. 创建备份
        emit_log(app, LogLevel::Info, "正在创建备份...");
        let backup_id = self.backup_manager.create_backup()
            .map_err(|e| anyhow::anyhow!("创建备份失败: {}", e))?;
        emit_log(
            app,
            LogLevel::Success,
            format!("备份创建成功: {}", backup_id),
        );

        // 3. 获取数据路径
        let data_path = self.get_data_path()?;
        emit_log(
            app,
            LogLevel::Info,
            format!("数据路径: {}", data_path.display()),
        );

        // 4. 阶段1: Telemetry修改 (20%-45%)
        let telemetry_result = self.reset_telemetry(app, &data_path)?;

        // 5. 阶段2: 数据库清理 (50%-65%)
        let database_result = self.reset_database(app, &data_path)?;

        // 6. 阶段3: 缓存清理 (80%-100%)
        let cache_result = self.reset_cache(app, &data_path)?;

        emit_log(
            app,
            LogLevel::Success,
            format!("========== {} 重置完成 ==========", self.config.display_name),
        );

        Ok(ResetResult {
            telemetry_result,
            database_result,
            cache_result,
        })
    }

    /// 阶段1: 重置 Telemetry
    fn reset_telemetry(&self, app: &AppHandle, data_path: &Path) -> Result<ModifyResult> {
        emit_log(
            app,
            LogLevel::Info,
            "【阶段 1/3】正在修改设备标识符...",
        );

        let mut total_files = 0;
        let mut total_updated = 0;
        let mut total_deleted = 0;

        // 查找所有数据库文件
        let db_files = self.find_database_files(data_path);
        emit_log(
            app,
            LogLevel::Info,
            format!("找到 {} 个数据库文件", db_files.len()),
        );

        for (index, db_file) in db_files.iter().enumerate() {
            emit_log(
                app,
                LogLevel::Info,
                format!(
                    "处理数据库 ({}/{}): {}",
                    index + 1,
                    db_files.len(),
                    db_file.file_name().unwrap_or_default().to_string_lossy()
                ),
            );

            match self.telemetry_modifier.process_sqlite_database(db_file) {
                Ok(result) => {
                    total_files += result.files_processed;
                    total_updated += result.keys_updated;
                    total_deleted += result.keys_deleted;
                }
                Err(e) => {
                    log::warn!("处理数据库失败 {:?}: {}", db_file, e);
                }
            }
        }

        // 查找并处理 JSON 文件
        let json_files = self.find_json_files(data_path);
        emit_log(
            app,
            LogLevel::Info,
            format!("找到 {} 个 JSON 配置文件", json_files.len()),
        );

        for (index, json_file) in json_files.iter().enumerate() {
            emit_log(
                app,
                LogLevel::Info,
                format!(
                    "处理 JSON ({}/{}): {}",
                    index + 1,
                    json_files.len(),
                    json_file.file_name().unwrap_or_default().to_string_lossy()
                ),
            );

            match self.telemetry_modifier.process_json_file(json_file) {
                Ok(result) => {
                    total_files += result.files_processed;
                    total_updated += result.keys_updated;
                    total_deleted += result.keys_deleted;
                }
                Err(e) => {
                    log::warn!("处理 JSON 失败 {:?}: {}", json_file, e);
                }
            }
        }

        emit_log(
            app,
            LogLevel::Success,
            format!(
                "Telemetry 修改完成: {} 个文件, {} 个键更新, {} 个键删除",
                total_files, total_updated, total_deleted
            ),
        );

        Ok(ModifyResult {
            files_processed: total_files,
            keys_updated: total_updated,
            keys_deleted: total_deleted,
        })
    }

    /// 阶段2: 重置数据库
    fn reset_database(&self, app: &AppHandle, data_path: &Path) -> Result<CleanResult> {
        emit_log(
            app,
            LogLevel::Info,
            "【阶段 2/3】正在清理数据库...",
        );

        let db_files = self.find_database_files(data_path);
        let mut total_databases = 0;
        let mut total_records = 0;

        for (index, db_file) in db_files.iter().enumerate() {
            emit_log(
                app,
                LogLevel::Info,
                format!(
                    "清理数据库 ({}/{}): {}",
                    index + 1,
                    db_files.len(),
                    db_file.file_name().unwrap_or_default().to_string_lossy()
                ),
            );

            match self.database_cleaner.clean_database(db_file) {
                Ok(result) => {
                    total_databases += result.databases_processed;
                    total_records += result.records_cleaned;
                }
                Err(e) => {
                    log::warn!("清理数据库失败 {:?}: {}", db_file, e);
                }
            }
        }

        emit_log(
            app,
            LogLevel::Success,
            format!(
                "数据库清理完成: {} 个数据库, {} 条记录",
                total_databases, total_records
            ),
        );

        Ok(CleanResult {
            databases_processed: total_databases,
            records_cleaned: total_records,
        })
    }

    /// 阶段3: 重置缓存
    fn reset_cache(&self, app: &AppHandle, data_path: &Path) -> Result<CacheCleanResult> {
        emit_log(
            app,
            LogLevel::Info,
            "【阶段 3/3】正在清理缓存...",
        );

        let result = self
            .cache_cleaner
            .clean_cache(data_path)
            .context("缓存清理失败")?;

        emit_log(
            app,
            LogLevel::Success,
            format!(
                "缓存清理完成: {} 个目录, {}",
                result.directories_cleaned,
                CacheCleaner::format_size(result.total_size_freed)
            ),
        );

        Ok(result)
    }

    /// 查找所有数据库文件
    fn find_database_files(&self, root: &Path) -> Vec<PathBuf> {
        let extensions = vec!["vscdb", "db", "sqlite", "sqlite3"];

        WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                if let Some(ext) = e.path().extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    extensions.contains(&ext_str.as_str())
                } else {
                    false
                }
            })
            .filter(|e| {
                // 排除备份文件
                let path_str = e.path().to_string_lossy().to_lowercase();
                !path_str.contains("backup") && !path_str.contains(".bak")
            })
            .map(|e| e.path().to_path_buf())
            .collect()
    }

    /// 查找配置 JSON 文件
    fn find_json_files(&self, root: &Path) -> Vec<PathBuf> {
        let target_files = vec!["storage.json", "preferences.json", "settings.json"];

        WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                if let Some(name) = e.path().file_name() {
                    let name_str = name.to_string_lossy().to_lowercase();
                    target_files.iter().any(|target| name_str == *target)
                } else {
                    false
                }
            })
            .map(|e| e.path().to_path_buf())
            .collect()
    }
}
