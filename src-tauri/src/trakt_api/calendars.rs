use crate::model::movie::Movie;
use crate::model::shows::{Episode, Show};
use crate::trakt_api::ApiClient;
use crate::trakt_api::API;
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Manager, Emitter};
use tokio::sync::Mutex;
use crate::db::{DbPool, cache};
use log::{info, error};
use chrono::{NaiveDate, Duration};
use std::collections::HashMap;

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

pub trait CalendarItemDate {
    fn get_date_str(&self) -> Option<String>;
}

impl CalendarItemDate for CalendarMovie {
    fn get_date_str(&self) -> Option<String> {
        self.released.clone()
    }
}

impl CalendarItemDate for CalendarShow {
    fn get_date_str(&self) -> Option<String> {
        // first_aired 是 ISO 8601 (2014-07-14T01:00:00.000Z)
        // 截取前10位 YYYY-MM-DD
        self.first_aired.as_ref().map(|s| s.chars().take(10).collect())
    }
}

#[command]
pub async fn get_calendar_movies(
    app: AppHandle,
    start_date: String,
    days: u32,
) -> Result<Vec<CalendarMovie>, u16> {
    handle_calendar_request(
        &app, 
        "calendar_movies", 
        &start_date, 
        days, 
        API.calendars.movies.method.as_str(),
        API.calendars.movies.uri.clone()
    ).await
}

#[command]
pub async fn get_calendar_shows(
    app: AppHandle,
    start_date: String,
    days: u32,
) -> Result<Vec<CalendarShow>, u16> {
    handle_calendar_request(
        &app, 
        "calendar_shows", 
        &start_date, 
        days,
        API.calendars.shows.method.as_str(),
        API.calendars.shows.uri.clone()
    ).await
}

#[command]
pub async fn get_calendar_new_shows(
    app: AppHandle,
    start_date: String,
    days: u32,
) -> Result<Vec<CalendarShow>, u16> {
    handle_calendar_request(
        &app, 
        "calendar_new_shows", 
        &start_date, 
        days,
        API.calendars.new_shows.method.as_str(),
        API.calendars.new_shows.uri.clone()
    ).await
}

#[command]
pub async fn get_calendar_premieres(
    app: AppHandle,
    start_date: String,
    days: u32,
) -> Result<Vec<CalendarShow>, u16> {
    handle_calendar_request(
        &app, 
        "calendar_premieres", 
        &start_date, 
        days,
        API.calendars.season_premieres.method.as_str(),
        API.calendars.season_premieres.uri.clone()
    ).await
}

#[command]
pub async fn get_my_calendar_shows(
    app: AppHandle,
    start_date: String,
    days: u32,
) -> Result<Vec<CalendarShow>, u16> {
    handle_calendar_request(
        &app, 
        "calendar_my_shows", 
        &start_date, 
        days,
        API.calendars.my_shows.method.as_str(),
        API.calendars.my_shows.uri.clone()
    ).await
}

