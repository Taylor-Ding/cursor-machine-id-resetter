use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use serde_json::{Value, json};
use crate::utils::paths::get_cursor_paths;
use crate::core::patcher::Patcher;
use crate::core::system::SystemIdManager;
use crate::utils::logger::{emit_log, LogLevel};
use tauri::AppHandle;

pub struct MachineIdResetter {
    storage_path: PathBuf,
    sqlite_path: PathBuf,
    #[allow(dead_code)]
    machine_id_path: PathBuf,
    cursor_path: PathBuf,
}

impl MachineIdResetter {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let paths = get_cursor_paths()?;
        
        Ok(Self {
            storage_path: paths.storage_path,
            sqlite_path: paths.sqlite_path,
            machine_id_path: paths.machine_id_path,
            cursor_path: paths.cursor_path,
        })
    }
    
    /// 获取 storage.json 路径
    pub fn get_storage_path(&self) -> PathBuf {
        self.storage_path.clone()
    }
    
    /// 获取 SQLite 数据库路径
    pub fn get_sqlite_path(&self) -> PathBuf {
        self.sqlite_path.clone()
    }
    
    /// 获取当前机器ID
    pub fn get_current_machine_id(&self) -> Option<String> {
        if self.storage_path.exists() {
            if let Ok(content) = fs::read_to_string(&self.storage_path) {
                if let Ok(json) = serde_json::from_str::<Value>(&content) {
                    return json.get("telemetry.devDeviceId")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                }
            }
        }
        None
    }
    
    /// 获取Cursor版本
    pub fn get_cursor_version(&self) -> Option<String> {
        let package_json = self.cursor_path.join("package.json");
        
        // 打印调试信息
        eprintln!("🔍 Checking package.json at: {:?}", package_json);
        eprintln!("📁 Package.json exists: {}", package_json.exists());
        
        if package_json.exists() {
            match fs::read_to_string(&package_json) {
                Ok(content) => {
                    eprintln!("✅ Successfully read package.json");
                    match serde_json::from_str::<Value>(&content) {
                        Ok(json) => {
                            eprintln!("✅ Successfully parsed JSON");
                            if let Some(version) = json.get("version").and_then(|v| v.as_str()) {
                                eprintln!("✅ Found version: {}", version);
                                return Some(version.to_string());
                            } else {
                                eprintln!("❌ No version field in JSON");
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to parse JSON: {}", e),
                    }
                }
                Err(e) => eprintln!("❌ Failed to read file: {}", e),
            }
        } else {
            eprintln!("❌ Package.json does not exist at path");
            
            // 尝试查找 Cursor 安装路径
            eprintln!("🔍 Trying to find Cursor installation...");
            eprintln!("📂 Cursor path: {:?}", self.cursor_path);
            
            // 检查父目录
            if let Some(parent) = self.cursor_path.parent() {
                eprintln!("📂 Parent directory: {:?}", parent);
                if parent.exists() {
                    eprintln!("✅ Parent directory exists");
                    if let Ok(entries) = fs::read_dir(parent) {
                        eprintln!("📋 Contents of parent directory:");
                        for entry in entries.flatten() {
                            eprintln!("  - {:?}", entry.path());
                        }
                    }
                } else {
                    eprintln!("❌ Parent directory does not exist");
                }
            }
        }
        
        None
    }
    
    /// 检查Cursor是否正在运行
    pub fn is_cursor_running(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            // 使用与 find_cursor_processes 相同的逻辑
            if let Ok(output) = Command::new("tasklist")
                .arg("/FI")
                .arg("IMAGENAME eq Cursor.exe")
                .arg("/FO")
                .arg("CSV")
                .arg("/NH")
                .output()
            {
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    // 检查是否有非空输出（排除 "INFO: No tasks..." 的情况）
                    let has_process = stdout
                        .lines()
                        .any(|line| !line.is_empty() && line.contains("Cursor.exe"));
                    return has_process;
                }
            }
            false
        }
        
        #[cfg(target_os = "macos")]
        {
            Command::new("pgrep")
                .arg("-x")
                .arg("Cursor")
                .output()
                .ok()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
        
        #[cfg(target_os = "linux")]
        {
            Command::new("pgrep")
                .arg("-x")
                .arg("cursor")
                .output()
                .ok()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
    }
    
    /// 创建文件备份
    fn backup_file(&self, app: &AppHandle, file_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
        if !file_path.exists() {
            emit_log(app, LogLevel::Warning, format!("文件不存在，无需备份: {:?}", file_path));
            return Err("文件不存在".into());
        }
        
        // 备份文件路径：原文件名 + .backup
        let backup_path = file_path.with_extension(
            format!("{}.backup", file_path.extension().and_then(|s| s.to_str()).unwrap_or(""))
        );
        
        emit_log(app, LogLevel::Info, format!("正在备份文件: {:?}", file_path));
        emit_log(app, LogLevel::Info, format!("备份到: {:?}", backup_path));
        
        // 复制文件
        fs::copy(file_path, &backup_path)?;
        
        emit_log(app, LogLevel::Success, format!("文件备份成功: {:?}", backup_path));
        Ok(backup_path)
    }
    
    /// 恢复备份文件
    pub fn restore_backup(&self, app: &AppHandle, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let backup_path = file_path.with_extension(
            format!("{}.backup", file_path.extension().and_then(|s| s.to_str()).unwrap_or(""))
        );
        
        if !backup_path.exists() {
            let error_msg = format!("备份文件不存在: {:?}", backup_path);
            emit_log(app, LogLevel::Error, &error_msg);
            return Err(error_msg.into());
        }
        
        emit_log(app, LogLevel::Info, format!("正在从备份恢复: {:?}", backup_path));
        emit_log(app, LogLevel::Info, format!("恢复到: {:?}", file_path));
        
        // 复制备份文件回原位置
        fs::copy(&backup_path, file_path)?;
        
        emit_log(app, LogLevel::Success, "文件恢复成功");
        Ok(())
    }
    
    /// 更新 storage.json
    pub fn update_storage_json(&mut self, app: &AppHandle, new_ids: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        emit_log(app, LogLevel::Info, format!("正在更新 storage.json: {:?}", self.storage_path));
        
        // 先备份文件
        if self.storage_path.exists() {
            match self.backup_file(app, &self.storage_path) {
                Ok(backup_path) => {
                    emit_log(app, LogLevel::Success, format!("已创建备份: {:?}", backup_path));
                }
                Err(e) => {
                    emit_log(app, LogLevel::Warning, format!("备份失败（将继续操作）: {}", e));
                }
            }
        }
        
        let mut config: Value = if self.storage_path.exists() {
            emit_log(app, LogLevel::Info, "读取现有的 storage.json 文件");
            let content = fs::read_to_string(&self.storage_path)?;
            serde_json::from_str(&content)?
        } else {
            emit_log(app, LogLevel::Warning, "storage.json 不存在，创建新文件");
            json!({})
        };
        
        // 记录旧值（用于对比）
        if let Some(obj) = config.as_object() {
            for key in new_ids.keys() {
                if let Some(old_value) = obj.get(key).and_then(|v| v.as_str()) {
                    emit_log(app, LogLevel::Info, format!("【{}】旧值: {}", key, old_value));
                }
            }
        }
        
        // 更新ID
        emit_log(app, LogLevel::Info, format!("正在更新 {} 个 ID 字段", new_ids.len()));
        if let Some(obj) = config.as_object_mut() {
            for (key, value) in new_ids {
                emit_log(app, LogLevel::Success, format!("【{}】新值: {}", key, value));
                obj.insert(key.clone(), Value::String(value.clone()));
            }
        }
        
        // 写回文件
        emit_log(app, LogLevel::Info, "写入更新后的配置到文件");
        let content = serde_json::to_string_pretty(&config)?;
        fs::write(&self.storage_path, content)?;
        
        emit_log(app, LogLevel::Success, "storage.json 更新完成");
        Ok(())
    }
    
    /// 更新 SQLite 数据库
    pub fn update_sqlite_db(&self, app: &AppHandle, new_ids: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        use rusqlite::Connection;
        
        emit_log(app, LogLevel::Info, format!("正在更新 SQLite 数据库: {:?}", self.sqlite_path));
        
        // 先备份数据库文件
        if self.sqlite_path.exists() {
            emit_log(app, LogLevel::Info, "找到现有的 SQLite 数据库");
            match self.backup_file(app, &self.sqlite_path) {
                Ok(backup_path) => {
                    emit_log(app, LogLevel::Success, format!("已创建备份: {:?}", backup_path));
                }
                Err(e) => {
                    emit_log(app, LogLevel::Warning, format!("备份失败（将继续操作）: {}", e));
                }
            }
        } else {
            emit_log(app, LogLevel::Warning, "SQLite 数据库不存在，将创建新数据库");
        }
        
        // 连接到 SQLite 数据库
        emit_log(app, LogLevel::Info, "正在连接到 SQLite 数据库...");
        let conn = Connection::open(&self.sqlite_path)?;
        emit_log(app, LogLevel::Success, "成功连接到 SQLite 数据库");
        
        // 创建 ItemTable 表（如果不存在）
        emit_log(app, LogLevel::Info, "正在确保 ItemTable 表存在...");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ItemTable (
                key TEXT PRIMARY KEY,
                value TEXT
            )",
            [],
        )?;
        emit_log(app, LogLevel::Success, "ItemTable 表已就绪");
        
        // 更新所有 ID
        emit_log(app, LogLevel::Info, format!("正在更新 {} 个条目...", new_ids.len()));
        let mut updated_count = 0;
        for (key, value) in new_ids {
            // 读取旧值
            let old_value: Result<String, _> = conn.query_row(
                "SELECT value FROM ItemTable WHERE key = ?1",
                [key],
                |row| row.get(0)
            );
            
            if let Ok(old) = old_value {
                emit_log(app, LogLevel::Info, format!("【{}】旧值: {}", key, old));
            }
            
            conn.execute(
                "INSERT OR REPLACE INTO ItemTable (key, value) VALUES (?1, ?2)",
                [key, value],
            )?;
            emit_log(app, LogLevel::Success, format!("【{}】新值: {}", key, value));
            updated_count += 1;
        }
        
        emit_log(app, LogLevel::Success, format!("成功更新 {} 个条目到 SQLite 数据库", updated_count));
        
        Ok(())
    }
    
    /// 更新系统级ID
    pub fn update_system_ids(&self, app: &AppHandle, new_ids: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        emit_log(app, LogLevel::Info, "正在更新系统级 ID...");
        let system_manager = SystemIdManager::new();
        system_manager.update_system_ids(app, new_ids)?;
        emit_log(app, LogLevel::Success, "系统级 ID 更新完成");
        Ok(())
    }
    
    /// 修补应用程序文件
    pub fn patch_application_files(&self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        emit_log(app, LogLevel::Info, "正在修补应用程序文件...");
        let patcher = Patcher::new(self.cursor_path.clone());
        patcher.patch_files(app)?;
        emit_log(app, LogLevel::Success, "应用程序文件修补完成");
        Ok(())
    }
}

