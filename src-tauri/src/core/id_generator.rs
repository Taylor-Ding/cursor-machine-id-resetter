use sha2::{Sha256, Sha512, Digest};
use uuid::Uuid;
use std::collections::HashMap;

/// 生成新的机器ID集合
pub fn generate_machine_ids() -> HashMap<String, String> {
    let mut ids = HashMap::new();
    
    // 生成 devDeviceId (UUID格式)
    let dev_device_id = Uuid::new_v4().to_string();
    ids.insert("telemetry.devDeviceId".to_string(), dev_device_id.clone());
    ids.insert("storage.serviceMachineId".to_string(), dev_device_id);
    
    // 生成 machineId (SHA256, 64字符)
    let mut hasher = Sha256::new();
    hasher.update(Uuid::new_v4().as_bytes());
    let machine_id = format!("{:x}", hasher.finalize());
    ids.insert("telemetry.machineId".to_string(), machine_id);
    
    // 生成 macMachineId (SHA512, 128字符)
    let mut hasher = Sha512::new();
    hasher.update(Uuid::new_v4().as_bytes());
    let mac_machine_id = format!("{:x}", hasher.finalize());
    ids.insert("telemetry.macMachineId".to_string(), mac_machine_id);
    
    // 生成 sqmId (大括号包裹的大写UUID)
    let sqm_id = format!("{{{}}}", Uuid::new_v4().to_string().to_uppercase());
    ids.insert("telemetry.sqmId".to_string(), sqm_id);
    
    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_machine_ids() {
        let ids = generate_machine_ids();
        
        assert!(ids.contains_key("telemetry.devDeviceId"));
        assert!(ids.contains_key("telemetry.machineId"));
        assert!(ids.contains_key("telemetry.macMachineId"));
        assert!(ids.contains_key("telemetry.sqmId"));
        assert!(ids.contains_key("storage.serviceMachineId"));
        
        // 检查格式
        assert_eq!(ids.get("telemetry.machineId").unwrap().len(), 64);
        assert_eq!(ids.get("telemetry.macMachineId").unwrap().len(), 128);
        assert!(ids.get("telemetry.sqmId").unwrap().starts_with("{"));
        assert!(ids.get("telemetry.sqmId").unwrap().ends_with("}"));
    }
}

