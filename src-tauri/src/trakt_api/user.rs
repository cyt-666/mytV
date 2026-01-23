use crate::model::movie::Movie;
use crate::model::shows::{Episode, Season, Show};
use crate::model::user::Stats;
use crate::model::user::UserProfile;
use crate::trakt_api::{ApiClient, API};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::command;
use tauri::{AppHandle, Manager, Emitter};
use tokio::sync::Mutex;
use crate::db::{DbPool, cache};
use log::{info, error};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Watched {
    pub plays: u32,
    pub last_watched_at: String,
    pub last_updated_at: String,
    pub movie: Option<Movie>,
    pub show: Option<Show>,
    pub season: Option<Season>,
    pub episode: Option<Episode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CollectionItem {
    pub collected_at: Option<String>,
    pub updated_at: Option<String>,
    pub movie: Option<Movie>,
    pub show: Option<Show>,
    pub season: Option<Season>,
    pub episode: Option<Episode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WatchlistItem {
    pub listed_at: String,
    pub movie: Option<Movie>,
    pub show: Option<Show>,
    pub season: Option<Season>,
    pub episode: Option<Episode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HistoryItem {
    pub id: u64,
    pub watched_at: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub movie: Option<Movie>,
    pub show: Option<Show>,
    pub episode: Option<Episode>,
}

#[command]
pub async fn get_user_profile(app: AppHandle) -> Result<UserProfile, u16> {
    let cache_key = "user_profile_me"; // 简化，默认获取当前登录用户
    // 如果需要支持查看他人 profile，需传入 username 并作为 key 的一部分
    
    let mut cache_data = None;
    let mut should_fetch = true;

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(result) = cache::get_user_data_cache(&pool.0, cache_key).await {
            if let Ok(profile) = serde_json::from_value::<UserProfile>(result.data) {
                cache_data = Some(profile);
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
            match fetch_and_cache_profile(&app_clone, cache_key).await {
                Ok(new_data) => {
                    let _ = app_clone.emit("user-data-update", serde_json::json!({
                        "key": cache_key,
                        "data": new_data
                    }));
                },
                Err(_) => {}
            }
        });
        return Ok(data);
    }

    fetch_and_cache_profile(&app, cache_key).await
}

async fn fetch_and_cache_profile(app: &AppHandle, cache_key: &str) -> Result<UserProfile, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client
        .req_api(
            app,
            API.user.profile.method.as_str(),
            API.user.profile.uri.clone(),
            None,
            None,
            None,
            None,
            true,
        )
        .await;
    match result {
        Ok(result) => {
            let user_profile = serde_json::from_value::<UserProfile>(result.clone()).unwrap();
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_user_data_cache(&pool.0, cache_key, &result).await;
            }
            Ok(user_profile)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn get_watched(
    app: AppHandle,
    id: String,
    select_type: Option<String>,
    no_season: bool,
) -> Result<Vec<Watched>, u16> {
    // Watched 列表通常用于进度计算，数据量可能较大
    // 同样适用 SWR
    let cache_key = format!("watched_{}_{}", id, select_type.clone().unwrap_or("all".to_string()));
    let mut cache_data = None;
    let mut should_fetch = true;

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(result) = cache::get_user_data_cache(&pool.0, &cache_key).await {
            if let Ok(watched) = serde_json::from_value::<Vec<Watched>>(result.data) {
                cache_data = Some(watched);
                should_fetch = result.is_stale;
            }
        }
    }

    if let Some(data) = cache_data {
        if !should_fetch {
            return Ok(data);
        }
        
        let app_clone = app.clone();
        let id_clone = id.clone();
        let type_clone = select_type.clone();
        
        tokio::spawn(async move {
            let _ = fetch_and_cache_watched(&app_clone, &id_clone, type_clone, &cache_key).await;
        });
        return Ok(data);
    }

    fetch_and_cache_watched(&app, &id, select_type, &cache_key).await
}

async fn fetch_and_cache_watched(
    app: &AppHandle, 
    id: &str, 
    select_type: Option<String>,
    cache_key: &str
) -> Result<Vec<Watched>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.watched.uri.clone();
    uri = uri.replace("id", id);
    if let Some(t) = select_type {
        uri = uri.replace("type", &t);
    }
    let result = client
        .req_api(
            app,
            API.user.watched.method.as_str(),
            uri,
            None,
            None,
            None,
            None,
            true,
        )
        .await;
    match result {
        Ok(result) => {
            let watched = serde_json::from_value::<Vec<Watched>>(result.clone()).unwrap();
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_user_data_cache(&pool.0, cache_key, &result).await;
            }
            Ok(watched)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn get_user_stats(app: AppHandle, id: String) -> Result<Stats, u16> {
    let cache_key = format!("stats_{}", id);
    let mut cache_data = None;
    let mut should_fetch = true;

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(result) = cache::get_user_data_cache(&pool.0, &cache_key).await {
            if let Ok(stats) = serde_json::from_value::<Stats>(result.data) {
                cache_data = Some(stats);
                should_fetch = result.is_stale;
            }
        }
    }

    if let Some(data) = cache_data {
        if !should_fetch {
            return Ok(data);
        }
        
        let app_clone = app.clone();
        let id_clone = id.clone();
        tokio::spawn(async move {
            match fetch_and_cache_stats(&app_clone, &id_clone, &cache_key).await {
                Ok(new_data) => {
                    let _ = app_clone.emit("user-data-update", serde_json::json!({
                        "key": cache_key,
                        "data": new_data
                    }));
                }
                Err(_) => {}
            }
        });
        return Ok(data);
    }

    fetch_and_cache_stats(&app, &id, &cache_key).await
}

async fn fetch_and_cache_stats(app: &AppHandle, id: &str, cache_key: &str) -> Result<Stats, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.stats.uri.clone();
    uri = uri.replace("id", id);
    let result = client
        .req_api(
            app,
            API.user.stats.method.as_str(),
            uri,
            None,
            None,
            None,
            None,
            true,
        )
        .await;
    match result {
        Ok(result) => {
            let user_stats = serde_json::from_value::<Stats>(result.clone()).unwrap();
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_user_data_cache(&pool.0, cache_key, &result).await;
            }
            Ok(user_stats)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn get_collection(
    app: AppHandle,
    id: String,
    select_type: String,
) -> Result<Vec<CollectionItem>, u16> {
    let cache_key = format!("collection_{}_{}", select_type, id);
    let mut cache_data = None;
    let mut should_fetch = true;

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(result) = cache::get_user_data_cache(&pool.0, &cache_key).await {
            if let Ok(collection) = serde_json::from_value::<Vec<CollectionItem>>(result.data) {
                cache_data = Some(collection);
                should_fetch = result.is_stale;
            }
        }
    }

    if let Some(data) = cache_data {
        if !should_fetch {
            return Ok(data);
        }
        
        let app_clone = app.clone();
        let id_clone = id.clone();
        let type_clone = select_type.clone();
        
        tokio::spawn(async move {
            match fetch_and_cache_collection(&app_clone, &id_clone, &type_clone, &cache_key).await {
                Ok(new_data) => {
                    info!("Background update success for collection {}/{}", id_clone, type_clone);
                    let _ = app_clone.emit("user-data-update", serde_json::json!({
                        "key": cache_key,
                        "data": new_data
                    }));
                },
                Err(e) => error!("Background update failed for collection: {}", e),
            }
        });
        
        return Ok(data);
    }

    fetch_and_cache_collection(&app, &id, &select_type, &cache_key).await
}

async fn fetch_and_cache_collection(
    app: &AppHandle, 
    id: &str, 
    select_type: &str,
    cache_key: &str
) -> Result<Vec<CollectionItem>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.collection.uri.clone();
    uri = uri.replace("id", id).replace("type", select_type);
    
    let result = client
        .req_api(
            app,
            API.user.collection.method.as_str(),
            uri,
            None,
            None,
            None,
            None,
            true,
        )
        .await;
        
    match result {
        Ok(result) => {
            let collection = serde_json::from_value::<Vec<CollectionItem>>(result.clone()).unwrap();
            
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_user_data_cache(&pool.0, cache_key, &result).await;
            }
            
            Ok(collection)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn get_watchlist(
    app: AppHandle,
    id: String,
    select_type: String,
) -> Result<Vec<WatchlistItem>, u16> {
    let cache_key = format!("watchlist_{}_{}", select_type, id);
    let mut cache_data = None;
    let mut should_fetch = true;

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(result) = cache::get_user_data_cache(&pool.0, &cache_key).await {
            if let Ok(watchlist) = serde_json::from_value::<Vec<WatchlistItem>>(result.data) {
                cache_data = Some(watchlist);
                should_fetch = result.is_stale;
            }
        }
    }

    if let Some(data) = cache_data {
        if !should_fetch {
            return Ok(data);
        }
        
        let app_clone = app.clone();
        let id_clone = id.clone();
        let type_clone = select_type.clone();
        
        tokio::spawn(async move {
            match fetch_and_cache_watchlist(&app_clone, &id_clone, &type_clone, &cache_key).await {
                Ok(new_data) => {
                    info!("Background update success for watchlist {}/{}", id_clone, type_clone);
                    let _ = app_clone.emit("user-data-update", serde_json::json!({
                        "key": cache_key,
                        "data": new_data
                    }));
                },
                Err(e) => error!("Background update failed for watchlist: {}", e),
            }
        });
        
        return Ok(data);
    }

    fetch_and_cache_watchlist(&app, &id, &select_type, &cache_key).await
}

async fn fetch_and_cache_watchlist(
    app: &AppHandle, 
    id: &str, 
    select_type: &str,
    cache_key: &str
) -> Result<Vec<WatchlistItem>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.watchlist.uri.clone();
    uri = uri.replace("id", id).replace("type", select_type);

    let mut params = HashMap::new();
    params.insert("extended".to_string(), "full".to_string());

    // limit 100
    let result = client
        .req_api(
            app,
            API.user.watchlist.method.as_str(),
            uri,
            Some(params),
            None,
            Some(100),
            None,
            true,
        )
        .await;

    match result {
        Ok(result) => {
            let watchlist = serde_json::from_value::<Vec<WatchlistItem>>(result.clone()).unwrap();
            
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_user_data_cache(&pool.0, cache_key, &result).await;
            }
            
            Ok(watchlist)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn get_history(
    app: AppHandle,
    id: String,
    page: Option<u32>,
    limit: Option<u32>,
) -> Result<Vec<HistoryItem>, u16> {
    let current_page = page.unwrap_or(1);
    let current_limit = limit.unwrap_or(10);
    let cache_key = format!("history_{}_p{}_l{}", id, current_page, current_limit);
    
    let mut cache_data = None;
    let mut should_fetch = true;

    // 仅缓存第一页
    let is_first_page = current_page == 1;
    
    if is_first_page {
        if let Some(pool) = app.try_state::<DbPool>() {
            if let Some(result) = cache::get_user_data_cache(&pool.0, &cache_key).await {
                if let Ok(history) = serde_json::from_value::<Vec<HistoryItem>>(result.data) {
                    cache_data = Some(history);
                    should_fetch = result.is_stale;
                }
            }
        }
    }

    if let Some(data) = cache_data {
        if !should_fetch {
            return Ok(data);
        }
        
        // SWR
        let app_clone = app.clone();
        let id_clone = id.clone();
        tokio::spawn(async move {
            match fetch_and_cache_history(&app_clone, &id_clone, page, limit, &cache_key, is_first_page).await {
                Ok(new_data) => {
                    let _ = app_clone.emit("user-data-update", serde_json::json!({
                        "key": cache_key,
                        "data": new_data
                    }));
                },
                Err(_) => {}
            }
        });
        return Ok(data);
    }

    fetch_and_cache_history(&app, &id, page, limit, &cache_key, is_first_page).await
}

async fn fetch_and_cache_history(
    app: &AppHandle, 
    id: &str,
    page: Option<u32>,
    limit: Option<u32>,
    cache_key: &str,
    should_cache: bool
) -> Result<Vec<HistoryItem>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.history.uri.clone();
    uri = uri.replace("id", id);

    let mut query_parts: Vec<String> = Vec::new();
    if let Some(p) = page {
        query_parts.push(format!("page={}", p));
    }
    if let Some(l) = limit {
        query_parts.push(format!("limit={}", l));
    }
    if !query_parts.is_empty() {
        uri = format!("{}?{}", uri, query_parts.join("&"));
    }

    let result = client
        .req_api(
            app,
            API.user.history.method.as_str(),
            uri,
            None,
            None,
            None,
            None,
            true,
        )
        .await;
        
    match result {
        Ok(result) => {
            let history = serde_json::from_value::<Vec<HistoryItem>>(result.clone()).unwrap();
            
            if should_cache {
                if let Some(pool) = app.try_state::<DbPool>() {
                    cache::set_user_data_cache(&pool.0, cache_key, &result).await;
                }
            }
            
            Ok(history)
        }
        Err(e) => Err(e)
    }
}
