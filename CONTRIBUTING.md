# 贡献指南 🤝

感谢你对 Cursor 重置工具的关注！我们欢迎各种形式的贡献。

---

## 📋 目录

- [行为准则](#行为准则)
- [如何贡献](#如何贡献)
- [开发流程](#开发流程)
- [代码规范](#代码规范)
- [提交规范](#提交规范)
- [Pull Request 流程](#pull-request-流程)

---

## 行为准则

### 我们的承诺

为了营造一个开放和友好的环境，我们承诺：

- ✅ 尊重不同的观点和经验
- ✅ 接受建设性的批评
- ✅ 关注对社区最有利的事情
- ✅ 对其他社区成员表示同理心

### 不可接受的行为

- ❌ 使用性化的语言或图像
- ❌ 人身攻击或侮辱性评论
- ❌ 公开或私下骚扰
- ❌ 未经许可发布他人的私人信息

---

## 如何贡献

### 报告 Bug

发现 Bug？请：

1. **检查现有 Issues** - 确保问题尚未报告
2. **使用 Bug 模板** - 填写详细信息
3. **提供复现步骤** - 帮助我们重现问题
4. **包含环境信息** - OS、版本等

### 建议功能

有新想法？请：

1. **检查现有 Issues** - 避免重复建议
2. **使用功能请求模板** - 说明需求和场景
3. **描述清楚** - 问题、解决方案、替代方案
4. **保持耐心** - 我们会评估所有建议

### 改进文档

文档永远可以更好：

- 修复错别字
- 改进说明
- 添加示例
- 翻译文档

---

## 开发流程

### 1. Fork 项目

```bash
# 访问 GitHub 页面点击 Fork
# 然后克隆你的 fork
git clone https://github.com/your-username/cursor-machine-id-resetter.git
cd cursor-machine-id-resetter
```

### 2. 设置开发环境

**前置要求：**
- Node.js 18+
- Rust 1.70+
- npm 9+

**安装依赖：**
```bash
npm install
```

### 3. 创建分支

```bash
# 从 main 创建特性分支
git checkout -b feature/amazing-feature

# 或者修复分支
git checkout -b fix/bug-description
```

### 4. 开发

```bash
# 启动开发服务器
npm run tauri:dev
```

### 5. 测试

```bash
# 前端构建测试
npm run build

# Rust 测试
cd src-tauri
cargo test
cargo clippy
```

### 6. 提交更改

遵循[提交规范](#提交规范)：

```bash
git add .
git commit -m "feat: add amazing feature"
```

### 7. 推送并创建 PR

```bash
git push origin feature/amazing-feature
```

然后在 GitHub 上创建 Pull Request。

---

## 代码规范

### TypeScript/Vue

- 使用 **Composition API**
- 启用 TypeScript **严格模式**
- 遵循 **ESLint** 规则
- 使用 **const** 而非 let（除非需要重新赋值）
- 优先使用 **箭头函数**

**示例：**
```typescript
// ✅ 好
const handleClick = async () => {
  await invoke('command')
}

// ❌ 不好
function handleClick() {
  invoke('command')
}
```

### Rust

- 遵循 **Clippy** 建议
- 使用 **rustfmt** 格式化
- 添加必要的**文档注释**
- 避免 **unsafe** 代码（除非必要）
- 使用 **Result** 进行错误处理

**示例：**
```rust
// ✅ 好
/// 生成新的机器 ID
pub fn generate_machine_id() -> Result<String, Box<dyn std::error::Error>> {
    // ...
}

// ❌ 不好
pub fn generate_machine_id() -> String {
    // 没有错误处理
}
```

### 命名规范

| 类型 | 规范 | 示例 |
|------|------|------|
| 文件名 | kebab-case | `machine-id.ts` |
| 组件名 | PascalCase | `ResetPanel.vue` |
| 函数名 | camelCase | `handleReset()` |
| 常量 | SCREAMING_SNAKE_CASE | `MAX_RETRY` |
| Rust 函数 | snake_case | `generate_id()` |

---

## 提交规范

遵循 [Conventional Commits](https://www.conventionalcommits.org/)：

### 格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type 类型

| 类型 | 说明 | 示例 |
|------|------|------|
| `feat` | 新功能 | `feat: add email generator` |
| `fix` | Bug 修复 | `fix: resolve permission error` |
| `docs` | 文档更新 | `docs: update README` |
| `style` | 代码格式 | `style: format code` |
| `refactor` | 重构 | `refactor: simplify logic` |
| `perf` | 性能优化 | `perf: optimize query` |
| `test` | 测试 | `test: add unit tests` |
| `chore` | 构建/工具 | `chore: update deps` |

### Scope 范围（可选）

- `ui` - 用户界面
- `core` - 核心功能
- `backup` - 备份功能
- `email` - 邮箱生成
- `i18n` - 国际化
- `build` - 构建系统

### 示例

```bash
# 新功能
git commit -m "feat(email): add password length config"

# Bug 修复
git commit -m "fix(core): resolve file permission issue"

# 文档
git commit -m "docs: add build instructions"

# 重构
git commit -m "refactor(ui): improve card layout"
```

---

## Pull Request 流程

### 1. 确保质量

在提交 PR 前：

- [ ] 代码通过所有测试
- [ ] 遵循代码规范
- [ ] 更新相关文档
- [ ] 添加必要的注释

### 2. 创建 PR

- 使用 PR 模板
- 填写完整信息
- 关联相关 Issue
- 添加截图（如有 UI 改动）

### 3. 代码审查

- 回应审查意见
- 进行必要的修改
- 保持讨论友好和专业

### 4. 合并

审查通过后，维护者会合并你的 PR。

---

## 开发技巧

### 快速调试

```bash
# 开启 Rust 日志
$env:RUST_LOG="debug"  # Windows
export RUST_LOG=debug   # macOS/Linux

npm run tauri:dev
```

### 检查代码

```bash
# 前端
npm run build

# Rust
cd src-tauri
cargo clippy -- -D warnings
cargo fmt --check
```

### 构建测试

```bash
# 快速构建（不打包）
npm run tauri build -- --bundles none
```

---

## 项目结构

```
cursor-machine-id-resetter/
├── src/                    # Vue 前端
│   ├── components/        # 组件
│   ├── stores/            # Pinia 状态
│   ├── i18n/              # 国际化
│   └── views/             # 视图
├── src-tauri/             # Rust 后端
│   ├── src/
│   │   ├── core/         # 核心模块
│   │   ├── commands.rs   # Tauri 命令
│   │   └── main.rs       # 入口
│   └── Cargo.toml        # Rust 依赖
├── .github/              # GitHub 配置
│   ├── workflows/        # CI/CD
│   └── ISSUE_TEMPLATE/   # Issue 模板
└── docs/                 # 文档
```

---

## 获取帮助

遇到问题？

- 📖 查看 [README.md](README.md)
- 🔍 搜索现有 [Issues](https://github.com/your-repo/issues)
- 💬 创建新的 [Discussion](https://github.com/your-repo/discussions)
- 📧 联系维护者

---

## 许可证

通过贡献，你同意你的贡献将在 [MIT License](LICENSE) 下授权。

---

## 致谢

感谢所有贡献者！你们让这个项目变得更好。

查看所有贡献者：[Contributors](https://github.com/your-repo/graphs/contributors)

---

<div align="center">

**感谢你的贡献！** ❤️

一起让 Cursor 重置工具变得更好！

</div>

