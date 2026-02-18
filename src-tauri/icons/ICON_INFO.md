# 🦞 OpenClaw Installer 图标

## 设计说明

**主题**: 小龙虾（Lobster）  
**寓意**: OpenClaw 中的 "Claw"（爪子）呼应小龙虾的大钳子  
**风格**: 现代扁平化设计，简洁可爱

## 设计元素

### 配色方案
- **背景渐变**: `#667eea` → `#764ba2`（紫色渐变，与 OpenClaw 品牌色一致）
- **小龙虾主体**: `#FF6B6B`（鲜艳红色）
- **深色细节**: `#EE5A6F`（深红色，用于阴影和分节）
- **高光**: 白色半透明，增加立体感

### 视觉特征
- **圆角背景**: 200px 圆角，现代化设计
- **小龙虾姿态**: 正面视角，展示大钳子
- **细节元素**:
  - 可爱的大眼睛（白色眼球 + 黑色瞳孔）
  - 长触须（顶部两根）
  - 分节的身体（4-5 节）
  - 对称的大钳子
  - 装饰性小腿
- **立体效果**: 顶部白色高光，增加玻璃感

## 文件清单

所有图标均由 `@tauri-apps/cli icon` 自动生成：

### macOS
- `icon.icns` - macOS 应用图标（132KB）
- `128x128.png`
- `128x128@2x.png`
- `32x32.png`

### Windows
- `icon.ico` - Windows 应用图标（26KB）
- `Square*Logo.png` - Microsoft Store 所需的各种尺寸

### 通用
- `icon.png` - 1024x1024 主图标（34KB）

## 使用方法

### 重新生成图标

如果需要修改主图标（`icon.png`），运行以下命令重新生成所有尺寸：

```bash
cd ~/Documents/Github/OpenClaw-installer
pnpm tauri icon src-tauri/icons/icon.png
```

### 查看图标

在 macOS 上：
```bash
open src-tauri/icons/icon.png
```

在 Finder 中按空格键预览：
```bash
open src-tauri/icons/
# 选中 icon.png，按空格
```

## 版权信息

图标由 AI 生成，为 OpenClaw Installer 项目专用。  
遵循项目的 MIT 开源协议。

---

**创建日期**: 2026-02-18  
**工具**: Python PIL + Tauri CLI  
**分辨率**: 1024x1024 (主图标)
