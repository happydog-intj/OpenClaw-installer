# 📤 GitHub 推送指南

## 🚀 快速开始（推荐）

### 使用自动化脚本

```bash
cd ~/Documents/Github/OpenClaw-installer
./推送到GitHub.sh
```

脚本会自动：
1. 确认你已创建 GitHub 仓库
2. 询问你的 GitHub 用户名
3. 配置远程仓库
4. 推送所有代码

---

## 📋 手动推送步骤

### 第1步：在 GitHub 上创建仓库

1. **访问**: https://github.com/new

2. **填写信息**:
   - **Repository name**: `OpenClaw-installer`
   - **Description**: `OpenClaw 一键安装工具 - 图形化界面，零技术门槛 🦞`
   - **可见性**: 
     - ✅ **Public** (推荐) - 开源项目，任何人可见
     - 或 **Private** - 私有项目，仅你可见
   
3. **重要配置** ⚠️:
   - ❌ **不要**勾选 "Add a README file" (我们已经有了)
   - ❌ **不要**选择 ".gitignore" (已经有了)
   - ❌ **不要**选择 "license" (已经有 MIT LICENSE)

4. **点击**: "Create repository"

### 第2步：配置远程仓库

创建完仓库后，GitHub 会显示推送指令。

**替换 `YOUR_USERNAME` 为你的 GitHub 用户名**：

```bash
cd ~/Documents/Github/OpenClaw-installer

# 添加远程仓库
git remote add origin https://github.com/YOUR_USERNAME/OpenClaw-installer.git

# 查看远程仓库
git remote -v
```

### 第3步：推送代码

```bash
# 推送到 GitHub
git push -u origin master
```

**第一次推送可能需要输入 GitHub 用户名和密码（或 Personal Access Token）**

---

## 🔐 认证方式

### 方式1: HTTPS + Personal Access Token (推荐)

如果推送时提示输入密码，你需要创建 Personal Access Token：

1. **访问**: https://github.com/settings/tokens
2. **点击**: "Generate new token" → "Generate new token (classic)"
3. **填写**:
   - Note: `OpenClaw Installer`
   - Expiration: `No expiration` 或自定义
   - **勾选**: `repo` (完整的仓库访问权限)
4. **复制 token**（只显示一次！）
5. **推送时**，用户名输入你的 GitHub 用户名，密码输入 token

### 方式2: SSH (更方便，一次配置长期使用)

```bash
# 1. 生成 SSH 密钥（如果没有）
ssh-keygen -t ed25519 -C "your_email@example.com"

# 2. 复制公钥
cat ~/.ssh/id_ed25519.pub | pbcopy

# 3. 添加到 GitHub
# 访问 https://github.com/settings/keys
# 点击 "New SSH key"，粘贴公钥

# 4. 修改远程仓库地址为 SSH
cd ~/Documents/Github/OpenClaw-installer
git remote set-url origin git@github.com:YOUR_USERNAME/OpenClaw-installer.git

# 5. 推送
git push -u origin master
```

---

## ✅ 验证推送成功

推送成功后，访问你的仓库：

```
https://github.com/YOUR_USERNAME/OpenClaw-installer
```

你应该能看到：
- ✅ 所有源代码文件
- ✅ README.md 显示在首页
- ✅ 最近的 commit 记录
- ✅ 图标文件、文档等

---

## 📦 推送后的下一步

### 1. 更新 README 中的链接

编辑 `README.md`，替换占位符：

```markdown
# 替换前
[⬇️ 下载 OpenClaw-Installer.dmg](https://github.com/你的用户名/OpenClaw-installer/releases/latest/download/...)

# 替换后
[⬇️ 下载 OpenClaw-Installer.dmg](https://github.com/YOUR_USERNAME/OpenClaw-installer/releases/latest/download/OpenClaw%20Installer_0.1.0_universal.dmg)
```

然后提交并推送：
```bash
git add README.md
git commit -m "Update download links with actual GitHub username"
git push
```

### 2. 创建 GitHub Release

详见 `RELEASE_GUIDE.md` 或 `发布清单.md`

**快速步骤**：
1. 访问你的仓库
2. 点击右侧 "Releases" → "Create a new release"
3. Tag: `v0.1.0`
4. Title: `OpenClaw Installer v0.1.0 - 首个公开版本`
5. 上传 DMG 文件：
   ```
   src-tauri/target/universal-apple-darwin/release/bundle/dmg/OpenClaw Installer_0.1.0_universal.dmg
   ```
6. 点击 "Publish release"

### 3. 设置仓库描述和主题

在仓库页面右上角 "About" 旁边的齿轮图标：
- **Description**: `OpenClaw 一键安装工具 - 图形化界面，零技术门槛 🦞`
- **Website**: 如果有官网
- **Topics**: `openclaw`, `installer`, `tauri`, `vue`, `ai-assistant`, `lobster`

---

## 🐛 常见问题

### Q: 推送时提示 "Permission denied"

**A**: 认证问题，使用 Personal Access Token 或配置 SSH

### Q: 推送时提示 "Repository not found"

**A**: 
- 检查仓库是否创建成功
- 确认用户名拼写正确
- 确认远程仓库地址：`git remote -v`

### Q: 推送被拒绝 "rejected"

**A**: 
- 如果提示 "non-fast-forward"，尝试先拉取：
  ```bash
  git pull origin master --rebase
  git push -u origin master
  ```

### Q: 如何更新 GitHub 用户名？

**A**:
```bash
git remote set-url origin https://github.com/NEW_USERNAME/OpenClaw-installer.git
```

---

## 📊 当前状态

### 本地代码已准备就绪 ✅

- ✅ 所有文件已提交 (commit: 64b3244)
- ✅ 包含完整的项目代码
- ✅ 4个正式图标设计方案
- ✅ 详细的文档和指南
- ✅ DMG 安装包已构建

### 等待推送的内容

**Commits**:
1. `5ae13b4` - Release v0.1.0 - 主要功能
2. `0d63c1f` - 小龙虾 logo 图标
3. `64b3244` - 4个正式图标设计方案

**文件统计**:
- 源代码: Vue, TypeScript, Rust
- 文档: README, LICENSE, 各种指南
- 图标: 5个版本（1个卡通 + 4个正式）
- 构建产物: DMG 安装包

---

## 🎉 推送成功后

你的 OpenClaw Installer 将：
- ✅ 托管在 GitHub 上
- ✅ 可以被全世界访问（如果是 Public）
- ✅ 准备好创建 Release
- ✅ 用户可以下载使用

---

**有问题？** 运行 `./推送到GitHub.sh` 或查看本指南的相关章节！

---

**最后更新**: 2026-02-18  
**当前 commit**: 64b3244
