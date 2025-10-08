# 图标文件

由于图标文件是二进制文件，需要您自行提供或使用在线工具生成。

## 🎨 生成图标

### 方法1: 使用在线工具

访问：https://www.favicon-generator.org/

1. 上传一张 PNG 图片（建议 512x512 或更大）
2. 下载生成的图标包
3. 将以下文件复制到此目录：
   - `icon.ico` (Windows)
   - `icon.icns` (macOS)
   - `32x32.png`
   - `128x128.png`
   - `128x128@2x.png`

### 方法2: 使用命令行工具

**安装 ImageMagick:**
```bash
# Windows (使用 Chocolatey)
choco install imagemagick

# 或从官网下载: https://imagemagick.org/
```

**生成图标:**
```bash
# 准备一张 512x512 的 PNG 图片作为源文件 (logo.png)

# 生成 Windows .ico
magick logo.png -define icon:auto-resize=256,128,64,48,32,16 icon.ico

# 生成 macOS .icns
png2icns icon.icns logo.png

# 生成各种尺寸的 PNG
magick logo.png -resize 32x32 32x32.png
magick logo.png -resize 128x128 128x128.png
magick logo.png -resize 256x256 128x128@2x.png
```

## 📦 临时解决方案

如果您现在想快速启动项目，可以：

1. 注释掉 tauri.conf.json 中的图标配置
2. 或使用系统默认图标

编辑 `tauri.conf.json`:
```json
{
  "bundle": {
    "icon": []  // 暂时清空
  }
}
```

