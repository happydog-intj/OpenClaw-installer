# 📦 发布指南

## 准备工作

### 1. 确保代码已提交
```bash
git status
git add .
git commit -m "准备发布 v0.1.0"
git push
```

### 2. 创建 GitHub Release

1. 访问你的 GitHub 仓库
2. 点击右侧 "Releases" → "Create a new release"
3. 填写以下信息：

**Tag version**: `v0.1.0`  
**Release title**: `OpenClaw Installer v0.1.0 - 首个公开版本`

**Release notes**:
```markdown
# 🎉 OpenClaw Installer v0.1.0

首个公开版本！现在你可以通过图形化界面轻松安装 OpenClaw。

## ✨ 主要功能

- ✅ **全可视化安装流程** - 无需命令行经验
- ✅ **智能依赖检测** - 自动安装 Node.js 和必需组件
- ✅ **配置向导** - 引导式设置工作目录和 API Keys
- ✅ **飞书一键配置** - 内置完整的飞书机器人配置教程
  - 自动跳转到飞书开放平台配置页面
  - 一键复制权限 JSON 和命令
  - 自动安装插件、保存凭证、重启网关
  - 实时显示安装日志

## 📥 下载

### macOS (通用版本，支持 Intel 和 Apple Silicon)

- [OpenClaw-Installer_universal.dmg](链接地址)

### 系统要求

- macOS 10.15+ (Catalina 及以上)
- 至少 200MB 可用磁盘空间
- 网络连接（用于下载依赖）

## 🚀 快速开始

1. 下载并安装 OpenClaw Installer
2. 打开应用，跟随向导完成安装
3. 在终端运行 `openclaw status` 验证安装

详细文档：[README.md](链接到 README)

## 🐛 已知问题

无

## 🔮 下一步计划

- Windows 版本支持
- Linux 版本支持
- 更多聊天平台配置向导（Telegram、Discord 等）
- 离线安装包

---

如果遇到问题，欢迎[提交 Issue](链接地址)！
```

4. 上传构建好的文件：
   - 将 `src-tauri/target/universal-apple-darwin/release/bundle/dmg/OpenClaw Installer_*_universal.dmg` 拖到 "Attach binaries" 区域

5. 点击 "Publish release"

## 3. 更新 README 中的下载链接

发布后，复制 Release 中 `.dmg` 文件的下载链接，更新 README.md：

```markdown
[⬇️ 下载 OpenClaw-Installer.dmg](https://github.com/你的用户名/OpenClaw-installer/releases/latest/download/OpenClaw-Installer_universal.dmg)
```

替换为实际的链接地址。

## 4. 宣传

- 在 OpenClaw 社区发布公告
- 在相关社交媒体分享
- 更新项目主页

## 构建产物位置

构建完成后，安装包位于：

```
src-tauri/target/universal-apple-darwin/release/bundle/dmg/
```

文件名类似：`OpenClaw Installer_0.1.0_universal.dmg`

## 常见问题

### Q: 如何签名应用？

A: 需要 Apple Developer 账号，使用以下命令：
```bash
codesign --sign "Developer ID Application: Your Name" --deep "OpenClaw Installer.app"
```

### Q: 如何公证应用？

A: 需要提交到 Apple 公证服务：
```bash
xcrun notarytool submit "OpenClaw Installer.dmg" \
  --apple-id "your@email.com" \
  --password "app-specific-password" \
  --team-id "TEAM_ID"
```

### Q: 构建失败怎么办？

A: 常见问题：
- 检查 Node.js 版本 (`node --version`)
- 检查 Rust 版本 (`rustc --version`)
- 清理缓存：`cargo clean && pnpm tauri build`

---

**提示**: 首次发布建议在小范围内测试，确认无误后再大规模推广。