// 泛型处理函数
async fn handle_calendar_request<T>(
    app: &AppHandle,
    cache_prefix: &str,
    start_date: &str,
    days: u32,
    method: &str,
    uri_template: String,
) -> Result<Vec<T>, u16>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static + Clone + CalendarItemDate,
{
    let mut final_results: Vec<T> = Vec::new();
    let mut missing_ranges: Vec<(NaiveDate, u32)> = Vec::new(); // (start_date, days)
    let mut stale_ranges: Vec<(NaiveDate, u32)> = Vec::new();   // (start_date, days)

    let start = match NaiveDate::parse_from_str(start_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            error!("Invalid start_date format: {}", start_date);
            return Err(400);
        }
    };

    // 1. 逐日检查缓存
    let mut current_missing_start: Option<NaiveDate> = None;
    let mut current_missing_count = 0;
    
    let mut current_stale_start: Option<NaiveDate> = None;
    let mut current_stale_count = 0;

    for i in 0..days {
        let current_date = start + Duration::days(i as i64);
        let date_str = current_date.format("%Y-%m-%d").to_string();
        let cache_key = format!("{}_{}", cache_prefix, date_str);
        
        let mut hit = false;
        let mut is_stale = false;
        
        if let Some(pool) = app.try_state::<DbPool>() {
            if let Some(result) = cache::get_user_data_cache(&pool.0, &cache_key).await {
                if let Ok(day_items) = serde_json::from_value::<Vec<T>>(result.data) {
                    final_results.extend(day_items);
                    hit = true;
                    is_stale = result.is_stale;
                }
            }
        }

        if !hit {
            // 记录缺失
            if current_missing_start.is_none() {
                current_missing_start = Some(current_date);
            }
            current_missing_count += 1;
        } else {
            // 命中缓存，中断缺失区间的记录
            if let Some(missing_start) = current_missing_start {
                missing_ranges.push((missing_start, current_missing_count));
                current_missing_start = None;
                current_missing_count = 0;
            }
            
            // 检查是否陈旧
            if is_stale {
                 if current_stale_start.is_none() {
                    current_stale_start = Some(current_date);
                }
                current_stale_count += 1;
            } else {
                // 不陈旧，中断陈旧区间的记录
                 if let Some(stale_start) = current_stale_start {
                    stale_ranges.push((stale_start, current_stale_count));
                    current_stale_start = None;
                    current_stale_count = 0;
                }
            }
        }
    }

    // 处理循环结束时可能存在的最后一段区间
    if let Some(missing_start) = current_missing_start {
        missing_ranges.push((missing_start, current_missing_count));
    }
    if let Some(stale_start) = current_stale_start {
        stale_ranges.push((stale_start, current_stale_count));
    }

    // 2. 获取缺失的数据 (阻塞)
    if !missing_ranges.is_empty() {
        info!("Calendar missing ranges: {:?}", missing_ranges);
        for (range_start, range_days) in missing_ranges {
            let range_start_str = range_start.format("%Y-%m-%d").to_string();
            
            match fetch_and_cache_range::<T>(app, cache_prefix, &range_start_str, range_days, method, uri_template.clone()).await {
                Ok(items) => {
                    final_results.extend(items);
                },
                Err(e) => return Err(e),
            }
        }
    }

    // 3. 后台刷新陈旧数据
    if !stale_ranges.is_empty() {
        info!("Calendar stale ranges to refresh: {:?}", stale_ranges);
        let app_clone = app.clone();
        let cache_prefix = cache_prefix.to_string();
        let method = method.to_string();
        let uri_template = uri_template.clone();
        
        tokio::spawn(async move {
            for (range_start, range_days) in stale_ranges {
                 let range_start_str = range_start.format("%Y-%m-%d").to_string();
                 match fetch_and_cache_range::<T>(&app_clone, &cache_prefix, &range_start_str, range_days, &method, uri_template.clone()).await {
                    Ok(_) => {
                         let _ = app_clone.emit("calendar-updated", ());
                    },
                    Err(_) => {}
                }
            }
        });
    }

    // 4. 结果去重和排序
    final_results.sort_by(|a, b| {
        let da = a.get_date_str().unwrap_or_default();
        let db = b.get_date_str().unwrap_or_default();
        da.cmp(&db)
    });
    
    Ok(final_results)
}

// 获取指定范围数据，按天拆分并缓存
async fn fetch_and_cache_range<T>(
    app: &AppHandle,
    cache_prefix: &str,
    start_date: &str,
    days: u32,
    method: &str,
    uri_template: String,
) -> Result<Vec<T>, u16>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static + CalendarItemDate + Clone,
{
    let client_state = app.state::<Mutex<ApiClient>>();
    let mut client_clone = {
        let client = client_state.lock().await;
        client.clone()
    };
    
    // 构造请求 URI (动态替换 start_date 和 days)
    let uri = uri_template.replace("start_date", start_date).replace("days", &days.to_string());

    let result = client_clone.req_api(app, method, uri, None, None, Some(100), Some(1), true).await;

    match result {
        Ok(result) => {
            let items = serde_json::from_value::<Vec<T>>(result.clone()).unwrap_or_default();
            
            // 按日期分组
            let mut grouped_items: HashMap<String, Vec<T>> = HashMap::new();
            
            // 初始化所有请求的日期为空列表（处理空数据的情况）
            if let Ok(start) = NaiveDate::parse_from_str(start_date, "%Y-%m-%d") {
                for i in 0..days {
                    let d = start + Duration::days(i as i64);
                    let d_str = d.format("%Y-%m-%d").to_string();
                    grouped_items.insert(d_str, Vec::new());
                }
            }
            
            // 填充数据
            for item in &items {
                if let Some(date_str) = item.get_date_str() {
                     // 确保日期在请求范围内 (Trakt 可能返回多余的或者跨时区的)
                     if grouped_items.contains_key(&date_str) {
                         if let Some(list) = grouped_items.get_mut(&date_str) {
                             list.push(item.clone());
                         }
                     }
                }
            }
            
            // 写入缓存
            if let Some(pool) = app.try_state::<DbPool>() {
                for (date_str, day_items) in grouped_items {
                    let cache_key = format!("{}_{}", cache_prefix, date_str);
                    let json_data = serde_json::to_value(&day_items).unwrap_or(serde_json::json!([]));
                    cache::set_user_data_cache(&pool.0, &cache_key, &json_data).await;
                }
            }
            
            Ok(items)
        }
        Err(e) => Err(e)
    }
}
