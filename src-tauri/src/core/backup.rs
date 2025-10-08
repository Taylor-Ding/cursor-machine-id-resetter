use std::fs;
use std::path::PathBuf;
use chrono::Local;
use serde_json::{Value, json};
use crate::utils::paths::get_cursor_paths;
use crate::utils::logger::{emit_log, LogLevel};
use tauri::AppHandle;

pub struct BackupManager {
    backup_dir: PathBuf,
    storage_path: PathBuf,
}

impl BackupManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let paths = get_cursor_paths()?;
        let backup_dir = paths.backup_path;
        
        // 确保备份目录存在
        fs::create_dir_all(&backup_dir)?;
        
        Ok(Self {
            backup_dir,
            storage_path: paths.storage_path,
        })
    }
    
    /// 创建备份
    pub fn create_backup(&self) -> Result<String, Box<dyn std::error::Error>> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let backup_id = format!("backup_{}", timestamp);
        let backup_file = self.backup_dir.join(format!("{}.json", backup_id));
        
        // 读取当前storage.json
        if self.storage_path.exists() {
            fs::copy(&self.storage_path, &backup_file)?;
        } else {
            return Err("Storage file does not exist".into());
        }
        
        Ok(backup_id)
    }
    
    /// 列出所有备份
    pub fn list_backups(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let mut backups = Vec::new();
        
        if !self.backup_dir.exists() {
            return Ok(backups);
        }
        
        for entry in fs::read_dir(&self.backup_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                    if filename.starts_with("backup_") {
                        let metadata = fs::metadata(&path)?;
                        let size = metadata.len();
                        
                        // 尝试读取machine_id
                        let mut machine_id = String::from("Unknown");
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                                if let Some(id) = json.get("telemetry.devDeviceId").and_then(|v| v.as_str()) {
                                    machine_id = id.to_string();
                                }
                            }
                        }
                        
                        // 从文件名提取时间戳
                        let timestamp_str = filename.trim_start_matches("backup_");
                        let timestamp = chrono::NaiveDateTime::parse_from_str(
                            timestamp_str,
                            "%Y%m%d_%H%M%S"
                        )
                        .ok()
                        .and_then(|dt| dt.and_local_timezone(chrono::Local).single())
                        .map(|dt| dt.timestamp_millis())
                        .unwrap_or(0);
                        
                        backups.push(json!({
                            "id": filename,
                            "timestamp": timestamp,
                            "machine_id": machine_id,
                            "size": size,
                            "path": path.to_string_lossy()
                        }));
                    }
                }
            }
        }
        
        // 按时间戳降序排列
        backups.sort_by(|a, b| {
            let a_time = a.get("timestamp").and_then(|v| v.as_i64()).unwrap_or(0);
            let b_time = b.get("timestamp").and_then(|v| v.as_i64()).unwrap_or(0);
            b_time.cmp(&a_time)
        });
        
        Ok(backups)
    }
    
    /// 恢复备份
    pub fn restore_backup(&self, app: &AppHandle, backup_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        emit_log(app, LogLevel::Info, format!("开始恢复备份: {}", backup_id));
        
        let backup_file = self.backup_dir.join(format!("{}.json", backup_id));
        
        if !backup_file.exists() {
            emit_log(app, LogLevel::Error, format!("备份文件不存在: {}", backup_id));
            return Err(format!("Backup {} not found", backup_id).into());
        }
        
        emit_log(app, LogLevel::Info, format!("找到备份文件: {:?}", backup_file));
        
        // 创建当前文件的备份
        if self.storage_path.exists() {
            emit_log(app, LogLevel::Info, "创建当前配置的安全备份");
            let restore_backup = format!("{}.restore_bak", self.storage_path.to_string_lossy());
            fs::copy(&self.storage_path, &restore_backup)?;
            emit_log(app, LogLevel::Success, "安全备份创建成功");
        }
        
        // 恢复备份
        emit_log(app, LogLevel::Info, "正在恢复备份文件...");
        fs::copy(&backup_file, &self.storage_path)?;
        emit_log(app, LogLevel::Success, "备份恢复完成");
        
        Ok(())
    }
    
    /// 删除备份
    pub fn delete_backup(&self, app: &AppHandle, backup_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        emit_log(app, LogLevel::Info, format!("开始删除备份: {}", backup_id));
        
        let backup_file = self.backup_dir.join(format!("{}.json", backup_id));
        
        if !backup_file.exists() {
            emit_log(app, LogLevel::Error, format!("备份文件不存在: {}", backup_id));
            return Err(format!("Backup {} not found", backup_id).into());
        }
        
        emit_log(app, LogLevel::Info, format!("正在删除备份文件: {:?}", backup_file));
        fs::remove_file(&backup_file)?;
        emit_log(app, LogLevel::Success, "备份删除成功");
        
        Ok(())
    }
}

