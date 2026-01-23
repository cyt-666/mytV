pub mod media;
pub mod user;
pub mod list;

use tauri_plugin_sql::{Builder, Migration, MigrationKind};

// 常量定义
pub const ONE_HOUR_MS: u64 = 60 * 60 * 1000;
pub const ONE_DAY_MS: u64 = 24 * ONE_HOUR_MS;
pub const THIRTY_DAYS_MS: u64 = 30 * ONE_DAY_MS;

pub fn get_current_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
