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
