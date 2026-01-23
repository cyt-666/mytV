use crate::app_conf::{get_config, AppConf};
use tauri::{command, AppHandle, Manager};
use crate::db::{DbPool, cache};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[command]
pub async fn get_app_config(app: AppHandle) -> Result<AppConf, String> {
    // 优先从 DB 读取
    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(val) = cache::get_config(&pool.0, "app_conf").await {
             if let Ok(conf) = serde_json::from_value::<AppConf>(val) {
                 return Ok(conf);
             }
        }
    }
    
    // 降级：从文件读取（仅用于首次迁移）
    // 注意：这里保留了读取旧文件的逻辑，以便平滑迁移
    let config_path = Path::new("app.conf.json");
    if config_path.exists() {
        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(conf) = serde_json::from_str::<AppConf>(&content) {
                // 迁移到 DB
                if let Some(pool) = app.try_state::<DbPool>() {
                    let _ = cache::set_config(&pool.0, "app_conf", &serde_json::to_value(&conf).unwrap()).await;
                }
                return Ok(conf);
            }
        }
    }
    
    Ok(get_config().clone())
}

#[command]
pub async fn update_log_level(app: AppHandle, level: String) -> Result<(), String> {
    let mut config = get_app_config(app.clone()).await?;

    // Update level
    config.log_level = level;

    // Save to DB
    if let Some(pool) = app.try_state::<DbPool>() {
         cache::set_config(&pool.0, "app_conf", &serde_json::to_value(&config).unwrap())
            .await
            .map_err(|e| e.to_string())?;
    }
    
    // 同时保存到文件以备不时之需（或者可以彻底移除）
    // 为了彻底迁移，这里不再写入文件
    
    Ok(())
}
