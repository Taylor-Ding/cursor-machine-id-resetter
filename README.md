# Cursor & Windsurf & Qoder 多 IDE 重置工具 🚀

<div align="center">

**功能强大的多 IDE 重置工具 · 支持 Cursor、Windsurf、Qoder**

基于 Tauri 2.0 + Vue 3 + Rust 构建

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.4-green.svg)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)

</div>

---

## 📋 功能特性

### 🎯 支持的 IDE

- ✅ **Cursor IDE** - 完整支持所有重置功能
- ✅ **Windsurf IDE** - 三阶段深度重置（Telemetry → 数据库 → 缓存）
- ✅ **Qoder IDE** - 深度清理 + 实时注入（仅 Windows）

### 🔄 核心功能

#### Cursor 重置
- 一键重置机器 ID
- 智能关闭进程（优雅→强制两阶段）
- 多重置选项（存储文件、SQLite、系统级标识、应用补丁）
- 自动备份与恢复（保留 30 天）
- 实时状态检查

#### Windsurf 三阶段重置
- **阶段 1 (20%-45%)**: Telemetry 修改 - 更新 12+ 设备标识符，删除 9+ 会话键
- **阶段 2 (50%-65%)**: 数据库清理 - 清空 8+ 缓存表，删除关键词记录
- **阶段 3 (80%-100%)**: 缓存清理 - 清理 14+ 缓存目录
- 智能表结构识别
- 自定义路径配置
- 跨平台支持（macOS / Windows / Linux）

#### Qoder 重置
- 完整重置（深度清理 + 重置 ID）
- 仅重置 ID（快速模式）
- 实时注入（运行时修改，需管理员权限，仅 Windows）
- 智能保留（MCP 配置、对话历史）
- 清理 30+ 缓存目录、40+ 身份文件

### 📧 邮箱生成器

- 随机账号生成（邮箱、密码、名字、姓氏）
- 基于 36,000+ 英文名字库
- 安全密码生成（大小写字母、数字、特殊字符）
- 域名自动保存，下次自动填充
- 可配置密码长度（8-32位）和时间戳长度（0-8位）

### ⚙️ 设置管理

- 支持自定义 Cursor / Windsurf / Qoder 路径
- 手动输入或文件浏览器选择
- 配置自动持久化到本地
- 优先级：用户配置 > 系统默认
- 邮箱域名自动保存

### 🎨 用户界面

- 现代化圆角扁平化设计
- 渐变卡片视觉效果
- 响应式布局
- 流畅动画
- 实时进度显示
- 动态文本（根据 IDE 自动调整）

### 📝 日志系统

- 实时日志记录
- 四级分类（Info / Success / Warning / Error）
- 搜索过滤
- 自动滚动
- 一键清空

### 🌍 多语言

- 简体中文
- English
- 运行时动态切换

### 🔒 安全特性

- 权限检测与提示
- 自动备份（30 天保留）
- 操作确认
- 事务处理
- 错误自动回滚

---

## 🚀 快速开始

### 下载预编译版本（推荐）

