# GitHub Actions 构建修复说明 🔧

## ❌ 原始问题

### 错误信息
```
Error: No artifacts were found.
Looking for artifacts in:
  - bundle/msi/Cursor reset tool_1.0.0_x64_en-US.msi
  - bundle/nsis/Cursor reset tool_1.0.0_x64-setup.exe
```

### 问题原因

1. **不正确的 targets 配置**
   ```json
   "targets": ["nsis"]  // ❌ 只适用于 Windows
   ```
   
2. **跨平台构建失败**
   - macOS 不支持 NSIS 格式
   - Linux 不支持 NSIS 格式
   - 导致 macOS 和 Linux 构建没有产物

3. **图标文件缺失**
   - 缺少 `icon.png`（macOS/Linux 需要）
   - 只有 `icon.ico`（仅 Windows 支持）

---

## ✅ 解决方案

### 1. 修改 Tauri 配置

**文件：** `src-tauri/tauri.conf.json`

```json
{
    "bundle": {
        "active": true,
        "targets": "all",  // ✅ 让 Tauri 自动选择平台适合的格式
        "icon": [
            "icons/icon.png"  // ✅ 跨平台通用格式
        ]
    }
}
```

### 2. 添加图标文件

```bash
# 复制 PNG 图标（从 temp-icon.png）
cp src-tauri/icons/temp-icon.png src-tauri/icons/icon.png
```

### 3. 平台自动打包格式

| 平台 | 自动生成的格式 |
|------|---------------|
| **Windows** | `.exe` (NSIS), `.msi` (WiX) |
| **macOS** | `.dmg`, `.app` |
| **Linux** | `.deb`, `.AppImage` |

---

## 🎯 配置详解

### `targets: "all"` 的含义

Tauri 会根据当前平台自动选择合适的打包格式：

```javascript
// Windows 上
"all" → ["nsis", "msi"]

// macOS 上  
"all" → ["dmg", "app"]

// Linux 上
"all" → ["deb", "appimage"]
```

### 图标文件选择

```json
"icon": ["icons/icon.png"]
```

**Tauri 自动处理：**
- 🪟 Windows: 从 PNG 生成 `.ico`
- 🍎 macOS: 从 PNG 生成 `.icns`
- 🐧 Linux: 直接使用 `.png`

**为什么不直接用 `.ico`？**
- `.ico` 是 Windows 专有格式
- macOS 和 Linux 不支持 `.ico`
- PNG 是跨平台通用格式

---

## 🔄 本地 vs GitHub Actions

### 本地构建（可能遇到下载问题）

**如果遇到 NSIS/WiX 下载失败：**

```bash
# 方法 1: 只构建可执行文件（推荐）
npm run tauri build -- --bundles none

# 方法 2: 指定特定格式
npm run tauri build -- --bundles msi

# 方法 3: 手动下载 NSIS 后构建
# 参考：手动配置NSIS.md
```

**生成的文件：**
```
src-tauri/target/release/cursor-machine-id-resetter.exe  ✅
```

---

### GitHub Actions 构建（推荐）

**优势：**
- ✅ 网络环境稳定，无下载问题
- ✅ 自动构建所有平台
- ✅ 生成完整的安装包
- ✅ 自动上传到 Release

**工作流配置：**
```yaml
# .github/workflows/release.yml
- name: Build Tauri app
  uses: tauri-apps/tauri-action@v0
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  with:
    args: ${{ matrix.args }}  # 使用 tauri.conf.json 配置
```

**触发方式：**
```bash
# 创建并推送标签
git tag v1.0.0
git push origin v1.0.0
```

**产物位置：**
- Windows: `bundle/nsis/*.exe`, `bundle/msi/*.msi`
- macOS: `bundle/dmg/*.dmg`, `bundle/macos/*.app`
- Linux: `bundle/deb/*.deb`, `bundle/appimage/*.AppImage`

---

## 📋 检查清单

构建前确认：

- [ ] ✅ `src-tauri/icons/icon.png` 存在
- [ ] ✅ `tauri.conf.json` 中 `targets: "all"`
- [ ] ✅ `tauri.conf.json` 中 `icon: ["icons/icon.png"]`
- [ ] ✅ GitHub Actions 工作流配置正确
- [ ] ✅ 已推送代码到 GitHub

---

## 🚀 发布流程

### 1. 准备发布

