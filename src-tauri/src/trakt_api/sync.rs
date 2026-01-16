use crate::trakt_api::ApiClient;
use tauri::{command, AppHandle, Manager};
use crate::trakt_api::API;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SyncResponse {
    pub added: SyncStats,
    pub deleted: SyncStats,
    pub existing: SyncStats,
    pub not_found: NotFound,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SyncStats {
    pub movies: u32,
    pub shows: u32,
    pub seasons: u32,
    pub episodes: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NotFound {
    pub movies: Vec<NotFoundItem>,
    pub shows: Vec<NotFoundItem>,
    pub seasons: Vec<NotFoundItem>,
    pub episodes: Vec<NotFoundItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NotFoundItem {
    pub ids: ItemIds,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ItemIds {
    pub trakt: Option<u32>,
    pub slug: Option<String>,
}

#[command]
pub async fn add_to_collection(
    app: AppHandle,
    media_type: String,
    trakt_id: u32
) -> Result<SyncResponse, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let body = if media_type == "movie" {
        json!({
            "movies": [
                {
                    "ids": {
                        "trakt": trakt_id
                    }
                }
            ]
        })
    } else {
        json!({
            "shows": [
                {
                    "ids": {
                        "trakt": trakt_id
                    }
                }
            ]
        })
    };
    
    let result = client.req_api(
        &app,
        API.sync.add_to_collection.method.as_str(),
        API.sync.add_to_collection.uri.clone(),
        None,
        Some(body),
        None,
        None,
        false
    ).await;
    
    if let Ok(result) = result {
        let response = serde_json::from_value::<SyncResponse>(result).unwrap();
        Ok(response)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn remove_from_collection(
    app: AppHandle,
    media_type: String,
    trakt_id: u32
) -> Result<SyncResponse, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let body = if media_type == "movie" {
        json!({
            "movies": [
                {
                    "ids": {
                        "trakt": trakt_id
                    }
                }
            ]
        })
    } else {
        json!({
            "shows": [
                {
                    "ids": {
                        "trakt": trakt_id
                    }
                }
            ]
        })
    };
    
    let result = client.req_api(
        &app,
        API.sync.remove_from_collection.method.as_str(),
        API.sync.remove_from_collection.uri.clone(),
        None,
        Some(body),
        None,
        None,
        false
    ).await;
    
    if let Ok(result) = result {
        let response = serde_json::from_value::<SyncResponse>(result).unwrap();
        Ok(response)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn add_to_watchlist(
    app: AppHandle,
    media_type: String,
    trakt_id: u32
) -> Result<SyncResponse, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let body = if media_type == "movie" {
        json!({
            "movies": [
                {
                    "ids": {
                        "trakt": trakt_id
                    }
                }
            ]
        })
    } else {
        json!({
            "shows": [
                {
                    "ids": {
                        "trakt": trakt_id
                    }
                }
            ]
        })
    };
    
    let result = client.req_api(
        &app,
        API.sync.add_to_watchlist.method.as_str(),
        API.sync.add_to_watchlist.uri.clone(),
        None,
        Some(body),
        None,
        None,
        false
    ).await;
    
    if let Ok(result) = result {
        let response = serde_json::from_value::<SyncResponse>(result).unwrap();
        Ok(response)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn remove_from_watchlist(
    app: AppHandle,
    media_type: String,
    trakt_id: u32
) -> Result<SyncResponse, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let body = if media_type == "movie" {
        json!({
            "movies": [
                {
                    "ids": {
                        "trakt": trakt_id
                    }
                }
            ]
        })
    } else {
        json!({
            "shows": [
                {
                    "ids": {
                        "trakt": trakt_id
                    }
                }
            ]
        })
    };
    
    let result = client.req_api(
        &app,
        API.sync.remove_from_watchlist.method.as_str(),
        API.sync.remove_from_watchlist.uri.clone(),
        None,
        Some(body),
        None,
        None,
        false
    ).await;
    
    if let Ok(result) = result {
        let response = serde_json::from_value::<SyncResponse>(result).unwrap();
        Ok(response)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn mark_as_watched(
    app: AppHandle,
    media_type: String,
    trakt_id: u32
) -> Result<SyncResponse, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let body = if media_type == "movie" {
        json!({
            "movies": [
                {
                    "ids": {
                        "trakt": trakt_id
                    }
                }
            ]
        })
    } else {
        json!({
            "shows": [
                {
                    "ids": {
                        "trakt": trakt_id
                    }
                }
            ]
        })
    };
    
    let result = client.req_api(
        &app,
        API.sync.add_to_history.method.as_str(),
        API.sync.add_to_history.uri.clone(),
        None,
        Some(body),
        None,
        None,
        false
    ).await;
    
    if let Ok(result) = result {
        let response = serde_json::from_value::<SyncResponse>(result).unwrap();
        Ok(response)
    } else {
        Err(result.unwrap_err())
    }
}
