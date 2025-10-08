use std::process::Command;
use std::thread;
use std::time::Duration;
use crate::utils::logger::{emit_log, LogLevel};
use tauri::AppHandle;

pub struct CursorQuitter {
    timeout: Duration,
}

impl CursorQuitter {
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            timeout: Duration::from_secs(timeout_secs),
        }
    }
    
    /// 查找所有 Cursor 进程
    fn find_cursor_processes(&self) -> Vec<u32> {
        let mut pids = Vec::new();
        
        #[cfg(target_os = "windows")]
        {
            if let Ok(output) = Command::new("tasklist")
                .arg("/FI")
                .arg("IMAGENAME eq Cursor.exe")
                .arg("/FO")
                .arg("CSV")
                .arg("/NH")
                .output()
            {
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    for line in stdout.lines() {
                        let parts: Vec<&str> = line.split(',').collect();
                        if parts.len() >= 2 {
                            let pid_str = parts[1].trim_matches('"').trim();
                            if let Ok(pid) = pid_str.parse::<u32>() {
                                pids.push(pid);
                            }
                        }
                    }
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = Command::new("pgrep")
                .arg("-x")
                .arg("Cursor")
                .output()
            {
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    for line in stdout.lines() {
                        if let Ok(pid) = line.trim().parse::<u32>() {
                            pids.push(pid);
                        }
                    }
                }
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if let Ok(output) = Command::new("pgrep")
                .arg("-x")
                .arg("cursor")
                .output()
            {
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    for line in stdout.lines() {
                        if let Ok(pid) = line.trim().parse::<u32>() {
                            pids.push(pid);
                        }
                    }
                }
            }
        }
        
        pids
    }
    
    /// 终止指定的进程
    fn terminate_process(&self, pid: u32, force: bool) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            let mut cmd = Command::new("taskkill");
            cmd.arg("/PID").arg(pid.to_string()).arg("/T");
            
            if force {
                cmd.arg("/F"); // 强制终止
            }
            
            let output = cmd.output()
                .map_err(|e| format!("Failed to terminate process {}: {}", pid, e))?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("taskkill failed for PID {}: {}", pid, stderr));
            }
        }
        
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            let signal = if force { "-KILL" } else { "-TERM" };
            
            let output = Command::new("kill")
                .arg(signal)
                .arg(pid.to_string())
                .output()
                .map_err(|e| format!("Failed to terminate process {}: {}", pid, e))?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("kill failed for PID {}: {}", pid, stderr));
            }
        }
        
        Ok(())
    }
    
    /// 检查进程是否还在运行
    fn is_process_running(&self, pid: u32) -> bool {
        #[cfg(target_os = "windows")]
        {
            if let Ok(output) = Command::new("tasklist")
                .arg("/FI")
                .arg(format!("PID eq {}", pid))
                .arg("/FO")
                .arg("CSV")
                .arg("/NH")
                .output()
            {
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    return !stdout.is_empty() && stdout.contains(&pid.to_string());
                }
            }
            false
        }
        
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            Command::new("kill")
                .arg("-0")
                .arg(pid.to_string())
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
    }
    
    /// 优雅地关闭 Cursor 进程
    pub fn quit_cursor(&self, app: &AppHandle) -> Result<bool, String> {
        emit_log(app, LogLevel::Info, "⚙️ 开始关闭 Cursor 进程...");
        
        // 查找所有 Cursor 进程
        let cursor_processes = self.find_cursor_processes();
        
        if cursor_processes.is_empty() {
            emit_log(app, LogLevel::Info, "ℹ️ 未发现 Cursor 进程正在运行");
            return Ok(true);
        }
        
        emit_log(
            app,
            LogLevel::Info,
            format!("🔍 发现 {} 个 Cursor 进程", cursor_processes.len())
        );
        
        // 第一阶段：尝试优雅地终止所有进程（不强制）
        emit_log(app, LogLevel::Info, "📝 第一阶段：尝试优雅关闭进程...");
        for &pid in &cursor_processes {
            emit_log(
                app,
                LogLevel::Info,
                format!("⚙️ 正在终止进程 PID: {}", pid)
            );
            
            if let Err(e) = self.terminate_process(pid, false) {
                emit_log(
                    app,
                    LogLevel::Warning,
                    format!("⚠️ 终止进程 {} 时出错: {}", pid, e)
                );
            }
        }
        
        // 等待进程自然关闭（较短的超时）
        let graceful_timeout = Duration::from_secs(self.timeout.as_secs() / 2);
        emit_log(
            app,
            LogLevel::Info,
            format!("⏳ 等待进程优雅关闭（超时 {} 秒）...", graceful_timeout.as_secs())
        );
        
        let start = std::time::Instant::now();
        let check_interval = Duration::from_millis(500);
        
        while start.elapsed() < graceful_timeout {
            let still_running: Vec<u32> = cursor_processes
                .iter()
                .filter(|&&pid| self.is_process_running(pid))
                .copied()
                .collect();
            
            if still_running.is_empty() {
                emit_log(app, LogLevel::Success, "✅ 所有 Cursor 进程已优雅关闭");
                return Ok(true);
            }
            
            thread::sleep(check_interval);
        }
        
        // 检查是否还有进程在运行
        let still_running: Vec<u32> = cursor_processes
            .iter()
            .filter(|&&pid| self.is_process_running(pid))
            .copied()
            .collect();
        
        if !still_running.is_empty() {
            // 第二阶段：强制终止仍在运行的进程
            let pids_str = still_running
                .iter()
                .map(|pid| pid.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            
            emit_log(
                app,
                LogLevel::Warning,
                format!("⚠️ 以下进程未能优雅关闭: {}", pids_str)
            );
            emit_log(app, LogLevel::Info, "💪 第二阶段：强制终止进程...");
            
            for &pid in &still_running {
                emit_log(
                    app,
                    LogLevel::Info,
                    format!("🔨 正在强制终止进程 PID: {}", pid)
                );
                
                if let Err(e) = self.terminate_process(pid, true) {
                    emit_log(
                        app,
                        LogLevel::Error,
                        format!("❌ 强制终止进程 {} 失败: {}", pid, e)
                    );
                }
            }
            
            // 再次等待进程关闭
            let force_timeout = Duration::from_secs(self.timeout.as_secs() / 2);
            emit_log(
                app,
                LogLevel::Info,
                format!("⏳ 等待强制终止完成（超时 {} 秒）...", force_timeout.as_secs())
            );
            
            let start = std::time::Instant::now();
            while start.elapsed() < force_timeout {
                let still_running: Vec<u32> = cursor_processes
                    .iter()
                    .filter(|&&pid| self.is_process_running(pid))
                    .copied()
                    .collect();
                
                if still_running.is_empty() {
                    emit_log(app, LogLevel::Success, "✅ 所有 Cursor 进程已强制关闭");
                    return Ok(true);
                }
                
                thread::sleep(check_interval);
            }
            
            // 最终检查
            let final_running: Vec<u32> = cursor_processes
                .iter()
                .filter(|&&pid| self.is_process_running(pid))
                .copied()
                .collect();
            
            if !final_running.is_empty() {
                let pids_str = final_running
                    .iter()
                    .map(|pid| pid.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                
                emit_log(
                    app,
                    LogLevel::Error,
                    format!("❌ 超时：以下进程仍在运行: {}", pids_str)
                );
                emit_log(
                    app,
                    LogLevel::Warning,
                    "💡 提示：请尝试以管理员身份运行本程序，或手动关闭 Cursor"
                );
                
                return Err(format!(
                    "无法在 {} 秒内关闭所有 Cursor 进程。仍在运行的进程: {}。请以管理员身份运行或手动关闭 Cursor。",
                    self.timeout.as_secs(),
                    pids_str
                ));
            }
        }
        
        Ok(true)
    }
}

/// 便捷函数：使用默认超时（10秒）关闭 Cursor
pub fn quit_cursor_default(app: &AppHandle) -> Result<bool, String> {
    let quitter = CursorQuitter::new(10);
    quitter.quit_cursor(app)
}

