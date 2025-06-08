use crate::trakt_api::ApiClient;
use tauri::{command, AppHandle, Manager};
use crate::trakt_api::API;
use crate::model::recommand::{MovieRecommand, ShowRecommand};
use tokio::sync::Mutex;


#[command]
pub async fn movies_recommand(app: AppHandle) -> Result<Vec<MovieRecommand>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let params = API.recommand.movies.params.clone().unwrap();
    // let params = params.iter().map(|(k, v)| (k.clone(), v.as_bool().unwrap().to_string())).collect();
    let result = client.req_api(&app, API.recommand.movies.method.as_str(), API.recommand.movies.uri.clone(), Some(params), None, Some(100), None, true).await;
    if let Ok(result) = result {
        let movies = serde_json::from_value::<Vec<MovieRecommand>>(result).unwrap();
        Ok(movies)
    } else {
        Err(result.unwrap_err())
    }
}


#[command]
pub async fn shows_recommand(app: AppHandle) -> Result<Vec<ShowRecommand>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let params = API.recommand.shows.params.clone().unwrap();
    // let params = params.iter().map(|(k, v)| (k.clone(), v.as_bool().unwrap().to_string())).collect();
    let result = client.req_api(&app, API.recommand.shows.method.as_str(), API.recommand.shows.uri.clone(), Some(params), None, Some(100), None, true).await;
    if let Ok(result) = result {
        let shows = serde_json::from_value::<Vec<ShowRecommand>>(result).unwrap();
        Ok(shows)
    } else {
        Err(result.unwrap_err())
    }
}


#[command]
pub async fn movies_recommand_page(app: AppHandle, page: u32, limit: u32) -> Result<Vec<MovieRecommand>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let params = API.recommand.movies.params.clone().unwrap();
    // let params = params.iter().map(|(k, v)| (k.clone(), v.as_bool().unwrap().to_string())).collect();
    let result = client.req_api(&app, API.recommand.movies.method.as_str(), API.recommand.movies.uri.clone(), Some(params), None, Some(limit), Some(page), true).await;
    if let Ok(result) = result {
        let movies = serde_json::from_value::<Vec<MovieRecommand>>(result).unwrap();
        Ok(movies)
    } else {
        Err(result.unwrap_err())
    }
}


#[command]
pub async fn shows_recommand_page(app: AppHandle, page: u32, limit: u32) -> Result<Vec<ShowRecommand>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let params = API.recommand.shows.params.clone().unwrap();
    // let params = params.iter().map(|(k, v)| (k.clone(), v.as_bool().unwrap().to_string())).collect();
    let result = client.req_api(&app, API.recommand.shows.method.as_str(), API.recommand.shows.uri.clone(), Some(params), None, Some(limit), Some(page), true).await;
    if let Ok(result) = result {
        let shows = serde_json::from_value::<Vec<ShowRecommand>>(result).unwrap();
        Ok(shows)
    } else {
        Err(result.unwrap_err())
    }
}   
