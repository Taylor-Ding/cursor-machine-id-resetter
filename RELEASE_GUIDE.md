# 发布指南 🚀

本文档说明如何使用 GitHub Actions 自动构建和发布 Cursor 重置工具。

---

## 📋 目录

- [前置要求](#前置要求)
- [发布流程](#发布流程)
- [GitHub Actions 工作流](#github-actions-工作流)
- [手动触发构建](#手动触发构建)
- [常见问题](#常见问题)

---

## 前置要求

### 1. GitHub 仓库设置

确保你的仓库已正确配置：

- ✅ 仓库已公开或私有（都支持）
- ✅ 已启用 GitHub Actions
- ✅ 已配置 `GITHUB_TOKEN`（自动提供）

### 2. 本地环境

- ✅ Git 已安装并配置
- ✅ 已克隆项目到本地
- ✅ 有仓库的推送权限

---

## 发布流程

### 方式 1: 通过 Git 标签自动发布（推荐）

#### 步骤 1: 更新版本号

编辑以下文件的版本号：

**`src-tauri/tauri.conf.json`:**
```json
{
    "version": "1.0.1"  // 更新版本号
}
```

**`package.json`:**
```json
{
    "version": "1.0.1"  // 更新版本号
}
```

**`src-tauri/Cargo.toml`:**
```toml
[package]
version = "1.0.1"  # 更新版本号
```

#### 步骤 2: 更新 CHANGELOG

编辑 `CHANGELOG.md`，添加新版本的更新内容：

```markdown
## [1.0.1] - 2025-10-09

### 修复
- 修复某个 bug

### 新增
- 添加某个功能
```

#### 步骤 3: 提交更改

```bash
git add .
git commit -m "chore: bump version to 1.0.1"
git push origin main
```

#### 步骤 4: 创建并推送标签

```bash
# 创建标签
git tag v1.0.1

# 推送标签到远程
git push origin v1.0.1
```

#### 步骤 5: 自动构建

推送标签后，GitHub Actions 会自动：
1. ✅ 检测到 `v*` 标签
2. ✅ 在 Windows、macOS、Linux 上并行构建
3. ✅ 创建 Draft Release
4. ✅ 上传构建产物

#### 步骤 6: 发布 Release

1. 访问 GitHub 仓库的 Releases 页面
2. 找到新创建的 Draft Release
3. 检查构建产物是否齐全
4. 编辑 Release 说明（可选）
5. 点击 **Publish Release**

---

### 方式 2: 手动触发构建

如果不想创建标签，可以手动触发：

#### 步骤 1: 访问 Actions 页面

1. 打开 GitHub 仓库
2. 点击 **Actions** 标签
3. 选择 **Release Build** 工作流

#### 步骤 2: 手动触发

1. 点击 **Run workflow** 按钮
2. 选择分支（通常是 `main`）
3. 点击 **Run workflow**

---

## GitHub Actions 工作流

### Release Build (`release.yml`)

**触发条件：**
- 推送 `v*` 标签（如 `v1.0.0`、`v1.2.3`）
- 手动触发

**构建平台：**
| 平台 | 目标 | 输出格式 |
|------|------|---------|
| Windows | x86_64-pc-windows-msvc | `.exe`, `.nsis` |
| macOS (Intel) | x86_64-apple-darwin | `.dmg`, `.app` |
| macOS (Apple Silicon) | aarch64-apple-darwin | `.dmg`, `.app` |
| Linux | x86_64-unknown-linux-gnu | `.AppImage`, `.deb` |

**工作流程：**
```
1. Checkout 代码
2. 设置 Node.js 20
3. 安装 Rust
4. 安装依赖（Ubuntu）
5. 安装前端依赖
6. 构建 Tauri 应用
7. 创建 Release
8. 上传构建产物
```

---

### Build Check (`build.yml`)

**触发条件：**
- 推送到 `main` 或 `develop` 分支
- 创建 Pull Request

**目的：**
- ✅ 验证代码可以成功编译
- ✅ 运行测试
- ✅ 上传构建产物作为 Artifacts

**不会创建 Release**

---

## 版本号规范

遵循 [语义化版本](https://semver.org/lang/zh-CN/)：

```
主版本号.次版本号.修订号

例如: 1.2.3
      │ │ │
      │ │ └─ 修订号：修复 bug
      │ └─── 次版本号：新增功能（向下兼容）
      └───── 主版本号：不兼容的 API 修改
```

**示例：**
- `v1.0.0` - 首次发布
- `v1.0.1` - 修复 bug
- `v1.1.0` - 添加新功能
- `v2.0.0` - 重大更新（不兼容）

---

## 发布检查清单

在发布新版本前，请确认：

### 代码质量
- [ ] 所有功能已测试
- [ ] 无 Linter 错误
- [ ] 无编译警告
- [ ] 已更新文档

### 版本管理
- [ ] 更新 `tauri.conf.json` 版本号
- [ ] 更新 `package.json` 版本号
- [ ] 更新 `Cargo.toml` 版本号
- [ ] 更新 `CHANGELOG.md`
- [ ] 版本号符合语义化规范

### Git 操作
- [ ] 提交所有更改
- [ ] 推送到 `main` 分支
- [ ] 创建版本标签
- [ ] 推送标签到远程

### Release 准备
- [ ] Release 说明已准备
- [ ] 截图或演示已准备
- [ ] 安装说明已更新

---

## 查看构建状态

### 方式 1: GitHub Actions 页面

1. 访问仓库的 **Actions** 标签
2. 查看工作流运行状态
3. 点击具体的运行查看详细日志

### 方式 2: 徽章

在 `README.md` 中添加状态徽章：

```markdown
[![Release](https://github.com/your-username/cursor-machine-id-resetter/actions/workflows/release.yml/badge.svg)](https://github.com/your-username/cursor-machine-id-resetter/actions/workflows/release.yml)

[![Build](https://github.com/your-username/cursor-machine-id-resetter/actions/workflows/build.yml/badge.svg)](https://github.com/your-username/cursor-machine-id-resetter/actions/workflows/build.yml)
```

---

## 构建时间参考

| 平台 | 构建时间 |
|------|---------|
| Windows | 8-12 分钟 |
| macOS (Intel) | 10-15 分钟 |
| macOS (Apple Silicon) | 10-15 分钟 |
| Linux | 8-12 分钟 |

**总计：** 约 40-55 分钟（并行执行）

---

## 下载构建产物

### Release Assets

发布后，用户可以从 Releases 页面下载：

```
https://github.com/your-username/cursor-machine-id-resetter/releases
```

**文件列表：**
```
Windows:
- Cursor-reset-tool_1.0.0_x64-setup.exe

macOS (Intel):
- Cursor-reset-tool_1.0.0_x64.dmg

macOS (Apple Silicon):
- Cursor-reset-tool_1.0.0_aarch64.dmg

Linux:
- cursor-reset-tool_1.0.0_amd64.AppImage
- cursor-reset-tool_1.0.0_amd64.deb
```

### Build Artifacts

对于开发构建（非 Release），可以从 Actions 页面下载 Artifacts：

1. 访问 Actions 页面
2. 选择构建运行
3. 滚动到底部找到 **Artifacts**
4. 下载对应平台的构建产物

---

## 常见问题

### Q1: 构建失败怎么办？

**A:** 检查以下几点：

1. **查看错误日志**
   - 访问 Actions 页面
   - 点击失败的运行
   - 查看具体的错误信息

2. **常见错误**
   - 依赖安装失败：检查 `package.json` 和 `Cargo.toml`
   - 编译错误：本地测试构建
   - 权限问题：检查 GITHUB_TOKEN

3. **解决方案**
   - 修复错误
   - 提交修复
   - 重新推送标签（需先删除旧标签）

**删除并重新推送标签：**
```bash
# 删除本地标签
git tag -d v1.0.0

# 删除远程标签
git push origin :refs/tags/v1.0.0

# 重新创建标签
git tag v1.0.0

# 重新推送标签
git push origin v1.0.0
```

---

### Q2: 如何更新已发布的 Release？

**A:** 两种方式：

**方式 1: 编辑 Release**
1. 访问 Releases 页面
2. 找到对应的 Release
3. 点击编辑按钮
4. 更新说明或上传新文件

**方式 2: 删除并重新发布**
```bash
# 删除 Release（手动在 GitHub 上删除）
# 删除标签
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0

# 重新创建标签
git tag v1.0.0
git push origin v1.0.0
```

---

### Q3: 构建产物在哪里？

**A:** 

**Release 构建：**
- 位置：`https://github.com/your-username/cursor-machine-id-resetter/releases`
- 每个 Release 下的 **Assets** 部分

**开发构建：**
- 位置：Actions 页面 → 具体运行 → Artifacts
- 保留 90 天后自动删除

---

### Q4: 如何创建预发布版本？

**A:** 修改 `release.yml` 中的 `prerelease` 字段：

```yaml
releaseBody: |
  ## ⚠️ 预发布版本
  
  这是一个测试版本，可能包含未完全测试的功能。
  
  ...
prerelease: true  # 改为 true
```

或者使用 `beta` 标签：
```bash
git tag v1.0.0-beta.1
git push origin v1.0.0-beta.1
```

---

### Q5: 能否只构建特定平台？

**A:** 可以，修改 `release.yml`：

**只构建 Windows：**
```yaml
strategy:
  matrix:
    include:
      - platform: 'windows-latest'
        args: '--target x86_64-pc-windows-msvc'
```

**只构建 macOS：**
```yaml
strategy:
  matrix:
    include:
      - platform: 'macos-latest'
        args: '--target x86_64-apple-darwin'
```

---

### Q6: 如何添加代码签名？

**A:** 需要配置证书：

**Windows (可选)：**
```yaml
env:
  WINDOWS_CERTIFICATE: ${{ secrets.WINDOWS_CERTIFICATE }}
  WINDOWS_CERTIFICATE_PASSWORD: ${{ secrets.WINDOWS_CERTIFICATE_PASSWORD }}
```

**macOS (可选)：**
```yaml
env:
  APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
  APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
  APPLE_SIGNING_IDENTITY: ${{ secrets.APPLE_SIGNING_IDENTITY }}
  APPLE_ID: ${{ secrets.APPLE_ID }}
  APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
```

---

## 最佳实践

### 1. 版本发布周期

- **主版本**：重大更新，6-12 个月
- **次版本**：新功能，1-2 个月
- **修订版本**：Bug 修复，随时

### 2. 发布前测试

```bash
# 本地构建测试
npm run tauri build

# 运行测试
cd src-tauri && cargo test

# 检查 Linter
npm run build
```

### 3. Release 说明模板

```markdown
## 🎉 新版本发布

### 新增功能
- ✨ 添加了某功能

### 修复问题
- 🐛 修复了某 bug

### 改进
- ⚡ 优化了性能

### 下载
请根据你的操作系统下载对应的安装包。

### 安装说明
详见 [README.md](README.md)
```

---

## 总结

### 快速发布流程

```bash
# 1. 更新版本号（3 个文件）
# 2. 更新 CHANGELOG.md
# 3. 提交更改
git add .
git commit -m "chore: bump version to 1.0.1"
git push

# 4. 创建并推送标签
git tag v1.0.1
git push origin v1.0.1

# 5. 等待 GitHub Actions 完成构建（40-55 分钟）
# 6. 发布 Draft Release
```

---

<div align="center">

**准备好发布了吗？祝发布顺利！** 🚀

</div>

