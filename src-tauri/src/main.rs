// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::utils::configloader;
use serde_json::Value;
use tauri::State;
use std::sync::Mutex;
use tauri::Manager;

struct ConfigState(Mutex<configloader::Config>);

#[tauri::command]
fn load_config_command(state: State<ConfigState>) -> configloader::Config {
    state.0.lock().unwrap().clone()
}

#[tauri::command]
fn save_config_command(
    app_handle: tauri::AppHandle,
    state: State<ConfigState>,
    section: &str,
    key: &str,
    value: String,
) -> Result<(), String> {
    let mut config = state.0.lock().unwrap();
    
    match section {
        "normal" => match key {
            "autostartup" => config.normal.autostartup = value.parse().unwrap_or(false),
            _ => return Err("Invalid key".to_string()),
        },
        "theme" => match key {
            "theme" => config.theme.theme = value,
            "accent_color" => config.theme.accent_color = value,
            "follow_system_accent" => config.theme.follow_system_accent = value.parse().unwrap_or(false),
            _ => return Err("Invalid key".to_string()),
        },
        "console" => match key {
            "encoding" => config.console.encoding = value,
            _ => return Err("Invalid key".to_string()),
        },
        "download" => match key {
            "source" => config.download.source = value,
            "aria2_enabled" => config.download.aria2_enabled = value.parse().unwrap_or(true),
            "aria2_threads" => config.download.aria2_threads = value.parse().unwrap_or(6),
            _ => return Err("Invalid key".to_string()),
        },
        "instance" => match key {
            "auto_accept_eula" => config.instance.auto_accept_eula = value.parse().unwrap_or(false),
            "auto_restart_on_crash" => config.instance.auto_restart_on_crash = value.parse().unwrap_or(false),
            _ => return Err("Invalid key".to_string()),
        },
        _ => return Err("Invalid section".to_string()),
    }

    configloader::save_config(&config);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(ConfigState(Mutex::new(configloader::Config::default())))
        .setup(|app| {
            let config = configloader::load_config();
            *app.try_state::<ConfigState>().unwrap().0.lock().unwrap() = config;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_config_command,
            save_config_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
