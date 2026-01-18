use crate::trakt_api::ApiClient;
use crate::trakt_api::API;
use crate::model::shows::{Show, ShowProgress, UpNextItem, ShowProgressSummary, Episode};
use tauri::{command, AppHandle, Manager};
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WatchedShow {
    pub plays: u32,
    pub last_watched_at: Option<String>,
    pub last_updated_at: Option<String>,
    pub show: Show,
}

#[command]
pub async fn get_show_progress(app: AppHandle, id: u32) -> Result<ShowProgress, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let uri = API.shows.progress.uri.replace("id", &id.to_string());
    
    let result = client.req_api(&app, API.shows.progress.method.as_str(), uri, None, None, None, None, false).await;
    
    match result {
        Ok(value) => {
            match serde_json::from_value::<ShowProgress>(value) {
                Ok(progress) => Ok(progress),
                Err(e) => {
                    log::error!("解析 ShowProgress 失败: {:?}", e);
                    Err(500)
                }
            }
        }
        Err(code) => Err(code)
    }
}

#[command]
pub async fn get_up_next(app: AppHandle, username: String) -> Result<Vec<UpNextItem>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    
    let watched_shows: Vec<WatchedShow> = {
        let mut client = client.lock().await;
        let uri = format!("/users/{}/watched/shows", username);
        
        let result = client.req_api(&app, "GET", uri, None, None, None, None, true).await;
        
        match result {
            Ok(value) => {
                match serde_json::from_value::<Vec<WatchedShow>>(value) {
                    Ok(shows) => shows,
                    Err(e) => {
                        log::error!("解析 watched shows 失败: {:?}", e);
                        return Err(500);
                    }
                }
            }
            Err(code) => return Err(code)
        }
    };
    
    let mut up_next_items: Vec<UpNextItem> = Vec::new();
    
    for watched in watched_shows.iter().take(20) {
        let progress: Option<ShowProgress> = {
            let mut client = client.lock().await;
            let uri = API.shows.progress.uri.replace("id", &watched.show.ids.trakt.to_string());
            
            match client.req_api(&app, API.shows.progress.method.as_str(), uri, None, None, None, None, false).await {
                Ok(value) => serde_json::from_value(value).ok(),
                Err(_) => None
            }
        };
        
        if let Some(progress) = progress {
            if progress.completed < progress.aired {
                if let Some(next_episode) = progress.next_episode {
                    up_next_items.push(UpNextItem {
                        show: watched.show.clone(),
                        next_episode,
                        progress: ShowProgressSummary {
                            aired: progress.aired,
                            completed: progress.completed,
                            last_watched_at: progress.last_watched_at,
                        },
                    });
                }
            }
        }
    }
    
    up_next_items.sort_by(|a, b| {
        let a_time = a.progress.last_watched_at.as_deref().unwrap_or("");
        let b_time = b.progress.last_watched_at.as_deref().unwrap_or("");
        b_time.cmp(a_time)
    });
    
    Ok(up_next_items)
}
