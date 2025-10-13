use std::fs;
use std::path::PathBuf;
use serde_json::{Value, json};
use directories::UserDirs;

pub struct SettingsManager {
    settings_path: PathBuf,
}

impl SettingsManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let user_dirs = UserDirs::new().ok_or("Failed to get user directories")?;
        let documents_dir = user_dirs.document_dir().ok_or("Failed to get documents directory")?;
        let config_dir = documents_dir.join(".cursor-machine-id-resetter");
        
        fs::create_dir_all(&config_dir)?;
        
        Ok(Self {
            settings_path: config_dir.join("settings.json"),
        })
    }
    
    pub fn load_settings(&self) -> Result<Value, Box<dyn std::error::Error>> {
        if self.settings_path.exists() {
            let content = fs::read_to_string(&self.settings_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(self.get_default_settings())
        }
    }
    
    pub fn save_settings(&self, settings: &Value) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(settings)?;
        fs::write(&self.settings_path, content)?;
        Ok(())
    }
    
    pub fn get_default_settings(&self) -> Value {
        json!({
            "autoBackup": true,
            "backupLimit": 10,
            "closeCursor": true,
            "cursorPath": "",
            "windsurfPath": "",
            "qoderPath": "",
            "backupPath": "",
            "patchWorkbench": true,
            "updateSystemId": true,
            "debugMode": false,
            "emailDomain": ""
        })
    }
}

