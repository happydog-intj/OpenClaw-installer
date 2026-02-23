#!/bin/bash

echo "📤 OpenClaw Installer - 推送到 GitHub"
echo "======================================"
echo ""
echo "步骤1: 确保你已在 GitHub 上创建了仓库"
echo "       https://github.com/new"
echo "       仓库名: OpenClaw-installer"
echo ""
read -p "已创建仓库？(y/n): " created

if [ "$created" != "y" ]; then
    echo "❌ 请先在 GitHub 上创建仓库，然后重新运行此脚本"
    exit 1
fi

echo ""
read -p "请输入你的 GitHub 用户名: " username

if [ -z "$username" ]; then
    echo "❌ 用户名不能为空"
    exit 1
fi

echo ""
echo "📋 配置远程仓库..."
git remote add origin "https://github.com/$username/OpenClaw-installer.git"

if [ $? -eq 0 ]; then
    echo "✅ 远程仓库已添加"
else
    echo "⚠️  远程仓库可能已存在，尝试更新..."
    git remote set-url origin "https://github.com/$username/OpenClaw-installer.git"
fi

echo ""
echo "📤 推送代码到 GitHub..."
git push -u origin master

if [ $? -eq 0 ]; then
    echo ""
    echo "🎉 成功推送到 GitHub！"
    echo ""
    echo "🔗 仓库地址: https://github.com/$username/OpenClaw-installer"
    echo ""
    echo "下一步："
    echo "  1. 访问你的仓库查看代码"
    echo "  2. 创建 GitHub Release（参考 发布清单.md）"
    echo "  3. 上传 DMG 安装包"
else
    echo ""
    echo "❌ 推送失败！"
    echo ""
    echo "可能的原因："
    echo "  1. 仓库不存在或名称错误"
    echo "  2. 没有权限（需要配置 SSH 或输入密码）"
    echo "  3. 网络问题"
    echo ""
    echo "手动推送命令："
    echo "  git remote add origin https://github.com/$username/OpenClaw-installer.git"
    echo "  git push -u origin master"
fi
