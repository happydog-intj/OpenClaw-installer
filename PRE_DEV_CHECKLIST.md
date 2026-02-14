# 开发前准备清单

## ✅ 已完成

- [x] 项目骨架创建
- [x] npm 依赖安装
- [x] 占位 Logo (SVG)
- [x] TypeScript 配置完善
- [x] 环境检查脚本

## 🔄 进行中

- [ ] Rust 安装（正在下载中...）

## ⏳ 待完成

### 必需（开发前）

- [ ] **Rust 安装完成**
  - 安装中，完成后需要重启终端或运行：
    ```bash
    source ~/.cargo/env
    ```
  - 验证：`rustc --version`

- [ ] **首次 Tauri 编译**
  - 第一次运行会很慢（3-5分钟）
  - 后续启动会很快

### 可选（改善体验）

- [ ] **应用图标**
  - 见 `src-tauri/icons/TODO.md`
  - 开发阶段可跳过

- [ ] **代码编辑器配置**
  - 推荐 VS Code + rust-analyzer 插件
  - 推荐 Vue Language Features (Volar) 插件

## 🚀 准备完成后

1. 重启终端或运行：
   ```bash
   source ~/.cargo/env
   ```

2. 验证 Rust 安装：
   ```bash
   ./check-env.sh
   ```

3. 启动开发服务器：
   ```bash
   pnpm tauri:dev
   ```

## 📝 首次运行注意事项

- **首次编译很慢** - 正常现象，Rust 需要编译所有依赖
- **窗口可能闪烁** - 热重载时正常
- **权限请求** - macOS 可能要求屏幕录制等权限（用于开发工具）

## 🐛 常见问题

### Rust 安装失败
```bash
# 手动安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 找不到 cargo 命令
```bash
# 重新加载环境
source ~/.cargo/env

# 或重启终端
```

### Tauri 编译错误
```bash
# 清理缓存重试
cd src-tauri
cargo clean
cd ..
pnpm tauri:dev
```

## 🎯 下一步

准备完成后，查看：
- `DEVELOPMENT.md` - 开发指南
- `NEXT_STEPS.md` - 功能路线图
