#!/bin/bash

# 临时解决方案：复制系统图标作为占位
# 后续需要替换为真实的 OpenClaw 图标

# 从系统复制一个图标作为基础
cp /System/Applications/Utilities/Terminal.app/Contents/Resources/Terminal.icns icon.icns 2>/dev/null || \
cp /System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/GenericApplicationIcon.icns icon.icns 2>/dev/null || \
echo "无法复制系统图标，请手动创建"

# Windows ico（需要在 Windows 上生成，这里创建占位说明）
echo "Windows icon placeholder" > icon.ico

# PNG 图标（需要图片工具生成）
echo "需要真实的 PNG 图标：32x32.png, 128x128.png, 128x128@2x.png"
echo "可以使用：pnpm tauri icon /path/to/1024x1024.png"
