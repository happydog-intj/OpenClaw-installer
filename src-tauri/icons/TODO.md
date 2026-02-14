# 图标 TODO

## 当前状态
❌ 缺少图标文件 - 开发模式可以运行，但会有警告

## 需要的图标文件

### 方案 A：自动生成（推荐）

1. 准备一个 1024x1024 的 PNG 源图像
2. 运行命令自动生成所有尺寸：

```bash
cd ~/Documents/Github/openclaw-installer
pnpm tauri icon /path/to/source-icon-1024.png
```

这会自动生成：
- 32x32.png
- 128x128.png
- 128x128@2x.png
- icon.icns (macOS)
- icon.ico (Windows)

### 方案 B：手动创建

如果没有设计资源，可以：

1. 使用在线工具设计：
   - [Canva](https://www.canva.com) - 创建 1024x1024 图标
   - [Figma](https://www.figma.com) - 矢量设计

2. 简单设计建议：
   - 紫色渐变背景 (#667eea → #764ba2)
   - "OC" 或 "OpenClaw" 文字
   - 圆角矩形或圆形
   - 保持简洁，图标小时也要清晰

3. 手动创建各尺寸（不推荐）

### 方案 C：临时占位（仅开发）

开发阶段可以暂时跳过图标，构建时会有警告但不影响功能。

**构建发布版前务必添加正式图标！**

## 快速检查

```bash
ls -l ~/Documents/Github/openclaw-installer/src-tauri/icons/
```

需要看到这些文件：
- ✅ icon.icns
- ✅ icon.ico  
- ✅ 32x32.png
- ✅ 128x128.png
- ✅ 128x128@2x.png
