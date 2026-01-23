pub mod schema;
pub mod cache;

use tauri::{AppHandle, Manager};
use tauri_plugin_sql::{Migration, MigrationKind};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use std::path::PathBuf;
use std::str::FromStr;
use log::info;

pub struct DbPool(pub SqlitePool);

pub fn get_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: schema::SCHEMA_V1,
            kind: MigrationKind::Up,
        }
    ]
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
    // 启用 WAL 模式以支持更好的并发
    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", db_path.to_string_lossy()))?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
        
    Ok(pool)
}
