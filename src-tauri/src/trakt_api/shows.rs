use crate::model::shows::{
    Season, SeasonTranslations, Show, ShowAnticipated, ShowDetails, ShowTrending, ShowTranslations,
    Episode, ShowWatched, ShowCollected,
};

use tauri::command;
use crate::trakt_api::{ApiClient, API};
use tauri::{AppHandle, Manager, Emitter};
use tokio::sync::Mutex;
use std::collections::HashMap;
use crate::db::{DbPool, cache};
use log::{info, error};

#[command]
pub async fn show_trending(app: AppHandle) -> Result<Vec<ShowTrending>, u16> {
    let cache_key = "api_show_trending";

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<ShowTrending>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client
        .req_api(&app, API.shows.trending.method.as_str(), API.shows.trending.uri.clone(), None, None, None, None, true)
        .await;
        
    match result {
        Ok(result) => {
            let show_trending = serde_json::from_value::<Vec<ShowTrending>>(result.clone()).unwrap();
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_api_response_cache(&pool.0, cache_key, &result).await;
            }
            Ok(show_trending)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn show_trending_page(app: AppHandle, page: u32, limit: u32) -> Result<Vec<ShowTrending>, u16> {
    let cache_key = format!("api_show_trending_p{}_l{}", page, limit);

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, &cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<ShowTrending>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client
        .req_api(&app, API.shows.trending.method.as_str(), API.shows.trending.uri.clone(), None, None, Some(limit), Some(page), true)
        .await;
        
    match result {
        Ok(result) => {
            let show_trending = serde_json::from_value::<Vec<ShowTrending>>(result.clone()).unwrap();
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_api_response_cache(&pool.0, &cache_key, &result).await;
            }
            Ok(show_trending)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn show_popular_page(app: AppHandle, page: u32, limit: u32) -> Result<Vec<Show>, u16> {
    let cache_key = format!("api_show_popular_p{}_l{}", page, limit);

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, &cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<Show>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client
        .req_api(&app, API.shows.popular.method.as_str(), API.shows.popular.uri.clone(), None, None, Some(limit), Some(page), true)
        .await;
        
    match result {
        Ok(result) => {
            let show_popular = serde_json::from_value::<Vec<Show>>(result.clone()).unwrap();
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_api_response_cache(&pool.0, &cache_key, &result).await;
            }
            Ok(show_popular)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn show_anticipated(app: AppHandle, page: u32, limit: u32) -> Result<Vec<ShowAnticipated>, u16> {
    let cache_key = format!("api_show_anticipated_p{}_l{}", page, limit);

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, &cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<ShowAnticipated>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client
        .req_api(&app, API.shows.anticipated.method.as_str(), API.shows.anticipated.uri.clone(), None, None, Some(limit), Some(page), true)
        .await;
        
    match result {
        Ok(result) => {
            let show_anticipated = serde_json::from_value::<Vec<ShowAnticipated>>(result.clone()).unwrap();
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_api_response_cache(&pool.0, &cache_key, &result).await;
            }
            Ok(show_anticipated)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn show_details(app: AppHandle, id: u32) -> Result<ShowDetails, u16> {
    let mut cache_data = None;
    let mut should_fetch = true;

    // 1. 尝试从缓存获取
    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(result) = cache::get_media_cache(&pool.0, "show", id).await {
            if let Ok(details) = serde_json::from_value::<ShowDetails>(result.data) {
                cache_data = Some(details);
                should_fetch = result.is_stale;
            }
        }
    }

    if let Some(data) = cache_data.clone() {
        if !should_fetch {
            return Ok(data);
        }
        
        // SWR
        let app_clone = app.clone();
        tokio::spawn(async move {
            match fetch_and_cache_show_details(&app_clone, id).await {
                Ok(new_data) => {
                    info!("Background update success for show {}", id);
                    let _ = app_clone.emit("media-update", serde_json::json!({
                        "type": "show",
                        "id": id,
                        "data": new_data
                    }));
                },
                Err(e) => error!("Background update failed for show {}: {}", id, e),
            }
        });
        
        return Ok(data);
    }

    fetch_and_cache_show_details(&app, id).await
}

async fn fetch_and_cache_show_details(app: &AppHandle, id: u32) -> Result<ShowDetails, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.shows.details.uri.clone();
    uri = uri.replace("id", &id.to_string());
    
    let mut params = HashMap::new();
    params.insert("extended".to_string(), "full".to_string());
    
    let result = client.req_api(app, API.shows.details.method.as_str(), uri, Some(params), None, None, None, true).await;
    
    match result {
        Ok(result) => {
            let show_details = serde_json::from_value::<ShowDetails>(result.clone()).unwrap();
            
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_media_cache(&pool.0, "show", id, &result, cache::CACHE_TTL_SHORT).await;
            }
            
            Ok(show_details)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn show_seasons(app: AppHandle, id: u32) -> Result<Vec<Season>, u16> {
    // 季度列表同样适用 SWR，因为可能会有新季度或者新集数信息
    let mut cache_data = None;
    let mut should_fetch = true;

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(result) = cache::get_media_cache(&pool.0, "show_seasons", id).await {
            if let Ok(seasons) = serde_json::from_value::<Vec<Season>>(result.data) {
                cache_data = Some(seasons);
                should_fetch = result.is_stale;
            }
        }
    }

    if let Some(data) = cache_data {
        if !should_fetch {
            return Ok(data);
        }
        
        let app_clone = app.clone();
        tokio::spawn(async move {
            match fetch_and_cache_show_seasons(&app_clone, id).await {
                Ok(new_data) => {
                    info!("Background update success for seasons of show {}", id);
                    let _ = app_clone.emit("media-update", serde_json::json!({
                        "type": "seasons",
                        "id": id,
                        "data": new_data
                    }));
                },
                Err(e) => error!("Background update failed for seasons of show {}: {}", id, e),
            }
        });
        
        return Ok(data);
    }

    fetch_and_cache_show_seasons(&app, id).await
}

async fn fetch_and_cache_show_seasons(app: &AppHandle, id: u32) -> Result<Vec<Season>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.shows.seasons.uri.clone();
    uri = uri.replace("id", &id.to_string());
    
    let mut params = HashMap::new();
    params.insert("extended".to_string(), "full".to_string());
    
    let result = client.req_api(app, API.shows.seasons.method.as_str(), uri, Some(params), None, None, None, true).await;
    
    match result {
        Ok(result) => {
            let seasons = serde_json::from_value::<Vec<Season>>(result.clone()).unwrap();
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_media_cache(&pool.0, "show_seasons", id, &result, cache::CACHE_TTL_SHORT).await;
            }
            Ok(seasons)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn get_season_episodes(app: AppHandle, id: u32, season: u32) -> Result<Vec<Episode>, u16> {
    let mut cache_data = None;
    let mut should_fetch = true;

    if let Some(pool) = app.try_state::<DbPool>() {
        // 复用 get_media_cache，但传入自定义的 ID
        // ... (注释略)
        
        if let Some(result) = cache::get_media_cache(&pool.0, &format!("season_{}", id), season).await {
            if let Ok(episodes) = serde_json::from_value::<Vec<Episode>>(result.data) {
                cache_data = Some(episodes);
                should_fetch = result.is_stale;
            }
        }
    }

    if let Some(data) = cache_data {
        if !should_fetch {
            return Ok(data);
        }
        
        let app_clone = app.clone();
        tokio::spawn(async move {
            match fetch_and_cache_season_episodes(&app_clone, id, season).await {
                Ok(new_data) => {
                    info!("Background update success for season {} of show {}", season, id);
                    let _ = app_clone.emit("media-update", serde_json::json!({
                        "type": "season",
                        "id": id,
                        "season": season,
                        "data": new_data
                    }));
                },
                Err(e) => error!("Background update failed for season {} of show {}: {}", season, id, e),
            }
        });
        
        return Ok(data);
    }

    fetch_and_cache_season_episodes(&app, id, season).await
}

async fn fetch_and_cache_season_episodes(app: &AppHandle, id: u32, season: u32) -> Result<Vec<Episode>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.shows.season_episodes.uri.clone();
    uri = uri.replace("id", &id.to_string()).replace("season_number", &season.to_string());
    
    let mut params = HashMap::new();
    params.insert("extended".to_string(), "full".to_string());
    
    let result = client.req_api(app, API.shows.season_episodes.method.as_str(), uri, Some(params), None, None, None, true).await;
    
    match result {
        Ok(result) => {
            let episodes = serde_json::from_value::<Vec<Episode>>(result.clone()).unwrap();
            if let Some(pool) = app.try_state::<DbPool>() {
                // 缓存 Key: season_{id}_{season}
                cache::set_media_cache(&pool.0, &format!("season_{}", id), season, &result, cache::CACHE_TTL_SHORT).await;
            }
            Ok(episodes)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn get_episode_details(app: AppHandle, id: u32, season: u32, episode: u32) -> Result<Episode, u16> {
    let mut cache_data = None;
    let mut should_fetch = true;
    // 构造缓存 Key: episode_{id}_{season}，且 trakt_id 为 episode_num
    // 最终 ID: episode_{id}_{season}_{episode}
    
    if let Some(pool) = app.try_state::<DbPool>() {
        let type_prefix = format!("episode_{}_{}", id, season);
        if let Some(result) = cache::get_media_cache(&pool.0, &type_prefix, episode).await {
            if let Ok(details) = serde_json::from_value::<Episode>(result.data) {
                cache_data = Some(details);
                should_fetch = result.is_stale;
            }
        }
    }

    if let Some(data) = cache_data {
        if !should_fetch {
            return Ok(data);
        }
        
        let app_clone = app.clone();
        tokio::spawn(async move {
            match fetch_and_cache_episode_details(&app_clone, id, season, episode).await {
                Ok(new_data) => {
                    info!("Background update success for episode S{}E{} of show {}", season, episode, id);
                    let _ = app_clone.emit("media-update", serde_json::json!({
                        "type": "episode",
                        "id": id,
                        "season": season,
                        "episode": episode,
                        "data": new_data
                    }));
                },
                Err(e) => error!("Background update failed for episode S{}E{} of show {}: {}", season, episode, id, e),
            }
        });
        
        return Ok(data);
    }

    fetch_and_cache_episode_details(&app, id, season, episode).await
}

async fn fetch_and_cache_episode_details(app: &AppHandle, id: u32, season: u32, episode: u32) -> Result<Episode, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.shows.episode_details.uri.clone();
    uri = uri
        .replace("id", &id.to_string())
        .replace("season_number", &season.to_string())
        .replace("episode_number", &episode.to_string());
        
    let mut params = HashMap::new();
    params.insert("extended".to_string(), "full".to_string());

    let result = client.req_api(app, API.shows.episode_details.method.as_str(), uri, Some(params), None, None, None, true).await;
    
    match result {
        Ok(result) => {
            let episode_details = serde_json::from_value::<Episode>(result.clone()).unwrap();
            if let Some(pool) = app.try_state::<DbPool>() {
                // 缓存 Key: episode_{id}_{season}_{episode}
                let type_prefix = format!("episode_{}_{}", id, season);
                cache::set_media_cache(&pool.0, &type_prefix, episode, &result, cache::CACHE_TTL_SHORT).await;
            }
            Ok(episode_details)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn show_translation(app: AppHandle, id: u32, language: String) -> Result<ShowTranslations, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.shows.trans.uri.clone();
    uri = uri.replace("id", &id.to_string()).replace("language", &language);
    let result = client.req_api(&app, API.shows.trans.method.as_str(), uri, None, None, None, None, true).await;
    if let Ok(result) = result {
        let show_translations = serde_json::from_value::<ShowTranslations>(result).unwrap();
        Ok(show_translations)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn show_watched_period(
    app: AppHandle,
    period: String,
    page: u32,
    limit: u32
) -> Result<Vec<ShowWatched>, u16> {
    let cache_key = format!("api_show_watched_{}_p{}_l{}", period, page, limit);

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, &cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<ShowWatched>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let mut uri = API.shows.watched.uri.clone();
    uri = uri.replace("period", &period);
    
    let result = client
        .req_api(
            &app,
            API.shows.watched.method.as_str(),
            uri,
            None,
            None,
            Some(limit),
            Some(page),
            true
        )
        .await;

    match result {
        Ok(result) => {
            let show_watched = serde_json::from_value::<Vec<ShowWatched>>(result.clone()).unwrap();
            
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_api_response_cache(&pool.0, &cache_key, &result).await;
            }
            
            Ok(show_watched)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn show_collected_period(
    app: AppHandle,
    period: String,
    page: u32,
    limit: u32
) -> Result<Vec<ShowCollected>, u16> {
    let cache_key = format!("api_show_collected_{}_p{}_l{}", period, page, limit);

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, &cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<ShowCollected>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let mut uri = API.shows.collected.uri.clone();
    uri = uri.replace("period", &period);
    
    let result = client
        .req_api(
            &app,
            API.shows.collected.method.as_str(),
            uri,
            None,
            None,
            Some(limit),
            Some(page),
            true
        )
        .await;

    match result {
        Ok(result) => {
            let show_collected = serde_json::from_value::<Vec<ShowCollected>>(result.clone()).unwrap();
            
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_api_response_cache(&pool.0, &cache_key, &result).await;
            }
            
            Ok(show_collected)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn season_trans(app: AppHandle, id: u32, season: u32, language: String) -> Result<SeasonTranslations, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.shows.season_trans.uri.clone();
    // 修正: 占位符是 season_number
    uri = uri.replace("id", &id.to_string()).replace("season_number", &season.to_string()).replace("language", &language);
    
    let result = client.req_api(&app, API.shows.season_trans.method.as_str(), uri, None, None, None, None, true).await;
    if let Ok(result) = result {
        let translations = serde_json::from_value::<SeasonTranslations>(result).unwrap();
        Ok(translations)
    } else {
        Err(result.unwrap_err())
    }
}
