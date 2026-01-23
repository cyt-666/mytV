use crate::model::movie::Movie;
use crate::model::shows::{Episode, Season, Show};
use crate::model::user::Stats;
use crate::model::user::UserProfile;
use crate::trakt_api::{ApiClient, API};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::command;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

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
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client
        .req_api(
            &app,
            API.user.profile.method.as_str(),
            API.user.profile.uri.clone(),
            None,
            None,
            None,
            None,
            true,
        )
        .await;
    if let Ok(result) = result {
        println!("{}", result);
        let user_profile = serde_json::from_value::<UserProfile>(result).unwrap();
        Ok(user_profile)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn get_watched(
    app: AppHandle,
    id: String,
    select_type: Option<String>,
    no_season: bool,
) -> Result<Vec<Watched>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.watched.uri.clone();
    uri = uri.replace("id", &id);
    if let Some(select_type) = select_type {
        uri = uri.replace("type", &select_type);
    }
    let result = client
        .req_api(
            &app,
            API.user.watched.method.as_str(),
            uri,
            None,
            None,
            None,
            None,
            true,
        )
        .await;
    if let Ok(result) = result {
        println!("{}", result);
        let watched = serde_json::from_value::<Vec<Watched>>(result).unwrap();
        Ok(watched)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn get_user_stats(app: AppHandle, id: String) -> Result<Stats, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.stats.uri.clone();
    uri = uri.replace("id", &id);
    let result = client
        .req_api(
            &app,
            API.user.stats.method.as_str(),
            uri,
            None,
            None,
            None,
            None,
            true,
        )
        .await;
    if let Ok(result) = result {
        println!("{}", result);
        let user_stats = serde_json::from_value::<Stats>(result).unwrap();
        Ok(user_stats)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn get_collection(
    app: AppHandle,
    id: String,
    select_type: String,
) -> Result<Vec<CollectionItem>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.collection.uri.clone();
    uri = uri.replace("id", &id).replace("type", &select_type);
    let result = client
        .req_api(
            &app,
            API.user.collection.method.as_str(),
            uri,
            None,
            None,
            None,
            None,
            true,
        )
        .await;
    if let Ok(result) = result {
        println!("{}", result);
        let collection = serde_json::from_value::<Vec<CollectionItem>>(result).unwrap();
        Ok(collection)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn get_watchlist(
    app: AppHandle,
    id: String,
    select_type: String,
) -> Result<Vec<WatchlistItem>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.watchlist.uri.clone();
    uri = uri.replace("id", &id).replace("type", &select_type);

    // Add extended=full to params to ensure we get rating, overview etc.
    let mut params = HashMap::new();
    params.insert("extended".to_string(), "full".to_string());

    // Use limit=100 to fetch more items at once
    let result = client
        .req_api(
            &app,
            API.user.watchlist.method.as_str(),
            uri,
            Some(params),
            None,
            Some(100),
            None,
            true,
        )
        .await;

    if let Ok(result) = result {
        let watchlist = serde_json::from_value::<Vec<WatchlistItem>>(result).unwrap();
        Ok(watchlist)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn get_history(
    app: AppHandle,
    id: String,
    page: Option<u32>,
    limit: Option<u32>,
) -> Result<Vec<HistoryItem>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.history.uri.clone();
    uri = uri.replace("id", &id);

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
            &app,
            API.user.history.method.as_str(),
            uri,
            None,
            None,
            None,
            None,
            true,
        )
        .await;
    if let Ok(result) = result {
        println!("{}", result);
        let history = serde_json::from_value::<Vec<HistoryItem>>(result).unwrap();
        Ok(history)
    } else {
        Err(result.unwrap_err())
    }
}
