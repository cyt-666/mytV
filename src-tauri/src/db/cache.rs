use sqlx::{SqlitePool, Row};
use serde_json::Value;
use log::{info, debug, error};

// 获取当前时间戳 (ms)
pub fn get_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

// 缓存常量
pub const CACHE_TTL_LONG: i64 = 30 * 24 * 60 * 60 * 1000; // 30天
pub const CACHE_TTL_SHORT: i64 = 24 * 60 * 60 * 1000;     // 1天
pub const CACHE_TTL_TRANSLATION: i64 = 7 * 24 * 60 * 60 * 1000; // 7天
pub const STALE_WHILE_REVALIDATE: i64 = 60 * 60 * 1000;   // 1小时后视为陈旧，需要后台更新

pub struct CacheResult {
    pub data: Value,
    pub is_stale: bool,
}

// 媒体详情缓存操作
pub async fn get_media_cache(pool: &SqlitePool, media_type: &str, trakt_id: u32) -> Option<CacheResult> {
    let id = format!("{}_{}", media_type, trakt_id);
    let now = get_timestamp();
    
    let result = sqlx::query("SELECT data, updated_at, expires_at FROM media_cache WHERE id = ?")
        .bind(&id)
        .fetch_optional(pool)
        .await;

    match result {
        Ok(Some(row)) => {
            let expires_at: i64 = row.get("expires_at");
            
            // 如果完全过期，删除并返回 None
            if now > expires_at {
                debug!("Cache expired for {}", id);
                let _ = sqlx::query("DELETE FROM media_cache WHERE id = ?").bind(&id).execute(pool).await;
                return None;
            }
            
            let data_str: String = row.get("data");
            if let Ok(json) = serde_json::from_str(&data_str) {
                // 检查是否陈旧 (updated_at + STALE_WHILE_REVALIDATE < now)
                let updated_at: i64 = row.get("updated_at");
                let is_stale = (updated_at + STALE_WHILE_REVALIDATE) < now;
                
                debug!("Cache hit for {} (stale: {})", id, is_stale);
                return Some(CacheResult {
                    data: json,
                    is_stale
                });
            }
        }
        Ok(None) => debug!("Cache miss for {}", id),
        Err(e) => error!("DB error getting cache for {}: {}", id, e),
    }
    
    None
}

pub async fn set_media_cache(pool: &SqlitePool, media_type: &str, trakt_id: u32, data: &Value, ttl_ms: i64) {
    let id = format!("{}_{}", media_type, trakt_id);
    let now = get_timestamp();
    let expires_at = now + ttl_ms;
    let data_str = data.to_string();

    let result = sqlx::query(
        "INSERT OR REPLACE INTO media_cache (id, media_type, trakt_id, data, updated_at, expires_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(media_type)
    .bind(trakt_id)
    .bind(data_str)
    .bind(now)
    .bind(expires_at)
    .execute(pool)
    .await;

    if let Err(e) = result {
        error!("Failed to save cache for {}: {}", id, e);
    } else {
        debug!("Cache saved for {}", id);
    }
}

// 翻译缓存操作
pub async fn get_translation_cache(pool: &SqlitePool, media_type: &str, id: u32) -> Option<Value> {
    let cache_id = format!("{}_{}", media_type, id);
    let now = get_timestamp();

    let result = sqlx::query("SELECT title, overview, tagline, expires_at FROM translation_cache WHERE id = ?")
        .bind(&cache_id)
        .fetch_optional(pool)
        .await;

    match result {
        Ok(Some(row)) => {
            let expires_at: i64 = row.get("expires_at");
            if now > expires_at {
                let _ = sqlx::query("DELETE FROM translation_cache WHERE id = ?").bind(&cache_id).execute(pool).await;
                return None;
            }

            let title: Option<String> = row.get("title");
            let overview: Option<String> = row.get("overview");
            let tagline: Option<String> = row.get("tagline");

            let json = serde_json::json!({
                "title": title,
                "overview": overview,
                "tagline": tagline,
                "updated_at": now // 模拟 updated_at 兼容旧结构
            });
            return Some(json);
        }
        _ => return None
    }
}

pub async fn set_translation_cache(pool: &SqlitePool, media_type: &str, id: u32, data: &Value) {
    let cache_id = format!("{}_{}", media_type, id);
    let now = get_timestamp();
    let expires_at = now + CACHE_TTL_TRANSLATION;

    let title = data.get("title").and_then(|v| v.as_str());
    let overview = data.get("overview").and_then(|v| v.as_str());
    let tagline = data.get("tagline").and_then(|v| v.as_str());

    let _ = sqlx::query(
        "INSERT OR REPLACE INTO translation_cache (id, title, overview, tagline, updated_at, expires_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(cache_id)
    .bind(title)
    .bind(overview)
    .bind(tagline)
    .bind(now)
    .bind(expires_at)
    .execute(pool)
    .await;
}

// 配置/Token 操作
pub async fn get_config(pool: &SqlitePool, key: &str) -> Option<Value> {
    let result = sqlx::query("SELECT value FROM app_config WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await;

    match result {
        Ok(Some(row)) => {
            let value_str: String = row.get("value");
            if let Ok(json) = serde_json::from_str(&value_str) {
                return Some(json);
            }
        }
        _ => {}
    }
    None
}

pub async fn set_config(pool: &SqlitePool, key: &str, value: &Value) -> Result<(), sqlx::Error> {
    let now = get_timestamp();
    let value_str = value.to_string();

    sqlx::query(
        "INSERT OR REPLACE INTO app_config (key, value, updated_at) VALUES (?, ?, ?)"
    )
    .bind(key)
    .bind(value_str)
    .bind(now)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn delete_config(pool: &SqlitePool, key: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM app_config WHERE key = ?")
        .bind(key)
        .execute(pool)
        .await?;
    Ok(())
}
