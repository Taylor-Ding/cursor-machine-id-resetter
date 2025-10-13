use std::fs;
use std::io;
use std::path::Path;
use serde_json::Value;
use crate::core::qoder::qoder_paths::QoderPaths;

/// Qoder 深度清理器
pub struct QoderCleaner {
    paths: QoderPaths,
}

#[derive(Debug, Clone)]
pub struct CleanupStats {
    pub files_removed: usize,
    pub dirs_removed: usize,
    pub telemetry_keys_cleared: usize,
    pub errors: Vec<String>,
}

impl QoderCleaner {
    /// 创建新的清理器
    pub fn new(paths: QoderPaths) -> Self {
        Self { paths }
    }
    
    /// 执行完整清理
    pub fn cleanup_all(&self) -> io::Result<CleanupStats> {
        let mut stats = CleanupStats {
            files_removed: 0,
            dirs_removed: 0,
            telemetry_keys_cleared: 0,
            errors: Vec::new(),
        };
        
        // 1. 清理额外的机器 ID 文件（保留主 machineid）
        self.cleanup_extra_machine_ids(&mut stats);
        
        // 2. 清理遥测数据
        if let Err(e) = self.cleanup_telemetry(&mut stats) {
            stats.errors.push(format!("遥测数据清理失败: {}", e));
        }
        
        // 3. 清理缓存
        self.cleanup_caches(&mut stats);
        
        // 4. 清理身份文件
        self.cleanup_identity_files(&mut stats);
        
        // 5. 清理硬件指纹
        self.cleanup_hardware_fingerprints(&mut stats);
        
        Ok(stats)
    }
    
    /// 清理额外的机器 ID 文件
    fn cleanup_extra_machine_ids(&self, stats: &mut CleanupStats) {
        let extra_ids = vec![
            "deviceid", "hardware_uuid", "system_uuid",
            "platform_id", "installation_id", "cpu_id", "gpu_id"
        ];
        
        for id_file in extra_ids {
            let file_path = self.paths.qoder_dir.join(id_file);
            if file_path.exists() {
                match fs::remove_file(&file_path) {
                    Ok(_) => stats.files_removed += 1,
                    Err(e) => stats.errors.push(format!("删除 {} 失败: {}", id_file, e)),
                }
            }
        }
    }
    
    /// 清理遥测数据
    fn cleanup_telemetry(&self, stats: &mut CleanupStats) -> io::Result<()> {
        if !self.paths.storage_json.exists() {
            return Ok(());
        }
        
        // 读取 storage.json
        let content = fs::read_to_string(&self.paths.storage_json)?;
        let mut data: Value = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        // 需要删除的遥测键
        let telemetry_keys = vec![
            "telemetry.machineId",
            "telemetry.devDeviceId",
            "telemetry.sessionId",
            "telemetry.installationId",
            "telemetry.clientId",
            "telemetry.userId",
            "telemetry.anonymousId",
            "telemetry.sqmId",
            "machineId",
            "deviceId",
            "installationId",
            "hardwareId",
            "platformId",
            "system.platform",
            "system.arch",
            "system.version",
            "system.timezone",
        ];
        
        if let Some(obj) = data.as_object_mut() {
            for key in telemetry_keys {
                if obj.remove(key).is_some() {
                    stats.telemetry_keys_cleared += 1;
                }
            }
        }
        
        // 写回文件
        let updated_content = serde_json::to_string_pretty(&data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&self.paths.storage_json, updated_content)?;
        
        Ok(())
    }
    
    /// 清理缓存目录
    fn cleanup_caches(&self, stats: &mut CleanupStats) {
        let cache_dirs = self.paths.get_cache_dirs();
        
        for cache_dir in cache_dirs {
            if cache_dir.exists() {
                // 特殊处理 SharedClientCache（保留 MCP 配置）
                if cache_dir.file_name().and_then(|n| n.to_str()) == Some("SharedClientCache") {
                    self.cleanup_shared_cache_selective(&cache_dir, stats);
                    continue;
                }
                
                match fs::remove_dir_all(&cache_dir) {
                    Ok(_) => stats.dirs_removed += 1,
                    Err(e) => stats.errors.push(format!("删除缓存目录 {:?} 失败: {}", cache_dir, e)),
                }
            }
        }
    }
    
