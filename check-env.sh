#!/bin/bash

echo "🔍 OpenClaw Installer - 环境检查"
echo "================================"
echo ""

# 检查 Node.js
echo -n "Node.js: "
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    echo "✅ $NODE_VERSION"
else
    echo "❌ 未安装"
fi

# 检查 pnpm
echo -n "pnpm: "
if command -v pnpm &> /dev/null; then
    PNPM_VERSION=$(pnpm --version)
    echo "✅ $PNPM_VERSION"
else
    echo "❌ 未安装 (运行: npm install -g pnpm)"
fi

# 检查 Rust
echo -n "Rust: "
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version | awk '{print $2}')
    echo "✅ $RUST_VERSION"
else
    echo "❌ 未安装 (访问: https://rustup.rs)"
fi

# 检查 cargo
echo -n "Cargo: "
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version | awk '{print $2}')
    echo "✅ $CARGO_VERSION"
else
    echo "❌ 未安装"
fi

echo ""
echo "📁 项目文件检查"
echo "================================"

# 检查依赖安装
echo -n "npm 依赖: "
if [ -d "node_modules" ]; then
    echo "✅ 已安装"
else
    echo "⚠️  未安装 (运行: pnpm install)"
fi

# 检查图标
echo -n "应用图标: "
if [ -f "src-tauri/icons/icon.icns" ] && [ -f "src-tauri/icons/32x32.png" ]; then
    echo "✅ 已准备"
else
    echo "⚠️  缺失 (见 src-tauri/icons/TODO.md)"
fi

echo ""
echo "🎯 下一步"
echo "================================"

if [ ! -d "node_modules" ]; then
    echo "1. 安装依赖: pnpm install"
fi

if [ ! -f "src-tauri/icons/icon.icns" ]; then
    echo "2. 准备图标: 见 src-tauri/icons/TODO.md"
    echo "   (可选，开发模式可跳过)"
fi

echo "3. 启动开发: pnpm tauri:dev"

echo ""
echo "✨ 准备就绪！"
