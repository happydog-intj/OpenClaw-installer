use serde::{Deserialize, Serialize};
use std::process::Command;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    // 检测 Node.js
    deps.push(check_nodejs().await?);
    
    // 检测 npm
    deps.push(check_npm().await?);
    
    // 检测 Git
    deps.push(check_git().await?);
    
    // 检测包管理器 (macOS: Homebrew, Windows: winget, Linux: apt)
    deps.push(check_package_manager().await?);

    Ok(deps)
}

async fn check_nodejs() -> Result<DependencyStatus, Box<dyn std::error::Error>> {
    let output = Command::new("node")
        .arg("--version")
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
        install_command: Some(get_nodejs_install_command()),
    })
}

async fn check_npm() -> Result<DependencyStatus, Box<dyn std::error::Error>> {
    let output = Command::new("npm")
        .arg("--version")
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

async fn check_package_manager() -> Result<DependencyStatus, Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        check_homebrew().await
    }

    #[cfg(target_os = "windows")]
    {
        check_winget().await
    }

    #[cfg(target_os = "linux")]
    {
        check_apt().await
    }
}

#[cfg(target_os = "macos")]
async fn check_homebrew() -> Result<DependencyStatus, Box<dyn std::error::Error>> {
    let output = Command::new("brew")
        .arg("--version")
        .output();

    let (installed, current_version) = match output {
        Ok(output) if output.status.success() => {
            let version_str = String::from_utf8_lossy(&output.stdout);
            let version = version_str.lines().next().unwrap_or("Unknown").to_string();
            (true, Some(version))
        }
        _ => (false, None),
    };

    Ok(DependencyStatus {
        name: "homebrew".to_string(),
        display_name: "Homebrew".to_string(),
        required: true,
        required_version: "any".to_string(),
        current_version,
        installed,
        needs_update: false,
        install_command: Some("/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"".to_string()),
    })
}

#[cfg(target_os = "windows")]
async fn check_winget() -> Result<DependencyStatus, Box<dyn std::error::Error>> {
    let output = Command::new("winget")
        .arg("--version")
        .output();

    let (installed, current_version) = match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            (true, Some(version))
        }
        _ => (false, None),
    };

    Ok(DependencyStatus {
        name: "winget".to_string(),
        display_name: "winget".to_string(),
        required: true,
        required_version: "any".to_string(),
        current_version,
        installed,
        needs_update: false,
        install_command: None, // winget 需要手动安装
    })
}

#[cfg(target_os = "linux")]
async fn check_apt() -> Result<DependencyStatus, Box<dyn std::error::Error>> {
    let output = Command::new("apt")
        .arg("--version")
        .output();

    let (installed, current_version) = match output {
        Ok(output) if output.status.success() => {
            let version_str = String::from_utf8_lossy(&output.stdout);
            let version = version_str.lines().next().unwrap_or("Unknown").to_string();
            (true, Some(version))
        }
        _ => (false, None),
    };

    Ok(DependencyStatus {
        name: "apt".to_string(),
        display_name: "apt".to_string(),
        required: true,
        required_version: "any".to_string(),
        current_version,
        installed,
        needs_update: false,
        install_command: None,
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

fn get_nodejs_install_command() -> String {
    #[cfg(target_os = "macos")]
    {
        "brew install node@22".to_string()
    }

    #[cfg(target_os = "windows")]
    {
        "winget install OpenJS.NodeJS.LTS".to_string()
    }

    #[cfg(target_os = "linux")]
    {
        "curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash - && sudo apt-get install -y nodejs".to_string()
    }
}

fn get_git_install_command() -> String {
    #[cfg(target_os = "macos")]
    {
        "brew install git".to_string()
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
