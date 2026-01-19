use crate::trakt_api::ApiClient;
use tauri::{command, AppHandle, Manager};
use crate::trakt_api::API;
use crate::model::shows::{ShowTrending, Show, ShowDetails, ShowTranslations, Seasons, SeasonTranslations, Episode};
use tokio::sync::Mutex;

#[command]
pub async fn get_season_episodes(app: AppHandle, id: u32, season: u32) -> Result<Vec<Episode>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.shows.season_episodes.uri.clone();
    uri = uri.replace("id", &id.to_string()).replace("season_number", &season.to_string());
    
    let mut params = API.shows.season_episodes.params.clone().unwrap_or_default();
    params.insert("extended".to_string(), "full,images".to_string());
    
    let result = client.req_api(&app, API.shows.season_episodes.method.as_str(), uri, Some(params), None, None, None, false).await;
    if let Ok(result) = result {
        let episodes = serde_json::from_value::<Vec<Episode>>(result).unwrap();
        Ok(episodes)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn get_episode_details(app: AppHandle, id: u32, season: u32, episode: u32) -> Result<Episode, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.shows.episode_details.uri.clone();
    uri = uri.replace("id", &id.to_string())
             .replace("season_number", &season.to_string())
             .replace("episode_number", &episode.to_string());
             
    let mut params = API.shows.episode_details.params.clone().unwrap_or_default();
    params.insert("extended".to_string(), "full,images".to_string());
    
    let result = client.req_api(&app, API.shows.episode_details.method.as_str(), uri, Some(params), None, None, None, false).await;
    if let Ok(result) = result {
        let episode_data = serde_json::from_value::<Episode>(result).unwrap();
        Ok(episode_data)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn show_trending(app: AppHandle) -> Result<Vec<ShowTrending>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client.req_api(&app, API.shows.trending.method.as_str(), API.shows.trending.uri.clone(), None, None, Some(40), Some(1), true).await;
    if let Ok(result) = result {
        let shows = serde_json::from_value::<Vec<ShowTrending>>(result).unwrap();
        Ok(shows)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn show_trending_page(app: AppHandle, page: u32, limit: u32) -> Result<Vec<ShowTrending>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client.req_api(&app, API.shows.trending.method.as_str(), API.shows.trending.uri.clone(), None, None, Some(limit), Some(page), true).await;
    if let Ok(result) = result {
        let shows = serde_json::from_value::<Vec<ShowTrending>>(result).unwrap();
        Ok(shows)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn show_popular_page(app: AppHandle, page: u32, limit: u32) -> Result<Vec<Show>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let result = client.req_api(&app, API.shows.popular.method.as_str(), API.shows.popular.uri.clone(), None, None, Some(limit), Some(page), true).await;
    if let Ok(result) = result {
        let shows = serde_json::from_value::<Vec<Show>>(result).unwrap();
        Ok(shows)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn show_details(app: AppHandle, id: u32) -> Result<ShowDetails, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.shows.details.uri.clone();
    uri = uri.replace("id", &id.to_string());
    let params = API.shows.details.params.clone();
    let result = client.req_api(&app, API.shows.details.method.as_str(), uri, params, None, None, None, false).await;
    if let Ok(result) = result {
        let show = serde_json::from_value::<ShowDetails>(result).unwrap();
        Ok(show)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn show_translation(app: AppHandle, id: u32, language: String) -> Result<ShowTranslations, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.shows.trans.uri.clone();
    uri = uri.replace("id", &id.to_string()).replace("language", &language);
    let result = client.req_api(&app, API.shows.trans.method.as_str(), uri, None, None, None, None, false).await;
    if let Ok(result) = result {
        let show = serde_json::from_value::<ShowTranslations>(result).unwrap();
        Ok(show)
    } else {
        Err(result.unwrap_err())
    }
}

#[command]
pub async fn show_seasons(app: AppHandle, id: u32) -> Result<Seasons, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.shows.seasons.uri.clone();
    uri = uri.replace("id", &id.to_string());
    let params = API.shows.seasons.params.clone();
    let result = client.req_api(&app, API.shows.seasons.method.as_str(), uri, params, None, None, None, false).await;
    if let Ok(result) = result {
        let seasons = serde_json::from_value::<Seasons>(result).unwrap();
        Ok(seasons)
    } else {
        Err(result.unwrap_err())
    }
}


#[command]
pub async fn season_trans(app: AppHandle, id: u32, season: u32, language: String) -> Result<SeasonTranslations, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    let mut uri = API.shows.season_trans.uri.clone();
    uri = uri.replace("id", &id.to_string()).replace("season_number", &season.to_string()).replace("language", &language);
    let result = client.req_api(&app, API.shows.season_trans.method.as_str(), uri, None, None, None, None, false).await;
    if let Ok(result) = result {
        let season = serde_json::from_value::<SeasonTranslations>(result).unwrap();
        Ok(season)
    } else {
        Err(result.unwrap_err())
    }
}



