# 开发指南

## 快速开始

### 1. 安装依赖

```bash
pnpm install
```

### 2. 启动开发服务器

```bash
pnpm tauri:dev
```

这会同时启动：
- Vite 开发服务器（前端热重载）
- Tauri 应用窗口（实时预览）

## 项目架构

### 前端（Vue 3）

**页面流程：**
1. `WelcomeScreen.vue` - 欢迎页面
2. `DependencyCheck.vue` - 依赖检测
3. `InstallOptions.vue` - 安装选项
4. `InstallProgress.vue` - 安装进度
5. `SuccessScreen.vue` - 完成页面

**状态管理：**
- 使用 Vue 3 Composition API
- 通过 props 和 events 通信
- Tauri events 监听后端进度

### 后端（Rust）

**核心模块：**

1. **detector.rs** - 依赖检测
   - `check_dependencies()` - 检测所有依赖
   - `check_nodejs()` - Node.js 版本检测
   - `check_git()` - Git 检测
   - `check_package_manager()` - 包管理器检测

2. **installer.rs** - 安装逻辑
   - `install_openclaw()` - 主安装流程
   - `install_nodejs()` - 安装 Node.js
   - `install_git()` - 安装 Git
   - `run_openclaw_setup()` - 运行 openclaw setup

3. **executor.rs** - 命令执行
   - `execute_with_output()` - 实时输出执行
   - `execute()` - 简单执行
   - `command_exists()` - 检查命令是否存在

## Tauri Commands

前端可调用的 Rust 命令：

```typescript
// 获取系统信息
await invoke('get_system_info')

// 检测依赖
await invoke('check_system_dependencies')

// 安装单个依赖
await invoke('install_dependency', { name: 'nodejs' })

// 开始安装 OpenClaw
await invoke('start_installation', { 
  options: { method: 'npm' } 
})
```

## 事件系统

后端发送到前端的事件：

```typescript
import { listen } from '@tauri-apps/api/event'

// 监听安装进度
listen('install-progress', (event) => {
  console.log(event.payload)
  // {
  //   step: "安装 Node.js",
  //   status: "running",
  //   progress: 50.0,
  //   message: "正在下载...",
  //   logs: ["log line 1", "log line 2"]
  // }
})
```

## 调试技巧

### 前端调试

1. 浏览器开发者工具（Cmd+Opt+I / F12）
2. Vue DevTools（Chrome 扩展）
3. console.log() 输出

### 后端调试

1. `println!()` 输出到终端
2. `dbg!()` 宏打印变量
3. Rust 调试器（VS Code + rust-analyzer）

### 查看 Tauri 日志

```bash
# 开发模式会自动输出到终端
pnpm tauri:dev

# 或手动运行
RUST_LOG=trace pnpm tauri:dev
```

## 常见问题

### Q: 依赖检测失败？

A: 检查 `detector.rs` 中的命令路径是否正确：
```rust
Command::new("node")  // 确保 node 在 PATH 中
```

### Q: 安装命令执行失败？

A: 查看 `installer.rs` 中的权限设置：
```rust
// macOS/Linux 可能需要 sudo
Command::new("sudo")
    .args(&["brew", "install", "node"])
```

### Q: 前端无法调用后端命令？

A: 确保命令已在 `main.rs` 中注册：
```rust
.invoke_handler(tauri::generate_handler![
    get_system_info,
    check_system_dependencies,
    // ...添加你的新命令
])
```

## 构建发布版

### macOS

```bash
pnpm tauri build --target universal-apple-darwin
```

输出：`src-tauri/target/release/bundle/dmg/`

### Windows

```bash
pnpm tauri build --target x86_64-pc-windows-msvc
```

输出：`src-tauri/target/release/bundle/msi/`

### Linux

```bash
pnpm tauri build --target x86_64-unknown-linux-gnu
```

输出：`src-tauri/target/release/bundle/appimage/`

## 下一步开发

### 短期任务

- [ ] 添加图标资源（icons/ 目录）
- [ ] 实现错误页面（InstallError.vue）
- [ ] 添加日志导出功能
- [ ] 优化进度条动画
- [ ] 添加取消安装功能

### 长期优化

- [ ] 离线安装包支持
- [ ] 自动更新检测
- [ ] 多语言支持（i18n）
- [ ] 自定义主题
- [ ] 卸载功能

## 测试

### 单元测试

```bash
# Rust 测试
cd src-tauri
cargo test

# Vue 测试（需要添加）
pnpm test
```

### 手动测试清单

- [ ] 欢迎页面正确显示系统信息
- [ ] 依赖检测准确（Node.js/npm/Git）
- [ ] 安装按钮在依赖未满足时禁用
- [ ] npm 安装流程完整执行
- [ ] git 安装流程完整执行
- [ ] 进度条实时更新
- [ ] 安装成功后显示完成页面
- [ ] 日志正确显示
- [ ] 错误处理正常

## 资源

- [Tauri 官方文档](https://tauri.app)
- [Vue 3 文档](https://vuejs.org)
- [Rust 文档](https://doc.rust-lang.org)