    /// 选择性清理 SharedClientCache
    fn cleanup_shared_cache_selective(&self, cache_dir: &Path, stats: &mut CleanupStats) {
        // 删除特定文件（包括 mcp.json，与官方脚本一致）
        let files_to_remove = vec![".info", ".lock", "mcp.json", "auth.json", "server.json"];
        
        for file in files_to_remove {
            let file_path = cache_dir.join(file);
            if file_path.exists() {
                match fs::remove_file(&file_path) {
                    Ok(_) => stats.files_removed += 1,
                    Err(e) => stats.errors.push(format!("删除 SharedClientCache/{} 失败: {}", file, e)),
                }
            }
        }
        
        // 清理临时文件（tmp*）
        if let Ok(entries) = fs::read_dir(cache_dir) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if file_name.starts_with("tmp") {
                        if let Err(e) = fs::remove_file(entry.path()) {
                            stats.errors.push(format!("删除临时文件 {} 失败: {}", file_name, e));
                        } else {
                            stats.files_removed += 1;
                        }
                    }
                }
            }
        }
        
        // 删除 cache 子目录
        let cache_subdir = cache_dir.join("cache");
        if cache_subdir.exists() {
            match fs::remove_dir_all(&cache_subdir) {
                Ok(_) => stats.dirs_removed += 1,
                Err(e) => stats.errors.push(format!("删除 SharedClientCache/cache 失败: {}", e)),
            }
        }
    }
    
    /// 清理身份文件
    fn cleanup_identity_files(&self, stats: &mut CleanupStats) {
        let identity_files = self.paths.get_identity_files();
        
        for file_path in identity_files {
            if file_path.exists() {
                let result = if file_path.is_dir() {
                    fs::remove_dir_all(&file_path).map(|_| stats.dirs_removed += 1)
                } else {
                    fs::remove_file(&file_path).map(|_| stats.files_removed += 1)
                };
                
                if let Err(e) = result {
                    stats.errors.push(format!("删除身份文件 {:?} 失败: {}", file_path, e));
                }
            }
        }
        
        // 清理其他存储目录
        let storage_dirs = vec![
            "Service Worker",
            "databases",
        ];
        
        for dir_name in storage_dirs {
            let dir_path = self.paths.qoder_dir.join(dir_name);
            if dir_path.exists() {
                match fs::remove_dir_all(&dir_path) {
                    Ok(_) => stats.dirs_removed += 1,
                    Err(e) => stats.errors.push(format!("删除存储目录 {} 失败: {}", dir_name, e)),
                }
            }
        }
    }
    
    /// 清理硬件指纹
    fn cleanup_hardware_fingerprints(&self, stats: &mut CleanupStats) {
        let hardware_files = vec![
            "hardware_detection.json",
            "device_capabilities.json",
            "system_features.json",
            "platform_detection.json",
        ];
        
        for file_name in hardware_files {
            let file_path = self.paths.qoder_dir.join(file_name);
            if file_path.exists() {
                match fs::remove_file(&file_path) {
                    Ok(_) => stats.files_removed += 1,
                    Err(e) => stats.errors.push(format!("删除硬件指纹文件 {} 失败: {}", file_name, e)),
                }
            }
        }
    }
    
    /// 重置机器 ID
    pub fn reset_machine_id(&self) -> io::Result<String> {
        let new_id = uuid::Uuid::new_v4().to_string();
        fs::write(&self.paths.machine_id_file, &new_id)?;
        Ok(new_id)
    }
    
    /// 获取当前机器 ID
    pub fn get_current_machine_id(&self) -> io::Result<String> {
        if !self.paths.machine_id_file.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "机器 ID 文件不存在"
            ));
        }
        
        fs::read_to_string(&self.paths.machine_id_file)
            .map(|s| s.trim().to_string())
    }
}