访问 [Releases](https://github.com/Taylor-Ding/cursor-machine-id-resetter/releases) 下载：
- Windows: `.exe` 或 `.msi`
- macOS: `.dmg`
- Linux: `.AppImage` 或 `.deb`

### 从源码构建

```bash
# 克隆项目
git clone https://github.com/Taylor-Ding/cursor-machine-id-resetter.git
cd cursor-machine-id-resetter

# 安装依赖
npm install

# 开发模式
npm run tauri:dev

# 生产构建
npm run tauri:build
```

**环境要求**: Node.js 18+, Rust 1.70+, npm 9+

---

## 💡 使用说明

### Cursor 重置

1. **查看信息** - 启动后 Cursor 标签页显示当前机器 ID、版本、备份数
2. **选择选项** - 勾选要重置的内容（默认全选）
3. **执行重置** - 点击"开始重置"，确认操作
4. **重启应用** - 重置完成后重启 Cursor

> ⚠️ Windows 需管理员权限，macOS 可能需要授予文件访问权限

### Windsurf 重置

1. **配置路径**（可选） - 设置 → Windsurf 路径 → 输入路径 → 保存
   - macOS: `~/Library/Application Support/Windsurf`
   - Windows: `%APPDATA%\Windsurf`
   - Linux: `~/.config/Windsurf`

2. **执行重置** - Windsurf 标签页 → 确保 Windsurf 已关闭 → 开始重置

3. **观察进度** - 三阶段进度显示：
   - 🔄 Telemetry 修改 (20%-45%)
   - 🗑️ 数据库清理 (50%-65%)
   - 🧹 缓存清理 (80%-100%)

### Qoder 重置

1. **选择模式**：
   - 完整重置：深度清理 + 重置 ID（需关闭 Qoder）
   - 仅重置 ID：快速模式（需关闭 Qoder）
   - 实时注入：运行时修改（需 Qoder 运行 + 管理员权限，仅 Windows）

2. **执行** - 点击"开始重置"，等待完成

### 邮箱生成器

1. **输入域名** - 例如 `example.com`（自动保存）
2. **配置参数** - 密码长度（8-32位），时间戳长度（0-8位）
3. **生成账号** - 点击"生成账号"
4. **复制信息** - 每个字段旁边都有复制按钮

### 备份恢复

1. **查看备份** - 恢复标签页显示所有历史备份
2. **恢复备份** - 选择备份 → 恢复
3. **删除备份** - 选择备份 → 删除

### 设置配置

- **路径配置**: 手动输入或浏览选择 Cursor / Windsurf / Qoder 路径
- **保存设置**: 点击"保存设置"按钮
- **恢复默认**: 点击"恢复默认"按钮

---

## 🏗️ 技术架构

### 技术栈

**前端**
- Vue 3.4 (Composition API) + TypeScript 5.3
- Vite 5.0 + Element Plus 2.5
- Tailwind CSS 3.4 + Lucide Icons
- Pinia 2.1 + Vue I18n 9.9

**后端**
- Tauri 2.0 + Rust 1.70+
- rusqlite (SQLite)
- sysinfo + winreg (Windows)
- serde + anyhow + tokio

### 核心模块

```
src-tauri/src/core/
├── Cursor 模块
│   ├── id_generator.rs      # ID 生成
│   ├── resetter.rs          # 重置逻辑
│   ├── backup.rs            # 备份管理
│   └── cursor_quitter.rs    # 进程管理
├── Windsurf 模块
│   ├── ide_config.rs        # 配置管理
│   ├── ide_resetter.rs      # 统一引擎
│   ├── telemetry_modifier.rs  # Telemetry 修改
│   ├── database_cleaner.rs  # 数据库清理
│   └── cache_cleaner.rs     # 缓存清理
├── Qoder 模块
│   └── qoder/
│       ├── qoder_paths.rs   # 路径管理
│       ├── qoder_cleaner.rs # 数据清理
│       ├── qoder_resetter.rs # 重置逻辑
│       └── qoder_injector.rs # 内存注入
└── 通用模块
    ├── settings.rs          # 设置管理
    ├── patcher.rs           # 文件修补
    └── email_generator.rs   # 邮箱生成
```

### 统一架构设计

- **引擎模式**: 三个 IDE 共享核心逻辑
- **配置系统**: 统一的 `IDEConfig`
- **优先级**: 用户配置 > 系统默认
- **模块化**: 易于扩展新 IDE

---

## 🔧 IDE 重置逻辑详解

### Cursor 重置逻辑

#### 1. 检测与关闭进程
```rust
// 两阶段进程关闭
1. 检测 Cursor 进程是否运行
2. 优雅关闭（SIGTERM）
3. 等待 5 秒
4. 强制关闭（SIGKILL）如果仍在运行
```

#### 2. 自动备份
```rust
// 备份策略
1. 创建时间戳命名的备份目录
2. 复制 storage.json
3. 复制 state.vscdb
4. 保存机器 ID 信息
5. 自动清理 30 天前的旧备份
```

#### 3. 重置操作
```rust
// 四大重置选项
Option 1: 重置存储文件
  - 删除 storage.json
  - 生成新的机器 ID
  - 写入新的 storage.json

Option 2: 重置 SQLite 数据库
  - 打开 state.vscdb
  - UPDATE ItemTable SET value = new_id WHERE key = 'telemetry.machineId'
  - UPDATE ItemTable SET value = new_id WHERE key = 'telemetry.devDeviceId'
  - 删除缓存表记录

Option 3: 更新系统级标识
  Windows:
    - 注册表: HKCU\Software\Cursor\machineId
    - 注册表: HKCU\Software\Cursor\deviceId
  macOS:
    - plist: ~/Library/Preferences/com.cursor.plist
  Linux:
    - Config: ~/.config/Cursor/config.json

Option 4: 修补应用程序文件
  - 替换 workbench.desktop.main.js 中的硬编码 ID
  - 修补 telemetry 相关代码
```

#### 4. 验证
```rust
// 验证新 ID
1. 读取 storage.json 确认新 ID
2. 查询 SQLite 数据库确认更新
3. 检查系统级配置
4. 记录操作日志
```

---

### Windsurf 三阶段重置逻辑

#### 阶段 1: Telemetry 修改器（20%-45%）

**目标**: 更新所有 Telemetry 设备标识符

```rust
// 1.1 处理 SQLite 数据库
for db_file in ["state.vscdb", "blob_storage.db"] {
    let conn = Connection::open(db_file)?;
    
    // 智能表结构发现
    let tables = discover_tables(&conn)?;
    // 可能的表名: ItemTable, items, KeyValueTable 等
    
    for (table, key_col, value_col) in tables {
        // 更新设备标识符（12+ 种）
        let device_keys = [
            "telemetry.machineId",
            "telemetry.macMachineId",
            "telemetry.devDeviceId",
            "telemetry.sqmId",
            "storage.serviceMachineId",
            // ... 更多
        ];
        
        for key in device_keys {
            UPDATE {table} 
            SET {value_col} = ? 
            WHERE {key_col} = ?
        }
        
        // 删除会话键（9+ 种）
        let session_keys = [
            "telemetry.sessionId",
            "telemetry.instanceId",
            "workspace.id",
            // ... 更多
        ];
        
        DELETE FROM {table} WHERE {key_col} IN (session_keys)
    }
}

// 1.2 处理 JSON 配置文件
for json_file in ["storage.json", "config.json"] {
    let mut config = read_json(json_file)?;
    config["telemetry.machineId"] = new_machine_id;
    config["telemetry.macMachineId"] = new_machine_id;
    write_json(json_file, config)?;
}

// 1.3 更新计数器
files_updated += db_files.len() + json_files.len();
keys_updated += device_keys.len() + session_keys_deleted;
```

#### 阶段 2: 数据库清理器（50%-65%）

**目标**: 清空缓存表和用户相关记录

```rust
// 2.1 清空缓存表（8+ 种）
let cache_tables = [
    "ItemTable WHERE key LIKE '%cache%'",
    "CacheStorage",
    "BlobCache",
    "FileCache",
    "NetworkCache",
    "CodeCache",
    "WorkspaceCache",
    "ExtensionCache",
];

for pattern in cache_tables {
    DELETE FROM {pattern}
    records_deleted += affected_rows;
}

// 2.2 删除关键词相关记录（10+ 种）
let keywords = [
    "user", "account", "session", "auth", 
    "token", "credential", "workspace", 
    "recent", "history", "mru"
];

for keyword in keywords {
    DELETE FROM ItemTable 
    WHERE key LIKE '%{keyword}%'
    records_deleted += affected_rows;
}

// 2.3 重置用户相关列
UPDATE ItemTable 
SET value = NULL 
WHERE key IN (
    'user.email',
    'user.name', 
    'user.avatar',
    'workbench.user'
)
```

#### 阶段 3: 缓存清理器（80%-100%）

**目标**: 递归清理所有缓存目录

```rust
// 3.1 定义缓存目录（14+ 种）
let cache_dirs = [
    "Cache",
    "CachedData", 
    "CachedExtensions",
    "Code Cache",
    "GPUCache",
    "Service Worker",
    "DawnCache",
    "logs",
    "crashDumps",
    "exthost Crash Reports",
    "User/workspaceStorage",
    "User/globalStorage",
    "User/History",
    "blob_storage",
];

// 3.2 递归清理
let mut total_size = 0;
for dir in cache_dirs {
    if dir.exists() {
        let size = calculate_dir_size(dir)?;
        remove_dir_contents(dir)?;  // 保留目录结构
        total_size += size;
        dirs_cleaned += 1;
    }
}

// 3.3 统计结果
println!("已清理 {} 个目录，释放 {} MB", 
    dirs_cleaned, 
    total_size / 1024 / 1024
);
```

---

### Qoder 重置逻辑

#### 模式 1: 完整重置

```rust
// 1. 检查 Qoder 进程
ensure_qoder_closed()?;

// 2. 备份（可选）
backup_qoder_data()?;

// 3. 深度清理（30+ 目录）
let clean_dirs = [
    "Cache", "CachedData", "Code Cache", "GPUCache",
    "Service Worker", "logs", "crashDumps",
    "User/workspaceStorage", "User/globalStorage",
    "User/History", "blob_storage",
    // ... 20+ more
];

for dir in clean_dirs {
    remove_dir_all(qoder_path.join(dir))?;
}

// 4. 删除身份文件（40+ 文件）
let identity_files = [
    "machineid", "storage.json", "state.vscdb",
    "languageModels.json", "globalState.json",
    // ... 35+ more
];

for file in identity_files {
    remove_file(qoder_path.join(file))?;
}

// 5. 清理遥测标识符（15+ 种）
clean_telemetry_identifiers()?;

// 6. 保留重要数据
preserve_mcp_configs();
preserve_chat_history();
```

#### 模式 2: 仅重置 ID

```rust
// 快速模式 - 只更新机器 ID
ensure_qoder_closed()?;

// 1. 更新 SQLite
UPDATE ItemTable 
SET value = new_id 
WHERE key IN (
    'telemetry.machineId',
    'telemetry.devDeviceId',
    'telemetry.sqmId'
);

// 2. 更新 JSON
update_json_machine_id(new_id)?;

// 3. 删除会话
DELETE FROM ItemTable 
WHERE key LIKE '%session%';
```

#### 模式 3: 实时注入（仅 Windows）

```rust
// 运行时内存修改
ensure_admin_rights()?;
ensure_qoder_running()?;

// 1. 查找 Qoder 进程
let pid = find_qoder_process()?;
let handle = OpenProcess(PROCESS_ALL_ACCESS, pid)?;

// 2. 搜索内存中的机器 ID 模式
let old_id_pattern = find_machine_id_in_memory(handle)?;

// 3. 生成新 ID
let new_id = generate_machine_id();

// 4. 直接修改内存
WriteProcessMemory(
    handle, 
    old_id_address, 
    new_id.as_bytes()
)?;

// 5. 验证
let verify_id = ReadProcessMemory(handle, old_id_address)?;
assert_eq!(verify_id, new_id);
```

---

## 🤖 GitHub Actions 自动构建

### CI/CD 工作流程

本项目使用 GitHub Actions 实现跨平台自动构建和发布。

#### 1. 构建工作流（`.github/workflows/build.yml`）

**触发条件**:
- Push 到 `main` 分支
- Pull Request 到 `main` 分支
- 手动触发

**构建矩阵**:
```yaml
strategy:
  matrix:
    platform:
      - macos-latest    # macOS (Intel + Apple Silicon)
      - ubuntu-latest   # Linux
      - windows-latest  # Windows
```

**构建步骤**:

```yaml
# 1. 环境准备
- Checkout 代码
- 安装 Node.js 18
- 安装 Rust (stable)
- 安装 pnpm

# 2. 缓存依赖
- 缓存 Rust cargo
- 缓存 Node modules

# 3. 安装系统依赖
macOS:
  - 无需额外依赖（已包含）
  
Linux:
  - libwebkit2gtk-4.0-dev
  - build-essential
  - curl
  - wget
  - file
  - libssl-dev
  - libgtk-3-dev
  - libayatana-appindicator3-dev
  - librsvg2-dev
  
Windows:
  - 无需额外依赖（已包含）

# 4. 安装项目依赖
npm install

# 5. 构建前端
npm run build

# 6. 构建 Tauri 应用
npm run tauri build

# 7. 上传构建产物
- Windows: .exe, .msi
- macOS: .dmg, .app
- Linux: .AppImage, .deb
```

#### 2. 发布工作流（`.github/workflows/release.yml`）

**触发条件**:
- 推送标签: `v*` (例如 v1.0.7)

**发布步骤**:

```yaml
# 1. 创建 GitHub Release
- 使用 标签名 作为 Release 名称
- 自动生成 Release Notes
- 标记为 Pre-release（如果包含 alpha/beta）

# 2. 跨平台构建
并行构建三个平台:

Windows:
  - 构建 .exe (NSIS 安装包)
  - 构建 .msi (MSI 安装包)
  - 签名（如果配置了代码签名证书）
  
macOS:
  - 构建 Universal Binary (Intel + Apple Silicon)
  - 构建 .dmg 安装包
  - 构建 .app
  - 公证（如果配置了 Apple 开发者账号）
  
Linux:
  - 构建 .AppImage
  - 构建 .deb (Debian/Ubuntu)
  - 构建 .rpm (Fedora/RHEL)

# 3. 上传到 Release
自动上传所有平台的安装包到 GitHub Release

# 4. 更新 latest Release
将当前 Release 标记为最新版本
```

#### 3. 自动化功能

**版本管理**:
```yaml
# 自动从 package.json 读取版本号
version=$(node -p "require('./package.json').version")

# 自动从 Cargo.toml 读取版本号
version=$(grep '^version' src-tauri/Cargo.toml | cut -d'"' -f2)

# 版本号同步检查
确保 package.json 和 Cargo.toml 版本一致
```

**构建缓存**:
```yaml
# Rust 依赖缓存
~/.cargo/registry
~/.cargo/git
target/

# Node 依赖缓存
node_modules/
.npm/

# 缓存有效期: 7 天
```

**错误处理**:
```yaml
# 构建失败时
- 发送通知到 Issue
- 保留构建日志
- 上传失败的构建产物用于调试

# 发布失败时
- 回滚 Release
- 删除不完整的 Release
- 通知维护者
```

#### 4. 使用 GitHub Actions

**创建新版本发布**:

```bash
# 1. 更新版本号
# package.json
"version": "1.0.8"

# src-tauri/Cargo.toml
version = "1.0.8"

# 2. 提交更改
git add .
git commit -m "chore: bump version to 1.0.8"
git push

# 3. 创建标签
git tag v1.0.8
git push origin v1.0.8

# 4. GitHub Actions 自动触发
# - 构建所有平台
# - 创建 Release
# - 上传安装包
```

**手动触发构建**:

```yaml
# 在 GitHub 仓库页面
Actions → Build → Run workflow → 选择分支 → Run
```

**查看构建状态**:

```yaml
# 构建状态徽章
[![Build](https://github.com/Taylor-Ding/cursor-machine-id-resetter/actions/workflows/build.yml/badge.svg)](https://github.com/Taylor-Ding/cursor-machine-id-resetter/actions/workflows/build.yml)

# 发布状态徽章  
[![Release](https://github.com/Taylor-Ding/cursor-machine-id-resetter/actions/workflows/release.yml/badge.svg)](https://github.com/Taylor-Ding/cursor-machine-id-resetter/actions/workflows/release.yml)
```

#### 5. 构建时间

| 平台 | 构建时间 | 安装包大小 |
|------|---------|-----------|
| Windows | ~8 分钟 | .exe: ~5MB<br>.msi: ~5MB |
| macOS | ~12 分钟 | .dmg: ~8MB<br>.app: ~10MB |
| Linux | ~6 分钟 | .AppImage: ~15MB<br>.deb: ~5MB |

**总构建时间**: ~15 分钟（并行构建）

#### 6. 配置文件示例

**`.github/workflows/release.yml` 核心代码**:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    strategy:
      matrix:
        platform: [macos-latest, ubuntu-latest, windows-latest]
    
    runs-on: ${{ matrix.platform }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '18'
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install dependencies (Linux)
        if: matrix.platform == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.0-dev \
            build-essential curl wget file libssl-dev \
            libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
      
      - name: Install app dependencies
        run: npm install
      
      - name: Build app
        run: npm run tauri build
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.platform }}-build
          path: src-tauri/target/release/bundle/
