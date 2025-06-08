use crate::trakt_api::ApiClient;
use tauri::{command, AppHandle, Manager};
use crate::trakt_api::API;
use crate::model::movie::{MovieTrending, MovieDetails, MovieTranslations};
use tokio::sync::Mutex;



#[command]
pub async fn movie_trending(app: AppHandle) -> Result<Vec<MovieTrending>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client.req_api(&app, API.movie.trending.method.as_str(), API.movie.trending.uri.clone(), None, None, Some(40), Some(1), true).await;
    if let Ok(result) = result {
        let movies = serde_json::from_value::<Vec<MovieTrending>>(result).unwrap();
        Ok(movies)
    } else {
        Err(result.unwrap_err())
    }
}


#[command]
pub async fn movie_trending_page(app: AppHandle, page: u32, limit: u32) -> Result<Vec<MovieTrending>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client.req_api(&app, API.movie.trending.method.as_str(), API.movie.trending.uri.clone(), None, None, Some(limit), Some(page), true).await;
    if let Ok(result) = result {
        let movies = serde_json::from_value::<Vec<MovieTrending>>(result).unwrap();
        Ok(movies)
    } else {
        Err(result.unwrap_err())
    }
}


#[command]
pub async fn movie_details(app: AppHandle, id: u32) -> Result<MovieDetails, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.movie.details.uri.clone();
    uri = uri.replace("id", &id.to_string());
    let params = API.movie.details.params.clone();
    let result = client.req_api(&app, API.movie.details.method.as_str(), uri, params, None, None, None, false).await;
    if let Ok(result) = result {
        let movie = serde_json::from_value::<MovieDetails>(result).unwrap();
        Ok(movie)
    } else {
        Err(result.unwrap_err())
    }
}


#[command]
pub async fn movie_translation(app: AppHandle, id: u32, language: String) -> Result<MovieTranslations, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.movie.trans.uri.clone();
    uri = uri.replace("id", &id.to_string()).replace("language", &language);
    let result = client.req_api(&app, API.movie.trans.method.as_str(), uri, None, None, None, None, false).await;
    if let Ok(result) = result {
        let movie = serde_json::from_value::<MovieTranslations>(result).unwrap();
        Ok(movie)
    } else {
        Err(result.unwrap_err())
    }
}





