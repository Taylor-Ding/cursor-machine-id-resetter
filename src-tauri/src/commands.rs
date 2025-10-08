use crate::core::{
    id_generator::generate_machine_ids,
    resetter::MachineIdResetter,
    backup::BackupManager,
    settings::SettingsManager,
    cursor_quitter::quit_cursor_default,
    email_generator::{EmailGenerator, AccountInfo},
};
use crate::utils::logger::{emit_log, LogLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineInfo {
    pub machine_id: String,
    pub cursor_version: String,
    pub backup_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CursorStatus {
    pub is_running: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetResult {
    pub success: bool,
    pub message: String,
    pub new_ids: HashMap<String, String>,
}

#[tauri::command]
pub async fn get_machine_info(app: AppHandle) -> Result<MachineInfo, String> {
    emit_log(&app, LogLevel::Info, "开始获取机器信息...");
    
    let resetter = MachineIdResetter::new().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("初始化重置器失败: {}", e));
        e.to_string()
    })?;
    
    let machine_id = resetter.get_current_machine_id().unwrap_or_else(|| "Unknown".to_string());
    emit_log(&app, LogLevel::Info, format!("当前机器 ID: {}", machine_id));
    
    let cursor_version = resetter.get_cursor_version().unwrap_or_else(|| "Unknown".to_string());
    emit_log(&app, LogLevel::Info, format!("Cursor 版本: {}", cursor_version));
    
    let backup_manager = BackupManager::new().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("初始化备份管理器失败: {}", e));
        e.to_string()
    })?;
    
    let backups = backup_manager.list_backups().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("获取备份列表失败: {}", e));
        e.to_string()
    })?;
    
    emit_log(&app, LogLevel::Success, format!("成功获取机器信息，共有 {} 个备份", backups.len()));
    
    Ok(MachineInfo {
        machine_id,
        cursor_version,
        backup_count: backups.len(),
    })
}

#[tauri::command]
pub async fn reset_machine_id(app: AppHandle, options: Vec<String>) -> Result<ResetResult, String> {
    emit_log(&app, LogLevel::Info, "========== 开始机器 ID 重置流程 ==========");
    emit_log(&app, LogLevel::Info, format!("重置选项: {:?}", options));
    
    // 初始化重置器
    emit_log(&app, LogLevel::Info, "正在初始化重置器...");
    let mut resetter = MachineIdResetter::new().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("初始化重置器失败: {}", e));
        e.to_string()
    })?;
    emit_log(&app, LogLevel::Success, "重置器初始化成功");
    
    // 创建备份
    emit_log(&app, LogLevel::Info, "正在创建备份...");
    let backup_manager = BackupManager::new().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("初始化备份管理器失败: {}", e));
        e.to_string()
    })?;
    
    match backup_manager.create_backup() {
        Ok(backup_id) => {
            emit_log(&app, LogLevel::Success, format!("备份创建成功: {}", backup_id));
        }
        Err(e) => {
            emit_log(&app, LogLevel::Error, format!("创建备份失败: {}", e));
            return Err(e.to_string());
        }
    }
    
    // 获取旧的 ID（用于对比）
    let old_machine_id = resetter.get_current_machine_id().unwrap_or_else(|| "未知".to_string());
    emit_log(&app, LogLevel::Info, format!("旧的机器 ID: {}", old_machine_id));
    
    // 生成新ID
    emit_log(&app, LogLevel::Info, "正在生成新的机器 ID...");
    let new_ids = generate_machine_ids();
    emit_log(&app, LogLevel::Success, format!("成功生成 {} 个新 ID", new_ids.len()));
    
    emit_log(&app, LogLevel::Info, "========== 新生成的 ID 详细信息 ==========");
    for (key, value) in &new_ids {
        emit_log(&app, LogLevel::Success, format!("【{}】", key));
        emit_log(&app, LogLevel::Info, format!("  值: {}", value));
    }
    emit_log(&app, LogLevel::Info, "=========================================");
    
    // 执行重置
    let mut success = true;
    let mut messages: Vec<String> = Vec::new();
    
    if options.contains(&"storage".to_string()) {
        emit_log(&app, LogLevel::Info, "正在更新 storage.json...");
        match resetter.update_storage_json(&app, &new_ids) {
            Ok(_) => {
                emit_log(&app, LogLevel::Success, "storage.json 更新成功");
                messages.push("Storage updated successfully".to_string());
            }
            Err(e) => {
                success = false;
                let error_msg = format!("Failed to update storage: {}", e);
                emit_log(&app, LogLevel::Error, &error_msg);
                messages.push(error_msg);
            }
        }
    }
    
    if options.contains(&"sqlite".to_string()) {
        emit_log(&app, LogLevel::Info, "正在更新 SQLite 数据库...");
        match resetter.update_sqlite_db(&app, &new_ids) {
            Ok(_) => {
                emit_log(&app, LogLevel::Success, "SQLite 数据库更新成功");
                messages.push("SQLite database updated successfully".to_string());
            }
            Err(e) => {
                success = false;
                let error_msg = format!("Failed to update SQLite: {}", e);
                emit_log(&app, LogLevel::Error, &error_msg);
                messages.push(error_msg);
            }
        }
    }
    
    if options.contains(&"system".to_string()) {
        emit_log(&app, LogLevel::Info, "正在更新系统 ID...");
        match resetter.update_system_ids(&app, &new_ids) {
            Ok(_) => {
                emit_log(&app, LogLevel::Success, "系统 ID 更新成功（可能跳过了需要管理员权限的操作）");
                messages.push("System IDs updated (some operations may require admin privileges)".to_string());
            }
            Err(e) => {
                emit_log(&app, LogLevel::Warning, format!("系统 ID 更新遇到问题: {}", e));
                emit_log(&app, LogLevel::Info, "系统 ID 更新不是必需的，可以继续其他操作");
                messages.push(format!("System IDs update had issues: {}", e));
            }
        }
    }
    
    if options.contains(&"patch".to_string()) {
        emit_log(&app, LogLevel::Info, "正在修补应用程序文件...");
        match resetter.patch_application_files(&app) {
            Ok(_) => {
                emit_log(&app, LogLevel::Success, "应用程序文件修补成功（可能跳过了需要管理员权限的操作）");
                messages.push("Application files patched (some operations may require admin privileges)".to_string());
            }
            Err(e) => {
                emit_log(&app, LogLevel::Warning, format!("文件修补遇到问题: {}", e));
                emit_log(&app, LogLevel::Info, "文件修补不是必需的，可以继续其他操作");
                messages.push(format!("File patching had issues: {}", e));
            }
        }
    }
    
    if success {
        emit_log(&app, LogLevel::Success, "========== 机器 ID 重置完成 ==========");
    } else {
        emit_log(&app, LogLevel::Error, "========== 机器 ID 重置失败 ==========");
    }
    
    Ok(ResetResult {
        success,
        message: messages.join("; "),
        new_ids: new_ids.into_iter().collect(),
    })
}

