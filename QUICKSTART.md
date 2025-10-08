# 快速开始指南 🚀

本指南帮助你在 5 分钟内快速构建和运行 Cursor 重置工具。

---

## 📋 前置要求

确保你的系统已安装：

- ✅ **Node.js 18+** - [下载地址](https://nodejs.org/)
- ✅ **Rust 1.70+** - [安装指南](https://rustup.rs/)

### 快速检查

```bash
# 检查 Node.js
node --version

# 检查 Rust
cargo --version
```

如果显示版本号，说明已正确安装。

---

## ⚡ 一键构建

### Windows 用户

**方式 1: 批处理脚本（推荐）**
```cmd
双击运行: build-windows.bat
```

**方式 2: PowerShell 脚本**
```powershell
.\build-windows.ps1
```

**方式 3: 手动命令**
```cmd
npm install
npm run tauri:build
```

---

### macOS 用户

**方式 1: Shell 脚本（推荐）**
```bash
chmod +x build-macos.sh
./build-macos.sh
```

**方式 2: 手动命令**
```bash
npm install
npm run tauri:build
```

---

### Linux 用户

**方式 1: Shell 脚本（推荐）**
```bash
chmod +x build-linux.sh
./build-linux.sh
```

**方式 2: 手动命令**
```bash
# 1. 安装系统依赖（仅首次需要）
# Ubuntu/Debian
sudo apt install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev

# Fedora
sudo dnf install -y webkit2gtk4.0-devel openssl-devel curl wget file gtk3-devel

# 2. 安装项目依赖并构建
npm install
npm run tauri:build
```

---

## 🎯 输出文件位置

构建完成后，安装包位于：

```
src-tauri/target/release/bundle/
```

### Windows
- `msi/` - MSI 安装包
- `nsis/` - NSIS 安装包

### macOS
- `dmg/` - DMG 镜像文件
- `macos/` - .app 应用包

### Linux
- `appimage/` - AppImage（推荐）
- `deb/` - Debian 包
- `rpm/` - RPM 包（如果生成）

---

## 🔧 开发模式

如果你想开发或测试：

```bash
# 1. 安装依赖（仅首次）
npm install

# 2. 启动开发服务器
npm run tauri:dev
```

开发模式特性：
- ✅ 热重载
- ✅ 开发者工具
- ✅ 实时预览

---

## 📦 只构建特定格式

### Windows - 只构建 MSI
```bash
npx tauri build --bundles msi
```

### macOS - 只构建 DMG
```bash
npx tauri build --bundles dmg
```

### Linux - 只构建 AppImage
```bash
npx tauri build --bundles appimage
```

---

## ❓ 常见问题

### Q: 构建失败，提示找不到 Rust？

**A:** 安装 Rust：
```bash
# Windows/macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 重启终端后再次尝试
```

---

### Q: Windows 构建失败，提示找不到 MSVC？

**A:** 安装 Visual Studio Build Tools：
1. 访问 https://visualstudio.microsoft.com/downloads/
2. 下载 "Build Tools for Visual Studio"
3. 安装时选择 "C++ build tools"

---

### Q: macOS 提示权限错误？

**A:** 给脚本添加可执行权限：
```bash
chmod +x build-macos.sh
```

---

### Q: Linux 缺少依赖？

**A:** 
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y libwebkit2gtk-4.0-dev build-essential

# Fedora
sudo dnf install -y webkit2gtk4.0-devel
```

---

### Q: 构建太慢？

**A:** 第一次构建会下载依赖，通常需要 5-10 分钟。后续构建会快很多（1-2 分钟）。

**加速技巧：**
1. 使用 SSD
2. 关闭杀毒软件
3. 使用国内镜像（中国用户）：
   ```bash
   # Rust 镜像
   export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
   
   # npm 镜像
   npm config set registry https://registry.npmmirror.com
   ```

---

## 🎉 构建成功后

### 测试安装包

**Windows:**
```cmd
# 运行可执行文件
src-tauri\target\release\Cursor reset tool.exe

# 或安装 MSI
双击 .msi 文件
```

**macOS:**
```bash
# 打开 app
open "src-tauri/target/release/bundle/macos/Cursor reset tool.app"

# 或打开 DMG
open "src-tauri/target/release/bundle/dmg/"
```

**Linux:**
```bash
# 运行 AppImage
chmod +x cursor-reset-tool_*.AppImage
./cursor-reset-tool_*.AppImage

# 或安装 Deb
sudo dpkg -i cursor-reset-tool_*.deb
```

---

## 📚 下一步

- 📖 阅读完整 [README.md](README.md) 了解所有功能
- 🔨 查看 [BUILD.md](BUILD.md) 了解高级构建选项
- 💬 有问题？提交 [Issue](https://github.com/your-repo/issues)

---

## 🔄 更新项目

```bash
# 拉取最新代码
git pull

# 清理旧依赖
rm -rf node_modules
npm install

# 重新构建
npm run tauri:build
```

---

## 📊 构建时间参考

| 平台 | 首次构建 | 后续构建 |
|------|---------|---------|
| Windows | 8-12 分钟 | 2-3 分钟 |
| macOS | 10-15 分钟 | 2-4 分钟 |
| Linux | 10-15 分钟 | 2-4 分钟 |

*时间取决于硬件配置和网络速度*

---

## 💡 提示

- ✅ 首次构建会比较慢，请耐心等待
- ✅ 确保有足够的磁盘空间（至少 5GB）
- ✅ 构建期间不要中断进程
- ✅ 使用脚本构建更简单（自动处理依赖）
- ✅ 遇到问题先查看终端错误信息

---

<div align="center">

**准备好了吗？选择你的平台开始构建！** 🚀

[Windows](#windows-用户) • [macOS](#macos-用户) • [Linux](#linux-用户)

</div>

