# OpenClaw Installer

OpenClaw 一键安装工具 - 让非程序员也能轻松安装 OpenClaw AI 助手。

## 🚀 功能特性

- ✨ **全可视化安装流程** - 无需命令行经验
- 🔍 **智能依赖检测** - 自动检测并安装所有必需组件
- 📊 **实时进度展示** - 清晰了解安装状态
- 🔧 **灵活安装选项** - 支持 npm 和 git 两种安装方式
- 🎯 **错误处理** - 友好的错误提示和恢复建议
- ⚡ **轻量高效** - 基于 Tauri，体积仅 ~8MB

## 🛠️ 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **后端**: Rust + Tauri
- **跨平台**: macOS / Windows / Linux

## 📦 开发

### 前置要求

- Node.js 22+
- Rust 1.70+
- pnpm

### 安装依赖

```bash
pnpm install
```

### 开发模式

```bash
pnpm tauri:dev
```

### 构建

```bash
# macOS
pnpm tauri build --target universal-apple-darwin

# Windows
pnpm tauri build --target x86_64-pc-windows-msvc

# Linux
pnpm tauri build --target x86_64-unknown-linux-gnu
```

## 📝 项目结构

```
openclaw-installer/
├── src/                    # 前端 Vue 代码
│   ├── components/         # UI 组件
│   ├── App.vue
│   └── main.ts
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── main.rs         # 主入口
│   │   ├── detector.rs     # 依赖检测
│   │   ├── installer.rs    # 安装逻辑
│   │   └── executor.rs     # 命令执行
│   ├── Cargo.toml
│   └── tauri.conf.json
└── package.json
```

## 🎯 安装流程

1. **欢迎页面** - 展示功能介绍
2. **依赖检测** - 检测并安装缺失组件
   - Node.js 22+
   - npm
   - Git (可选)
   - 包管理器 (Homebrew/winget/apt)
3. **安装选项** - 选择 npm 或 git 安装
4. **安装进度** - 实时显示安装状态
5. **完成页面** - 下一步操作指引

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 License

MIT License

## 🔗 相关链接

- [OpenClaw 官网](https://openclaw.ai)
- [OpenClaw 文档](https://docs.openclaw.ai)
- [OpenClaw GitHub](https://github.com/openclaw/openclaw)
