use tauri::{command, AppHandle, Manager};
use tauri_plugin_store::StoreExt;
use log::debug;
use crate::model::translation::{TranslationData, TranslationCacheItem};
use crate::model::movie::MovieTranslations;
use crate::model::shows::{ShowTranslations, SeasonTranslations};

const TRANSLATION_STORE_FILE: &str = "translations.json";
const CACHE_DURATION_MS: u64 = 7 * 24 * 60 * 60 * 1000; // 7天

#[command]
pub async fn get_movie_translation_cached(app: AppHandle, id: u32) -> Result<Option<TranslationData>, u16> {
    let cache_key = format!("movie_{}", id);
    
    // 先检查缓存
    if let Some(cached_data) = get_cached_translation(&app, &cache_key).await {
        return Ok(Some(cached_data));
    }
    
    // 缓存未命中，调用现有的movie_translation函数
    match super::movie::movie_translation(app.clone(), id, "zh".to_string()).await {
        Ok(translations) => {
            let translation_data = process_movie_translations(translations);
            
            // 保存到缓存
            let _ = set_cached_translation(&app, &cache_key, translation_data.clone()).await;
            
            Ok(translation_data)
        }
        Err(code) => Err(code)
    }
}

#[command]
pub async fn get_show_translation_cached(app: AppHandle, id: u32) -> Result<Option<TranslationData>, u16> {
    let cache_key = format!("show_{}", id);
    
    // 先检查缓存
    if let Some(cached_data) = get_cached_translation(&app, &cache_key).await {
        return Ok(Some(cached_data));
    }
    
    // 缓存未命中，调用现有的show_translation函数
    match super::shows::show_translation(app.clone(), id, "zh".to_string()).await {
        Ok(translations) => {
            let translation_data = process_show_translations(translations);
            
            // 保存到缓存
            let _ = set_cached_translation(&app, &cache_key, translation_data.clone()).await;
            
            Ok(translation_data)
        }
        Err(code) => Err(code)
    }
}

#[command]
pub async fn get_season_translation_cached(app: AppHandle, show_id: u32, season: u32) -> Result<Option<TranslationData>, u16> {
    let cache_key = format!("season_{}_{}", show_id, season);
    
    // 先检查缓存
    if let Some(cached_data) = get_cached_translation(&app, &cache_key).await {
        return Ok(Some(cached_data));
    }
    
    // 缓存未命中，调用现有的season_trans函数
    match super::shows::season_trans(app.clone(), show_id, season, "zh".to_string()).await {
        Ok(translations) => {
            let translation_data = process_season_translations(translations);
            
            // 保存到缓存
            let _ = set_cached_translation(&app, &cache_key, translation_data.clone()).await;
            
            Ok(translation_data)
        }
        Err(code) => Err(code)
    }
}

#[command]
pub async fn clear_expired_translations(app: AppHandle) -> Result<u32, String> {
    match app.store(TRANSLATION_STORE_FILE) {
        Ok(store) => {
            let entries = store.entries();
            let mut cleared_count = 0u32;
            
            for (key, value) in entries {
                if let Ok(cache_item) = serde_json::from_value::<TranslationCacheItem>(value) {
                    if cache_item.is_expired() {
                        let _ = store.delete(key);
                        cleared_count += 1;
                    }
                }
            }
            
            let _ = store.save();
            Ok(cleared_count)
        }
        Err(e) => Err(format!("访问翻译缓存失败: {}", e))
    }
}

#[command]
pub async fn get_translation_cache_stats(app: AppHandle) -> Result<(u32, u32), String> {
    match app.store(TRANSLATION_STORE_FILE) {
        Ok(store) => {
            let entries = store.entries();
            let mut total = 0u32;
            let mut expired = 0u32;
            
            for (_, value) in entries {
                if let Ok(cache_item) = serde_json::from_value::<TranslationCacheItem>(value) {
                    total += 1;
                    if cache_item.is_expired() {
                        expired += 1;
                    }
                }
            }
            
            Ok((total, expired))
        }
        Err(e) => Err(format!("访问翻译缓存失败: {}", e))
    }
}

// 私有辅助函数
async fn get_cached_translation(app: &AppHandle, cache_key: &str) -> Option<TranslationData> {
    match app.store(TRANSLATION_STORE_FILE) {
        Ok(store) => {
            if let Some(value) = store.get(cache_key) {
                if let Ok(cache_item) = serde_json::from_value::<TranslationCacheItem>(value) {
                    if !cache_item.is_expired() {
                        debug!("从缓存获取翻译: {}", cache_key);
                        return cache_item.data;
                    } else {
                        debug!("翻译缓存已过期: {}", cache_key);
                        let _ = store.delete(cache_key);
                        let _ = store.save();
                    }
                }
            }
        }
        Err(e) => debug!("访问翻译缓存失败: {}", e)
    }
    None
}

async fn set_cached_translation(app: &AppHandle, cache_key: &str, data: Option<TranslationData>) -> Result<(), String> {
    match app.store(TRANSLATION_STORE_FILE) {
        Ok(store) => {
            let cache_item = TranslationCacheItem::new(data, CACHE_DURATION_MS);
            let _ = store.set(cache_key, serde_json::to_value(&cache_item).unwrap());
            let _ = store.save();
            debug!("翻译已缓存: {}", cache_key);
            Ok(())
        }
        Err(e) => Err(format!("保存翻译缓存失败: {}", e))
    }
}

fn process_movie_translations(translations: MovieTranslations) -> Option<TranslationData> {
    if translations.is_empty() {
        return None;
    }
    
    // 优先选择简体中文
    let preferred = translations.iter()
        .find(|t| t.country.as_deref() == Some("cn"))
        .or_else(|| translations.iter().find(|t| t.country.as_deref() == Some("tw")))
        .or_else(|| translations.iter().find(|t| t.country.as_deref() == Some("hk")))
        .or_else(|| translations.first());
    
    preferred.map(|t| TranslationData {
        title: t.title.clone(),
        overview: t.overview.clone(),
        tagline: t.tagline.clone(),
        updated_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    })
}

fn process_show_translations(translations: ShowTranslations) -> Option<TranslationData> {
    if translations.is_empty() {
        return None;
    }
    
    // 优先选择简体中文
    let preferred = translations.iter()
        .find(|t| t.country.as_deref() == Some("cn"))
        .or_else(|| translations.iter().find(|t| t.country.as_deref() == Some("tw")))
        .or_else(|| translations.iter().find(|t| t.country.as_deref() == Some("hk")))
        .or_else(|| translations.first());
    
    preferred.map(|t| TranslationData {
        title: t.title.clone(),
        overview: t.overview.clone(),
        tagline: t.tagline.clone(),
        updated_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    })
}

fn process_season_translations(translations: SeasonTranslations) -> Option<TranslationData> {
    if translations.is_empty() {
        return None;
    }
    
    // 优先选择简体中文
    let preferred = translations.iter()
        .find(|t| t.country.as_deref() == Some("cn"))
        .or_else(|| translations.iter().find(|t| t.country.as_deref() == Some("tw")))
        .or_else(|| translations.iter().find(|t| t.country.as_deref() == Some("hk")))
        .or_else(|| translations.first());
    
    preferred.map(|t| TranslationData {
        title: t.title.clone(),
        overview: t.overview.clone(),
        tagline: None, // 季度通常没有tagline
        updated_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    })
} 