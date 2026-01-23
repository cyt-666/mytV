use crate::db::{DbPool, cache};
use crate::model::shows::{Show, ShowProgress, ShowProgressSummary, UpNextItem};
use crate::trakt_api::ApiClient;
use crate::trakt_api::API;
use log::{info, error};
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Manager, Emitter};
use tokio::sync::Mutex;

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

    let result = client
        .req_api(
            &app,
            API.shows.progress.method.as_str(),
            uri,
            None,
            None,
            None,
            None,
            false,
        )
        .await;

    match result {
        Ok(value) => match serde_json::from_value::<ShowProgress>(value) {
            Ok(progress) => Ok(progress),
            Err(e) => {
                log::error!("解析 ShowProgress 失败: {:?}", e);
                Err(500)
            }
        },
        Err(code) => Err(code),
    }
}

#[command]
pub async fn get_up_next(
    app: AppHandle,
    username: String,
    page: Option<usize>,
    limit: Option<usize>,
) -> Result<Vec<UpNextItem>, u16> {
    let current_page = page.unwrap_or(1);
    let current_limit = limit.unwrap_or(20);
    let cache_key = format!("up_next_{}_p{}", username, current_page);
    
    let mut cache_data = None;
    let mut should_fetch = true;

    // 尝试从缓存读取
    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(result) = cache::get_user_data_cache(&pool.0, &cache_key).await {
            if let Ok(items) = serde_json::from_value::<Vec<UpNextItem>>(result.data) {
                cache_data = Some(items);
                should_fetch = result.is_stale;
            }
        }
    }

    // 如果有缓存数据
    if let Some(data) = cache_data {
        if !should_fetch {
            // 缓存新鲜，直接返回
            info!("Up Next 缓存命中且新鲜，直接返回");
            return Ok(data);
        }
        
        // 缓存陈旧，先返回旧数据，后台刷新
        info!("Up Next 缓存陈旧，返回旧数据并后台刷新");
        let app_clone = app.clone();
        let username_clone = username.clone();
        let cache_key_clone = cache_key.clone();
        
        tokio::spawn(async move {
            match fetch_up_next(&app_clone, &username_clone, current_page, current_limit).await {
                Ok(new_data) => {
                    info!("Up Next 后台刷新成功，共 {} 条", new_data.len());
                    // 保存到缓存
                    if let Some(pool) = app_clone.try_state::<DbPool>() {
                        let json_data = serde_json::to_value(&new_data).unwrap_or_default();
                        cache::set_user_data_cache(&pool.0, &cache_key_clone, &json_data).await;
                    }
                    // 通知前端更新
                    let _ = app_clone.emit("user-data-update", serde_json::json!({
                        "key": cache_key_clone,
                        "data": new_data
                    }));
                },
                Err(e) => error!("Up Next 后台刷新失败: {}", e),
            }
        });
        
        return Ok(data);
    }

    // 无缓存，直接请求
    info!("Up Next 无缓存，直接请求 API");
    let result = fetch_up_next(&app, &username, current_page, current_limit).await?;
    
    // 保存到缓存
    if let Some(pool) = app.try_state::<DbPool>() {
        let json_data = serde_json::to_value(&result).unwrap_or_default();
        cache::set_user_data_cache(&pool.0, &cache_key, &json_data).await;
    }
    
    Ok(result)
}

/// 实际获取 Up Next 数据的内部函数
async fn fetch_up_next(
    app: &AppHandle,
    username: &str,
    page: usize,
    limit: usize,
) -> Result<Vec<UpNextItem>, u16> {
    let client = app.state::<Mutex<ApiClient>>();

    let watched_shows: Vec<WatchedShow> = {
        let mut client = client.lock().await;
        let uri = format!("/users/{}/watched/shows", username);

        let result = client
            .req_api(app, "GET", uri, None, None, None, None, true)
            .await;

        match result {
            Ok(value) => match serde_json::from_value::<Vec<WatchedShow>>(value) {
                Ok(shows) => shows,
                Err(e) => {
                    error!("解析 watched shows 失败: {:?}", e);
                    return Err(500);
                }
            },
            Err(code) => return Err(code),
        }
    };

    let mut handles = Vec::new();

    // 扫描窗口：为了找到足够的 "未完成" 剧集，扫描 3 倍的数量
    let scan_size = limit * 3;
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
            let uri = API
                .shows
                .progress
                .uri
                .replace("id", &watched.show.ids.trakt.to_string());

            let progress_result = client
                .req_api(
                    &app_handle,
                    API.shows.progress.method.as_str(),
                    uri,
                    None,
                    None,
                    None,
                    None,
                    false,
                )
                .await;

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
                }
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

    // 按最后观看时间排序
    up_next_items.sort_by(|a, b| {
        let a_time = a.progress.last_watched_at.as_deref().unwrap_or("");
        let b_time = b.progress.last_watched_at.as_deref().unwrap_or("");
        b_time.cmp(a_time)
    });

    Ok(up_next_items)
}
