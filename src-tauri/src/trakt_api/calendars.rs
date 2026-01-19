use crate::trakt_api::ApiClient;
use tauri::{command, AppHandle, Manager};
use crate::trakt_api::API;
use crate::model::movie::Movie;
use crate::model::shows::{Show, Episode};
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

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
pub async fn get_calendar_movies(app: AppHandle, start_date: String, days: u32) -> Result<Vec<CalendarMovie>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let mut uri = API.calendars.movies.uri.clone();
    uri = uri.replace("start_date", &start_date).replace("days", &days.to_string());
    
    let result = client.req_api(&app, API.calendars.movies.method.as_str(), uri, None, None, Some(100), Some(1), true).await;
    
    if let Ok(result) = result {
        let movies = serde_json::from_value::<Vec<CalendarMovie>>(result).unwrap_or_default();
        Ok(movies)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn get_calendar_shows(app: AppHandle, start_date: String, days: u32) -> Result<Vec<CalendarShow>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let mut uri = API.calendars.shows.uri.clone();
    uri = uri.replace("start_date", &start_date).replace("days", &days.to_string());
    
    let result = client.req_api(&app, API.calendars.shows.method.as_str(), uri, None, None, Some(100), Some(1), true).await;
    
    if let Ok(result) = result {
        let shows = serde_json::from_value::<Vec<CalendarShow>>(result).unwrap_or_default();
        Ok(shows)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn get_calendar_new_shows(app: AppHandle, start_date: String, days: u32) -> Result<Vec<CalendarShow>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let mut uri = API.calendars.new_shows.uri.clone();
    uri = uri.replace("start_date", &start_date).replace("days", &days.to_string());
    
    let result = client.req_api(&app, API.calendars.new_shows.method.as_str(), uri, None, None, Some(100), Some(1), true).await;
    
    if let Ok(result) = result {
        let shows = serde_json::from_value::<Vec<CalendarShow>>(result).unwrap_or_default();
        Ok(shows)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn get_calendar_premieres(app: AppHandle, start_date: String, days: u32) -> Result<Vec<CalendarShow>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let mut uri = API.calendars.season_premieres.uri.clone();
    uri = uri.replace("start_date", &start_date).replace("days", &days.to_string());
    
    let result = client.req_api(&app, API.calendars.season_premieres.method.as_str(), uri, None, None, Some(100), Some(1), true).await;
    
    if let Ok(result) = result {
        let shows = serde_json::from_value::<Vec<CalendarShow>>(result).unwrap_or_default();
        Ok(shows)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn get_my_calendar_shows(app: AppHandle, start_date: String, days: u32) -> Result<Vec<CalendarShow>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let mut uri = API.calendars.my_shows.uri.clone();
    uri = uri.replace("start_date", &start_date).replace("days", &days.to_string());
    
    let result = client.req_api(&app, API.calendars.my_shows.method.as_str(), uri, None, None, Some(100), Some(1), true).await;
    
    if let Ok(result) = result {
        let shows = serde_json::from_value::<Vec<CalendarShow>>(result).unwrap_or_default();
        Ok(shows)
    } else {
        Err(result.unwrap_err())
    }
}
