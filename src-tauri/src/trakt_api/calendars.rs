use crate::model::movie::Movie;
use crate::model::shows::{Episode, Show};
use crate::trakt_api::ApiClient;
use crate::trakt_api::API;
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Manager, Emitter};
use tokio::sync::Mutex;
use crate::db::{DbPool, cache};
use log::{info, error};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalendarMovie {
    pub released: Option<String>,
    pub movie: Movie,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalendarShow {
    pub first_aired: Option<String>,
    pub episode: Option<Episode>,
    pub show: Show,
}

#[command]
pub async fn get_calendar_movies(
    app: AppHandle,
    start_date: String,
    days: u32,
) -> Result<Vec<CalendarMovie>, u16> {
    let cache_key = format!("calendar_movies_{}_{}", start_date, days);
    let start_date_clone = start_date.clone();
    let app_clone = app.clone();
    
    handle_calendar_request(&app, &cache_key, &start_date, days, move |mut client, uri| async move {
        let uri = uri.replace("start_date", &start_date_clone).replace("days", &days.to_string());
        client.req_api(&app_clone, API.calendars.movies.method.as_str(), uri, None, None, Some(100), Some(1), true).await
    }).await
}

#[command]
pub async fn get_calendar_shows(
    app: AppHandle,
    start_date: String,
    days: u32,
) -> Result<Vec<CalendarShow>, u16> {
    let cache_key = format!("calendar_shows_{}_{}", start_date, days);
    let start_date_clone = start_date.clone();
    let app_clone = app.clone();
    
    handle_calendar_request(&app, &cache_key, &start_date, days, move |mut client, uri| async move {
        let uri = uri.replace("start_date", &start_date_clone).replace("days", &days.to_string());
        client.req_api(&app_clone, API.calendars.shows.method.as_str(), uri, None, None, Some(100), Some(1), true).await
    }).await
}

#[command]
pub async fn get_calendar_new_shows(
    app: AppHandle,
    start_date: String,
    days: u32,
) -> Result<Vec<CalendarShow>, u16> {
    let cache_key = format!("calendar_new_shows_{}_{}", start_date, days);
    let start_date_clone = start_date.clone();
    let app_clone = app.clone();
    
    handle_calendar_request(&app, &cache_key, &start_date, days, move |mut client, uri| async move {
        let uri = uri.replace("start_date", &start_date_clone).replace("days", &days.to_string());
        client.req_api(&app_clone, API.calendars.new_shows.method.as_str(), uri, None, None, Some(100), Some(1), true).await
    }).await
}

#[command]
pub async fn get_calendar_premieres(
    app: AppHandle,
    start_date: String,
    days: u32,
) -> Result<Vec<CalendarShow>, u16> {
    let cache_key = format!("calendar_premieres_{}_{}", start_date, days);
    let start_date_clone = start_date.clone();
    let app_clone = app.clone();
    
    handle_calendar_request(&app, &cache_key, &start_date, days, move |mut client, uri| async move {
        let uri = uri.replace("start_date", &start_date_clone).replace("days", &days.to_string());
        client.req_api(&app_clone, API.calendars.season_premieres.method.as_str(), uri, None, None, Some(100), Some(1), true).await
    }).await
}

#[command]
pub async fn get_my_calendar_shows(
    app: AppHandle,
    start_date: String,
    days: u32,
) -> Result<Vec<CalendarShow>, u16> {
    let cache_key = format!("calendar_my_shows_{}_{}", start_date, days);
    let start_date_clone = start_date.clone();
    let app_clone = app.clone();
    
    handle_calendar_request(&app, &cache_key, &start_date, days, move |mut client, uri| async move {
        let uri = uri.replace("start_date", &start_date_clone).replace("days", &days.to_string());
        client.req_api(&app_clone, API.calendars.my_shows.method.as_str(), uri, None, None, Some(100), Some(1), true).await
    }).await
}

// 泛型处理函数
async fn handle_calendar_request<T, F, Fut>(
    app: &AppHandle,
    cache_key: &str,
    start_date: &str,
    days: u32,
    fetch_fn: F,
) -> Result<Vec<T>, u16>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static + Clone,
    F: FnOnce(ApiClient, String) -> Fut + Send + 'static + Clone,
    Fut: std::future::Future<Output = Result<serde_json::Value, u16>> + Send,
{
    let mut cache_data = None;
    let mut should_fetch = true;

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(result) = cache::get_user_data_cache(&pool.0, cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<T>>(result.data) {
                cache_data = Some(data);
                should_fetch = result.is_stale;
            }
        }
    }

    if let Some(data) = cache_data {
        if !should_fetch {
            return Ok(data);
        }
        
        let app_clone = app.clone();
        let cache_key_clone = cache_key.to_string();
        let start_date_clone = start_date.to_string();
        let fetch_fn_clone = fetch_fn.clone();
        
        tokio::spawn(async move {
            match fetch_and_cache::<T, F, Fut>(&app_clone, &cache_key_clone, &start_date_clone, days, fetch_fn_clone).await {
                Ok(new_data) => {
                    let _ = app_clone.emit("user-data-update", serde_json::json!({
                        "key": cache_key_clone,
                        "data": new_data
                    }));
                },
                Err(_) => {}
            }
        });
        return Ok(data);
    }

    fetch_and_cache::<T, F, Fut>(app, cache_key, start_date, days, fetch_fn).await
}

async fn fetch_and_cache<T, F, Fut>(
    app: &AppHandle,
    cache_key: &str,
    _start_date: &str,
    _days: u32,
    fetch_fn: F,
) -> Result<Vec<T>, u16>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
    F: FnOnce(ApiClient, String) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<serde_json::Value, u16>> + Send,
{
    let client_state = app.state::<Mutex<ApiClient>>();
    let client_clone = {
        let client = client_state.lock().await;
        client.clone()
    };
    
    // URI logic here...
    let uri = if cache_key.contains("calendar_movies") {
        API.calendars.movies.uri.clone()
    } else if cache_key.contains("calendar_shows") {
        API.calendars.shows.uri.clone()
    } else if cache_key.contains("calendar_new_shows") {
        API.calendars.new_shows.uri.clone()
    } else if cache_key.contains("calendar_premieres") {
        API.calendars.season_premieres.uri.clone()
    } else {
        API.calendars.my_shows.uri.clone()
    };

    let result = fetch_fn(client_clone, uri).await;

    match result {
        Ok(result) => {
            let items = serde_json::from_value::<Vec<T>>(result.clone()).unwrap_or_default();
            if let Some(pool) = app.try_state::<DbPool>() {
                cache::set_user_data_cache(&pool.0, cache_key, &result).await;
            }
            Ok(items)
        }
        Err(e) => Err(e)
    }
}
