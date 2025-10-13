use sysinfo::{System, Signal};
use std::io;
use std::ffi::OsStr;

/// Qoder 进程管理器
pub struct QoderProcess;

impl QoderProcess {
    /// 检查 Qoder 是否正在运行
    pub fn is_running() -> bool {
        let mut system = System::new_all();
        system.refresh_all();
        
        system.processes().values().any(|process| {
            Self::process_name_contains(process.name(), "qoder")
        })
    }
    
    /// 获取所有 Qoder 进程的 PID
    pub fn get_pids() -> Vec<u32> {
        let mut system = System::new_all();
        system.refresh_all();
        
        system.processes()
            .iter()
            .filter(|(_, process)| {
                Self::process_name_contains(process.name(), "qoder")
            })
            .map(|(pid, _)| pid.as_u32())
            .collect()
    }
    
    /// 尝试关闭 Qoder 进程（优雅关闭）
    pub fn try_quit() -> io::Result<bool> {
        let mut system = System::new_all();
        system.refresh_all();
        
        let qoder_processes: Vec<_> = system.processes()
            .iter()
            .filter(|(_, process)| {
                Self::process_name_contains(process.name(), "qoder")
            })
            .collect();
        
        if qoder_processes.is_empty() {
            return Ok(false);
        }
        
        // 尝试优雅关闭
        for (_, process) in qoder_processes {
            #[cfg(not(target_os = "windows"))]
            {
                process.kill_with(Signal::Term);
            }
            
            #[cfg(target_os = "windows")]
            {
                process.kill();
            }
        }
        
        // 等待进程结束
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        // 检查是否还在运行
        let still_running = Self::is_running();
        
        if still_running {
            // 如果还在运行，尝试强制关闭
            system.refresh_all();
            for (_, process) in system.processes()
                .iter()
                .filter(|(_, p)| Self::process_name_contains(p.name(), "qoder"))
            {
                process.kill();
            }
            
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        
        Ok(!Self::is_running())
    }
    
    /// 获取 Qoder 进程信息
    pub fn get_process_info() -> Vec<(u32, String)> {
        let mut system = System::new_all();
        system.refresh_all();
        
        system.processes()
            .iter()
            .filter(|(_, process)| {
                Self::process_name_contains(process.name(), "qoder")
            })
            .map(|(pid, process)| {
                let name = process.name().to_string_lossy().to_string();
                (pid.as_u32(), name)
            })
            .collect()
    }
    
    /// 辅助函数：检查进程名是否包含指定字符串（不区分大小写）
    fn process_name_contains(name: &OsStr, target: &str) -> bool {
        name.to_string_lossy()
            .to_lowercase()
            .contains(target)
    }
}
