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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
