# 更新日志 📝

所有重要的更改都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

---

## [1.0.0] - 2025-10-08

### 🎉 首次发布

#### 新增功能

##### 核心功能
- ✅ **机器 ID 重置** - 一键重置 Cursor 机器标识
  - 重置存储文件 (storage.json)
  - 重置 SQLite 数据库 (state.vscdb)
  - 更新系统级标识（注册表/系统配置）
  - 修补应用程序文件（Workbench）

- ✅ **智能进程管理**
  - 两阶段关闭 Cursor（优雅 → 强制）
  - 自动检测 Cursor 运行状态
  - 智能超时处理

- ✅ **文件备份系统** 💾
  - 修改前自动备份 storage.json
  - 修改前自动备份 state.vscdb
  - 支持从备份恢复
  - 详细的备份日志

- ✅ **备份与恢复**
  - 自动备份机制
  - 查看历史备份列表
  - 一键恢复任意备份
  - 删除不需要的备份

##### 邮箱生成器 📧
- ✅ **随机账号生成**
  - 基于 36,000+ 名字数据集
  - 安全随机密码生成
  - 可配置域名
  - 可配置密码长度（8-32位）
  - 可配置时间戳长度（0-8位）
  - 一键复制所有信息

##### 用户界面 🎨
- ✅ **现代化设计**
  - 圆角扁平化卡片（18px 圆角）
  - 精美渐变色效果
  - 流畅的动画过渡
  - 响应式布局

- ✅ **多语言支持** 🌍
  - 完整的简体中文支持
  - 完整的 English 支持
  - 运行时动态切换

##### 日志系统 📝
- ✅ **实时日志记录**
  - 四级日志（Info、Success、Warning、Error）
  - 颜色分级显示
  - 搜索过滤功能
  - 自动滚动
  - 一键清空

##### 设置管理 ⚙️
- ✅ **路径配置**
  - Cursor 安装路径
  - Cursor 数据路径
  - 自动检测默认路径
  - 文件浏览器选择
  - 设置持久化

##### 安全特性 🔒
- ✅ **权限检测**
  - 自动识别权限错误
  - 详细的解决方案提示
  - 操作确认机制

##### 跨平台支持 💻
- ✅ **Windows 10/11**
  - 注册表操作
  - NSIS 安装包
  
- ✅ **macOS**
  - plist 配置修改
  - DMG 镜像
  - 通用二进制（Intel + Apple Silicon）
  
- ✅ **Linux**
  - 基础功能支持
  - AppImage、Deb、RPM

#### 技术栈
- **前端**: Vue 3.4 + TypeScript 5.3 + Vite 5.0
- **UI**: Element Plus 2.5 + Tailwind CSS 3.4
- **后端**: Tauri 2.0 + Rust 1.70+
- **状态管理**: Pinia 2.1
- **国际化**: Vue I18n 9.9

#### 已修复的问题
- ✅ 修复选项卡片描述文字覆盖问题
- ✅ 优化 UI 为圆角扁平化设计
- ✅ 添加权限错误智能检测和提示
- ✅ 修复 Cursor 进程关闭失败问题
- ✅ 修复状态检查不准确问题
- ✅ 修复邮箱生成逻辑与 Python 项目不一致
- ✅ 修复 WiX 下载失败导致打包失败
- ✅ 修复打包后找不到 names-dataset.txt 文件

#### 文档
- ✅ README.md - 完整的项目说明
- ✅ BUILD.md - 详细的构建和打包指南
- ✅ QUICKSTART.md - 5 分钟快速开始
- ✅ PACKAGING_FIX.md - 打包问题修复说明
- ✅ BACKUP_FEATURE.md - 文件备份功能说明
- ✅ PROJECT_SUMMARY.md - 项目完成总结
- ✅ 手动配置NSIS.md - NSIS 配置指南

#### 构建脚本
- ✅ build-windows.bat - Windows 批处理脚本
- ✅ build-windows.ps1 - Windows PowerShell 脚本
- ✅ build-macos.sh - macOS 构建脚本
- ✅ build-linux.sh - Linux 构建脚本

---

## [未来计划]

### v1.1.0（计划中）
- [ ] 深色主题支持
- [ ] 自动更新检查
- [ ] 配置导入/导出
- [ ] 更多语言支持（繁体中文、日语等）

### v1.2.0（计划中）
- [ ] 批量操作支持
- [ ] 云端备份（可选）
- [ ] 插件系统
- [ ] 主题定制

---

## 版本说明

### 语义化版本规则

```
主版本号.次版本号.修订号

主版本号: 不兼容的 API 修改
次版本号: 向下兼容的功能性新增
修订号: 向下兼容的问题修正
```

### 标签类型

- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式（不影响代码运行）
- `refactor`: 重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

---

## 贡献指南

如果你想为项目做出贡献，请：

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'feat: Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

---

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

---

<div align="center">

**感谢使用 Cursor 重置工具！** ❤️

Made with ❤️ by Taylor Ding

</div>

