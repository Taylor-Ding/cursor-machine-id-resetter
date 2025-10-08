#!/bin/bash
# ====================================
# Cursor 重置工具 - Linux 构建脚本
# ====================================

set -e  # 遇到错误立即退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo ""
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}  Cursor 重置工具 - Linux 构建${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

# 检查 Node.js
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    echo -e "${GREEN}✓ Node.js: $NODE_VERSION${NC}"
else
    echo -e "${RED}✗ 未找到 Node.js，请先安装${NC}"
    echo -e "${YELLOW}  Ubuntu/Debian: sudo apt install nodejs npm${NC}"
    echo -e "${YELLOW}  Fedora: sudo dnf install nodejs${NC}"
    exit 1
fi

# 检查 Rust
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(cargo --version)
    echo -e "${GREEN}✓ Rust: $RUST_VERSION${NC}"
else
    echo -e "${RED}✗ 未找到 Rust，请先安装${NC}"
    echo -e "${YELLOW}  安装命令: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
    exit 1
fi

# 检查系统依赖
echo ""
echo -e "${YELLOW}检查系统依赖...${NC}"

# 检测发行版
if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO=$ID
else
    DISTRO="unknown"
fi

MISSING_DEPS=()

# 检查 WebKit
if ! pkg-config --exists webkit2gtk-4.0 2>/dev/null; then
    MISSING_DEPS+=("webkit2gtk")
fi

# 检查 GTK
if ! pkg-config --exists gtk+-3.0 2>/dev/null; then
    MISSING_DEPS+=("gtk3")
fi

if [ ${#MISSING_DEPS[@]} -ne 0 ]; then
    echo -e "${RED}✗ 缺少系统依赖${NC}"
    echo -e "${YELLOW}缺少的依赖: ${MISSING_DEPS[*]}${NC}"
    echo ""
    
    case $DISTRO in
        ubuntu|debian|linuxmint|pop)
            echo -e "${YELLOW}请运行以下命令安装依赖:${NC}"
            echo "sudo apt update"
            echo "sudo apt install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev"
            ;;
        fedora|rhel|centos)
            echo -e "${YELLOW}请运行以下命令安装依赖:${NC}"
            echo "sudo dnf install -y webkit2gtk4.0-devel openssl-devel curl wget file gtk3-devel libappindicator-gtk3-devel librsvg2-devel"
            ;;
        arch|manjaro)
            echo -e "${YELLOW}请运行以下命令安装依赖:${NC}"
            echo "sudo pacman -S --needed webkit2gtk base-devel curl wget file openssl gtk3 libappindicator-gtk3 librsvg"
            ;;
        *)
            echo -e "${YELLOW}请参考 Tauri 文档安装系统依赖${NC}"
            ;;
    esac
    
    exit 1
else
    echo -e "${GREEN}✓ 系统依赖已满足${NC}"
fi

echo ""

# 清理旧构建
echo -e "${YELLOW}[1/4] 清理旧的构建文件...${NC}"
if [ -d "src-tauri/target/release" ]; then
    rm -rf src-tauri/target/release
    echo -e "${GREEN}      完成！${NC}"
else
    echo -e "      无需清理"
fi
echo ""

# 安装依赖
echo -e "${YELLOW}[2/4] 安装依赖...${NC}"
npm install
echo -e "${GREEN}      完成！${NC}"
echo ""

# 构建
echo -e "${YELLOW}[3/4] 开始构建...${NC}"
echo "      这可能需要几分钟时间，请耐心等待..."
echo "      构建格式: AppImage, Deb"
npm run tauri:build
echo -e "${GREEN}      完成！${NC}"
echo ""

# 检查构建结果
echo -e "${YELLOW}[4/4] 检查构建结果...${NC}"
BUNDLE_DIR="src-tauri/target/release/bundle"
FOUND_PACKAGES=()

if [ -d "$BUNDLE_DIR/appimage" ]; then
    echo -e "${GREEN}      ✓ AppImage 已生成${NC}"
    FOUND_PACKAGES+=("AppImage")
fi
if [ -d "$BUNDLE_DIR/deb" ]; then
    echo -e "${GREEN}      ✓ Debian 包已生成${NC}"
    FOUND_PACKAGES+=("Deb")
fi
if [ -d "$BUNDLE_DIR/rpm" ]; then
    echo -e "${GREEN}      ✓ RPM 包已生成${NC}"
    FOUND_PACKAGES+=("RPM")
fi

if [ ${#FOUND_PACKAGES[@]} -eq 0 ]; then
    echo -e "${YELLOW}      ⚠ 未找到安装包${NC}"
fi

echo ""
echo -e "${CYAN}========================================${NC}"
echo -e "${GREEN}  构建成功！${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""
echo -e "输出目录: ${CYAN}$BUNDLE_DIR${NC}"
echo -e "生成的包: ${CYAN}${FOUND_PACKAGES[*]}${NC}"
echo ""

# AppImage 设置可执行权限
if [ -d "$BUNDLE_DIR/appimage" ]; then
    echo -e "${YELLOW}设置 AppImage 可执行权限...${NC}"
    chmod +x "$BUNDLE_DIR/appimage"/*.AppImage
    echo -e "${GREEN}完成！${NC}"
    echo ""
fi

# 询问是否打开目录
read -p "是否打开输出目录？(Y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
    if command -v xdg-open &> /dev/null; then
        xdg-open "$BUNDLE_DIR"
    elif command -v nautilus &> /dev/null; then
        nautilus "$BUNDLE_DIR"
    elif command -v dolphin &> /dev/null; then
        dolphin "$BUNDLE_DIR"
    else
        echo -e "${YELLOW}请手动打开目录: $BUNDLE_DIR${NC}"
    fi
fi

