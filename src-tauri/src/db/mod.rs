pub mod schema;
pub mod cache;

use tauri::{AppHandle, Manager};
use tauri_plugin_sql::{Migration, MigrationKind};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use sqlx::{Executor, Row};
use std::path::PathBuf;
use std::str::FromStr;
use log::{info, error};

pub struct DbPool(pub SqlitePool);

// 保留此函数是为了兼容 tauri-plugin-sql 的接口，防止编译错误，但实际上我们不再依赖它来建表
pub fn get_migrations() -> Vec<Migration> {
    vec![] 
}

pub async fn init_db_pool(app: &AppHandle) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let app_dir = app.path().app_data_dir()?;
    let db_path = app_dir.join("mytv.db");
    
    info!("Database path: {:?}", db_path);
    
    // 确保目录存在
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    
    // 连接数据库
    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", db_path.to_string_lossy()))?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
        
    // 手动执行建表语句，确保表一定存在！
    info!("Executing DB schema initialization...");
    if let Err(e) = pool.execute(schema::FULL_SCHEMA).await {
        error!("Failed to execute schema: {}", e);
        return Err(Box::new(e));
    }
    info!("DB schema initialized successfully.");
        
    Ok(pool)
}
