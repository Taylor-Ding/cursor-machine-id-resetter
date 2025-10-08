# 手动配置 NSIS 解决打包问题

## 问题说明

Tauri 打包时需要下载 NSIS 工具，但由于网络问题下载失败：
```
failed to bundle project: `protocol: http response missing version`
```

## 解决方案

### 方案 1: 直接使用编译好的 exe（推荐）

**位置：**
```
src-tauri\target\release\cursor-machine-id-resetter.exe
```

这个 exe 文件已经完全可用，包含所有功能！

---

### 方案 2: 手动下载并配置 NSIS

#### 步骤 1: 下载 NSIS

访问以下任一地址下载：

**官方下载（推荐）：**
```
https://nsis.sourceforge.io/Download
```
下载最新版本（例如 3.09）

**或使用 Tauri 使用的版本：**
```
https://github.com/tauri-apps/binary-releases/releases/download/nsis-3/nsis-3.zip
```

#### 步骤 2: 解压 NSIS

解压到任意目录，例如：
```
C:\NSIS\
```

#### 步骤 3: 配置环境变量

**方式 1: 系统环境变量（永久）**

1. 右键"此电脑" → 属性
2. 高级系统设置 → 环境变量
3. 在"系统变量"中找到 `Path`
4. 点击"编辑" → "新建"
5. 添加 NSIS 的 bin 目录：
   ```
   C:\NSIS\Bin
   ```
6. 确定保存

**方式 2: PowerShell 临时环境变量**

```powershell
$env:Path += ";C:\NSIS\Bin"
npm run tauri:build
```

#### 步骤 4: 验证安装

```cmd
makensis /VERSION
```

如果显示版本号，说明安装成功。

#### 步骤 5: 重新打包

```cmd
npm run tauri:build
```

---

### 方案 3: 禁用安装包生成（只生成 exe）

如果只需要 exe 文件，不需要安装包：

**修改 `src-tauri\tauri.conf.json`:**

```json
{
    "bundle": {
        "active": false,  // 禁用打包
        "targets": []
    }
}
```

然后运行：
```cmd
npm run tauri:build
```

只会生成 exe，不会尝试创建安装包。

---

## 推荐方案对比

| 方案 | 优点 | 缺点 | 推荐度 |
|------|------|------|--------|
| 直接使用 exe | ✅ 无需额外配置<br>✅ 立即可用 | ⚠️ 需要手动分发 | ⭐⭐⭐⭐⭐ |
| 手动配置 NSIS | ✅ 生成正式安装包<br>✅ 更专业 | ⚠️ 需要下载配置 | ⭐⭐⭐⭐ |
| 禁用打包 | ✅ 构建快速<br>✅ 无依赖 | ⚠️ 只有 exe | ⭐⭐⭐ |

---

## 使用 exe 的最佳实践

如果选择直接使用 exe（方案 1）：

### 1. 重命名文件

```cmd
cd src-tauri\target\release
ren cursor-machine-id-resetter.exe "Cursor重置工具.exe"
```

### 2. 创建快捷方式

1. 右键 exe → 发送到 → 桌面快捷方式
2. 右键快捷方式 → 属性
3. 点击"更改图标"
4. 浏览到项目的 `src-tauri\icons\icon.ico`

### 3. 以管理员身份运行

右键快捷方式 → 属性 → 兼容性 → 勾选"以管理员身份运行此程序"

---

## 常见问题

### Q: exe 可以独立运行吗？

A: 是的！Tauri 生成的是单文件可执行程序，包含所有依赖。

### Q: exe 能在其他电脑运行吗？

A: 可以，但需要：
- Windows 10/11（需要 WebView2，通常已预装）
- 管理员权限（用于某些功能）

### Q: 安装包和 exe 有什么区别？

A: 
- **exe**: 单文件，绿色版，无需安装
- **安装包**: 会在开始菜单添加图标，注册卸载程序，更专业

---

## 总结

**对于个人使用或测试：** 直接使用 exe 即可  
**对于正式发布：** 建议手动配置 NSIS 生成安装包

当前已编译的 exe 完全可用，所有功能（包括邮箱生成）都已修复！✅

