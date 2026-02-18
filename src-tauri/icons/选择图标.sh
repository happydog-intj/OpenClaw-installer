#!/bin/bash

echo "🎨 OpenClaw Installer 正式图标选择器"
echo "======================================"
echo ""
echo "请查看以下4个设计方案："
echo ""
echo "1️⃣  极简几何爪子 (Geometric Claw)"
echo "   - 风格: 现代几何，抽象"
echo "   - 颜色: 深蓝背景 + 白色/紫色"
echo "   - 特点: 圆环 + 对称弧形钳子 + 字母 C"
echo "   - 适合: 科技感、专业感强"
echo ""
echo "2️⃣  金色线条艺术 (Line Art Lobster)"
echo "   - 风格: 优雅线条，精致"
echo "   - 颜色: 深紫黑背景 + 金色线条"
echo "   - 特点: 龙虾轮廓描边，细腻优雅"
echo "   - 适合: 高端品牌，艺术感"
echo ""
echo "3️⃣  现代极简 OC (Modern Minimalist)"
echo "   - 风格: 极简字母组合"
echo "   - 颜色: 深蓝灰背景 + 白色/紫色"
echo "   - 特点: 大字母 OC + 抽象爪子元素"
echo "   - 适合: 简洁明了，品牌识别强"
echo ""
echo "4️⃣  专业盾牌徽章 (Professional Shield)"
echo "   - 风格: 经典盾牌 + 剪影"
echo "   - 颜色: 深蓝背景 + 白色龙虾剪影"
echo "   - 特点: 安全感、可靠感"
echo "   - 适合: 企业级、严肃正式场合"
echo ""
echo "======================================"
echo ""
echo "预览方法："
echo "  在 Finder 中打开 src-tauri/icons/ 文件夹"
echo "  选中 icon-formal-v*.png 文件，按空格键预览"
echo ""
echo "或者在终端运行："
echo "  open src-tauri/icons/icon-formal-v1.png  # 查看方案1"
echo "  open src-tauri/icons/icon-formal-v2.png  # 查看方案2"
echo "  open src-tauri/icons/icon-formal-v3.png  # 查看方案3"
echo "  open src-tauri/icons/icon-formal-v4.png  # 查看方案4"
echo ""
echo "======================================"
echo ""
read -p "请输入你选择的方案编号 (1-4): " choice

case $choice in
    1)
        echo "✅ 你选择了: 极简几何爪子"
        cp icon-formal-v1.png icon.png
        ;;
    2)
        echo "✅ 你选择了: 金色线条艺术"
        cp icon-formal-v2.png icon.png
        ;;
    3)
        echo "✅ 你选择了: 现代极简 OC"
        cp icon-formal-v3.png icon.png
        ;;
    4)
        echo "✅ 你选择了: 专业盾牌徽章"
        cp icon-formal-v4.png icon.png
        ;;
    *)
        echo "❌ 无效选择，请输入 1-4"
        exit 1
        ;;
esac

echo ""
echo "📦 正在生成所有平台的图标..."
cd ../..
npx --yes @tauri-apps/cli icon src-tauri/icons/icon.png

echo ""
echo "✅ 图标已更新！"
echo ""
echo "下一步："
echo "  1. 运行 'pnpm tauri build --target universal-apple-darwin' 重新构建"
echo "  2. 查看新图标效果"
echo "  3. 如果满意，提交到 git"
echo ""
