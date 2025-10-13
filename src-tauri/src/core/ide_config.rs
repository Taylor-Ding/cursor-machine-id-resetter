use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDEConfig {
    pub name: String,
    pub display_name: String,
    pub process_names: Vec<String>,
    pub data_paths: Vec<PathBuf>,
    pub telemetry_keys: Vec<String>,
    pub session_keys: Vec<String>,
    pub database_keywords: Vec<String>,
    pub cache_table_patterns: Vec<String>,
    pub cache_directories: Vec<String>,
}

impl IDEConfig {
    /// 创建 Cursor 配置
    pub fn cursor() -> Self {
        Self {
            name: "cursor".to_string(),
            display_name: "Cursor".to_string(),
            process_names: vec![
                "cursor".to_string(),
                "cursor.exe".to_string(),
                "Cursor".to_string(),
            ],
            data_paths: Self::get_cursor_data_paths(),
            telemetry_keys: Self::get_telemetry_keys(),
            session_keys: Self::get_session_keys(),
            database_keywords: Self::get_database_keywords(),
            cache_table_patterns: Self::get_cache_table_patterns(),
            cache_directories: Self::get_cache_directories(),
        }
    }

    /// 创建 Windsurf 配置
    pub fn windsurf() -> Self {
        Self {
            name: "windsurf".to_string(),
            display_name: "Windsurf".to_string(),
            process_names: vec![
                "windsurf".to_string(),
                "windsurf.exe".to_string(),
                "Windsurf".to_string(),
            ],
            data_paths: Self::get_windsurf_data_paths(),
            telemetry_keys: Self::get_telemetry_keys(),
            session_keys: Self::get_session_keys(),
            database_keywords: Self::get_database_keywords(),
            cache_table_patterns: Self::get_cache_table_patterns(),
            cache_directories: Self::get_cache_directories(),
        }
    }

    /// 获取 Cursor 数据路径
    #[cfg(target_os = "macos")]
    fn get_cursor_data_paths() -> Vec<PathBuf> {
        let home = std::env::var("HOME").unwrap_or_default();
        vec![
            PathBuf::from(format!("{}/Library/Application Support/Cursor", home)),
            PathBuf::from(format!("{}/Library/Application Support/cursor-ai", home)),
        ]
    }

    #[cfg(target_os = "windows")]
    fn get_cursor_data_paths() -> Vec<PathBuf> {
        let appdata = std::env::var("APPDATA").unwrap_or_default();
        let localappdata = std::env::var("LOCALAPPDATA").unwrap_or_default();
        vec![
            PathBuf::from(format!("{}\\Cursor", appdata)),
            PathBuf::from(format!("{}\\Cursor", localappdata)),
            PathBuf::from(format!("{}\\cursor-ai", appdata)),
            PathBuf::from(format!("{}\\cursor-ai", localappdata)),
        ]
    }

    #[cfg(target_os = "linux")]
    fn get_cursor_data_paths() -> Vec<PathBuf> {
        let home = std::env::var("HOME").unwrap_or_default();
        vec![
            PathBuf::from(format!("{}/.config/Cursor", home)),
            PathBuf::from(format!("{}/.config/cursor-ai", home)),
        ]
    }

    /// 获取 Windsurf 数据路径
    #[cfg(target_os = "macos")]
    fn get_windsurf_data_paths() -> Vec<PathBuf> {
        let home = std::env::var("HOME").unwrap_or_default();
        vec![
            PathBuf::from(format!("{}/Library/Application Support/Windsurf", home)),
            PathBuf::from(format!("{}/Library/Application Support/windsurf-ai", home)),
            PathBuf::from(format!("{}/Library/Application Support/Codeium/Windsurf", home)),
        ]
    }

    #[cfg(target_os = "windows")]
    fn get_windsurf_data_paths() -> Vec<PathBuf> {
        let appdata = std::env::var("APPDATA").unwrap_or_default();
        let localappdata = std::env::var("LOCALAPPDATA").unwrap_or_default();
        vec![
            PathBuf::from(format!("{}\\Windsurf", appdata)),
            PathBuf::from(format!("{}\\Windsurf", localappdata)),
            PathBuf::from(format!("{}\\windsurf-ai", appdata)),
            PathBuf::from(format!("{}\\windsurf-ai", localappdata)),
            PathBuf::from(format!("{}\\Codeium\\Windsurf", appdata)),
            PathBuf::from(format!("{}\\Codeium\\Windsurf", localappdata)),
        ]
    }

