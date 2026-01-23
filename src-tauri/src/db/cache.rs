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
pub const STALE_WHILE_REVALIDATE_USER: i64 = 5 * 60 * 1000; // 用户数据 5分钟后视为陈旧
pub const CACHE_TTL_API: i64 = 4 * 60 * 60 * 1000;        // 列表API缓存4小时

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
                info!("🔴 Cache EXPIRED for {}", id);
                let _ = sqlx::query("DELETE FROM media_cache WHERE id = ?").bind(&id).execute(pool).await;
                return None;
            }
            
            let data_str: String = row.get("data");
            if let Ok(json) = serde_json::from_str(&data_str) {
                // 检查是否陈旧
                let updated_at: i64 = row.get("updated_at");
                let is_stale = (updated_at + STALE_WHILE_REVALIDATE) < now;
                
                info!("🟢 Cache HIT for {} (Stale: {})", id, is_stale);
                return Some(CacheResult {
                    data: json,
                    is_stale
                });
            }
        }
        Ok(None) => info!("⚪️ Cache MISS for {}", id),
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
        info!("💾 Cache SAVED for {}", id);
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
                "updated_at": now
            });
            info!("🟢 Translation Cache HIT for {}", cache_id);
            return Some(json);
        }
        _ => {
            // Translation misses happen often and are noisy, keeping it debug
            debug!("⚪️ Translation Cache MISS for {}", cache_id);
            return None;
        }
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

// 用户数据缓存操作
pub async fn get_user_data_cache(pool: &SqlitePool, key: &str) -> Option<CacheResult> {
    let now = get_timestamp();
    
    let result = sqlx::query("SELECT data, updated_at FROM user_data_cache WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await;

    match result {
        Ok(Some(row)) => {
            let data_str: String = row.get("data");
            if let Ok(json) = serde_json::from_str(&data_str) {
                let updated_at: i64 = row.get("updated_at");
                let is_stale = (updated_at + STALE_WHILE_REVALIDATE_USER) < now;
                
                info!("🟢 User Data Cache HIT for {} (Stale: {})", key, is_stale);
                return Some(CacheResult {
                    data: json,
                    is_stale
                });
            }
        }
        Ok(None) => info!("⚪️ User Data Cache MISS for {}", key),
        Err(e) => error!("DB error getting user data cache for {}: {}", key, e),
    }
    
    None
}

pub async fn set_user_data_cache(pool: &SqlitePool, key: &str, data: &Value) {
    let now = get_timestamp();
    let data_str = data.to_string();

    let result = sqlx::query(
        "INSERT OR REPLACE INTO user_data_cache (key, data, updated_at, is_dirty) VALUES (?, ?, ?, 0)"
    )
    .bind(key)
    .bind(data_str)
    .bind(now)
    .execute(pool)
    .await;

    if let Err(e) = result {
        error!("Failed to save user data cache for {}: {}", key, e);
    } else {
        info!("💾 User Data Cache SAVED for {}", key);
    }
}

// 列表API响应缓存操作
pub async fn get_api_response_cache(pool: &SqlitePool, key: &str) -> Option<Value> {
    let now = get_timestamp();
    
    let result = sqlx::query("SELECT data, expires_at FROM api_response_cache WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await;

    match result {
        Ok(Some(row)) => {
            let expires_at: i64 = row.get("expires_at");
            if now > expires_at {
                info!("🔴 API Cache EXPIRED for {}", key);
                let _ = sqlx::query("DELETE FROM api_response_cache WHERE key = ?").bind(key).execute(pool).await;
                return None;
            }
            
            let data_str: String = row.get("data");
            if let Ok(json) = serde_json::from_str(&data_str) {
                info!("🟢 API Cache HIT for {}", key);
                return Some(json);
            }
        }
        Ok(None) => info!("⚪️ API Cache MISS for {}", key),
        Err(e) => error!("DB error getting API cache for {}: {}", key, e),
    }
    
    None
}

pub async fn set_api_response_cache(pool: &SqlitePool, key: &str, data: &Value) {
    let now = get_timestamp();
    let expires_at = now + CACHE_TTL_API;
    let data_str = data.to_string();

    let result = sqlx::query(
        "INSERT OR REPLACE INTO api_response_cache (key, data, updated_at, expires_at) VALUES (?, ?, ?, ?)"
    )
    .bind(key)
    .bind(data_str)
    .bind(now)
    .bind(expires_at)
    .execute(pool)
    .await;

    if let Err(e) = result {
        error!("Failed to save API cache for {}: {}", key, e);
    } else {
        info!("💾 API Cache SAVED for {}", key);
    }
}