#[tauri::command]
pub async fn check_cursor_status(app: AppHandle) -> Result<CursorStatus, String> {
    emit_log(&app, LogLevel::Info, "正在检查 Cursor 运行状态...");
    
    let resetter = MachineIdResetter::new().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("初始化重置器失败: {}", e));
        e.to_string()
    })?;
    
    let is_running = resetter.is_cursor_running();
    
    if is_running {
        emit_log(&app, LogLevel::Warning, "Cursor 正在运行，请先关闭");
    } else {
        emit_log(&app, LogLevel::Success, "Cursor 未运行");
    }
    
    Ok(CursorStatus {
        is_running,
        message: if is_running {
            "Cursor is currently running. Please close it before resetting.".to_string()
        } else {
            "Cursor is not running.".to_string()
        },
    })
}

#[tauri::command]
pub async fn get_backups(app: AppHandle) -> Result<Vec<serde_json::Value>, String> {
    emit_log(&app, LogLevel::Info, "正在获取备份列表...");
    
    let backup_manager = BackupManager::new().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("初始化备份管理器失败: {}", e));
        e.to_string()
    })?;
    
    let backups = backup_manager.list_backups().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("获取备份列表失败: {}", e));
        e.to_string()
    })?;
    
    emit_log(&app, LogLevel::Success, format!("成功获取 {} 个备份", backups.len()));
    Ok(backups)
}

#[tauri::command]
pub async fn restore_from_backup(app: AppHandle, backup_id: String) -> Result<String, String> {
    emit_log(&app, LogLevel::Info, format!("正在恢复备份: {}", backup_id));
    
    let backup_manager = BackupManager::new().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("初始化备份管理器失败: {}", e));
        e.to_string()
    })?;
    
    backup_manager.restore_backup(&app, &backup_id).map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("恢复备份失败: {}", e));
        e.to_string()
    })?;
    
    emit_log(&app, LogLevel::Success, format!("备份恢复成功: {}", backup_id));
    Ok("Backup restored successfully".to_string())
}

