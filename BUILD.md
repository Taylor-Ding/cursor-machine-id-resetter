# 构建和打包指南

本文档提供了在不同平台上构建和打包 Cursor 重置工具的详细说明。

## 📋 目录

- [前置要求](#前置要求)
- [环境准备](#环境准备)
- [开发模式](#开发模式)
- [生产构建](#生产构建)
- [平台特定说明](#平台特定说明)
- [常见问题](#常见问题)

---

## 前置要求

### 通用要求

- **Node.js**: 18.0.0 或更高版本
- **npm**: 9.0.0 或更高版本
- **Rust**: 1.70.0 或更高版本
- **Git**: 用于版本控制

### 平台特定要求

#### Windows
- **Visual Studio 2019/2022** 或 **Build Tools for Visual Studio**
- **WebView2 Runtime** (Windows 10/11 通常已预装)

#### macOS
- **Xcode Command Line Tools**:
  ```bash
  xcode-select --install
  ```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

#### Linux (Fedora)
```bash
sudo dnf install -y \
    webkit2gtk4.0-devel \
    openssl-devel \
    curl \
    wget \
    file \
    gtk3-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel
```

---

## 环境准备

### 1. 安装 Rust

**Windows/macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

验证安装：
```bash
rustc --version
cargo --version
```

### 2. 克隆项目

```bash
git clone <repository-url>
cd cursor-machine-id-resetter
```

### 3. 安装依赖

```bash
# 安装 Node.js 依赖
npm install

# Tauri CLI 会随依赖自动安装
```

---

## 开发模式

### 启动开发服务器

```bash
# 方式 1: 使用 npm 脚本
npm run tauri:dev

# 方式 2: 使用 Tauri CLI
npx tauri dev
```

开发服务器特性：
- ✅ 热重载 (Hot Reload)
- ✅ 实时代码检查
- ✅ 开发者工具
- ✅ 快速迭代

### 调试模式

**启用 Rust 日志：**
```bash
# Windows (PowerShell)
$env:RUST_LOG="debug"
npm run tauri:dev

# macOS/Linux
RUST_LOG=debug npm run tauri:dev
```

---

## 生产构建

### Windows 平台

#### 方式 1: 使用 npm 脚本（推荐）

```bash
npm run tauri:build
```

#### 方式 2: 使用 Tauri CLI

```bash
npx tauri build
```

#### 方式 3: 指定目标架构

```bash
# x64 架构
npx tauri build --target x86_64-pc-windows-msvc

# ARM64 架构
npx tauri build --target aarch64-pc-windows-msvc
```

#### 输出文件位置

```
src-tauri/target/release/
├── Cursor reset tool.exe           # 可执行文件
└── bundle/
    ├── msi/
    │   └── Cursor reset tool_1.0.0_x64_en-US.msi  # MSI 安装包
    └── nsis/
        └── Cursor reset tool_1.0.0_x64-setup.exe   # NSIS 安装包
```

---

### macOS 平台

#### 方式 1: 使用 npm 脚本（推荐）

```bash
npm run tauri:build
```

#### 方式 2: 使用 Tauri CLI

```bash
npx tauri build
```

#### 方式 3: 指定目标架构

```bash
# Intel (x86_64)
npx tauri build --target x86_64-apple-darwin

# Apple Silicon (M1/M2)
npx tauri build --target aarch64-apple-darwin

# 通用二进制（同时支持 Intel 和 Apple Silicon）
npx tauri build --target universal-apple-darwin
```

#### 输出文件位置

```
src-tauri/target/release/
└── bundle/
    ├── dmg/
    │   └── Cursor reset tool_1.0.0_x64.dmg        # DMG 镜像
    └── macos/
        └── Cursor reset tool.app                   # macOS 应用包
```

#### 代码签名（可选）

如果你有 Apple 开发者账号，可以进行代码签名：

```bash
# 设置环境变量
export APPLE_CERTIFICATE="Developer ID Application: Your Name (TEAM_ID)"
export APPLE_CERTIFICATE_PASSWORD="your-password"
export APPLE_SIGNING_IDENTITY="Developer ID Application"

# 构建并签名
npm run tauri:build
```

---

### Linux 平台

#### 方式 1: 使用 npm 脚本（推荐）

```bash
npm run tauri:build
```

#### 方式 2: 使用 Tauri CLI

```bash
npx tauri build
```

#### 方式 3: 构建特定格式

```bash
# AppImage
npx tauri build --bundles appimage

# Debian (.deb)
npx tauri build --bundles deb

# RPM
npx tauri build --bundles rpm
```

#### 输出文件位置

```
src-tauri/target/release/
└── bundle/
    ├── appimage/
    │   └── cursor-reset-tool_1.0.0_amd64.AppImage  # AppImage
    ├── deb/
    │   └── cursor-reset-tool_1.0.0_amd64.deb       # Debian 包
    └── rpm/
        └── cursor-reset-tool-1.0.0-1.x86_64.rpm    # RPM 包
```

---

## 构建优化

### 1. 发布模式（优化性能）

```bash
# 默认已使用发布模式
npm run tauri:build
```

### 2. 减小包体积

在 `src-tauri/Cargo.toml` 中添加：

```toml
[profile.release]
opt-level = "z"     # 优化包体积
lto = true          # 启用链接时优化
codegen-units = 1   # 更好的优化
panic = "abort"     # 减小二进制大小
strip = true        # 去除调试符号
```

### 3. 只构建特定安装包

```bash
# 仅构建 MSI (Windows)
npx tauri build --bundles msi

# 仅构建 DMG (macOS)
npx tauri build --bundles dmg

# 仅构建 AppImage (Linux)
npx tauri build --bundles appimage
```

---

## 平台特定说明

### Windows

#### 以管理员身份运行构建

某些构建步骤可能需要管理员权限：

```powershell
# PowerShell (以管理员身份)
npm run tauri:build
```

#### 生成 MSI vs NSIS

- **MSI**: 企业环境推荐，支持 Group Policy 部署
- **NSIS**: 更灵活的安装选项，文件更小

指定安装包类型：
```bash
# 仅 MSI
npx tauri build --bundles msi

# 仅 NSIS
npx tauri build --bundles nsis

# 两者都生成
npx tauri build --bundles msi,nsis
```

---

### macOS

#### 公证 (Notarization)

如果要分发给其他用户，需要公证：

```bash
# 设置环境变量
export APPLE_ID="your-apple-id@example.com"
export APPLE_PASSWORD="app-specific-password"
export APPLE_TEAM_ID="TEAM_ID"

# 构建并公证
npm run tauri:build
```

#### 构建通用二进制

```bash
# 支持 Intel 和 Apple Silicon
npx tauri build --target universal-apple-darwin
```

---

### Linux

#### 构建多种格式

```bash
# 构建所有支持的格式
npx tauri build --bundles appimage,deb,rpm
```

#### AppImage 优势

- ✅ 无需安装，直接运行
- ✅ 兼容大多数 Linux 发行版
- ✅ 便携，可放在 U 盘中

---

## 自动化构建脚本

### Windows (PowerShell)

创建 `build.ps1`:

```powershell
#!/usr/bin/env pwsh

Write-Host "开始构建 Cursor 重置工具..." -ForegroundColor Green

# 清理旧的构建文件
Write-Host "清理旧构建..." -ForegroundColor Yellow
Remove-Item -Path "src-tauri/target/release" -Recurse -Force -ErrorAction SilentlyContinue

# 安装依赖
Write-Host "安装依赖..." -ForegroundColor Yellow
npm install

# 构建
Write-Host "开始构建..." -ForegroundColor Yellow
npm run tauri:build

# 检查构建结果
if ($LASTEXITCODE -eq 0) {
    Write-Host "构建成功！" -ForegroundColor Green
    Write-Host "输出目录: src-tauri/target/release/bundle" -ForegroundColor Cyan
} else {
    Write-Host "构建失败！" -ForegroundColor Red
    exit 1
}
```

运行：
```powershell
.\build.ps1
```

---

### macOS/Linux (Bash)

创建 `build.sh`:

```bash
#!/bin/bash

set -e  # 遇到错误立即退出

echo "🚀 开始构建 Cursor 重置工具..."

# 清理旧的构建文件
echo "🧹 清理旧构建..."
rm -rf src-tauri/target/release

# 安装依赖
echo "📦 安装依赖..."
npm install

# 构建
echo "🔨 开始构建..."
npm run tauri:build

echo "✅ 构建成功！"
echo "📂 输出目录: src-tauri/target/release/bundle"
```

运行：
```bash
chmod +x build.sh
./build.sh
```

---

## 常见问题

### 1. 构建失败：找不到 Rust

**解决方案：**
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 重启终端或运行
source $HOME/.cargo/env
```

---

### 2. Windows: 找不到 MSVC

**解决方案：**
安装 Visual Studio Build Tools：
```
https://visualstudio.microsoft.com/downloads/
```

---

### 3. macOS: 权限被拒绝

**解决方案：**
```bash
# 给予可执行权限
chmod +x build.sh

# 或使用 sudo
sudo npm run tauri:build
```

---

### 4. Linux: 缺少依赖

**解决方案：**
```bash
# Ubuntu/Debian
sudo apt install -y libwebkit2gtk-4.0-dev build-essential

# Fedora
sudo dnf install -y webkit2gtk4.0-devel
```

---

### 5. 构建速度慢

**优化方案：**

1. **使用 SSD**
2. **增加 Rust 编译缓存**:
   ```bash
   export CARGO_INCREMENTAL=1
   ```
3. **使用更快的链接器 (Linux)**:
   ```bash
   cargo install -f cargo-binutils
   rustup component add llvm-tools-preview
   ```

---

### 6. 磁盘空间不足

Rust 构建需要大量临时空间。解决方案：

```bash
# 清理 Rust 缓存
cargo clean

# 清理 npm 缓存
npm cache clean --force
```

---

## 验证构建

### 检查可执行文件

**Windows:**
```powershell
# 检查文件是否存在
Test-Path "src-tauri/target/release/Cursor reset tool.exe"

# 运行
& "src-tauri/target/release/Cursor reset tool.exe"
```

**macOS:**
```bash
# 检查
ls -la "src-tauri/target/release/bundle/macos/Cursor reset tool.app"

# 运行
open "src-tauri/target/release/bundle/macos/Cursor reset tool.app"
```

**Linux:**
```bash
# 检查
ls -la src-tauri/target/release/bundle/appimage/

# 运行 AppImage
chmod +x cursor-reset-tool_*.AppImage
./cursor-reset-tool_*.AppImage
```

---

## 持续集成 (CI/CD)

### GitHub Actions 示例

创建 `.github/workflows/build.yml`:

```yaml
name: Build

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        platform: [windows-latest, macos-latest, ubuntu-latest]
    
    runs-on: ${{ matrix.platform }}
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: 18
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install dependencies (Ubuntu)
        if: matrix.platform == 'ubuntu-latest'
        run: |
          sudo apt update
          sudo apt install -y libwebkit2gtk-4.0-dev build-essential
      
      - name: Install dependencies
        run: npm install
      
      - name: Build
        run: npm run tauri:build
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.platform }}-build
          path: src-tauri/target/release/bundle/
```

---

## 总结

| 平台 | 命令 | 输出格式 |
|------|------|---------|
| Windows | `npm run tauri:build` | `.exe`, `.msi`, `.nsis` |
| macOS | `npm run tauri:build` | `.app`, `.dmg` |
| Linux | `npm run tauri:build` | `.AppImage`, `.deb`, `.rpm` |

**推荐构建流程：**

1. ✅ 克隆项目
2. ✅ 安装依赖 (`npm install`)
3. ✅ 开发测试 (`npm run tauri:dev`)
4. ✅ 生产构建 (`npm run tauri:build`)
5. ✅ 测试安装包
6. ✅ 发布

---

## 获取帮助

如果遇到问题：

1. 查看 [Tauri 文档](https://tauri.app/v1/guides/)
2. 检查 [常见问题](#常见问题)
3. 提交 Issue

祝构建顺利！🎉

