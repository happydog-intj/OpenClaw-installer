# 图标资源

在构建应用前，需要提供以下图标：

## 必需图标

- `32x32.png` - 32x32 PNG
- `128x128.png` - 128x128 PNG
- `128x128@2x.png` - 256x256 PNG (Retina)
- `icon.icns` - macOS 图标
- `icon.ico` - Windows 图标

## 快速生成

使用 Tauri 的图标生成工具：

```bash
# 从一个 1024x1024 的 PNG 生成所有尺寸
pnpm tauri icon /path/to/your/app-icon.png
```

## 临时方案

开发阶段可以使用任意图片，但构建发布版前务必替换为正式图标。

建议找一个 OpenClaw logo 或创建一个简单的图标（例如：紫色渐变背景 + "OC" 字母）。
