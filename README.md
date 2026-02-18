# OpenClaw Installer - 一键安装工具

> 让非程序员也能轻松安装 OpenClaw AI 助手 🚀

OpenClaw Installer 是一个全可视化的安装工具，无需任何命令行经验，只需点击几下鼠标，就能完成 OpenClaw 的安装和配置。

## ✨ 为什么选择 Installer？

- **🎯 零技术门槛** - 图形化界面，像安装其他应用一样简单
- **🔍 智能检测** - 自动检测并安装所需组件（Node.js、npm 等）
- **📊 实时进度** - 清晰了解每一步的安装状态
- **🦜 一键配置飞书** - 内置飞书机器人配置向导，无需手动输入命令
- **⚡ 轻量高效** - 基于 Tauri 构建，体积小、速度快

## 📥 下载安装

### macOS

**Apple Silicon (M1/M2/M3) 和 Intel Mac 通用版本**

[⬇️ 下载 OpenClaw-Installer.dmg](https://github.com/你的用户名/OpenClaw-installer/releases/latest/download/OpenClaw-Installer_universal.dmg)

1. 下载 `.dmg` 文件
2. 双击打开
3. 将 OpenClaw Installer 拖到"应用程序"文件夹
4. 首次打开可能需要在"系统偏好设置 > 隐私与安全性"中允许

### Windows (即将支持)

🚧 Windows 版本正在开发中，敬请期待！

### Linux (即将支持)

🚧 Linux 版本正在开发中，敬请期待！

## 🚀 快速开始

### 第一步：启动安装程序

打开 OpenClaw Installer 应用，你会看到欢迎界面。

### 第二步：依赖检测（自动）

安装程序会自动检测你的系统环境：
- ✅ Node.js 22+（如果没有会自动安装）
- ✅ npm
- ✅ Git（可选）

### 第三步：安装 OpenClaw

选择安装方式：
- **npm 安装**（推荐）- 快速稳定
- **Git 安装** - 适合开发者

点击"开始安装"，等待完成即可。

### 第四步：配置向导

安装完成后，进入配置向导：

1. **工作目录** - 设置 OpenClaw 的工作空间（默认 `~/clawd`）
2. **API Keys**（可选）- 配置 AI 模型提供商
   - Qwen (通义千问)
   - Kimi (月之暗面)
   - Claude (Anthropic)
   - GPT (OpenAI)
   - 等等...
3. **飞书机器人**（可选）- 一键配置飞书接入
   - 填写 App ID 和 App Secret
   - 自动安装插件、配置凭证、重启网关
   - 实时查看安装日志

### 第五步：开始使用

配置完成后，在终端运行：

```bash
openclaw status
```

如果看到 Gateway 运行中，恭喜你，安装成功！🎉

## 🦜 飞书配置详细教程

OpenClaw Installer 内置了完整的飞书配置向导，包括：

### 自动化功能

- **智能跳转** - 填写 App ID 后，自动生成飞书开放平台各配置页面的链接
- **一键复制** - 权限 JSON 和命令一键复制到剪贴板
- **自动安装** - 填写凭证后，自动执行：
  1. 安装 @openclaw/feishu 插件
  2. 保存 App ID 和 App Secret
  3. 重启 OpenClaw 网关
  4. 显示实时安装日志

### 配置步骤

1. 创建飞书应用
2. 启用机器人能力
3. 配置权限（支持一键导入 JSON）
4. 配置事件订阅
5. 记下凭证（App ID 和 App Secret）
6. 发布应用
7. 在 Installer 中一键配置

详细教程会在配置步骤中逐步展示。

## 🛠️ 常见问题

### Q: 为什么安装失败？

**A:** 可能的原因：
- 网络连接问题 - 检查网络并重试
- 权限不足 - 确保有管理员权限
- 磁盘空间不足 - 清理至少 200MB 空间

### Q: 如何卸载？

**A:** 运行以下命令：
```bash
openclaw uninstall
```

然后删除 OpenClaw Installer 应用。

### Q: 支持哪些系统？

**A:** 
- ✅ macOS 10.15+ (Catalina 及以上)
- 🚧 Windows 10/11 (开发中)
- 🚧 Ubuntu/Debian Linux (开发中)

### Q: 安装后在哪里运行 OpenClaw？

**A:** OpenClaw 是一个命令行工具，安装后在终端中运行：
```bash
openclaw --help
```

### Q: 如何更新 OpenClaw？

**A:** 在终端运行：
```bash
openclaw update
```

## 🔗 相关链接

- [OpenClaw 官网](https://openclaw.ai)
- [OpenClaw 文档](https://docs.openclaw.ai)
- [OpenClaw GitHub](https://github.com/openclaw/openclaw)
- [飞书插件文档](https://github.com/AlexAnys/openclaw-feishu)
- [问题反馈](https://github.com/你的用户名/OpenClaw-installer/issues)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

如果你在使用中遇到问题或有功能建议，请[提交 Issue](https://github.com/你的用户名/OpenClaw-installer/issues/new)。

## 📄 开源协议

MIT License - 详见 [LICENSE](LICENSE) 文件

## 🙏 致谢

- [OpenClaw](https://github.com/openclaw/openclaw) - 强大的 AI 助手框架
- [Tauri](https://tauri.app) - 高性能跨平台应用框架
- [Vue 3](https://vuejs.org) - 渐进式 JavaScript 框架

---

**开发者**: OpenClaw 社区  
**版本**: 0.1.0  
**最后更新**: 2026-02-18

如果觉得有用，欢迎给个 ⭐ Star！
