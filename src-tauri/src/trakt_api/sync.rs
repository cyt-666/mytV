use crate::trakt_api::ApiClient;
use crate::trakt_api::API;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{command, AppHandle, Manager};
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SyncResponse {
    pub added: Option<SyncStats>,
    pub deleted: Option<SyncStats>,
    pub existing: Option<SyncStats>,
    pub not_found: Option<NotFound>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SyncStats {
    #[serde(default)]
    pub movies: u32,
    #[serde(default)]
    pub shows: u32,
    #[serde(default)]
    pub seasons: u32,
    #[serde(default)]
    pub episodes: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NotFound {
    #[serde(default)]
    pub movies: Vec<NotFoundItem>,
    #[serde(default)]
    pub shows: Vec<NotFoundItem>,
    #[serde(default)]
    pub seasons: Vec<NotFoundItem>,
    #[serde(default)]
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
    trakt_id: u32,
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
    } else if media_type == "season" {
        json!({
            "seasons": [
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

    let result = client
        .req_api(
            &app,
            API.sync.add_to_collection.method.as_str(),
            API.sync.add_to_collection.uri.clone(),
            None,
            Some(body),
            None,
            None,
            false,
        )
        .await;

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
    trakt_id: u32,
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
    } else if media_type == "season" {
        json!({
            "seasons": [
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

    let result = client
        .req_api(
            &app,
            API.sync.remove_from_collection.method.as_str(),
            API.sync.remove_from_collection.uri.clone(),
            None,
            Some(body),
            None,
            None,
            false,
        )
        .await;

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
    trakt_id: u32,
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
    } else if media_type == "season" {
        json!({
            "seasons": [
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

    let result = client
        .req_api(
            &app,
            API.sync.add_to_watchlist.method.as_str(),
            API.sync.add_to_watchlist.uri.clone(),
            None,
            Some(body),
            None,
            None,
            false,
        )
        .await;

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
    trakt_id: u32,
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
    } else if media_type == "season" {
        json!({
            "seasons": [
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

    let result = client
        .req_api(
            &app,
            API.sync.remove_from_watchlist.method.as_str(),
            API.sync.remove_from_watchlist.uri.clone(),
            None,
            Some(body),
            None,
            None,
            false,
        )
        .await;

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
    trakt_id: u32,
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
    } else if media_type == "season" {
        json!({
            "seasons": [
                {
                    "ids": {
                        "trakt": trakt_id
                    }
                }
            ]
        })
    } else if media_type == "episode" {
        json!({
            "episodes": [
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

    let result = client
        .req_api(
            &app,
            API.sync.add_to_history.method.as_str(),
            API.sync.add_to_history.uri.clone(),
            None,
            Some(body),
            None,
            None,
            false,
        )
        .await;

    if let Ok(result) = result {
        let response = serde_json::from_value::<SyncResponse>(result).unwrap();
        Ok(response)
    } else {
        Err(result.unwrap_err())
    }
}
