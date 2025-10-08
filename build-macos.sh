#!/bin/bash
# ====================================
# Cursor 重置工具 - macOS 构建脚本
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
echo -e "${CYAN}  Cursor 重置工具 - macOS 构建${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

# 检查 Node.js
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    echo -e "${GREEN}✓ Node.js: $NODE_VERSION${NC}"
else
    echo -e "${RED}✗ 未找到 Node.js，请先安装${NC}"
    echo -e "${YELLOW}  安装命令: brew install node${NC}"
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

# 检测架构并构建
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    echo "      检测到 Apple Silicon (M1/M2)，构建通用二进制..."
    npm run tauri:build -- --target universal-apple-darwin
else
    echo "      检测到 Intel 处理器，构建 x86_64..."
    npm run tauri:build
fi

echo -e "${GREEN}      完成！${NC}"
echo ""

# 检查构建结果
echo -e "${YELLOW}[4/4] 检查构建结果...${NC}"
BUNDLE_DIR="src-tauri/target/release/bundle"

if [ -d "$BUNDLE_DIR/dmg" ]; then
    echo -e "${GREEN}      ✓ DMG 镜像已生成${NC}"
fi
if [ -d "$BUNDLE_DIR/macos" ]; then
    echo -e "${GREEN}      ✓ .app 应用包已生成${NC}"
fi

echo ""
echo -e "${CYAN}========================================${NC}"
echo -e "${GREEN}  构建成功！${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""
echo -e "输出目录: ${CYAN}$BUNDLE_DIR${NC}"
echo ""

# 询问是否打开目录
read -p "是否打开输出目录？(Y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
    open "$BUNDLE_DIR"
fi

