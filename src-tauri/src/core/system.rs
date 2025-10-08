use std::collections::HashMap;
use crate::utils::logger::{emit_log, LogLevel};
use tauri::AppHandle;

pub struct SystemIdManager;

impl SystemIdManager {
    pub fn new() -> Self {
        Self
    }
    
    pub fn update_system_ids(&self, app: &AppHandle, new_ids: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "windows")]
        {
            emit_log(app, LogLevel::Info, "正在更新 Windows 系统 ID");
            self.update_windows_ids(app, new_ids)?;
        }
        
        #[cfg(target_os = "macos")]
        {
            emit_log(app, LogLevel::Info, "正在更新 macOS 系统 ID");
            self.update_macos_ids(app, new_ids)?;
        }
        
        #[cfg(target_os = "linux")]
        {
            emit_log(app, LogLevel::Info, "Linux 系统不需要更新系统级 ID");
        }
        
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    fn update_windows_ids(&self, app: &AppHandle, new_ids: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        use winreg::RegKey;
        use winreg::enums::*;
        
        emit_log(app, LogLevel::Warning, "注意：修改系统注册表需要管理员权限");
        
        // 更新 MachineGuid
        if let Some(guid) = new_ids.get("telemetry.devDeviceId") {
            emit_log(app, LogLevel::Info, "正在更新注册表 MachineGuid");
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            
            match hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Cryptography", KEY_WRITE) {
                Ok(crypto_key) => {
                    match crypto_key.set_value("MachineGuid", guid) {
                        Ok(_) => {
                            emit_log(app, LogLevel::Success, format!("MachineGuid 更新为: {}", guid));
                        }
                        Err(e) => {
                            if e.kind() == std::io::ErrorKind::PermissionDenied {
                                emit_log(app, LogLevel::Error, "权限不足：请以管理员身份运行此程序");
                                emit_log(app, LogLevel::Warning, "跳过注册表 MachineGuid 更新");
                            } else {
                                return Err(Box::new(e));
                            }
                        }
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::PermissionDenied {
                        emit_log(app, LogLevel::Error, "权限不足：无法访问注册表");
                        emit_log(app, LogLevel::Warning, "跳过注册表更新");
                        return Ok(()); // 不阻断整个流程
                    }
                    return Err(Box::new(e));
                }
            }
        }
        
        // 更新 SQMClient MachineId
        if let Some(sqm_id) = new_ids.get("telemetry.sqmId") {
            emit_log(app, LogLevel::Info, "正在更新注册表 SQMClient MachineId");
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            
            match hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\SQMClient", KEY_WRITE) {
                Ok(sqm_key) => {
                    match sqm_key.set_value("MachineId", sqm_id) {
                        Ok(_) => {
                            emit_log(app, LogLevel::Success, format!("SQMClient MachineId 更新为: {}", sqm_id));
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                            emit_log(app, LogLevel::Warning, "权限不足，跳过 SQMClient MachineId 更新");
                        }
                        Err(e) => return Err(Box::new(e)),
                    }
                }
                Err(_) => {
                    // 尝试创建键
                    match hklm.create_subkey("SOFTWARE\\Microsoft\\SQMClient") {
                        Ok((sqm_key, _)) => {
                            match sqm_key.set_value("MachineId", sqm_id) {
                                Ok(_) => {
                                    emit_log(app, LogLevel::Success, format!("SQMClient MachineId 创建并设置为: {}", sqm_id));
                                }
                                Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                                    emit_log(app, LogLevel::Warning, "权限不足，跳过创建 SQMClient");
                                }
                                Err(e) => return Err(Box::new(e)),
                            }
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                            emit_log(app, LogLevel::Warning, "权限不足，无法创建 SQMClient 注册表键");
                        }
                        Err(e) => return Err(Box::new(e)),
                    }
                }
            }
        }
        
        emit_log(app, LogLevel::Success, "Windows 系统 ID 更新完成（可能跳过了部分需要管理员权限的操作）");
        Ok(())
    }
    
    #[cfg(target_os = "macos")]
    fn update_macos_ids(&self, app: &AppHandle, new_ids: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        use std::process::Command;
        
        if let Some(mac_id) = new_ids.get("telemetry.macMachineId") {
            let uuid_file = "/var/root/Library/Preferences/SystemConfiguration/com.apple.platform.uuid.plist";
            
            emit_log(app, LogLevel::Info, format!("正在更新 macOS UUID: {}", uuid_file));
            
            if std::path::Path::new(uuid_file).exists() {
                emit_log(app, LogLevel::Info, "找到 UUID 配置文件");
                let cmd = format!(
                    r#"sudo plutil -replace "UUID" -string "{}" "{}""#,
                    mac_id, uuid_file
                );
                
                emit_log(app, LogLevel::Info, "执行 plutil 命令更新 UUID");
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(&cmd)
                    .output()?;
                
                if output.status.success() {
                    emit_log(app, LogLevel::Success, format!("macOS UUID 更新为: {}", mac_id));
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    emit_log(app, LogLevel::Warning, format!("更新 UUID 可能失败: {}", error));
                }
            } else {
                emit_log(app, LogLevel::Warning, "UUID 配置文件不存在，跳过更新");
            }
        }
        
        emit_log(app, LogLevel::Success, "macOS 系统 ID 更新完成");
        Ok(())
    }
}