    #[cfg(target_os = "linux")]
    fn get_windsurf_data_paths() -> Vec<PathBuf> {
        let home = std::env::var("HOME").unwrap_or_default();
        vec![
            PathBuf::from(format!("{}/.config/Windsurf", home)),
            PathBuf::from(format!("{}/.config/windsurf-ai", home)),
            PathBuf::from(format!("{}/.config/Codeium/Windsurf", home)),
        ]
    }

    /// 获取需要修改的 Telemetry 键
    fn get_telemetry_keys() -> Vec<String> {
        vec![
            "machineId".to_string(),
            "telemetry.machineId".to_string(),
            "telemetryMachineId".to_string(),
            "deviceId".to_string(),
            "telemetry.deviceId".to_string(),
            "lastSessionId".to_string(),
            "sessionId".to_string(),
            "installationId".to_string(),
            "sqmUserId".to_string(),
            "sqmMachineId".to_string(),
            "clientId".to_string(),
            "instanceId".to_string(),
        ]
    }

    /// 获取需要删除的 Session 键
    fn get_session_keys() -> Vec<String> {
        vec![
            "lastSessionDate".to_string(),
            "sessionStartTime".to_string(),
            "userSession".to_string(),
            "authToken".to_string(),
            "accessToken".to_string(),
            "refreshToken".to_string(),
            "bearerToken".to_string(),
            "apiKey".to_string(),
            "userToken".to_string(),
        ]
    }

    /// 获取数据库清理关键词
    fn get_database_keywords() -> Vec<String> {
        vec![
            "augment".to_string(),
            "account".to_string(),
            "session".to_string(),
            "user".to_string(),
            "login".to_string(),
            "auth".to_string(),
            "token".to_string(),
            "credential".to_string(),
            "profile".to_string(),
            "identity".to_string(),
        ]
    }

    /// 获取缓存表模式
    fn get_cache_table_patterns() -> Vec<String> {
        vec![
            "cache".to_string(),
            "session".to_string(),
            "temp".to_string(),
            "log".to_string(),
            "history".to_string(),
            "recent".to_string(),
            "workspace".to_string(),
            "project".to_string(),
        ]
    }

    /// 获取缓存目录列表
    fn get_cache_directories() -> Vec<String> {
        vec![
            "IndexedDB".to_string(),
            "Local Storage".to_string(),
            "Cache".to_string(),
            "Code Cache".to_string(),
            "GPUCache".to_string(),
            "blob_storage".to_string(),
            "logs".to_string(),
            "User/workspaceStorage".to_string(),
            "User/History".to_string(),
            "User/logs".to_string(),
            "CachedData".to_string(),
            "CachedExtensions".to_string(),
            "ShaderCache".to_string(),
            "WebStorage".to_string(),
        ]
    }

    /// 设置自定义数据路径（用户手动配置）
    pub fn set_custom_path(&mut self, custom_path: PathBuf) {
        // 将自定义路径添加到最前面，优先使用
        self.data_paths.insert(0, custom_path);
    }

    /// 获取实际存在的数据路径
    pub fn get_existing_data_path(&self) -> Option<PathBuf> {
        for path in &self.data_paths {
            if path.exists() {
                return Some(path.clone());
            }
        }
        None
    }

    /// 检查 IDE 是否已安装
    pub fn is_installed(&self) -> bool {
        self.get_existing_data_path().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_config() {
        let config = IDEConfig::cursor();
        assert_eq!(config.name, "cursor");
        assert_eq!(config.display_name, "Cursor");
        assert!(!config.process_names.is_empty());
        assert!(!config.telemetry_keys.is_empty());
    }

    #[test]
    fn test_windsurf_config() {
        let config = IDEConfig::windsurf();
        assert_eq!(config.name, "windsurf");
        assert_eq!(config.display_name, "Windsurf");
        assert!(!config.process_names.is_empty());
        assert!(!config.telemetry_keys.is_empty());
    }
}
