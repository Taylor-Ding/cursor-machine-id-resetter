use std::path::PathBuf;
use std::io::{self, ErrorKind};

/// Qoder 路径管理器
#[derive(Debug, Clone)]
pub struct QoderPaths {
    pub qoder_dir: PathBuf,
    pub machine_id_file: PathBuf,
    pub storage_json: PathBuf,
    pub user_dir: PathBuf,
    pub global_storage_dir: PathBuf,
}

impl QoderPaths {
    /// 创建新的 Qoder 路径管理器
    pub fn new() -> io::Result<Self> {
        let qoder_dir = Self::get_qoder_dir()?;
        
        let machine_id_file = qoder_dir.join("machineid");
        let user_dir = qoder_dir.join("User");
        let global_storage_dir = user_dir.join("globalStorage");
        let storage_json = global_storage_dir.join("storage.json");
        
        Ok(Self {
            qoder_dir,
            machine_id_file,
            storage_json,
            user_dir,
            global_storage_dir,
        })
    }
    
    /// 获取 Qoder 数据目录（跨平台）
    fn get_qoder_dir() -> io::Result<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            if let Some(appdata) = std::env::var_os("APPDATA") {
                return Ok(PathBuf::from(appdata).join("Qoder"));
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            if let Some(home) = dirs::home_dir() {
                return Ok(home.join("Library/Application Support/Qoder"));
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if let Some(config) = dirs::config_dir() {
                return Ok(config.join("Qoder"));
            }
        }
        
        Err(io::Error::new(
            ErrorKind::NotFound,
            "无法确定 Qoder 数据目录"
        ))
    }
    
    /// 检查 Qoder 是否已安装
    pub fn is_qoder_installed(&self) -> bool {
        self.qoder_dir.exists()
    }
    
    /// 获取缓存目录列表
    pub fn get_cache_dirs(&self) -> Vec<PathBuf> {
        vec![
            self.qoder_dir.join("Cache"),
            self.qoder_dir.join("blob_storage"),
            self.qoder_dir.join("Code Cache"),
            self.qoder_dir.join("GPUCache"),
            self.qoder_dir.join("DawnGraphiteCache"),
            self.qoder_dir.join("DawnWebGPUCache"),
            self.qoder_dir.join("ShaderCache"),
            self.qoder_dir.join("CachedData"),
            self.qoder_dir.join("CachedProfilesData"),
            self.qoder_dir.join("CachedExtensions"),
            self.qoder_dir.join("IndexedDB"),
            self.qoder_dir.join("CacheStorage"),
            self.qoder_dir.join("WebSQL"),
            self.qoder_dir.join("Dictionaries"),
            self.qoder_dir.join("DawnCache"),
            self.qoder_dir.join("MediaCache"),
            self.qoder_dir.join("MetadataCache"),
            self.qoder_dir.join("ThumbnailCache"),
            self.qoder_dir.join("SharedClientCache"),
        ]
    }
    
    /// 获取身份文件列表
    pub fn get_identity_files(&self) -> Vec<PathBuf> {
        vec![
            // 网络和传输相关
            self.qoder_dir.join("Network Persistent State"),
            self.qoder_dir.join("TransportSecurity"),
            self.qoder_dir.join("Trust Tokens"),
            self.qoder_dir.join("Trust Tokens-journal"),
            
            // 存储相关
            self.qoder_dir.join("SharedStorage"),
            self.qoder_dir.join("SharedStorage-wal"),
            self.qoder_dir.join("Local Storage"),
            self.qoder_dir.join("Session Storage"),
            self.qoder_dir.join("WebStorage"),
            self.qoder_dir.join("Shared Dictionary"),
            
            // 偏好设置
            self.qoder_dir.join("Preferences"),
            self.qoder_dir.join("Secure Preferences"),
            self.qoder_dir.join("Local State"),
            
            // 设备和硬件信息
            self.qoder_dir.join("DeviceMetadata"),
            self.qoder_dir.join("HardwareInfo"),
            self.qoder_dir.join("SystemInfo"),
            
            // Cookie 和 Web 数据
            self.qoder_dir.join("Cookies"),
            self.qoder_dir.join("Cookies-journal"),
            self.qoder_dir.join("Web Data"),
            self.qoder_dir.join("Web Data-journal"),
            
            // 登录数据
            self.qoder_dir.join("Login Credentials"),
            self.qoder_dir.join("Login Data"),
            self.qoder_dir.join("Login Data-journal"),
            
            // 浏览器功能相关（新增）
            self.qoder_dir.join("AutofillStrikeDatabase"),
            self.qoder_dir.join("AutofillStrikeDatabase-journal"),
            self.qoder_dir.join("Feature Engagement Tracker"),
            self.qoder_dir.join("Platform Notifications"),
            self.qoder_dir.join("VideoDecodeStats"),
            self.qoder_dir.join("OriginTrials"),
            self.qoder_dir.join("BrowserMetrics"),
            self.qoder_dir.join("SafeBrowsing"),
            self.qoder_dir.join("QuotaManager"),
            self.qoder_dir.join("QuotaManager-journal"),
            self.qoder_dir.join("Network Action Predictor"),
        ]
    }
    
    /// 获取需要保留的 MCP 配置文件
    pub fn get_preserve_files(&self) -> Vec<PathBuf> {
        vec![
            self.qoder_dir.join("SharedClientCache/mcp.json"),
            self.qoder_dir.join("SharedClientCache/extension/local/mcp.json"),
        ]
    }
    
    /// 获取需要保留的对话目录
    pub fn get_preserve_dirs(&self) -> Vec<String> {
        vec![
            "User/workspaceStorage".to_string(),
            "User/History".to_string(),
        ]
    }
}