#[tauri::command]
pub async fn delete_backup(app: AppHandle, backup_id: String) -> Result<String, String> {
    emit_log(&app, LogLevel::Info, format!("正在删除备份: {}", backup_id));
    
    let backup_manager = BackupManager::new().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("初始化备份管理器失败: {}", e));
        e.to_string()
    })?;
    
    backup_manager.delete_backup(&app, &backup_id).map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("删除备份失败: {}", e));
        e.to_string()
    })?;
    
    emit_log(&app, LogLevel::Success, format!("备份删除成功: {}", backup_id));
    Ok("Backup deleted successfully".to_string())
}

#[tauri::command]
pub async fn get_settings() -> Result<serde_json::Value, String> {
    let settings_manager = SettingsManager::new().map_err(|e| e.to_string())?;
    settings_manager.load_settings().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_settings(settings: serde_json::Value) -> Result<String, String> {
    let settings_manager = SettingsManager::new().map_err(|e| e.to_string())?;
    settings_manager.save_settings(&settings).map_err(|e| e.to_string())?;
    Ok("Settings saved successfully".to_string())
}

#[tauri::command]
pub async fn get_default_settings() -> Result<serde_json::Value, String> {
    let settings_manager = SettingsManager::new().map_err(|e| e.to_string())?;
    Ok(settings_manager.get_default_settings())
}

#[tauri::command]
pub async fn quit_cursor(app: AppHandle) -> Result<bool, String> {
    emit_log(&app, LogLevel::Info, "========== 开始关闭 Cursor IDE ==========");
    
    match quit_cursor_default(&app) {
        Ok(success) => {
            if success {
                emit_log(&app, LogLevel::Success, "========== Cursor IDE 已成功关闭 ==========");
            }
            Ok(success)
        }
        Err(e) => {
            emit_log(&app, LogLevel::Error, format!("关闭 Cursor IDE 失败: {}", e));
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn generate_account(
    app: AppHandle,
    domain: String,
    password_length: Option<usize>,
    timestamp_length: Option<usize>,
) -> Result<AccountInfo, String> {
    emit_log(&app, LogLevel::Info, "========== 开始生成账号信息 ==========");
    emit_log(&app, LogLevel::Info, format!("域名: {}", domain));
    
    if domain.is_empty() {
        emit_log(&app, LogLevel::Error, "域名不能为空");
        return Err("域名不能为空".to_string());
    }
    
    let generator = EmailGenerator::new(domain.clone()).map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("初始化邮箱生成器失败: {}", e));
        e.to_string()
    })?;
    
    let account_info = generator.generate_account_info(password_length, timestamp_length);
    
    emit_log(&app, LogLevel::Success, format!("生成邮箱: {}", account_info.email));
    emit_log(&app, LogLevel::Success, format!("名: {}", account_info.first_name));
    emit_log(&app, LogLevel::Success, format!("姓: {}", account_info.last_name));
    emit_log(&app, LogLevel::Info, format!("密码长度: {}", account_info.password.len()));
    emit_log(&app, LogLevel::Success, "========== 账号生成完成 ==========");
    
    Ok(account_info)
}

/// 恢复文件备份
#[tauri::command]
pub async fn restore_file_backup(app: AppHandle, file_type: String) -> Result<String, String> {
    emit_log(&app, LogLevel::Info, "========== 开始恢复备份 ==========");
    emit_log(&app, LogLevel::Info, format!("文件类型: {}", file_type));
    
    let resetter = MachineIdResetter::new().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("初始化重置器失败: {}", e));
        e.to_string()
    })?;
    
    let result = match file_type.as_str() {
        "storage" => {
            emit_log(&app, LogLevel::Info, "正在恢复 storage.json...");
            let storage_path = resetter.get_storage_path();
            resetter.restore_backup(&app, &storage_path)
                .map(|_| "storage.json 已成功从备份恢复".to_string())
        }
        "sqlite" => {
            emit_log(&app, LogLevel::Info, "正在恢复 state.vscdb...");
            let sqlite_path = resetter.get_sqlite_path();
            resetter.restore_backup(&app, &sqlite_path)
                .map(|_| "state.vscdb 已成功从备份恢复".to_string())
        }
        _ => {
            let error_msg = format!("未知的文件类型: {}", file_type);
            emit_log(&app, LogLevel::Error, &error_msg);
            Err(error_msg.into())
        }
    };
    
    match result {
        Ok(msg) => {
            emit_log(&app, LogLevel::Success, &msg);
            emit_log(&app, LogLevel::Success, "========== 备份恢复完成 ==========");
            Ok(msg)
        }
        Err(e) => {
            let error_msg = format!("恢复备份失败: {}", e);
            emit_log(&app, LogLevel::Error, &error_msg);
            emit_log(&app, LogLevel::Error, "========== 备份恢复失败 ==========");
            Err(error_msg)
        }
    }
}