```bash
# 更新版本号
# 编辑 src-tauri/tauri.conf.json
{
    "version": "1.0.1"  // 更新版本
}

# 更新 CHANGELOG.md
# 添加新版本的更新内容

# 提交更改
git add .
git commit -m "chore: bump version to 1.0.1"
git push
```

### 2. 创建发布标签

```bash
# 创建标签
git tag -a v1.0.1 -m "Release version 1.0.1"

# 推送标签（触发 GitHub Actions）
git push origin v1.0.1
```

### 3. GitHub Actions 自动构建

- ⏳ 等待约 10-15 分钟
- 🔍 查看进度：GitHub → Actions 标签页
- ✅ 成功后自动创建草稿 Release

### 4. 完成发布

1. 前往 GitHub → Releases
2. 找到草稿 Release
3. 编辑发布说明（可选）
4. 点击 "Publish release"

---

## 📦 产物说明

### Windows

| 文件 | 说明 | 推荐 |
|------|------|------|
| `*-setup.exe` | NSIS 安装包 | ⭐⭐⭐⭐⭐ |
| `*.msi` | MSI 安装包 | ⭐⭐⭐ |

### macOS

| 文件 | 说明 | 推荐 |
|------|------|------|
| `*.dmg` | 磁盘镜像 | ⭐⭐⭐⭐⭐ |
| `*.app.tar.gz` | 应用程序包 | ⭐⭐⭐ |

### Linux

| 文件 | 说明 | 推荐 |
|------|------|------|
| `*.AppImage` | 便携版 | ⭐⭐⭐⭐⭐ |
| `*.deb` | Debian/Ubuntu 包 | ⭐⭐⭐⭐ |

---

## ⚠️ 常见问题

### Q: 本地构建失败，提示找不到 NSIS？

**A:** 使用以下命令跳过打包：
```bash
npm run tauri build -- --bundles none
```

### Q: GitHub Actions 构建失败？

**A:** 检查：
1. `icon.png` 文件是否存在
2. `tauri.conf.json` 配置是否正确
3. GitHub Secrets 是否配置（如需要）

### Q: 为什么本地能构建，GitHub Actions 不行？

**A:** 可能的原因：
1. 图标文件未提交到 Git
2. 配置文件不同步
3. 依赖版本不一致

**解决方法：**
```bash
# 确保所有文件已提交
git add -A
git commit -m "fix: add missing files"
git push
```

### Q: macOS 构建报错 `icon.png not found`？

**A:** 确保文件存在且已提交：
```bash
ls -la src-tauri/icons/icon.png  # 检查文件
git add src-tauri/icons/icon.png  # 添加到 Git
git commit -m "fix: add icon.png"
git push
```

---

## 📊 构建时间参考

| 平台 | 平均时间 | 说明 |
|------|---------|------|
| Windows | 8-12 分钟 | 包含 NSIS/MSI 打包 |
| macOS (x86_64) | 10-15 分钟 | 包含代码签名 |
| macOS (aarch64) | 10-15 分钟 | ARM 架构构建 |
| Linux | 6-10 分钟 | 最快 |

**总计：** 约 15-20 分钟（所有平台并行）

---

## ✅ 验证成功

构建成功的标志：

1. **GitHub Actions 全部通过**
   - ✅ Windows build 成功
   - ✅ macOS (x86_64) build 成功
   - ✅ macOS (aarch64) build 成功
   - ✅ Linux build 成功

2. **Release 页面有产物**
   - ✅ Windows 安装包（.exe, .msi）
   - ✅ macOS 磁盘镜像（.dmg）
   - ✅ Linux 包（.deb, .AppImage）

3. **所有文件可下载**
   - ✅ 文件大小合理（通常 5-15 MB）
   - ✅ 可以正常安装和运行

---

## 🎉 总结

### 关键修复点

1. ✅ **图标格式：** 使用跨平台的 PNG
2. ✅ **打包配置：** `targets: "all"` 自动适配
3. ✅ **文件完整性：** 确保所有必需文件已提交

### 最佳实践

- 🔥 **开发阶段：** 使用 `--bundles none` 快速测试
- 🚀 **发布阶段：** 使用 GitHub Actions 自动构建
- 📝 **版本管理：** 遵循语义化版本（SemVer）
- 🏷️ **Git 标签：** 用标签触发自动发布

---

<div align="center">

**问题已解决！** 🎊

现在可以正常使用 GitHub Actions 进行多平台构建了！

</div>
