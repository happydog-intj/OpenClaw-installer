// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod detector;
mod installer;
mod executor;

use detector::{check_dependencies, DependencyStatus};
use installer::{install_openclaw, InstallOptions, InstallProgress};

#[tauri::command]
async fn check_system_dependencies() -> Result<Vec<DependencyStatus>, String> {
    detector::check_dependencies()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn install_dependency(
    window: tauri::Window,
    name: String,
) -> Result<(), String> {
    installer::install_single_dependency(&window, &name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn start_installation(
    window: tauri::Window,
    options: InstallOptions,
) -> Result<String, String> {
    installer::install_openclaw(&window, options)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_system_info() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "os_version": get_os_version(),
    }))
}

#[tauri::command]
async fn load_existing_config() -> Result<serde_json::Value, String> {
    use std::fs;
    use std::env;
    
    let home = env::var("HOME").map_err(|_| "无法获取 HOME 目录".to_string())?;
    let config_path = format!("{}/.openclaw/openclaw.json", home);
    
    // 检查配置文件是否存在
    if !std::path::Path::new(&config_path).exists() {
        return Ok(serde_json::json!({
            "exists": false
        }));
    }
    
    // 读取配置文件
    let config_content = fs::read_to_string(&config_path)
        .map_err(|e| format!("读取配置文件失败: {}", e))?;
    
    let config: serde_json::Value = serde_json::from_str(&config_content)
        .map_err(|e| format!("解析配置文件失败: {}", e))?;
    
    // 提取相关配置
    let mut result = serde_json::json!({
        "exists": true,
        "mode": "local",
        "workspace": "~/clawd",
        "apiKeys": {}
    });
    
    // 读取 workspace
    if let Some(workspace) = config["agents"]["defaults"]["workspace"].as_str() {
        result["workspace"] = serde_json::json!(workspace);
    }
    
    // 读取 API Keys 和模型信息
    if let Some(profiles) = config["auth"]["profiles"].as_object() {
        let mut api_keys = serde_json::Map::new();
        let mut configured_models = serde_json::Map::new();
        
        for (key, value) in profiles {
            // 提取 provider 名称 (例如 "anthropic:default" -> "anthropic")
            if let Some(provider) = key.split(':').next() {
                if let Some(api_key) = value["apiKey"].as_str() {
                    api_keys.insert(provider.to_string(), serde_json::json!(api_key));
                    
                    // 收集该 provider 的模型信息
                    let mut provider_info = serde_json::json!({
                        "provider": provider,
                        "profile": key,
                        "hasKey": true
                    });
                    
                    // 如果有 provider 名称
                    if let Some(provider_name) = value["provider"].as_str() {
                        provider_info["providerName"] = serde_json::json!(provider_name);
                    }
                    
                    configured_models.insert(provider.to_string(), provider_info);
                }
            }
        }
        
        result["apiKeys"] = serde_json::json!(api_keys);
        result["configuredModels"] = serde_json::json!(configured_models);
    }
    
    // 读取自定义模型配置
    if let Some(providers) = config["models"]["providers"].as_object() {
        let mut models_list = Vec::new();
        
        for (provider_name, provider_config) in providers {
            if let Some(models) = provider_config["models"].as_array() {
                for model in models {
                    if let Some(model_id) = model["id"].as_str() {
                        models_list.push(serde_json::json!({
                            "provider": provider_name,
                            "id": model_id,
                            "name": model.get("name").and_then(|v| v.as_str()).unwrap_or(model_id)
                        }));
                    }
                }
            }
        }
        
        result["models"] = serde_json::json!(models_list);
    }
    
    Ok(result)
}

