use std::path::PathBuf;
use directories::UserDirs;

pub struct CursorPaths {
    pub storage_path: PathBuf,
    pub sqlite_path: PathBuf,
    pub machine_id_path: PathBuf,
    pub cursor_path: PathBuf,
    pub backup_path: PathBuf,
}

/// 查找 Cursor 安装路径（Windows）
#[cfg(target_os = "windows")]
fn find_cursor_installation() -> Option<PathBuf> {
    // 尝试多个可能的路径
    let localappdata = std::env::var("LOCALAPPDATA").ok()?;
    
    let possible_paths = vec![
        PathBuf::from(&localappdata).join("Programs/Cursor/resources/app"),
        PathBuf::from(&localappdata).join("Programs/cursor/resources/app"),
        PathBuf::from("C:/Program Files/Cursor/resources/app"),
        PathBuf::from("C:/Program Files (x86)/Cursor/resources/app"),
    ];
    
    for path in possible_paths {
        eprintln!("🔍 Checking Cursor path: {:?}", path);
        let package_json = path.join("package.json");
        if package_json.exists() {
            eprintln!("✅ Found Cursor at: {:?}", path);
            return Some(path);
        }
    }
    
    eprintln!("❌ Could not find Cursor installation");
    // 返回默认路径
    Some(PathBuf::from(&localappdata).join("Programs/Cursor/resources/app"))
}

pub fn get_cursor_paths() -> Result<CursorPaths, Box<dyn std::error::Error>> {
    let user_dirs = UserDirs::new().ok_or("Failed to get user directories")?;
    let documents_dir = user_dirs.document_dir().ok_or("Failed to get documents directory")?;
    
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA")?;
        let cursor_path = find_cursor_installation()
            .ok_or("Failed to find Cursor installation")?;
        
        Ok(CursorPaths {
            storage_path: PathBuf::from(&appdata).join("Cursor/User/globalStorage/storage.json"),
            sqlite_path: PathBuf::from(&appdata).join("Cursor/User/globalStorage/state.vscdb"),
            machine_id_path: PathBuf::from(&appdata).join("Cursor/machineId"),
            cursor_path,
            backup_path: documents_dir.join(".cursor-machine-id-resetter/backups"),
        })
    }
    
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME")?;
        
        Ok(CursorPaths {
            storage_path: PathBuf::from(&home).join("Library/Application Support/Cursor/User/globalStorage/storage.json"),
            sqlite_path: PathBuf::from(&home).join("Library/Application Support/Cursor/User/globalStorage/state.vscdb"),
            machine_id_path: PathBuf::from(&home).join("Library/Application Support/Cursor/machineId"),
            cursor_path: PathBuf::from("/Applications/Cursor.app/Contents/Resources/app"),
            backup_path: documents_dir.join(".cursor-machine-id-resetter/backups"),
        })
    }
    
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME")?;
        
        Ok(CursorPaths {
            storage_path: PathBuf::from(&home).join(".config/Cursor/User/globalStorage/storage.json"),
            sqlite_path: PathBuf::from(&home).join(".config/Cursor/User/globalStorage/state.vscdb"),
            machine_id_path: PathBuf::from(&home).join(".config/Cursor/machineid"),
            cursor_path: PathBuf::from("/opt/Cursor/resources/app"),
            backup_path: documents_dir.join(".cursor-machine-id-resetter/backups"),
        })
    }
}

