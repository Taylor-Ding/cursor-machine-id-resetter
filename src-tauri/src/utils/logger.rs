use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Serialize, Clone)]
pub struct LogMessage {
    pub level: String,
    pub message: String,
    pub timestamp: i64,
}

/// 日志级别
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Info => "info",
            LogLevel::Success => "success",
            LogLevel::Warning => "warning",
            LogLevel::Error => "error",
        }
    }
}

/// 发送日志到前端
pub fn emit_log(app: &AppHandle, level: LogLevel, message: impl Into<String>) {
    let log = LogMessage {
        level: level.as_str().to_string(),
        message: message.into(),
        timestamp: chrono::Local::now().timestamp_millis(),
    };
    
    // 同时在控制台打印
    match level {
        LogLevel::Info => eprintln!("ℹ️ {}", log.message),
        LogLevel::Success => eprintln!("✅ {}", log.message),
        LogLevel::Warning => eprintln!("⚠️ {}", log.message),
        LogLevel::Error => eprintln!("❌ {}", log.message),
    }
    
    // 发送到前端
    let _ = app.emit("log-message", log);
}

/// 辅助宏，用于快速记录日志
#[macro_export]
macro_rules! log_info {
    ($app:expr, $($arg:tt)*) => {
        $crate::utils::logger::emit_log($app, $crate::utils::logger::LogLevel::Info, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_success {
    ($app:expr, $($arg:tt)*) => {
        $crate::utils::logger::emit_log($app, $crate::utils::logger::LogLevel::Success, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warning {
    ($app:expr, $($arg:tt)*) => {
        $crate::utils::logger::emit_log($app, $crate::utils::logger::LogLevel::Warning, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($app:expr, $($arg:tt)*) => {
        $crate::utils::logger::emit_log($app, $crate::utils::logger::LogLevel::Error, format!($($arg)*))
    };
}