#[tauri::command]
async fn install_feishu_plugin(
    app_id: String,
    app_secret: String,
) -> Result<serde_json::Value, String> {
    use std::process::Command;
    
    let mut logs = Vec::new();
    
    // 1. 检查插件是否已安装
    logs.push("🔍 检查飞书插件状态...".to_string());
    
    let check_output = Command::new("bash")
        .arg("-c")
        .arg("source ~/.nvm/nvm.sh 2>/dev/null && openclaw plugins list --json")
        .output()
        .map_err(|e| format!("检查插件失败: {}", e))?;
    
    let plugins_output = String::from_utf8_lossy(&check_output.stdout);
    let already_installed = plugins_output.contains("@openclaw/feishu") || plugins_output.contains("\"feishu\"");
    
    if already_installed {
        logs.push("✅ 飞书插件已安装，跳过安装步骤".to_string());
    } else {
        // 安装飞书插件
        logs.push("📦 正在安装 @openclaw/feishu 插件...".to_string());
        
        let install_output = Command::new("bash")
            .arg("-c")
            .arg("source ~/.nvm/nvm.sh 2>/dev/null && openclaw plugins install @openclaw/feishu")
            .output()
            .map_err(|e| format!("执行安装命令失败: {}", e))?;
        
        if !install_output.status.success() {
            let error = String::from_utf8_lossy(&install_output.stderr);
            // 检查是否是重复安装错误
            if error.contains("duplicate plugin id") {
                logs.push("✅ 飞书插件已存在（检测到重复ID）".to_string());
            } else {
                logs.push(format!("❌ 插件安装失败: {}", error));
                return Ok(serde_json::json!({
                    "success": false,
                    "error": error.to_string(),
                    "logs": logs
                }));
            }
        } else {
            logs.push("✅ 插件安装成功".to_string());
        }
    }
    
    // 2. 配置飞书渠道
    logs.push("🔧 正在配置飞书渠道...".to_string());
    
    let config_cmd = format!(
        "source ~/.nvm/nvm.sh 2>/dev/null && openclaw config set channels.feishu.appId '{}' && openclaw config set channels.feishu.appSecret '{}'",
        app_id, app_secret
    );
    
    let config_output = Command::new("bash")
        .arg("-c")
        .arg(&config_cmd)
        .output()
        .map_err(|e| format!("执行配置命令失败: {}", e))?;
    
    if !config_output.status.success() {
        let error = String::from_utf8_lossy(&config_output.stderr);
        logs.push(format!("❌ 配置失败: {}", error));
        return Ok(serde_json::json!({
            "success": false,
            "error": error.to_string(),
            "logs": logs
        }));
    }
    
    logs.push("✅ 飞书凭证已保存".to_string());
    
    // 3. 重启网关
    logs.push("🔄 正在重启 OpenClaw 网关...".to_string());
    
    let restart_output = Command::new("bash")
        .arg("-c")
        .arg("source ~/.nvm/nvm.sh 2>/dev/null && openclaw gateway restart")
        .output()
        .map_err(|e| format!("执行重启命令失败: {}", e))?;
    
    if !restart_output.status.success() {
        let error = String::from_utf8_lossy(&restart_output.stderr);
        logs.push(format!("⚠️ 网关重启失败: {}", error));
        logs.push("💡 请手动运行: openclaw gateway restart".to_string());
    } else {
        logs.push("✅ 网关已重启".to_string());
    }
    
    logs.push("".to_string());
    logs.push("🎉 配置完成！下一步：".to_string());
    logs.push("1. 在飞书中搜索你的机器人并发送消息".to_string());
    logs.push("2. 如果收到配对码，运行: openclaw pairing approve feishu <配对码>".to_string());
    logs.push("3. 查看日志: openclaw logs --follow".to_string());
    
    Ok(serde_json::json!({
        "success": true,
        "logs": logs
    }))
}

#[tauri::command]
async fn save_config(config: serde_json::Value) -> Result<String, String> {
    use std::process::Command;
    
    // 提取配置值
    let mode = config["mode"].as_str().unwrap_or("local");
    let workspace = config["workspace"].as_str().unwrap_or("~/clawd");
    
    // 构建 openclaw setup 命令
    let mut args = vec![
        "setup",
        "--non-interactive",
        "--accept-risks",
        "--mode", mode,
        "--workspace", workspace
    ];
    
    // 如果是远程模式，添加远程配置
    let remote_url = config["remoteUrl"].as_str().unwrap_or("");
    let remote_token = config["remoteToken"].as_str().unwrap_or("");
    
    if mode == "remote" && !remote_url.is_empty() {
        args.push("--remote-url");
        args.push(remote_url);
        
        if !remote_token.is_empty() {
            args.push("--remote-token");
            args.push(remote_token);
        }
    }
    
    // 执行 openclaw setup
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("source ~/.nvm/nvm.sh 2>/dev/null && openclaw {}", args.join(" ")))
        .output()
        .map_err(|e| format!("执行 openclaw setup 失败: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("配置失败: {}", error));
    }
    
    // 保存 API Keys
    if let Some(api_keys) = config["apiKeys"].as_object() {
        for (provider, key) in api_keys {
            if let Some(key_str) = key.as_str() {
                if !key_str.is_empty() {
                    let config_cmd = format!(
                        "source ~/.nvm/nvm.sh 2>/dev/null && openclaw config set auth.profiles.{}:default.apiKey '{}'",
                        provider, key_str
                    );
                    
                    let _ = Command::new("bash")
                        .arg("-c")
                        .arg(&config_cmd)
                        .output();
                }
            }
        }
    }
    
    Ok("配置保存成功".to_string())
}

fn get_os_version() -> String {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let output = Command::new("sw_vers")
            .arg("-productVersion")
            .output();
        
        if let Ok(output) = output {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }
    
    "Unknown".to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_system_dependencies,
            install_dependency,
            start_installation,
            get_system_info,
            load_existing_config,
            save_config,
            install_feishu_plugin,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
