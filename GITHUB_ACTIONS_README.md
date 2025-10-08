# GitHub Actions 自动构建指南 ⚡

本项目已配置自动化 CI/CD 流程，可在多平台自动构建和发布。

---

## 🎯 快速发布步骤

### 发布新版本（5 步搞定）

```bash
# 1. 更新版本号（3 个文件）
#    - src-tauri/tauri.conf.json
#    - package.json  
#    - src-tauri/Cargo.toml

# 2. 更新 CHANGELOG.md
# 3. 提交更改
git add .
git commit -m "chore: bump version to 1.0.1"
git push

# 4. 创建并推送标签
git tag v1.0.1
git push origin v1.0.1

# 5. 等待 Actions 完成（约 40-55 分钟）
# 然后在 GitHub 上发布 Draft Release
```

---

## 📦 已配置的工作流

### 1. Release Build (`.github/workflows/release.yml`)

**触发条件：**
- 推送 `v*` 标签（如 `v1.0.0`）
- 手动触发

**构建矩阵：**
| 平台 | 目标 | 输出 |
|------|------|------|
| Windows | x86_64-pc-windows-msvc | .exe, .nsis |
| macOS (Intel) | x86_64-apple-darwin | .dmg, .app |
| macOS (Apple Silicon) | aarch64-apple-darwin | .dmg, .app |
| Linux (Ubuntu 22.04) | x86_64-unknown-linux-gnu | .AppImage, .deb |

**自动操作：**
- ✅ 多平台并行构建
- ✅ 创建 Draft Release
- ✅ 上传构建产物
- ✅ 生成 Release 说明

---

### 2. Build Check (`.github/workflows/build.yml`)

**触发条件：**
- 推送到 `main` 或 `develop` 分支
- Pull Request

**检查项目：**
- ✅ 代码编译
- ✅ 运行测试
- ✅ 上传 Artifacts

**不会创建 Release**

---

## 🔧 使用说明

### 方式 1: 通过标签自动发布（推荐）

```bash
# 确保代码已提交
git add .
git commit -m "chore: ready for release"
git push

# 创建版本标签
git tag v1.0.1
git push origin v1.0.1
```

**GitHub Actions 会自动：**
1. 检测到新标签
2. 在 Windows、macOS、Linux 上并行构建
3. 创建 Draft Release
4. 上传所有构建产物

**你需要做的：**
1. 访问 Releases 页面
2. 编辑 Draft Release（可选）
3. 点击 "Publish release"

---

### 方式 2: 手动触发

如果不想创建标签：

1. 访问 **Actions** 标签
2. 选择 **Release Build** 工作流
3. 点击 **Run workflow**
4. 选择分支并运行

---

## 📋 发布前检查清单

在推送标签前，请确认：

### 版本号
- [ ] `src-tauri/tauri.conf.json` - 更新 `"version"`
- [ ] `package.json` - 更新 `"version"`
- [ ] `src-tauri/Cargo.toml` - 更新 `version`

### 文档
- [ ] `CHANGELOG.md` - 添加新版本说明
- [ ] `README.md` - 更新相关信息（如需要）

### 代码质量
- [ ] 本地构建成功：`npm run tauri:build`
- [ ] 无 Linter 错误
- [ ] 所有功能已测试

### Git
- [ ] 所有更改已提交
- [ ] 已推送到 `main` 分支
- [ ] 标签格式正确（`v*`，如 `v1.0.0`）

---

## 📊 查看构建状态

### Actions 页面

访问：`https://github.com/your-username/cursor-machine-id-resetter/actions`

**状态说明：**
- 🟢 绿色 ✓ - 构建成功
- 🔴 红色 ✗ - 构建失败
- 🟡 黄色 ● - 正在构建

### 徽章（Badges）

在 README.md 顶部显示构建状态：

```markdown
[![Release](https://github.com/your-username/cursor-machine-id-resetter/actions/workflows/release.yml/badge.svg)](...)
[![Build](https://github.com/your-username/cursor-machine-id-resetter/actions/workflows/build.yml/badge.svg)](...)
```

