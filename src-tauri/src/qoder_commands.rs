use crate::core::qoder::QoderResetter;
use crate::utils::logger::{emit_log, LogLevel};
use tauri::AppHandle;

/// 获取 Qoder 信息
#[tauri::command]
pub async fn get_qoder_info(app: AppHandle) -> Result<serde_json::Value, String> {
    emit_log(&app, LogLevel::Info, "开始获取 Qoder 信息...");
    
    let resetter = QoderResetter::new().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("初始化 Qoder 重置器失败: {}", e));
        e.to_string()
    })?;
    
    let info = resetter.get_info();
    emit_log(&app, LogLevel::Success, format!("成功获取 Qoder 信息"));
    
    serde_json::to_value(&info).map_err(|e| e.to_string())
}

/// 检查 Qoder 状态
#[tauri::command]
pub async fn check_qoder_status(app: AppHandle) -> Result<serde_json::Value, String> {
    emit_log(&app, LogLevel::Info, "检查 Qoder 运行状态...");
    
    let (is_running, processes) = QoderResetter::check_status();
    
    let status = serde_json::json!({
        "is_running": is_running,
        "process_count": processes.len(),
        "processes": processes,
    });
    
    if is_running {
        emit_log(&app, LogLevel::Warning, format!("Qoder 正在运行，共 {} 个进程", processes.len()));
    } else {
        emit_log(&app, LogLevel::Success, "Qoder 未运行");
    }
    
    Ok(status)
}

/// 关闭 Qoder
#[tauri::command]
pub async fn quit_qoder(app: AppHandle) -> Result<bool, String> {
    emit_log(&app, LogLevel::Info, "正在尝试关闭 Qoder...");
    
    match QoderResetter::quit_qoder() {
        Ok(success) => {
            if success {
                emit_log(&app, LogLevel::Success, "Qoder 已成功关闭");
            } else {
                emit_log(&app, LogLevel::Warning, "Qoder 关闭失败或未运行");
            }
            Ok(success)
        }
        Err(e) => {
            emit_log(&app, LogLevel::Error, format!("关闭 Qoder 时发生错误: {}", e));
            Err(e.to_string())
        }
    }
}

/// 完整重置（清理 + 重置机器 ID）
#[tauri::command]
pub async fn reset_qoder_full(app: AppHandle) -> Result<serde_json::Value, String> {
    emit_log(&app, LogLevel::Info, "========== 开始 Qoder 完整重置 ==========");
    
    let resetter = QoderResetter::new().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("初始化重置器失败: {}", e));
        e.to_string()
    })?;
    
    emit_log(&app, LogLevel::Info, "正在执行深度清理...");
    
    let result = resetter.reset_full().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("重置失败: {}", e));
        e.to_string()
    })?;
    
    if result.success {
        emit_log(&app, LogLevel::Success, format!("🎉 重置成功！新机器 ID: {}", result.new_machine_id));
        
        if let Some(ref stats) = result.stats {
            emit_log(&app, LogLevel::Info, format!("📊 清理统计:"));
            emit_log(&app, LogLevel::Info, format!("  - 删除文件: {}", stats.files_removed));
            emit_log(&app, LogLevel::Info, format!("  - 删除目录: {}", stats.dirs_removed));
            emit_log(&app, LogLevel::Info, format!("  - 清理遥测键: {}", stats.telemetry_keys_cleared));
            
            if !stats.errors.is_empty() {
                emit_log(&app, LogLevel::Warning, format!("  - 遇到 {} 个错误", stats.errors.len()));
                for error in &stats.errors {
                    emit_log(&app, LogLevel::Warning, format!("    • {}", error));
                }
            }
        }
        
        emit_log(&app, LogLevel::Success, "========== 重置完成 ==========");
    } else {
        if let Some(ref error) = result.error {
            emit_log(&app, LogLevel::Error, format!("❌ 重置失败: {}", error));
        }
    }
    
    serde_json::to_value(&result).map_err(|e| e.to_string())
}

/// 仅重置机器 ID
#[tauri::command]
pub async fn reset_qoder_machine_id(app: AppHandle) -> Result<serde_json::Value, String> {
    emit_log(&app, LogLevel::Info, "开始重置 Qoder 机器 ID...");
    
    let resetter = QoderResetter::new().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("初始化重置器失败: {}", e));
        e.to_string()
    })?;
    
    let result = resetter.reset_machine_id_only().map_err(|e| {
        emit_log(&app, LogLevel::Error, format!("重置失败: {}", e));
        e.to_string()
    })?;
    
    if result.success {
        emit_log(&app, LogLevel::Success, format!("✅ 机器 ID 已重置: {}", result.new_machine_id));
    } else {
        if let Some(ref error) = result.error {
            emit_log(&app, LogLevel::Error, format!("❌ 重置失败: {}", error));
        }
    }
    
    serde_json::to_value(&result).map_err(|e| e.to_string())
}

