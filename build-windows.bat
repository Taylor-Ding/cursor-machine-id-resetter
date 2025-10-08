@echo off
REM ====================================
REM Cursor 重置工具 - Windows 构建脚本
REM ====================================

echo.
echo ========================================
echo   Cursor 重置工具 - Windows 构建
echo ========================================
echo.

REM 检查 Node.js
where node >nul 2>nul
if %errorlevel% neq 0 (
    echo [错误] 未找到 Node.js，请先安装 Node.js
    echo 下载地址: https://nodejs.org/
    pause
    exit /b 1
)

REM 检查 Rust
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [错误] 未找到 Rust，请先安装 Rust
    echo 下载地址: https://rustup.rs/
    pause
    exit /b 1
)

echo [1/4] 清理旧的构建文件...
if exist "src-tauri\target\release" (
    rmdir /s /q "src-tauri\target\release"
)
echo       完成！
echo.

echo [2/4] 安装依赖...
call npm install
if %errorlevel% neq 0 (
    echo [错误] 依赖安装失败
    pause
    exit /b 1
)
echo       完成！
echo.

echo [3/4] 开始构建...
echo       这可能需要几分钟时间，请耐心等待...
call npm run tauri:build
if %errorlevel% neq 0 (
    echo [错误] 构建失败
    pause
    exit /b 1
)
echo       完成！
echo.

echo [4/4] 检查构建结果...
if exist "src-tauri\target\release\bundle\msi" (
    echo       ✓ MSI 安装包已生成
)
if exist "src-tauri\target\release\bundle\nsis" (
    echo       ✓ NSIS 安装包已生成
)
echo.

echo ========================================
echo   构建成功！
echo ========================================
echo.
echo 输出目录: src-tauri\target\release\bundle
echo.
echo 按任意键打开输出目录...
pause >nul
explorer "src-tauri\target\release\bundle"

