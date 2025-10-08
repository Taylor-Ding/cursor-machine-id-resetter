# 文件备份功能说明 💾

## 📋 功能概述

在修改 `storage.json` 和 `state.vscdb` 文件之前，系统会**自动创建备份文件**，防止意外情况导致数据丢失。

---

## ✨ 功能特性

### 1. 自动备份

每次重置机器 ID 时，系统会自动执行以下操作：

1. **备份 storage.json**
   - 源文件：`storage.json`
   - 备份文件：`storage.json.backup`
   - 时机：修改前自动备份

2. **备份 state.vscdb**
   - 源文件：`state.vscdb`
   - 备份文件：`state.vscdb.backup`
   - 时机：修改前自动备份

### 2. 备份覆盖策略

- 如果备份文件已存在，会被新备份**覆盖**
- 始终保留最近一次的备份

### 3. 手动恢复

提供 Tauri 命令 `restore_file_backup`，可以从备份恢复文件。

---

## 🔧 技术实现

### 备份函数

```rust
/// 创建文件备份
fn backup_file(&self, app: &AppHandle, file_path: &PathBuf) 
    -> Result<PathBuf, Box<dyn std::error::Error>> 
{
    if !file_path.exists() {
        emit_log(app, LogLevel::Warning, "文件不存在，无需备份");
        return Err("文件不存在".into());
    }
    
    // 备份文件路径：原文件名 + .backup
    let backup_path = file_path.with_extension(
        format!("{}.backup", file_path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or(""))
    );
    
    emit_log(app, LogLevel::Info, format!("正在备份文件: {:?}", file_path));
    emit_log(app, LogLevel::Info, format!("备份到: {:?}", backup_path));
    
    // 复制文件
    fs::copy(file_path, &backup_path)?;
    
    emit_log(app, LogLevel::Success, "文件备份成功");
    Ok(backup_path)
}
```

### 恢复函数

```rust
/// 恢复备份文件
pub fn restore_backup(&self, app: &AppHandle, file_path: &PathBuf) 
    -> Result<(), Box<dyn std::error::Error>> 
{
    let backup_path = file_path.with_extension(
        format!("{}.backup", file_path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or(""))
    );
    
    if !backup_path.exists() {
        let error_msg = format!("备份文件不存在: {:?}", backup_path);
        emit_log(app, LogLevel::Error, &error_msg);
        return Err(error_msg.into());
    }
    
    emit_log(app, LogLevel::Info, "正在从备份恢复...");
    
    // 复制备份文件回原位置
    fs::copy(&backup_path, file_path)?;
    
    emit_log(app, LogLevel::Success, "文件恢复成功");
    Ok(())
}
```

### 集成到重置流程

**storage.json 更新：**
```rust
pub fn update_storage_json(&mut self, app: &AppHandle, new_ids: &HashMap<String, String>) 
    -> Result<(), Box<dyn std::error::Error>> 
{
    // 先备份文件
    if self.storage_path.exists() {
        match self.backup_file(app, &self.storage_path) {
            Ok(backup_path) => {
                emit_log(app, LogLevel::Success, 
                    format!("已创建备份: {:?}", backup_path));
            }
            Err(e) => {
                emit_log(app, LogLevel::Warning, 
                    format!("备份失败（将继续操作）: {}", e));
            }
        }
    }
    
    // ... 更新文件内容 ...
}
```

**state.vscdb 更新：**
```rust
pub fn update_sqlite_db(&self, app: &AppHandle, new_ids: &HashMap<String, String>) 
    -> Result<(), Box<dyn std::error::Error>> 
{
    // 先备份数据库文件
    if self.sqlite_path.exists() {
        match self.backup_file(app, &self.sqlite_path) {
            Ok(backup_path) => {
                emit_log(app, LogLevel::Success, 
                    format!("已创建备份: {:?}", backup_path));
            }
            Err(e) => {
                emit_log(app, LogLevel::Warning, 
                    format!("备份失败（将继续操作）: {}", e));
            }
        }
    }
    
    // ... 更新数据库 ...
}
```

---

## 📍 备份文件位置

### Windows

```
storage.json:
C:\Users\<用户名>\AppData\Roaming\Cursor\User\globalStorage\storage.json
C:\Users\<用户名>\AppData\Roaming\Cursor\User\globalStorage\storage.json.backup  ← 备份

state.vscdb:
C:\Users\<用户名>\AppData\Roaming\Cursor\User\globalStorage\state.vscdb
C:\Users\<用户名>\AppData\Roaming\Cursor\User\globalStorage\state.vscdb.backup  ← 备份
```

### macOS

```
storage.json:
~/Library/Application Support/Cursor/User/globalStorage/storage.json
~/Library/Application Support/Cursor/User/globalStorage/storage.json.backup  ← 备份

state.vscdb:
~/Library/Application Support/Cursor/User/globalStorage/state.vscdb
~/Library/Application Support/Cursor/User/globalStorage/state.vscdb.backup  ← 备份
```

