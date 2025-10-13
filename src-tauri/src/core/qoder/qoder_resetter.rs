use std::io;
use std::fs;
use serde_json::{json, Value};
use crate::core::qoder::{
    qoder_paths::QoderPaths,
    qoder_process::QoderProcess,
    qoder_cleaner::{QoderCleaner, CleanupStats},
    hardware_faker::HardwareFaker,
};

/// Qoder 主重置器
pub struct QoderResetter {
    paths: QoderPaths,
    cleaner: QoderCleaner,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QoderInfo {
    pub machine_id: String,
    pub is_running: bool,
    pub is_installed: bool,
    pub process_count: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResetResult {
    pub success: bool,
    pub new_machine_id: String,
    pub stats: Option<CleanupStatsSerializable>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CleanupStatsSerializable {
    pub files_removed: usize,
    pub dirs_removed: usize,
    pub telemetry_keys_cleared: usize,
    pub errors: Vec<String>,
}

impl From<CleanupStats> for CleanupStatsSerializable {
    fn from(stats: CleanupStats) -> Self {
        Self {
            files_removed: stats.files_removed,
            dirs_removed: stats.dirs_removed,
            telemetry_keys_cleared: stats.telemetry_keys_cleared,
            errors: stats.errors,
        }
    }
}

impl QoderResetter {
    /// 创建新的 Qoder 重置器
    pub fn new() -> io::Result<Self> {
        let paths = QoderPaths::new()?;
        let cleaner = QoderCleaner::new(paths.clone());
        
        Ok(Self {
            paths,
            cleaner,
        })
    }
    
    /// 获取 Qoder 信息
    pub fn get_info(&self) -> QoderInfo {
        let machine_id = self.cleaner.get_current_machine_id()
            .unwrap_or_else(|_| "未安装".to_string());
        
        let is_running = QoderProcess::is_running();
        let is_installed = self.paths.is_qoder_installed();
        let process_count = QoderProcess::get_pids().len();
        
        QoderInfo {
            machine_id,
            is_running,
            is_installed,
            process_count,
        }
    }
    
    /// 执行完整重置（需要 Qoder 关闭）
    pub fn reset_full(&self) -> io::Result<ResetResult> {
        // 检查 Qoder 是否在运行
        if QoderProcess::is_running() {
            return Ok(ResetResult {
                success: false,
                new_machine_id: String::new(),
                stats: None,
                error: Some("Qoder 正在运行，请先关闭 Qoder".to_string()),
            });
        }
        
        // 执行深度清理
        let stats = self.cleaner.cleanup_all()?;
        
        // 重置机器 ID
        let new_machine_id = self.cleaner.reset_machine_id()?;
        
        // 创建额外的 ID 文件
        self.create_additional_ids()?;
        
        // 更新 storage.json 的遥测和系统信息
        self.update_storage_json_comprehensive(&new_machine_id)?;
        
        // 创建假的硬件文件
        if let Err(e) = HardwareFaker::create_all_fake_hardware(&self.paths.qoder_dir) {
            // 硬件伪造失败不影响整体重置，只记录错误
            eprintln!("Warning: 创建硬件伪造文件失败: {}", e);
        }
        
        Ok(ResetResult {
            success: true,
            new_machine_id,
            stats: Some(stats.into()),
            error: None,
        })
    }
    
    /// 仅重置机器 ID（不清理）
    pub fn reset_machine_id_only(&self) -> io::Result<ResetResult> {
        if QoderProcess::is_running() {
            return Ok(ResetResult {
                success: false,
                new_machine_id: String::new(),
                stats: None,
                error: Some("Qoder 正在运行，请先关闭 Qoder".to_string()),
            });
        }
        
        let new_machine_id = self.cleaner.reset_machine_id()?;
        
        Ok(ResetResult {
            success: true,
            new_machine_id,
            stats: None,
            error: None,
        })
    }
    
    /// 创建额外的 ID 文件
    fn create_additional_ids(&self) -> io::Result<()> {
        let extra_ids = vec![
            "deviceid",
            "hardware_uuid",
            "system_uuid",
            "platform_id",
            "installation_id",
        ];
        
        for id_file in extra_ids {
            let file_path = self.paths.qoder_dir.join(id_file);
            let new_id = uuid::Uuid::new_v4().to_string();
            fs::write(&file_path, new_id)?;
        }
        
        Ok(())
    }
    
    /// 全面更新 storage.json（遥测 + 系统信息）
    fn update_storage_json_comprehensive(&self, machine_id: &str) -> io::Result<()> {
        if !self.paths.storage_json.exists() {
            // 如果文件不存在，创建空的 JSON 对象
            fs::create_dir_all(self.paths.storage_json.parent().unwrap())?;
            fs::write(&self.paths.storage_json, "{}")?;
        }
        
        let content = fs::read_to_string(&self.paths.storage_json)?;
        let mut data: Value = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        // 生成新的 UUID
        let sha256_machine_id = format!("{:x}", md5::compute(machine_id));
        
        if let Some(obj) = data.as_object_mut() {
            // 遥测数据
            obj.insert("telemetry.machineId".to_string(), json!(sha256_machine_id));
            obj.insert("telemetry.devDeviceId".to_string(), json!(uuid::Uuid::new_v4().to_string()));
            obj.insert("telemetry.sessionId".to_string(), json!(uuid::Uuid::new_v4().to_string()));
            obj.insert("telemetry.installationId".to_string(), json!(uuid::Uuid::new_v4().to_string()));
            obj.insert("telemetry.clientId".to_string(), json!(uuid::Uuid::new_v4().to_string()));
            obj.insert("telemetry.userId".to_string(), json!(uuid::Uuid::new_v4().to_string()));
            obj.insert("telemetry.anonymousId".to_string(), json!(uuid::Uuid::new_v4().to_string()));
            obj.insert("telemetry.sqmId".to_string(), json!(uuid::Uuid::new_v4().to_string()));
            
            // 硬件 ID
            obj.insert("hardwareId".to_string(), json!(uuid::Uuid::new_v4().to_string()));
            obj.insert("platformId".to_string(), json!(uuid::Uuid::new_v4().to_string()));
            obj.insert("cpuId".to_string(), json!(uuid::Uuid::new_v4().to_string()));
            obj.insert("gpuId".to_string(), json!(uuid::Uuid::new_v4().to_string()));
            obj.insert("memoryId".to_string(), json!(uuid::Uuid::new_v4().to_string()));
            
            // 系统信息（根据平台）
            #[cfg(target_os = "macos")]
            {
                obj.insert("system.platform".to_string(), json!("macos"));
                obj.insert("system.arch".to_string(), json!("arm64"));
                obj.insert("system.version".to_string(), json!("14.0.0"));
                obj.insert("system.build".to_string(), json!("23A344"));
                obj.insert("system.locale".to_string(), json!("en-US"));
                obj.insert("system.timezone".to_string(), json!("America/New_York"));
            }
            
            #[cfg(target_os = "windows")]
            {
                obj.insert("system.platform".to_string(), json!("windows"));
                obj.insert("system.arch".to_string(), json!("x64"));
                obj.insert("system.version".to_string(), json!("10.0.22621"));
                obj.insert("system.build".to_string(), json!("22621"));
                obj.insert("system.locale".to_string(), json!("en-US"));
                obj.insert("system.timezone".to_string(), json!("Eastern Standard Time"));
            }
            
            #[cfg(target_os = "linux")]
            {
                obj.insert("system.platform".to_string(), json!("linux"));
                obj.insert("system.arch".to_string(), json!("x64"));
                obj.insert("system.version".to_string(), json!("6.0.0"));
                obj.insert("system.build".to_string(), json!("6.0.0"));
                obj.insert("system.locale".to_string(), json!("en-US"));
                obj.insert("system.timezone".to_string(), json!("UTC"));
            }
        }
        
        // 写回文件
        let updated_content = serde_json::to_string_pretty(&data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&self.paths.storage_json, updated_content)?;
        
        Ok(())
    }
    
    /// 尝试关闭 Qoder
    pub fn quit_qoder() -> io::Result<bool> {
        QoderProcess::try_quit()
    }
    
    /// 检查 Qoder 状态
    pub fn check_status() -> (bool, Vec<(u32, String)>) {
        let is_running = QoderProcess::is_running();
        let processes = QoderProcess::get_process_info();
        (is_running, processes)
    }
}
