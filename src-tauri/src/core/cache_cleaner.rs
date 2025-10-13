use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct CacheCleanResult {
    pub directories_cleaned: usize,
    pub total_size_freed: u64,
}

pub struct CacheCleaner {
    cache_directories: Vec<String>,
}

impl CacheCleaner {
    pub fn new(cache_directories: Vec<String>) -> Self {
        Self { cache_directories }
    }

    pub fn clean_cache(&self, app_path: &Path) -> Result<CacheCleanResult> {
        let mut total_size = 0u64;
        let mut cleaned_dirs = 0;

        for cache_name in &self.cache_directories {
            let found_dirs = self.find_directories(app_path, cache_name);

            log::debug!(
                "查找缓存目录 '{}': 找到 {} 个",
                cache_name,
                found_dirs.len()
            );

            for dir in found_dirs {
                let size = self.calculate_directory_size(&dir);

                match self.clear_directory_contents(&dir) {
                    Ok(_) => {
                        total_size += size;
                        cleaned_dirs += 1;
                        log::info!(
                            "清理缓存目录: {} ({} bytes)",
                            dir.display(),
                            size
                        );
                    }
                    Err(e) => {
                        log::warn!("清理目录失败 {}: {}", dir.display(), e);
                    }
                }
            }
        }

        log::info!(
            "缓存清理完成: {} 个目录, {} bytes",
            cleaned_dirs,
            total_size
        );

        Ok(CacheCleanResult {
            directories_cleaned: cleaned_dirs,
            total_size_freed: total_size,
        })
    }

    fn find_directories(&self, root: &Path, target: &str) -> Vec<PathBuf> {
        // 处理嵌套路径，如 "User/workspaceStorage"
        if target.contains('/') {
            return self.find_nested_directories(root, target);
        }

        WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_dir())
            .filter(|e| e.file_name().to_string_lossy() == target)
            .map(|e| e.path().to_path_buf())
            .collect()
    }

    fn find_nested_directories(&self, root: &Path, target: &str) -> Vec<PathBuf> {
        let parts: Vec<&str> = target.split('/').collect();
        let last_part = parts.last().unwrap();

        WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_dir())
            .filter(|e| {
                let path = e.path();
                let file_name = e.file_name().to_string_lossy();

                // 检查最后一部分是否匹配
                if file_name == *last_part {
                    // 检查父路径是否包含前面的部分
                    let path_str = path.to_string_lossy();
                    parts.iter().all(|part| path_str.contains(part))
                } else {
                    false
                }
            })
            .map(|e| e.path().to_path_buf())
            .collect()
    }

    fn clear_directory_contents(&self, dir: &Path) -> Result<()> {
        let mut failed_items = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            let result = if path.is_dir() {
                fs::remove_dir_all(&path)
            } else {
                fs::remove_file(&path)
            };

            if let Err(e) = result {
                log::warn!("删除失败 {}: {}", path.display(), e);
                failed_items.push(path);
            }
        }

        if !failed_items.is_empty() {
            log::warn!("清理目录 {} 时有 {} 个项目失败", dir.display(), failed_items.len());
        }

        Ok(())
    }

    fn calculate_directory_size(&self, dir: &Path) -> u64 {
        WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter_map(|e| e.metadata().ok())
            .map(|m| m.len())
            .sum()
    }

    /// 格式化大小显示
    pub fn format_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }
}
