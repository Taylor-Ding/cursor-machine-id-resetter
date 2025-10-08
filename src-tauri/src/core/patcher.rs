use std::fs;
use std::path::PathBuf;
use regex::Regex;
use crate::utils::logger::{emit_log, LogLevel};
use tauri::AppHandle;

pub struct Patcher {
    cursor_path: PathBuf,
}

impl Patcher {
    pub fn new(cursor_path: PathBuf) -> Self {
        Self { cursor_path }
    }
    
    pub fn patch_files(&self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        emit_log(app, LogLevel::Info, "开始修补应用程序文件");
        self.patch_main_js(app)?;
        self.patch_workbench_js(app)?;
        emit_log(app, LogLevel::Success, "所有文件修补完成");
        Ok(())
    }
    
    /// 修补 main.js
    fn patch_main_js(&self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        let main_js_path = self.cursor_path.join("out/main.js");
        
        emit_log(app, LogLevel::Info, format!("正在修补 main.js: {:?}", main_js_path));
        
        if !main_js_path.exists() {
            emit_log(app, LogLevel::Warning, "main.js 文件不存在，跳过修补");
            return Ok(());
        }
        
        emit_log(app, LogLevel::Info, "读取 main.js 文件内容");
        let content = match fs::read_to_string(&main_js_path) {
            Ok(c) => c,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    emit_log(app, LogLevel::Error, "权限不足：无法读取 main.js");
                    emit_log(app, LogLevel::Warning, "请以管理员身份运行程序，或手动修改文件权限");
                    emit_log(app, LogLevel::Info, "跳过 main.js 修补");
                    return Ok(());
                }
                return Err(Box::new(e));
            }
        };
        
        // 创建备份
        emit_log(app, LogLevel::Info, "创建 main.js 备份");
        let backup_path = format!("{}.bak", main_js_path.to_string_lossy());
        match fs::copy(&main_js_path, &backup_path) {
            Ok(_) => emit_log(app, LogLevel::Success, format!("备份文件创建: {}", backup_path)),
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                emit_log(app, LogLevel::Warning, "权限不足，无法创建备份文件");
            }
            Err(e) => return Err(Box::new(e)),
        }
        
        // 应用补丁
        emit_log(app, LogLevel::Info, "应用修补规则...");
        let pattern1 = Regex::new(r"async getMachineId\(\)\{return [^??]+\?\?([^}]+)\}")?;
        let pattern2 = Regex::new(r"async getMacMachineId\(\)\{return [^??]+\?\?([^}]+)\}")?;
        
        let mut patched_content = pattern1.replace_all(&content, "async getMachineId(){return $1}").to_string();
        let matches1 = pattern1.captures_iter(&content).count();
        emit_log(app, LogLevel::Info, format!("修补规则 1: 找到 {} 处匹配", matches1));
        
        patched_content = pattern2.replace_all(&patched_content, "async getMacMachineId(){return $1}").to_string();
        let matches2 = pattern2.captures_iter(&content).count();
        emit_log(app, LogLevel::Info, format!("修补规则 2: 找到 {} 处匹配", matches2));
        
        emit_log(app, LogLevel::Info, "写入修补后的文件");
        match fs::write(&main_js_path, patched_content) {
            Ok(_) => {
                emit_log(app, LogLevel::Success, "main.js 修补完成");
            }
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                emit_log(app, LogLevel::Error, "权限不足：无法写入 main.js");
                emit_log(app, LogLevel::Warning, "请以管理员身份运行程序");
                emit_log(app, LogLevel::Info, "main.js 修补失败，但不影响其他操作");
                return Ok(());
            }
            Err(e) => return Err(Box::new(e)),
        }
        
        Ok(())
    }
    
    /// 修补 workbench.desktop.main.js
    fn patch_workbench_js(&self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        let workbench_path = self.cursor_path.join("out/vs/workbench/workbench.desktop.main.js");
        
        emit_log(app, LogLevel::Info, format!("正在修补 workbench.desktop.main.js: {:?}", workbench_path));
        
        if !workbench_path.exists() {
            emit_log(app, LogLevel::Warning, "workbench.desktop.main.js 文件不存在，跳过修补");
            return Ok(());
        }
        
        emit_log(app, LogLevel::Info, "读取 workbench.desktop.main.js 文件内容");
        let content = match fs::read_to_string(&workbench_path) {
            Ok(c) => c,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    emit_log(app, LogLevel::Error, "权限不足：无法读取 workbench.desktop.main.js");
                    emit_log(app, LogLevel::Warning, "请以管理员身份运行程序");
                    emit_log(app, LogLevel::Info, "跳过 workbench.desktop.main.js 修补");
                    return Ok(());
                }
                return Err(Box::new(e));
            }
        };
        
        // 创建备份
        emit_log(app, LogLevel::Info, "创建 workbench.desktop.main.js 备份");
        let backup_path = format!("{}.backup", workbench_path.to_string_lossy());
        match fs::copy(&workbench_path, &backup_path) {
            Ok(_) => emit_log(app, LogLevel::Success, format!("备份文件创建: {}", backup_path)),
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                emit_log(app, LogLevel::Warning, "权限不足，无法创建备份文件");
            }
            Err(e) => return Err(Box::new(e)),
        }
        
        // 应用补丁
        emit_log(app, LogLevel::Info, "应用修补规则...");
        let mut patched_content = content.clone();
        let mut patch_count = 0;
        
        // 替换按钮
        let new_content = patched_content.replace(
            r#"title:"Upgrade to Pro""#,
            r#"title:"yeongpin GitHub""#
        );
        if new_content != patched_content {
            patch_count += 1;
            emit_log(app, LogLevel::Info, "修补: 替换升级按钮文本");
            patched_content = new_content;
        }
        
        // 替换Badge
        let new_content = patched_content.replace(
            r#"<div>Pro Trial"#,
            r#"<div>Pro"#
        );
        if new_content != patched_content {
            patch_count += 1;
            emit_log(app, LogLevel::Info, "修补: 替换 Pro Trial 标签");
            patched_content = new_content;
        }
        
        // 隐藏Toast
        let new_content = patched_content.replace(
            r#"notifications-toasts"#,
            r#"notifications-toasts hidden"#
        );
        if new_content != patched_content {
            patch_count += 1;
            emit_log(app, LogLevel::Info, "修补: 隐藏通知提示");
            patched_content = new_content;
        }
        
        emit_log(app, LogLevel::Success, format!("应用了 {} 个修补规则", patch_count));
        
        emit_log(app, LogLevel::Info, "写入修补后的文件");
        match fs::write(&workbench_path, patched_content) {
            Ok(_) => {
                emit_log(app, LogLevel::Success, "workbench.desktop.main.js 修补完成");
            }
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                emit_log(app, LogLevel::Error, "权限不足：无法写入 workbench.desktop.main.js");
                emit_log(app, LogLevel::Warning, "请以管理员身份运行程序");
                emit_log(app, LogLevel::Info, "workbench.desktop.main.js 修补失败，但不影响其他操作");
                return Ok(());
            }
            Err(e) => return Err(Box::new(e)),
        }
        
        Ok(())
    }
}

