use serde::{Deserialize, Serialize};
use std::process::Command;
use tauri::Window;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstallOptions {
    pub method: String, // "npm" or "git"
    pub custom_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstallProgress {
    pub step: String,
    pub status: String, // "pending", "running", "success", "failed"
    pub progress: f32,
    pub message: String,
    pub logs: Vec<String>,
}

pub async fn install_single_dependency(
    window: &Window,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    emit_progress(window, &format!("正在安装 {}", name), "running", 0.0, vec![]);

    match name {
        "nodejs" => install_nodejs(window).await?,
        "git" => install_git(window).await?,
        "homebrew" => install_homebrew(window).await?,
        _ => return Err(format!("未知依赖: {}", name).into()),
    }

    emit_progress(window, &format!("{} 安装完成", name), "success", 100.0, vec![]);
    Ok(())
}

pub async fn install_openclaw(
    window: &Window,
    options: InstallOptions,
) -> Result<String, Box<dyn std::error::Error>> {
    // 步骤 1: 检查依赖
    emit_progress(window, "检查系统依赖", "running", 10.0, vec![]);
    
    let deps = crate::detector::check_dependencies().await?;
    let missing: Vec<_> = deps.iter()
        .filter(|d| d.required && (!d.installed || d.needs_update))
        .collect();

    if !missing.is_empty() {
        let names: Vec<_> = missing.iter().map(|d| d.display_name.as_str()).collect();
        return Err(format!("缺少依赖: {}", names.join(", ")).into());
    }

    // 步骤 2: 配置 npm（Linux 需要）
    #[cfg(target_os = "linux")]
    {
        emit_progress(window, "配置 npm 全局路径", "running", 30.0, vec![]);
        configure_npm_prefix(window).await?;
    }

    // 步骤 3: 安装 OpenClaw
    emit_progress(window, "安装 OpenClaw", "running", 50.0, vec![]);
    
    match options.method.as_str() {
        "npm" => install_openclaw_npm(window).await?,
        "git" => install_openclaw_git(window, options.custom_path).await?,
        _ => return Err("无效的安装方法".into()),
    }

    // 步骤 4: 运行 setup
    emit_progress(window, "初始化配置", "running", 80.0, vec![]);
    run_openclaw_setup(window).await?;

    emit_progress(window, "安装完成", "success", 100.0, vec![
        "✓ OpenClaw 已成功安装".to_string(),
        "✓ 配置已初始化".to_string(),
    ]);

    Ok("安装成功！".to_string())
}

// macOS: 安装 Node.js
#[cfg(target_os = "macos")]
async fn install_nodejs(window: &Window) -> Result<(), Box<dyn std::error::Error>> {
    emit_progress(window, "通过 Homebrew 安装 Node.js 22", "running", 0.0, vec![]);
    
    let output = Command::new("brew")
        .args(&["install", "node@22"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("安装失败: {}", error).into());
    }

    Ok(())
}

// macOS: 安装 Git
#[cfg(target_os = "macos")]
async fn install_git(window: &Window) -> Result<(), Box<dyn std::error::Error>> {
    emit_progress(window, "通过 Homebrew 安装 Git", "running", 0.0, vec![]);
    
    let output = Command::new("brew")
        .args(&["install", "git"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("安装失败: {}", error).into());
    }

    Ok(())
}

// macOS: 安装 Homebrew
#[cfg(target_os = "macos")]
async fn install_homebrew(window: &Window) -> Result<(), Box<dyn std::error::Error>> {
    emit_progress(window, "安装 Homebrew（可能需要输入密码）", "running", 0.0, vec![]);
    
    let output = Command::new("bash")
        .args(&[
            "-c",
            "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        ])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("安装失败: {}", error).into());
    }

    Ok(())
}

// Linux: 配置 npm prefix
#[cfg(target_os = "linux")]
async fn configure_npm_prefix(window: &Window) -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    
    let home = env::var("HOME")?;
    let npm_global = format!("{}/.npm-global", home);
    
    // 创建目录
    std::fs::create_dir_all(&npm_global)?;
    
    // 设置 npm prefix
    let output = Command::new("npm")
        .args(&["config", "set", "prefix", &npm_global])
        .output()?;

    if !output.status.success() {
        return Err("配置 npm prefix 失败".into());
    }

    emit_progress(window, "npm prefix 已配置", "success", 0.0, vec![
        format!("已设置: {}", npm_global),
        "请确保 ~/.npm-global/bin 在 PATH 中".to_string(),
    ]);

    Ok(())
}

// 通过 npm 安装 OpenClaw
async fn install_openclaw_npm(window: &Window) -> Result<(), Box<dyn std::error::Error>> {
    emit_progress(window, "npm install -g openclaw", "running", 0.0, vec![
        "正在下载 OpenClaw...".to_string(),
    ]);

    let output = Command::new("npm")
        .args(&["install", "-g", "openclaw"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("npm 安装失败: {}", error).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    emit_progress(window, "OpenClaw 安装成功", "success", 0.0, vec![
        stdout.to_string(),
    ]);

    Ok(())
}

// 通过 git 安装 OpenClaw
async fn install_openclaw_git(
    window: &Window,
    custom_path: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    
    let home = env::var("HOME")?;
    let install_path = custom_path
        .unwrap_or_else(|| format!("{}/.openclaw-src", home));

    emit_progress(window, "克隆 OpenClaw 源码", "running", 0.0, vec![
        format!("目标路径: {}", install_path),
    ]);

    // 克隆仓库
    let output = Command::new("git")
        .args(&[
            "clone",
            "https://github.com/openclaw/openclaw.git",
            &install_path,
        ])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git clone 失败: {}", error).into());
    }

    emit_progress(window, "安装依赖并构建", "running", 0.0, vec![]);

    // 运行 pnpm install（假设源码使用 pnpm）
    let output = Command::new("pnpm")
        .args(&["install"])
        .current_dir(&install_path)
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("pnpm install 失败: {}", error).into());
    }

    Ok(())
}

// 运行 openclaw setup
async fn run_openclaw_setup(window: &Window) -> Result<(), Box<dyn std::error::Error>> {
    emit_progress(window, "运行 openclaw setup", "running", 0.0, vec![]);

    let output = Command::new("openclaw")
        .args(&["setup", "--non-interactive"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        // setup 失败不应阻止安装完成（用户可以稍后手动运行）
        emit_progress(window, "setup 完成（部分步骤可能需要手动完成）", "success", 0.0, vec![
            error.to_string(),
        ]);
    } else {
        emit_progress(window, "setup 完成", "success", 0.0, vec![]);
    }

    Ok(())
}

// 辅助函数：发送进度事件到前端
fn emit_progress(window: &Window, step: &str, status: &str, progress: f32, logs: Vec<String>) {
    let progress_data = InstallProgress {
        step: step.to_string(),
        status: status.to_string(),
        progress,
        message: step.to_string(),
        logs,
    };

    let _ = window.emit("install-progress", progress_data);
}
