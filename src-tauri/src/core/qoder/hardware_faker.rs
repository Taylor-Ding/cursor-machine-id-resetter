use std::fs;
use std::io;
use std::path::Path;
use serde_json::json;

/// 硬件指纹伪造器
pub struct HardwareFaker;

impl HardwareFaker {
    /// 创建假的硬件 ID 文件
    pub fn create_fake_hardware_ids(qoder_dir: &Path) -> io::Result<usize> {
        let hardware_files = vec![
            "cpu_id",
            "gpu_id",
            "memory_id",
            "board_serial",
            "bios_uuid",
        ];
        
        let mut created_count = 0;
        for file_name in hardware_files {
            let file_path = qoder_dir.join(file_name);
            let fake_id = uuid::Uuid::new_v4().to_string();
            fs::write(&file_path, fake_id)?;
            created_count += 1;
        }
        
        Ok(created_count)
    }
    
    /// 创建假的硬件检测 JSON 文件
    pub fn create_fake_hardware_detection(qoder_dir: &Path) -> io::Result<()> {
        let hardware_data = Self::generate_hardware_data();
        let hardware_json = serde_json::to_string_pretty(&hardware_data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        fs::write(qoder_dir.join("hardware_detection.json"), hardware_json)?;
        Ok(())
    }
    
    /// 创建假的设备能力文件
    pub fn create_fake_device_capabilities(qoder_dir: &Path) -> io::Result<()> {
        let capabilities = json!({
            "capabilities": [
                "gpu_acceleration",
                "hardware_video_decode",
                "webgl2",
                "hardware_video_encode",
                "av1_decode",
                "vp9_decode"
            ]
        });
        
        let capabilities_json = serde_json::to_string_pretty(&capabilities)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        fs::write(qoder_dir.join("device_capabilities.json"), capabilities_json)?;
        Ok(())
    }
    
    /// 创建假的系统特性文件
    pub fn create_fake_system_features(qoder_dir: &Path) -> io::Result<()> {
        let features = json!({
            "features": [
                "avx2",
                "sse4",
                "aes_ni",
                "virtualization",
                "tpm",
                "secure_boot"
            ]
        });
        
        let features_json = serde_json::to_string_pretty(&features)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        fs::write(qoder_dir.join("system_features.json"), features_json)?;
        Ok(())
    }
    
    /// 生成假的硬件数据（根据平台）
    fn generate_hardware_data() -> serde_json::Value {
        #[cfg(target_os = "macos")]
        {
            json!({
                "cpu": {
                    "name": "Apple M3 Pro",
                    "cores": 12,
                    "threads": 12,
                    "frequency": "3.2GHz"
                },
                "gpu": {
                    "name": "Apple M3 Pro GPU",
                    "memory": "24GB",
                    "cores": 19
                },
                "memory": {
                    "total": "32GB",
                    "type": "LPDDR5",
                    "speed": "7467MT/s"
                }
            })
        }
        
        #[cfg(target_os = "windows")]
        {
            json!({
                "cpu": {
                    "name": "Intel Core i7-13700K",
                    "cores": 16,
                    "threads": 24,
                    "frequency": "3.4GHz"
                },
                "gpu": {
                    "name": "NVIDIA GeForce RTX 4070",
                    "memory": "12GB",
                    "driver_version": "545.84"
                },
                "memory": {
                    "total": "32GB",
                    "type": "DDR5",
                    "speed": "5600MHz"
                }
            })
        }
        
        #[cfg(target_os = "linux")]
        {
            json!({
                "cpu": {
                    "name": "AMD Ryzen 7 7700X",
                    "cores": 8,
                    "threads": 16,
                    "frequency": "3.8GHz"
                },
                "gpu": {
                    "name": "NVIDIA GeForce RTX 4060",
                    "memory": "8GB",
                    "cores": 1024
                },
                "memory": {
                    "total": "32GB",
                    "type": "DDR5",
                    "speed": "4800MHz"
                }
            })
        }
    }
    
    /// 创建所有假的硬件文件
    pub fn create_all_fake_hardware(qoder_dir: &Path) -> io::Result<()> {
        Self::create_fake_hardware_ids(qoder_dir)?;
        Self::create_fake_hardware_detection(qoder_dir)?;
        Self::create_fake_device_capabilities(qoder_dir)?;
        Self::create_fake_system_features(qoder_dir)?;
        Ok(())
    }
}
