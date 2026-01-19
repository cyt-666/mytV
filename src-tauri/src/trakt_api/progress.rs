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
pub async fn get_up_next(app: AppHandle, username: String, page: Option<usize>, limit: Option<usize>) -> Result<Vec<UpNextItem>, u16> {
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
    
    let mut handles = Vec::new();
    
    let page = page.unwrap_or(1);
    let limit_val = limit.unwrap_or(20);
    
    // Improved pagination logic:
    // Instead of just taking a slice of `watched_shows` based on page * limit,
    // we need to scan through `watched_shows` until we find enough "up next" items
    // or until we run out of source items.
    
    // However, since this is a stateless API request, we can't easily "resume" scanning
    // from where we left off without scanning everything before it again or tracking indices.
    
    // A robust compromise for infinite scroll on a filtered list:
    // 1. We still use "page" and "limit" to slice the SOURCE list (watched_shows).
    // 2. But we increase the scan window. If we want 20 items, we might need to scan 40 or 60
    //    watched shows to find 20 that are "active" (not completed).
    
    // Let's assume a 50% hit rate (conservative). We scan `limit * 2` items from the source.
    // AND we must return all valid items found in that scan window.
    // The frontend will handle receiving fewer than `limit` items differently if we signal it.
    
    // Actually, sticking to strict source pagination is safer for consistency.
    // Page 1: source items 0-50.
    // Page 2: source items 50-100.
    // The result size will vary (e.g., 10 items, 25 items).
    // The frontend should just append whatever it gets.
    // The "Stop" condition is when source items runs out.
    
    // Let's increase the scanning window to ensure we likely get *some* results per page.
    let scan_size = limit_val * 3; // Scan 60 source items to try to find 20 active ones
    let skip = (page - 1) * scan_size;
    
    let page_items = watched_shows.into_iter().skip(skip).take(scan_size);
    
    for watched in page_items {
        let app_handle = app.clone();
        let client_clone = {
            let client_guard = client.lock().await;
            client_guard.clone()
        };
        
        handles.push(tokio::spawn(async move {
            let mut client = client_clone;
            let uri = API.shows.progress.uri.replace("id", &watched.show.ids.trakt.to_string());
            
            let progress_result = client.req_api(&app_handle, API.shows.progress.method.as_str(), uri, None, None, None, None, false).await;
            
            match progress_result {
                Ok(value) => {
                    if let Ok(progress) = serde_json::from_value::<ShowProgress>(value) {
                        if progress.completed < progress.aired {
                            if let Some(next_episode) = progress.next_episode {
                                return Some(UpNextItem {
                                    show: watched.show,
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
                },
                Err(_) => {}
            }
            None
        }));
    }
    
    let mut up_next_items = Vec::new();
    for handle in handles {
        if let Ok(Some(item)) = handle.await {
            up_next_items.push(item);
        }
    }
    
    // Sort by last watched
    up_next_items.sort_by(|a, b| {
        let a_time = a.progress.last_watched_at.as_deref().unwrap_or("");
        let b_time = b.progress.last_watched_at.as_deref().unwrap_or("");
        b_time.cmp(a_time)
    });
    
    // No explicit "limit" on result size here - return everything found in the scan window.
    // The frontend will receive a variable number of items.
    
    Ok(up_next_items)
}
