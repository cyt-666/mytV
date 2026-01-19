use tauri::{command, AppHandle, Manager};
use crate::app_conf::{AppConf, get_config, APP_CONFIG};
use std::fs;
use std::path::Path;

#[command]
pub async fn get_app_config() -> Result<AppConf, String> {
    Ok(get_config().clone())
}

#[command]
pub async fn update_log_level(level: String) -> Result<(), String> {
    let config_path = Path::new("app.conf.json");
    
    // Read existing config or create default
    let mut config = if config_path.exists() {
        let content = fs::read_to_string(config_path).map_err(|e| e.to_string())?;
        serde_json::from_str::<AppConf>(&content).map_err(|e| e.to_string())?
    } else {
        get_config().clone()
    };

    // Update level
    config.log_level = level;

    // Save back to file
    let content = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(config_path, content).map_err(|e| e.to_string())?;

    Ok(())
}