### Linux

```
storage.json:
~/.config/Cursor/User/globalStorage/storage.json
~/.config/Cursor/User/globalStorage/storage.json.backup  ← 备份

state.vscdb:
~/.config/Cursor/User/globalStorage/state.vscdb
~/.config/Cursor/User/globalStorage/state.vscdb.backup  ← 备份
```

---

## 🚀 使用方法

### 自动备份（无需操作）

点击"开始重置"时，系统会自动：
1. ✅ 检查文件是否存在
2. ✅ 创建 `.backup` 备份文件
3. ✅ 执行重置操作
4. ✅ 在日志中显示备份路径

**日志示例：**
```
[Info] 正在备份文件: "C:\Users\...\storage.json"
[Info] 备份到: "C:\Users\...\storage.json.backup"
[Success] 文件备份成功: "C:\Users\...\storage.json.backup"
[Success] 已创建备份: "C:\Users\...\storage.json.backup"
```

### 手动恢复备份（Tauri 命令）

```typescript
import { invoke } from '@tauri-apps/api/core'

// 恢复 storage.json
await invoke('restore_file_backup', { fileType: 'storage' })

// 恢复 state.vscdb
await invoke('restore_file_backup', { fileType: 'sqlite' })
```

---

## 🔍 故障排除

### 问题 1: 备份失败

**可能原因：**
- 文件不存在
- 权限不足
- 磁盘空间不足

**解决方案：**
- 检查文件是否存在
- 以管理员身份运行
- 确保有足够的磁盘空间

**日志示例：**
```
[Warning] 备份失败（将继续操作）: 文件不存在
```

⚠️ **注意：** 即使备份失败，重置操作仍会继续执行。

---

### 问题 2: 恢复失败

**可能原因：**
- 备份文件不存在
- 权限不足

**解决方案：**
- 确认备份文件存在
- 以管理员身份运行

**日志示例：**
```
[Error] 备份文件不存在: "C:\Users\...\storage.json.backup"
[Error] 恢复备份失败: 备份文件不存在
```

---

### 问题 3: 找不到备份文件

**检查位置：**

Windows:
```cmd
dir "%APPDATA%\Cursor\User\globalStorage\*.backup"
```

macOS/Linux:
```bash
ls -la ~/Library/Application\ Support/Cursor/User/globalStorage/*.backup  # macOS
ls -la ~/.config/Cursor/User/globalStorage/*.backup  # Linux
```

---

## 💡 最佳实践

### 1. 重置前检查

在执行重置操作前：
- ✅ 查看日志，确认备份已创建
- ✅ 记录备份文件路径
- ✅ 如有重要数据，额外手动备份

### 2. 备份管理

建议：
- 📅 定期清理旧备份（只保留最新）
- 💾 重要数据额外备份到其他位置
- 🔍 重置后检查功能是否正常

### 3. 出现问题时

如果重置后 Cursor 出现问题：
1. 📋 查看日志，确认备份路径
2. 🔄 使用 `restore_file_backup` 命令恢复
3. 🔁 重启 Cursor IDE
4. ✅ 验证功能

---

## 📊 对比 Python 项目

| 特性 | Python 项目 | Rust/Tauri 项目 |
|------|------------|----------------|
| 自动备份 | ❌ 无 | ✅ 有 |
| 备份命名 | - | `.backup` 后缀 |
| 恢复功能 | ❌ 无 | ✅ 有 |
| 日志记录 | 部分 | ✅ 完整 |
| 错误处理 | 基础 | ✅ 详细 |

**改进点：**
- ✅ 自动备份机制
- ✅ 详细的日志记录
- ✅ 完善的错误处理
- ✅ 提供恢复功能

---

## 🎯 未来改进（可选）

### 1. 多版本备份

```rust
// 支持多个备份版本
storage.json.backup.1
storage.json.backup.2
storage.json.backup.3
```

### 2. 时间戳备份

```rust
// 包含时间戳
storage.json.backup.2025-10-08-14-30-00
state.vscdb.backup.2025-10-08-14-30-00
```

### 3. 自动清理

```rust
// 自动删除超过 N 天的备份
cleanup_old_backups(days: 7)
```

---

## 📝 总结

✅ **已实现功能：**
- 自动备份 storage.json
- 自动备份 state.vscdb
- 提供恢复命令
- 详细日志记录
- 错误处理机制

⚠️ **注意事项：**
- 备份文件会被覆盖（只保留最新）
- 备份失败不会阻止重置操作
- 需要手动调用命令恢复

💡 **最佳实践：**
- 重置前查看日志确认备份成功
- 重要数据额外手动备份
- 出现问题及时恢复

---

<div align="center">

**备份功能已完全实现！数据更安全！** ✅

</div>