---

## 🎯 构建产物

### Windows
```
Cursor-reset-tool_1.0.0_x64-setup.exe  (NSIS 安装包)
```

### macOS
```
Cursor-reset-tool_1.0.0_x64.dmg        (Intel)
Cursor-reset-tool_1.0.0_aarch64.dmg    (Apple Silicon)
```

### Linux
```
cursor-reset-tool_1.0.0_amd64.AppImage  (通用格式)
cursor-reset-tool_1.0.0_amd64.deb       (Debian/Ubuntu)
```

---

## ⏱️ 预计构建时间

| 平台 | 时间 |
|------|------|
| Windows | 8-12 分钟 |
| macOS (Intel) | 10-15 分钟 |
| macOS (Apple Silicon) | 10-15 分钟 |
| Linux | 8-12 分钟 |

**总计：** 约 40-55 分钟（并行执行）

---

## ❓ 常见问题

### Q: 构建失败怎么办？

1. **查看日志**
   - Actions 页面 → 点击失败的运行
   - 查看具体错误信息

2. **常见错误**
   - 依赖安装失败 → 检查 `package.json`
   - 编译错误 → 本地测试 `npm run tauri:build`
   - 版本不一致 → 确保 3 个文件版本号相同

3. **修复并重试**
   ```bash
   # 修复问题后
   git add .
   git commit -m "fix: resolve build issue"
   git push
   
   # 删除并重新创建标签
   git tag -d v1.0.0
   git push origin :refs/tags/v1.0.0
   git tag v1.0.0
   git push origin v1.0.0
   ```

---

### Q: 如何删除错误的 Release？

1. **删除 GitHub 上的 Release**
   - Releases 页面 → 找到 Release → Delete

2. **删除标签**
   ```bash
   # 删除本地标签
   git tag -d v1.0.0
   
   # 删除远程标签
   git push origin :refs/tags/v1.0.0
   ```

3. **重新创建**
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

---

### Q: 能否只构建特定平台？

可以，修改 `.github/workflows/release.yml` 中的 `matrix`：

**只构建 Windows：**
```yaml
matrix:
  include:
    - platform: 'windows-latest'
      args: '--target x86_64-pc-windows-msvc'
```

---

### Q: 如何创建预发布版本？

使用 `beta`、`alpha`、`rc` 等后缀：

```bash
git tag v1.0.0-beta.1
git push origin v1.0.0-beta.1
```

修改 `release.yml` 中的 `prerelease` 为 `true`。

---

## 🛠️ 自定义配置

### 修改 Release 说明

编辑 `.github/workflows/release.yml` 中的 `releaseBody`:

```yaml
releaseBody: |
  ## 🎉 自定义标题
  
  ### 新增功能
  - 你的功能说明
  
  ### 下载
  - Windows: 下载 .exe
  - macOS: 下载 .dmg
  - Linux: 下载 .AppImage
```

### 添加代码签名（可选）

配置 GitHub Secrets：

- `APPLE_CERTIFICATE`
- `APPLE_PASSWORD`
- `WINDOWS_CERTIFICATE`

---

## 📚 相关资源

- [GitHub Actions 文档](https://docs.github.com/actions)
- [Tauri Action](https://github.com/tauri-apps/tauri-action)
- [语义化版本](https://semver.org/lang/zh-CN/)
- [CHANGELOG.md](CHANGELOG.md)
- [RELEASE_GUIDE.md](RELEASE_GUIDE.md)

---

## ✅ 总结

### 快速发布流程

```
1. 更新版本号（3 个文件）
   ↓
2. 更新 CHANGELOG
   ↓
3. 提交 & 推送代码
   ↓
4. 创建 & 推送标签
   ↓
5. 等待自动构建（~45 分钟）
   ↓
6. 发布 Draft Release
   ↓
7. ✅ 完成！
```

---

<div align="center">

**自动化让发布变得简单！** 🚀

有问题？查看 [RELEASE_GUIDE.md](RELEASE_GUIDE.md) 或提交 Issue

</div>