```

---

## 📦 构建打包

```bash
# 开发模式
npm run tauri:dev

# 生产构建
npm run tauri:build

# 平台特定
npx tauri build --bundles msi    # Windows MSI
npx tauri build --bundles dmg    # macOS DMG
npx tauri build --bundles deb    # Linux DEB
```

---

## ❓ 常见问题

### Q1: Windows 提示"权限不足"？
**A**: 右键应用图标 → "以管理员身份运行"

### Q2: 重置后 Cursor 无法启动？
**A**: 
1. 检查 Cursor 是否完全关闭
2. 从备份恢复
3. 重新安装 Cursor

### Q3: macOS 提示"无法打开应用"？
**A**: 
```bash
sudo spctl --master-disable
xattr -cr "/Applications/Cursor reset tool.app"
sudo spctl --master-enable
```

### Q4: 如何卸载？
**Windows**: 控制面板 → 卸载，删除 `%APPDATA%\cursor-machine-id-resetter`
**macOS**: 删除应用，删除 `~/Library/Application Support/cursor-machine-id-resetter`
**Linux**: `sudo apt remove cursor-reset-tool`，删除 `~/.config/cursor-machine-id-resetter`

---

## 🤝 贡献指南

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'feat: Add AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

**代码规范**: Vue Composition API, TypeScript 严格模式, Rust Clippy

---

## 📄 许可证

本项目采用 [MIT License](LICENSE) 开源协议

---

## 🙏 致谢

- [Tauri](https://tauri.app/) - 跨平台应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Element Plus](https://element-plus.org/) - Vue 3 组件库
- [Rust](https://www.rust-lang.org/) - 系统编程语言

---

<div align="center">

**如果这个项目对你有帮助，请给个 ⭐ Star 支持一下！**

Made with ❤️ by [Taylor Ding](https://github.com/Taylor-Ding)

</div>
