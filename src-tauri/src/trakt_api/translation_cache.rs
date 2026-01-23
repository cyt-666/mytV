use tauri::{command, AppHandle, Manager};
use log::debug;
use crate::model::translation::TranslationData;
use crate::model::movie::MovieTranslations;
use crate::model::shows::{ShowTranslations, SeasonTranslations};
use crate::db::{DbPool, cache};

#[command]
pub async fn get_movie_translation_cached(app: AppHandle, id: u32) -> Result<Option<TranslationData>, u16> {
    // 1. 检查 DB 缓存
    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(data) = cache::get_translation_cache(&pool.0, "movie", id).await {
            if let Ok(translation) = serde_json::from_value::<TranslationData>(data) {
                return Ok(Some(translation));
            }
        }
    }
    
    // 2. 缓存未命中，调用 API
    match super::movie::movie_translation(app.clone(), id, "zh".to_string()).await {
        Ok(translations) => {
            let translation_data = process_movie_translations(translations);
            
            // 3. 保存到缓存
            if let Some(data) = &translation_data {
                if let Some(pool) = app.try_state::<DbPool>() {
                    cache::set_translation_cache(&pool.0, "movie", id, &serde_json::to_value(data).unwrap()).await;
                }
            }
            
            Ok(translation_data)
        }
        Err(code) => Err(code)
    }
}

#[command]
pub async fn get_show_translation_cached(app: AppHandle, id: u32) -> Result<Option<TranslationData>, u16> {
    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(data) = cache::get_translation_cache(&pool.0, "show", id).await {
            if let Ok(translation) = serde_json::from_value::<TranslationData>(data) {
                return Ok(Some(translation));
            }
        }
    }
    
    match super::shows::show_translation(app.clone(), id, "zh".to_string()).await {
        Ok(translations) => {
            let translation_data = process_show_translations(translations);
            
            if let Some(data) = &translation_data {
                if let Some(pool) = app.try_state::<DbPool>() {
                    cache::set_translation_cache(&pool.0, "show", id, &serde_json::to_value(data).unwrap()).await;
                }
            }
            
            Ok(translation_data)
        }
        Err(code) => Err(code)
    }
}

#[command]
pub async fn get_season_translation_cached(app: AppHandle, show_id: u32, season: u32) -> Result<Option<TranslationData>, u16> {
    // 使用 show_id + season 作为唯一标识
    // 为了复用 cache::get_translation_cache (id: u32)，我们需要一种映射方式
    // 这里简单地构造一个 composite key (show_id), 但 media_type 为 "season_{season}"
    let type_key = format!("season_{}", season);
    
    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(data) = cache::get_translation_cache(&pool.0, &type_key, show_id).await {
            if let Ok(translation) = serde_json::from_value::<TranslationData>(data) {
                return Ok(Some(translation));
            }
        }
    }
    
    match super::shows::season_trans(app.clone(), show_id, season, "zh".to_string()).await {
        Ok(translations) => {
            let translation_data = process_season_translations(translations);
            
            if let Some(data) = &translation_data {
                if let Some(pool) = app.try_state::<DbPool>() {
                    cache::set_translation_cache(&pool.0, &type_key, show_id, &serde_json::to_value(data).unwrap()).await;
                }
            }
            
            Ok(translation_data)
        }
        Err(code) => Err(code)
    }
}

#[command]
pub async fn clear_expired_translations(app: AppHandle) -> Result<u32, String> {
    if let Some(pool) = app.try_state::<DbPool>() {
        let now = cache::get_timestamp();
        let result = sqlx::query("DELETE FROM translation_cache WHERE expires_at < ?")
            .bind(now)
            .execute(&pool.0)
            .await;
            
        match result {
            Ok(res) => Ok(res.rows_affected() as u32),
            Err(e) => Err(e.to_string())
        }
    } else {
        Err("Database not initialized".to_string())
    }
}

#[command]
pub async fn get_translation_cache_stats(app: AppHandle) -> Result<(u32, u32), String> {
    if let Some(pool) = app.try_state::<DbPool>() {
        let now = cache::get_timestamp();
        
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM translation_cache")
            .fetch_one(&pool.0)
            .await
            .unwrap_or(0);
            
        let expired: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM translation_cache WHERE expires_at < ?")
            .bind(now)
            .fetch_one(&pool.0)
            .await
            .unwrap_or(0);
            
        Ok((total as u32, expired as u32))
    } else {
        Err("Database not initialized".to_string())
    }
}

fn process_movie_translations(translations: MovieTranslations) -> Option<TranslationData> {
    if translations.is_empty() {
        return None;
    }
    
    let preferred = translations.iter()
        .find(|t| t.country.as_deref() == Some("cn"))
        .or_else(|| translations.iter().find(|t| t.country.as_deref() == Some("tw")))
        .or_else(|| translations.iter().find(|t| t.country.as_deref() == Some("hk")))
        .or_else(|| translations.first());
    
    preferred.map(|t| TranslationData {
        title: t.title.clone(),
        overview: t.overview.clone(),
        tagline: t.tagline.clone(),
        updated_at: cache::get_timestamp() as u64,
    })
}

fn process_show_translations(translations: ShowTranslations) -> Option<TranslationData> {
    if translations.is_empty() {
        return None;
    }
    
    let preferred = translations.iter()
        .find(|t| t.country.as_deref() == Some("cn"))
        .or_else(|| translations.iter().find(|t| t.country.as_deref() == Some("tw")))
        .or_else(|| translations.iter().find(|t| t.country.as_deref() == Some("hk")))
        .or_else(|| translations.first());
    
    preferred.map(|t| TranslationData {
        title: t.title.clone(),
        overview: t.overview.clone(),
        tagline: t.tagline.clone(),
        updated_at: cache::get_timestamp() as u64,
    })
}

fn process_season_translations(translations: SeasonTranslations) -> Option<TranslationData> {
    if translations.is_empty() {
        return None;
    }
    
    let preferred = translations.iter()
        .find(|t| t.country.as_deref() == Some("cn"))
        .or_else(|| translations.iter().find(|t| t.country.as_deref() == Some("tw")))
        .or_else(|| translations.iter().find(|t| t.country.as_deref() == Some("hk")))
        .or_else(|| translations.first());
    
    preferred.map(|t| TranslationData {
        title: t.title.clone(),
        overview: t.overview.clone(),
        tagline: None,
        updated_at: cache::get_timestamp() as u64,
    })
}
