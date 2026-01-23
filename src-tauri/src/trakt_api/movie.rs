use crate::model::movie::{MovieAnticipated, MovieDetails, MovieTrending, MovieTranslations, Movie};
use tauri::command;
use crate::trakt_api::{ApiClient, API};
use tauri::{AppHandle, Manager, Emitter};
use tokio::sync::Mutex;
use std::collections::HashMap;
use crate::db::{DbPool, cache};
use log::{info, error};

#[command]
pub async fn movie_trending(app: AppHandle) -> Result<Vec<MovieTrending>, u16> {
    let cache_key = "api_movie_trending";
    
    // 1. Check Cache
    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<MovieTrending>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client
        .req_api(&app, API.movie.trending.method.as_str(), API.movie.trending.uri.clone(), None, None, None, None, true)
        .await;
        
    match result {
        Ok(result) => {
            let movie_trending = serde_json::from_value::<Vec<MovieTrending>>(result.clone()).unwrap();
            
            // 2. Save Cache
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_api_response_cache(&pool.0, cache_key, &result).await;
            }
            
            Ok(movie_trending)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn movie_trending_page(app: AppHandle, page: u32, limit: u32) -> Result<Vec<MovieTrending>, u16> {
    let cache_key = format!("api_movie_trending_p{}_l{}", page, limit);

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, &cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<MovieTrending>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client
        .req_api(&app, API.movie.trending.method.as_str(), API.movie.trending.uri.clone(), None, None, Some(limit), Some(page), true)
        .await;
        
    match result {
        Ok(result) => {
            let movie_trending = serde_json::from_value::<Vec<MovieTrending>>(result.clone()).unwrap();
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_api_response_cache(&pool.0, &cache_key, &result).await;
            }
            Ok(movie_trending)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn movie_popular_page(app: AppHandle, page: u32, limit: u32) -> Result<Vec<Movie>, u16> {
    let cache_key = format!("api_movie_popular_p{}_l{}", page, limit);

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, &cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<Movie>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client
        .req_api(&app, API.movie.popular.method.as_str(), API.movie.popular.uri.clone(), None, None, Some(limit), Some(page), true)
        .await;
        
    match result {
        Ok(result) => {
            let movie_popular = serde_json::from_value::<Vec<Movie>>(result.clone()).unwrap();
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_api_response_cache(&pool.0, &cache_key, &result).await;
            }
            Ok(movie_popular)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn movie_anticipated(app: AppHandle, page: u32, limit: u32) -> Result<Vec<MovieAnticipated>, u16> {
    let cache_key = format!("api_movie_anticipated_p{}_l{}", page, limit);

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, &cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<MovieAnticipated>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client
        .req_api(&app, API.movie.anticipated.method.as_str(), API.movie.anticipated.uri.clone(), None, None, Some(limit), Some(page), true)
        .await;
        
    match result {
        Ok(result) => {
            let movie_anticipated = serde_json::from_value::<Vec<MovieAnticipated>>(result.clone()).unwrap();
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_api_response_cache(&pool.0, &cache_key, &result).await;
            }
            Ok(movie_anticipated)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn movie_details(app: AppHandle, id: u32) -> Result<MovieDetails, u16> {
    let mut cache_data = None;
    let mut should_fetch = true;

    // 1. 尝试从缓存获取
    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(result) = cache::get_media_cache(&pool.0, "movie", id).await {
            if let Ok(details) = serde_json::from_value::<MovieDetails>(result.data) {
                cache_data = Some(details);
                should_fetch = result.is_stale;
            }
        }
    }

    // 2. 如果有缓存且新鲜，直接返回
    if let Some(data) = cache_data.clone() {
        if !should_fetch {
            return Ok(data);
        }
        // 如果数据陈旧 (stale)，启动后台刷新任务，但先返回旧数据
        let app_clone = app.clone();
        tokio::spawn(async move {
            match fetch_and_cache_movie_details(&app_clone, id).await {
                Ok(new_data) => {
                    info!("Background update success for movie {}", id);
                    // 通知前端刷新
                    let _ = app_clone.emit("media-update", serde_json::json!({
                        "type": "movie",
                        "id": id,
                        "data": new_data
                    }));
                },
                Err(e) => error!("Background update failed for movie {}: {}", id, e),
            }
        });
        
        return Ok(data);
    }

    // 3. 缓存未命中，同步请求
    fetch_and_cache_movie_details(&app, id).await
}

// 辅助函数：请求并缓存
async fn fetch_and_cache_movie_details(app: &AppHandle, id: u32) -> Result<MovieDetails, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let mut uri = API.movie.details.uri.clone();
    uri = uri.replace("id", &id.to_string());
    
    let mut params = HashMap::new();
    params.insert("extended".to_string(), "full".to_string());
    
    let result = client.req_api(app, API.movie.details.method.as_str(), uri, Some(params), None, None, None, true).await;
    
    match result {
        Ok(result) => {
            let movie_details = serde_json::from_value::<MovieDetails>(result.clone()).unwrap();
            
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_media_cache(&pool.0, "movie", id, &result, cache::CACHE_TTL_SHORT).await;
            }
            
            Ok(movie_details)
        }
        Err(e) => Err(e)
    }
}

#[command]
pub async fn movie_translation(app: AppHandle, id: u32, language: String) -> Result<MovieTranslations, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.movie.trans.uri.clone();
    uri = uri.replace("id", &id.to_string()).replace("language", &language);
    let result = client.req_api(&app, API.movie.trans.method.as_str(), uri, None, None, None, None, true).await;
    if let Ok(result) = result {
        let movie_translations = serde_json::from_value::<MovieTranslations>(result).unwrap();
        Ok(movie_translations)
    } else {
        Err(result.unwrap_err())
    }
}
