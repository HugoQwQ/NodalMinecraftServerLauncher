use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize)]
pub struct NormalConfig {
    pub autostartup: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub theme: String,
    pub accent_color: String,
    pub follow_system_accent: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsoleConfig {
    pub encoding: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadConfig {
    pub source: String,
    pub aria2_enabled: bool,
    pub aria2_threads: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceConfig {
    pub auto_accept_eula: bool,
    pub auto_restart_on_crash: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub normal: NormalConfig,
    pub theme: ThemeConfig,
    pub console: ConsoleConfig,
    pub download: DownloadConfig,
    pub instance: InstanceConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            normal: NormalConfig {
                autostartup: true,
            },
            theme: ThemeConfig {
                theme: "system".to_string(),
                accent_color: "#0078D4".to_string(),
                follow_system_accent: false,
            },
            console: ConsoleConfig {
                encoding: "ANSI".to_string(),
            },
            download: DownloadConfig {
                source: "FastMirror镜像源".to_string(),
                aria2_enabled: true,
                aria2_threads: 6,
            },
            instance: InstanceConfig {
                auto_accept_eula: false,
                auto_restart_on_crash: false,
            },
        }
    }
}

pub fn get_config_file_path(app_handle: &AppHandle) -> PathBuf {
    let config_dir = app_handle
        .app_config_dir()
        .expect("Failed to get config directory");
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    config_dir.join("config.toml")
}

pub fn load_config(app_handle: &AppHandle) -> Config {
    let config_path = get_config_file_path(app_handle);
    if !config_path.exists() {
        let default_config = Config::default();
        save_config(app_handle, &default_config);
        return default_config;
    }

    let config_str = fs::read_to_string(config_path).expect("Failed to read config file");
    toml::from_str(&config_str).unwrap_or_else(|_| {
        let default_config = Config::default();
        save_config(app_handle, &default_config);
        default_config
    })
}

pub fn save_config(app_handle: &AppHandle, config: &Config) {
    let config_path = get_config_file_path(app_handle);
    let config_str = toml::to_string(config).expect("Failed to serialize config");
    fs::write(config_path, config_str).expect("Failed to write config file");
} 