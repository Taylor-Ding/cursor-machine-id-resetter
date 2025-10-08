#!/usr/bin/env pwsh
# ====================================
# Cursor 重置工具 - Windows 构建脚本 (PowerShell)
# ====================================

$ErrorActionPreference = "Stop"

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Cursor 重置工具 - Windows 构建" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 检查 Node.js
try {
    $nodeVersion = node --version
    Write-Host "✓ Node.js: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "✗ 未找到 Node.js，请先安装" -ForegroundColor Red
    Write-Host "  下载地址: https://nodejs.org/" -ForegroundColor Yellow
    exit 1
}

# 检查 Rust
try {
    $rustVersion = cargo --version
    Write-Host "✓ Rust: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "✗ 未找到 Rust，请先安装" -ForegroundColor Red
    Write-Host "  下载地址: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

Write-Host ""

# 清理旧构建
Write-Host "[1/4] 清理旧的构建文件..." -ForegroundColor Yellow
if (Test-Path "src-tauri/target/release") {
    Remove-Item -Path "src-tauri/target/release" -Recurse -Force
    Write-Host "      完成！" -ForegroundColor Green
} else {
    Write-Host "      无需清理" -ForegroundColor Gray
}
Write-Host ""

# 安装依赖
Write-Host "[2/4] 安装依赖..." -ForegroundColor Yellow
npm install
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ 依赖安装失败" -ForegroundColor Red
    exit 1
}
Write-Host "      完成！" -ForegroundColor Green
Write-Host ""

# 构建
Write-Host "[3/4] 开始构建..." -ForegroundColor Yellow
Write-Host "      这可能需要几分钟时间，请耐心等待..." -ForegroundColor Gray
npm run tauri:build
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ 构建失败" -ForegroundColor Red
    exit 1
}
Write-Host "      完成！" -ForegroundColor Green
Write-Host ""

# 检查构建结果
Write-Host "[4/4] 检查构建结果..." -ForegroundColor Yellow
$bundleDir = "src-tauri/target/release/bundle"
$foundPackages = @()

if (Test-Path "$bundleDir/msi") {
    Write-Host "      ✓ MSI 安装包已生成" -ForegroundColor Green
    $foundPackages += "MSI"
}
if (Test-Path "$bundleDir/nsis") {
    Write-Host "      ✓ NSIS 安装包已生成" -ForegroundColor Green
    $foundPackages += "NSIS"
}

if ($foundPackages.Count -eq 0) {
    Write-Host "      ⚠ 未找到安装包" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  构建成功！" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "输出目录: " -NoNewline
Write-Host "$bundleDir" -ForegroundColor Cyan
Write-Host ""
Write-Host "生成的包: " -NoNewline
Write-Host ($foundPackages -join ", ") -ForegroundColor Cyan
Write-Host ""

# 询问是否打开目录
$response = Read-Host "是否打开输出目录？(Y/n)"
if ($response -eq "" -or $response -eq "Y" -or $response -eq "y") {
    Start-Process $bundleDir
}

