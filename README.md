# Cursor 重置工具 🚀

<div align="center">

**一个功能强大、界面美观的 Cursor IDE 重置工具**

基于 Tauri 2.0 + Vue 3 + Rust 构建

[![Release](https://github.com/your-username/cursor-machine-id-resetter/actions/workflows/release.yml/badge.svg)](https://github.com/your-username/cursor-machine-id-resetter/actions/workflows/release.yml)
[![Build](https://github.com/your-username/cursor-machine-id-resetter/actions/workflows/build.yml/badge.svg)](https://github.com/your-username/cursor-machine-id-resetter/actions/workflows/build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.4-green.svg)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![GitHub release](https://img.shields.io/github/v/release/your-username/cursor-machine-id-resetter)](https://github.com/your-username/cursor-machine-id-resetter/releases)
[![Downloads](https://img.shields.io/github/downloads/your-username/cursor-machine-id-resetter/total)](https://github.com/your-username/cursor-machine-id-resetter/releases)

[功能特性](#-功能特性) •
[快速开始](#-快速开始) •
[使用说明](#-使用说明) •
[构建打包](#-构建打包) •
[常见问题](#-常见问题) •
[贡献指南](#-贡献指南)

</div>

---

## 📋 功能特性

### 🔄 核心功能

- ✅ **一键重置机器 ID** - 快速重置 Cursor 机器标识
- ✅ **智能关闭进程** - 自动检测并关闭 Cursor 进程（两阶段：优雅→强制）
- ✅ **多重置选项** - 可选择重置存储文件、SQLite 数据库、系统级标识、应用程序补丁
- ✅ **自动备份** - 重置前自动备份原始配置，支持一键恢复
- ✅ **备份管理** - 查看历史备份、恢复任意备份、删除不需要的备份
- ✅ **状态检查** - 实时检测 Cursor 是否在运行

### 📧 邮箱生成器

- ✅ **随机账号生成** - 自动生成域名邮箱账号信息
- ✅ **完整信息** - 包含邮箱地址、密码、名字、姓氏
- ✅ **可配置参数** - 自定义密码长度（8-32位）和时间戳长度（0-8位）
- ✅ **安全密码** - 自动生成包含大小写字母、数字和特殊字符的强密码
- ✅ **一键复制** - 快速复制所有生成的信息
- ✅ **名字数据集** - 基于 36,000+ 英文名字库随机选择

### 🎨 用户界面

- ✅ **现代化设计** - 圆角扁平化设计，美观大方
- ✅ **渐变卡片** - 精美的渐变色卡片，视觉效果出色
- ✅ **响应式布局** - 适配不同屏幕尺寸
- ✅ **深色模式** - 支持明暗主题（待实现）
- ✅ **流畅动画** - 丝滑的过渡动画和悬停效果

### 🌍 多语言支持

- ✅ **中文** - 完整的简体中文支持
- ✅ **English** - Full English support
- ✅ **动态切换** - 运行时即时切换语言

### 🔒 安全特性

- ✅ **权限检测** - 智能检测操作系统权限问题
- ✅ **详细提示** - 权限不足时提供清晰的解决方案
- ✅ **数据备份** - 所有操作前自动备份，防止数据丢失
- ✅ **操作确认** - 重要操作需要用户确认

### 📝 日志系统

- ✅ **实时日志** - 所有操作的详细日志记录
- ✅ **分级日志** - Info、Success、Warning、Error 四个级别
- ✅ **时间戳** - 每条日志带有精确时间
- ✅ **搜索过滤** - 快速查找特定日志
- ✅ **一键清空** - 清空所有日志记录

### ⚙️ 设置管理

- ✅ **路径配置** - 自定义 Cursor 安装路径和数据路径
- ✅ **自动检测** - 智能检测 Cursor 安装位置
- ✅ **浏览选择** - 通过文件浏览器选择路径
- ✅ **配置持久化** - 设置自动保存到本地
- ✅ **恢复默认** - 一键恢复默认设置

### 🖥️ 跨平台支持

- ✅ **Windows 10/11** - 完整支持，包括注册表操作
- ✅ **macOS** - 支持系统配置修改（plist）
- ✅ **Linux** - 基础功能支持

---

## 🚀 快速开始

### 方式 1: 下载预编译版本（推荐）

1. 访问 [Releases](https://github.com/your-repo/releases) 页面
2. 下载对应平台的安装包：
   - **Windows**: `Cursor-reset-tool_1.0.0_x64-setup.exe` 或 `.msi`
   - **macOS**: `Cursor-reset-tool_1.0.0_x64.dmg`
   - **Linux**: `cursor-reset-tool_1.0.0_amd64.AppImage` 或 `.deb`
3. 安装并运行

### 方式 2: 从源码构建

#### 环境要求

- Node.js 18+ 
- Rust 1.70+
- npm 9+

#### 安装步骤

```bash
# 1. 克隆项目
git clone <repository-url>
cd cursor-machine-id-resetter

# 2. 安装依赖
npm install

# 3. 开发模式运行
npm run tauri:dev

# 4. 构建生产版本
npm run tauri:build
```

详细构建说明请查看 [BUILD.md](BUILD.md)

---

## 💡 使用说明

### 1️⃣ 机器 ID 重置

<details>
<summary><b>点击展开详细步骤</b></summary>

#### 步骤 1: 查看当前信息
- 启动应用后，首页自动显示当前机器 ID、Cursor 版本和备份数量

#### 步骤 2: 选择重置选项
选择要重置的内容（默认全选）：
- **重置存储文件** - `storage.json` 配置文件
- **重置 SQLite 数据库** - `state.vscdb` 数据库
- **更新系统级标识** - 注册表（Windows）或系统配置（macOS/Linux）
- **修补应用程序文件** - Workbench 相关文件

#### 步骤 3: 执行重置
1. 点击 **"开始重置"** 按钮
2. 确认重置操作
3. 工具自动关闭 Cursor 进程（如果在运行）
4. 执行重置操作
5. 显示新生成的机器 ID

#### 步骤 4: 重启 Cursor
重置完成后，重启 Cursor IDE 使更改生效

> ⚠️ **重要提示**
> - Windows 用户需要以**管理员身份**运行此工具
> - macOS 用户可能需要授予文件访问权限
> - 重置前会自动备份，可随时恢复

</details>

---

### 2️⃣ 备份恢复

<details>
<summary><b>点击展开详细步骤</b></summary>

#### 查看备份列表
切换到 **"恢复"** 标签页，查看所有历史备份：
- 备份时间
- 机器 ID
- 文件大小
- 备份路径

#### 恢复备份
1. 在备份列表中选择要恢复的备份
2. 点击 **"恢复"** 按钮
3. 确认恢复操作
4. 等待恢复完成

#### 删除备份
1. 在备份列表中找到不需要的备份
2. 点击 **"删除"** 按钮
3. 确认删除

> 💡 **提示**
> - 备份文件保存在 `%APPDATA%/cursor-machine-id-resetter/backups/` (Windows)
> - 建议定期清理旧备份释放磁盘空间

</details>

---

### 3️⃣ 邮箱生成器

<details>
<summary><b>点击展开详细步骤</b></summary>

#### 配置参数
切换到 **"邮箱生成"** 标签页：

1. **输入域名** - 例如：`example.com`
2. **设置密码长度** - 使用滑块选择 8-32 位（默认 16 位）
3. **设置时间戳长度** - 使用滑块选择 0-8 位（默认 4 位）

#### 生成账号
点击 **"生成账号"** 按钮，系统会生成：
- **邮箱地址**: `firstName + timestamp@domain`
- **密码**: 包含大小写字母、数字、特殊字符的安全密码
- **名字 (First Name)**: 从名字库随机选择
- **姓氏 (Last Name)**: 从名字库随机选择

#### 复制信息
每个字段旁边都有 **复制按钮**，一键复制到剪贴板

#### 生成规则说明

**邮箱格式:**
```
firstName + timestamp + @domain
```

**时间戳规则:**
- 随机长度：0 到设置的最大值之间随机
- 长度为 0：使用完整时间戳（10 位）
- 长度为 N：使用时间戳后 N 位

**示例:**
- 域名：`example.com`
- 名字：`john`
- 时间戳长度：4
- 当前时间戳：`1735142400`

可能的结果：
- 随机到 0：`john1735142400@example.com`
- 随机到 1：`john0@example.com`
- 随机到 2：`john00@example.com`
- 随机到 4：`john2400@example.com`

</details>

---

### 4️⃣ 日志查看

<details>
<summary><b>点击展开详细步骤</b></summary>

切换到 **"日志"** 标签页：

#### 功能
- 📜 **查看所有操作日志**
- 🔍 **搜索过滤** - 输入关键词快速查找
- 🎨 **分级显示** - 不同级别用不同颜色标识
  - 🔵 Info - 信息
  - 🟢 Success - 成功
  - 🟡 Warning - 警告
  - 🔴 Error - 错误
- 🗑️ **一键清空** - 清除所有日志

#### 使用技巧
- 日志自动滚动到最新
- 支持复制日志内容
- 可通过搜索框过滤特定日志

</details>

---

### 5️⃣ 设置配置

<details>
<summary><b>点击展开详细步骤</b></summary>

切换到 **"设置"** 标签页：

#### 路径配置

**Cursor 安装路径**
- 默认会自动检测
- 可手动输入或点击 **"浏览"** 选择
- Windows 默认: `C:\Users\YourName\AppData\Local\Programs\Cursor`

**Cursor 数据路径**
- 存储配置和数据的位置
- Windows 默认: `C:\Users\YourName\AppData\Roaming\Cursor`

#### 操作按钮

- **💾 保存设置** - 保存当前配置
- **🔄 恢复默认** - 恢复所有设置为默认值

> 💡 **提示**
> - 修改路径后需要点击"保存设置"
> - 设置会自动持久化到本地
> - 如果路径错误会导致功能异常

</details>

---

### 6️⃣ 状态检查

点击 **"检查状态"** 按钮可以：
- ✅ 检测 Cursor 是否正在运行
- ✅ 显示运行进程数量
- ✅ 提示是否需要关闭

---

## 🏗️ 技术栈

### 前端
- **框架**: Vue 3.4 (Composition API)
- **语言**: TypeScript 5.3
- **构建工具**: Vite 5.0
- **UI 组件**: Element Plus 2.5
- **图标**: Lucide Vue Next
- **样式**: Tailwind CSS 3.4
- **状态管理**: Pinia 2.1
- **国际化**: Vue I18n 9.9

### 后端
- **框架**: Tauri 2.0
- **语言**: Rust 1.70+
- **数据库**: rusqlite (SQLite)
- **Windows**: winreg (注册表操作)
- **随机**: rand 0.8
- **序列化**: serde, serde_json

### 开发工具
- **包管理**: npm
- **代码检查**: ESLint
- **类型检查**: vue-tsc
- **热重载**: Vite HMR

---

## 📦 构建打包

### 快速构建

```bash
# 开发模式
npm run tauri:dev

# 生产构建
npm run tauri:build
```

### 平台特定构建

<details>
<summary><b>Windows 构建</b></summary>

```bash
# 默认构建（生成 .exe, .msi, .nsis）
npm run tauri:build

# 仅 MSI
npx tauri build --bundles msi

# 仅 NSIS 安装包
npx tauri build --bundles nsis
```

**输出位置:** `src-tauri/target/release/bundle/`

</details>

<details>
<summary><b>macOS 构建</b></summary>

```bash
# 默认构建（生成 .app, .dmg）
npm run tauri:build

# 通用二进制（Intel + Apple Silicon）
npx tauri build --target universal-apple-darwin

# 仅 DMG
npx tauri build --bundles dmg
```

**输出位置:** `src-tauri/target/release/bundle/`

</details>

<details>
<summary><b>Linux 构建</b></summary>

```bash
# 默认构建（生成 .AppImage, .deb）
npm run tauri:build

# 仅 AppImage
npx tauri build --bundles appimage

# 仅 Debian 包
npx tauri build --bundles deb

# 仅 RPM 包
npx tauri build --bundles rpm
```

**输出位置:** `src-tauri/target/release/bundle/`

</details>

详细构建文档请查看 [BUILD.md](BUILD.md)

---

## 📁 项目结构

```
cursor-machine-id-resetter/
├── src/                          # 前端源码
│   ├── components/               # Vue 组件
│   │   ├── ResetPanel.vue       # 重置面板
│   │   ├── RestorePanel.vue     # 恢复面板
│   │   ├── EmailGeneratorPanel.vue  # 邮箱生成器
│   │   ├── LogsPanel.vue        # 日志面板
│   │   └── SettingsPanel.vue    # 设置面板
│   ├── stores/                   # Pinia 状态管理
│   │   ├── app.ts               # 应用状态
│   │   └── log.ts               # 日志状态
│   ├── i18n/                     # 国际化
│   │   └── locales/
│   │       ├── zh_cn.json       # 中文
│   │       └── en.json          # 英文
│   ├── views/                    # 页面视图
│   │   └── MainView.vue         # 主视图
│   ├── App.vue                   # 根组件
│   └── main.ts                   # 入口文件
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── core/                # 核心模块
│   │   │   ├── id_generator.rs  # ID 生成器
│   │   │   ├── resetter.rs      # 重置逻辑
│   │   │   ├── backup.rs        # 备份管理
│   │   │   ├── settings.rs      # 设置管理
│   │   │   ├── patcher.rs       # 文件修补
│   │   │   ├── system.rs        # 系统操作
│   │   │   ├── cursor_quitter.rs # 进程管理
│   │   │   └── email_generator.rs # 邮箱生成
│   │   ├── commands.rs          # Tauri 命令
│   │   ├── logger.rs            # 日志系统
│   │   └── main.rs              # 后端入口
│   ├── icons/                    # 应用图标
│   ├── Cargo.toml               # Rust 依赖
│   └── tauri.conf.json          # Tauri 配置
├── public/                       # 静态资源
├── names-dataset.txt            # 名字数据集
├── package.json                 # Node 依赖
├── vite.config.ts              # Vite 配置
├── tailwind.config.js          # Tailwind 配置
├── tsconfig.json               # TypeScript 配置
├── README.md                    # 本文件
└── BUILD.md                     # 构建文档
```

---

## 🎯 核心功能实现

### 机器 ID 生成算法

使用 UUID v4 算法生成符合标准的机器 ID：

```rust
use uuid::Uuid;

pub fn generate_machine_id() -> String {
    Uuid::new_v4()
        .to_string()
        .replace("-", "")
}
```

### 进程管理（两阶段关闭）

```rust
// 阶段 1: 优雅关闭
taskkill /PID <pid> /T

// 阶段 2: 强制关闭（如果需要）
taskkill /PID <pid> /T /F
```

### 邮箱生成逻辑

```rust
// 1. 随机选择名字
let first_name = random_from_dataset();
let last_name = random_from_dataset();

// 2. 生成时间戳
let length = random(0, max_length);
let timestamp = if length == 0 {
    full_timestamp()  // 完整时间戳
} else {
    last_n_digits(timestamp, length)
};

// 3. 组合邮箱
format!("{}{}@{}", first_name, timestamp, domain)
```

---

## ❓ 常见问题

### Q1: Windows 上提示"权限不足"怎么办？

**A:** 右键点击应用图标，选择 **"以管理员身份运行"**。

---

### Q2: 重置后 Cursor 无法启动？

**A:** 
1. 检查 Cursor 是否已完全关闭
2. 尝试从备份恢复
3. 重新安装 Cursor

---

### Q3: macOS 上提示"无法打开应用"？

**A:** 
```bash
# 方式 1: 系统设置
系统偏好设置 > 安全性与隐私 > 通用 > 仍要打开

# 方式 2: 命令行
sudo spctl --master-disable
xattr -cr "/Applications/Cursor reset tool.app"
sudo spctl --master-enable
```

---

### Q4: 邮箱生成的时间戳太长？

**A:** 调整"时间戳长度"滑块到较大值（如 6-8），这样随机到 0 的概率较小。

---

### Q5: 如何完全卸载？

**Windows:**
```powershell
# 卸载程序
控制面板 > 程序和功能 > 卸载

# 删除数据目录
Remove-Item -Recurse "$env:APPDATA\cursor-machine-id-resetter"
```

**macOS:**
```bash
# 删除应用
rm -rf "/Applications/Cursor reset tool.app"

# 删除数据
rm -rf "~/Library/Application Support/cursor-machine-id-resetter"
```

**Linux:**
```bash
# 卸载 (Debian/Ubuntu)
sudo apt remove cursor-reset-tool

# 删除数据
rm -rf "~/.config/cursor-machine-id-resetter"
```

---

### Q6: 构建时遇到 Rust 错误？

**A:** 
```bash
# 更新 Rust
rustup update

# 清理缓存
cargo clean

# 重新构建
npm run tauri:build
```

---

## 🤝 贡献指南

欢迎贡献代码！请遵循以下步骤：

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

### 代码规范

- **Vue**: 使用 Composition API
- **TypeScript**: 启用严格模式
- **Rust**: 遵循 Clippy 建议
- **提交**: 使用语义化提交信息

---

## 📄 许可证

本项目采用 [MIT License](LICENSE) 开源协议。

---

## 🙏 致谢

- [Tauri](https://tauri.app/) - 跨平台应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Element Plus](https://element-plus.org/) - Vue 3 组件库
- [Rust](https://www.rust-lang.org/) - 系统编程语言

---

## 🤝 贡献指南

我们欢迎各种形式的贡献！

### 如何贡献

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'feat: Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

### 贡献类型

- 🐛 报告 Bug
- ✨ 提出新功能
- 📝 改进文档
- 🎨 优化 UI/UX
- ⚡ 性能优化
- 🌍 添加翻译

详细信息请查看 [CONTRIBUTING.md](CONTRIBUTING.md)

---

## 📄 文档

- [README.md](README.md) - 项目说明
- [BUILD.md](BUILD.md) - 构建指南
- [QUICKSTART.md](QUICKSTART.md) - 快速开始
- [RELEASE_GUIDE.md](RELEASE_GUIDE.md) - 发布指南
- [CONTRIBUTING.md](CONTRIBUTING.md) - 贡献指南
- [CHANGELOG.md](CHANGELOG.md) - 更新日志
- [BACKUP_FEATURE.md](BACKUP_FEATURE.md) - 备份功能说明

---

## 🔄 更新日志

完整更新日志请查看 [CHANGELOG.md](CHANGELOG.md)

### v1.0.0 (2025-10-08)

#### 新增功能
- ✅ 机器 ID 重置功能
- ✅ 自动备份和恢复
- ✅ 邮箱生成器
- ✅ 多语言支持（中英文）
- ✅ 权限检测和提示
- ✅ 实时日志系统
- ✅ 设置管理
- ✅ 进程自动关闭（两阶段）

#### 优化
- ✅ 圆角扁平化 UI 设计
- ✅ 去除冗余描述文字
- ✅ 优化卡片视觉效果
- ✅ 改进错误提示

---

<div align="center">

**如果这个项目对你有帮助，请给个 ⭐ Star 支持一下！**

Made with ❤️ by [Taylor Ding]

</div>
