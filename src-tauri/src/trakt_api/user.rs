use crate::model::user::UserProfile;
use crate::model::user::Stats;
use crate::model::movie::Movie;
use crate::model::shows::Show;
use tauri::command;
use crate::trakt_api::{ApiClient, API};
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Watched {
    pub plays: u32,
    pub last_watched_at: String,
    pub last_updated_at: String,
    pub movie: Option<Movie>,
    pub show: Option<Show>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CollectionItem {
    pub collected_at: String,
    pub updated_at: String,
    pub movie: Option<Movie>,
    pub show: Option<Show>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WatchlistItem {
    pub listed_at: String,
    pub movie: Option<Movie>,
    pub show: Option<Show>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HistoryItem {
    pub id: u64,
    pub watched_at: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub movie: Option<Movie>,
    pub show: Option<Show>,
}

#[command]
pub async fn get_user_profile(app: AppHandle) -> Result<UserProfile, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client.req_api(&app, API.user.profile.method.as_str(), API.user.profile.uri.clone(), None, None, None, None, true).await;
    if let Ok(result) = result {
        println!("{}", result);
        let user_profile = serde_json::from_value::<UserProfile>(result).unwrap();
        Ok(user_profile)
    } else {
        Err(result.unwrap_err())
    }
}


#[command]
pub async fn get_watched(app: AppHandle, id: String, select_type: Option<String>, no_season: bool) -> Result<Vec<Watched>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.watched.uri.clone();
    uri = uri.replace("id", &id);
    if let Some(select_type) = select_type {
        uri = uri.replace("type", &select_type);
    }
    let result = client.req_api(&app, API.user.watched.method.as_str(), uri, None, None, None, None, true).await;
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
    let result = client.req_api(&app, API.user.stats.method.as_str(), uri, None, None, None, None, true).await;
    if let Ok(result) = result {
        println!("{}", result);
        let user_stats = serde_json::from_value::<Stats>(result).unwrap();
        Ok(user_stats)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn get_collection(app: AppHandle, id: String, select_type: String) -> Result<Vec<CollectionItem>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.collection.uri.clone();
    uri = uri.replace("id", &id).replace("type", &select_type);
    let result = client.req_api(&app, API.user.collection.method.as_str(), uri, None, None, None, None, true).await;
    if let Ok(result) = result {
        println!("{}", result);
        let collection = serde_json::from_value::<Vec<CollectionItem>>(result).unwrap();
        Ok(collection)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn get_watchlist(app: AppHandle, id: String, select_type: String) -> Result<Vec<WatchlistItem>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.watchlist.uri.clone();
    uri = uri.replace("id", &id).replace("type", &select_type);
    let result = client.req_api(&app, API.user.watchlist.method.as_str(), uri, None, None, None, None, true).await;
    if let Ok(result) = result {
        println!("{}", result);
        let watchlist = serde_json::from_value::<Vec<WatchlistItem>>(result).unwrap();
        Ok(watchlist)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn get_history(app: AppHandle, id: String) -> Result<Vec<HistoryItem>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.user.history.uri.clone();
    uri = uri.replace("id", &id);
    let result = client.req_api(&app, API.user.history.method.as_str(), uri, None, None, None, None, true).await;
    if let Ok(result) = result {
        println!("{}", result);
        let history = serde_json::from_value::<Vec<HistoryItem>>(result).unwrap();
        Ok(history)
    } else {
        Err(result.unwrap_err())
    }
}
