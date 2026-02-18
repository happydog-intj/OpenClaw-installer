use serde::{Deserialize, Serialize};
use std::process::Command;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DependencyStatus {
    pub name: String,
    pub display_name: String,
    pub required: bool,
    pub required_version: String,
    pub current_version: Option<String>,
    pub installed: bool,
    pub needs_update: bool,
    pub install_command: Option<String>,
}

pub async fn check_dependencies() -> Result<Vec<DependencyStatus>, Box<dyn std::error::Error>> {
    let mut deps = Vec::new();

    // 检测 OpenClaw
    deps.push(check_openclaw().await?);

    // 检测 nvm
    deps.push(check_nvm().await?);
    
    // 检测 Node.js
    deps.push(check_nodejs().await?);
    
    // 检测 npm
    deps.push(check_npm().await?);
    
    // 检测 Git
    deps.push(check_git().await?);
    
    // macOS: 检测 Xcode Command Line Tools
    #[cfg(target_os = "macos")]
    {
        deps.push(check_xcode_tools().await?);
    }

    Ok(deps)
}

async fn check_openclaw() -> Result<DependencyStatus, Box<dyn std::error::Error>> {
    // 尝试运行 openclaw --version
    let output = Command::new("bash")
        .arg("-c")
        .arg("source ~/.nvm/nvm.sh 2>/dev/null && openclaw --version 2>/dev/null || openclaw --version")
        .output();

    let (installed, current_version, needs_update) = match output {
        Ok(output) if output.status.success() => {
            let version_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            
            // 提取版本号，格式如 "🦞 OpenClaw 2026.2.14 (c1feda1) ..."
            let version = if let Some(ver_line) = version_str.lines().next() {
                // 提取日期版本 "2026.2.14"
                if let Some(ver) = ver_line.split_whitespace().nth(2) {
                    ver.to_string()
                } else {
                    version_str.lines().next().unwrap_or("unknown").to_string()
                }
            } else {
                "unknown".to_string()
            };
            
            // TODO: 可以添加版本比较逻辑判断是否需要更新
            (true, Some(version), false)
        }
        _ => (false, None, false),
    };

    Ok(DependencyStatus {
        name: "openclaw".to_string(),
        display_name: "OpenClaw".to_string(),
        required: false, // 检测阶段不标记为必需，让用户选择
        required_version: "latest".to_string(),
        current_version,
        installed,
        needs_update,
        install_command: None, // OpenClaw 安装由主流程处理
    })
}

async fn check_nvm() -> Result<DependencyStatus, Box<dyn std::error::Error>> {
    // 检测 nvm 是否存在（检查 ~/.nvm 目录或 NVM_DIR 环境变量）
    let nvm_dir = std::env::var("NVM_DIR")
        .unwrap_or_else(|_| format!("{}/.nvm", std::env::var("HOME").unwrap_or_default()));
    
    let installed = std::path::Path::new(&nvm_dir).exists();
    let current_version = if installed {
        Some("installed".to_string())
    } else {
        None
    };

    Ok(DependencyStatus {
        name: "nvm".to_string(),
        display_name: "nvm (Node Version Manager)".to_string(),
        required: true,
        required_version: "0.39+".to_string(),
        current_version,
        installed,
        needs_update: false,
        install_command: Some("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash".to_string()),
    })
}

async fn check_nodejs() -> Result<DependencyStatus, Box<dyn std::error::Error>> {
    // 使用 bash 执行，因为 nvm 是 shell 函数
    let output = Command::new("bash")
        .arg("-c")
        .arg("source ~/.nvm/nvm.sh 2>/dev/null && node --version")
        .output();

    let (installed, current_version, needs_update) = match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let version_num = parse_node_version(&version);
            let needs_update = version_num.map(|v| v < 22).unwrap_or(true);
            (true, Some(version), needs_update)
        }
        _ => (false, None, false),
    };

    Ok(DependencyStatus {
        name: "nodejs".to_string(),
        display_name: "Node.js".to_string(),
        required: true,
        required_version: "22+".to_string(),
        current_version,
        installed,
        needs_update,
        install_command: Some("nvm install 22 && nvm use 22 && nvm alias default 22".to_string()),
    })
}

async fn check_npm() -> Result<DependencyStatus, Box<dyn std::error::Error>> {
    let output = Command::new("bash")
        .arg("-c")
        .arg("source ~/.nvm/nvm.sh 2>/dev/null && npm --version")
        .output();

    let (installed, current_version) = match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            (true, Some(version))
        }
        _ => (false, None),
    };

    Ok(DependencyStatus {
        name: "npm".to_string(),
        display_name: "npm".to_string(),
        required: true,
        required_version: "10+".to_string(),
        current_version,
        installed,
        needs_update: false,
        install_command: None, // npm 随 Node.js 安装
    })
}

async fn check_git() -> Result<DependencyStatus, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("--version")
        .output();

    let (installed, current_version) = match output {
        Ok(output) if output.status.success() => {
            let version_str = String::from_utf8_lossy(&output.stdout);
            let version = extract_git_version(&version_str);
            (true, Some(version))
        }
        _ => (false, None),
    };

    Ok(DependencyStatus {
        name: "git".to_string(),
        display_name: "Git".to_string(),
        required: false,
        required_version: "2.0+".to_string(),
        current_version,
        installed,
        needs_update: false,
        install_command: Some(get_git_install_command()),
    })
}

#[cfg(target_os = "macos")]
async fn check_xcode_tools() -> Result<DependencyStatus, Box<dyn std::error::Error>> {
    // 检测 Xcode Command Line Tools 是否安装
    let output = Command::new("xcode-select")
        .arg("-p")
        .output();

    let (installed, current_version) = match output {
        Ok(output) if output.status.success() => {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            
            // 尝试获取版本信息
            let version_output = Command::new("xcode-select")
                .arg("--version")
                .output();
            
            let version = if let Ok(ver_out) = version_output {
                if ver_out.status.success() {
                    let ver_str = String::from_utf8_lossy(&ver_out.stdout);
                    // 提取版本号，格式如 "xcode-select version 2384."
                    if let Some(ver) = ver_str.split_whitespace().nth(2) {
                        format!("installed ({})", ver.trim_end_matches('.'))
                    } else {
                        "installed".to_string()
                    }
                } else {
                    "installed".to_string()
                }
            } else {
                "installed".to_string()
            };
            
            (true, Some(version))
        }
        _ => (false, None),
    };

    Ok(DependencyStatus {
        name: "xcode-tools".to_string(),
        display_name: "Xcode Command Line Tools".to_string(),
        required: false,
        required_version: "any".to_string(),
        current_version,
        installed,
        needs_update: false,
        install_command: Some("xcode-select --install".to_string()),
    })
}

// 辅助函数

fn parse_node_version(version_str: &str) -> Option<u32> {
    let re = Regex::new(r"v?(\d+)\.").ok()?;
    let caps = re.captures(version_str)?;
    caps.get(1)?.as_str().parse().ok()
}

fn extract_git_version(version_str: &str) -> String {
    let re = Regex::new(r"git version ([\d.]+)").unwrap();
    if let Some(caps) = re.captures(version_str) {
        return caps.get(1).unwrap().as_str().to_string();
    }
    "Unknown".to_string()
}

fn get_git_install_command() -> String {
    #[cfg(target_os = "macos")]
    {
        "xcode-select --install".to_string()
    }

    #[cfg(target_os = "windows")]
    {
        "winget install Git.Git".to_string()
    }

    #[cfg(target_os = "linux")]
    {
        "sudo apt-get install -y git".to_string()
    }
}
