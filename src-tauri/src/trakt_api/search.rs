use crate::trakt_api::ApiClient;
use crate::model::movie::Movie;
use crate::model::shows::Show;
use tauri::{command, AppHandle, Manager};
use crate::trakt_api::API;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchResult {
    #[serde(rename = "type")]
    pub result_type: String,
    pub score: f64,
    pub movie: Option<Movie>,
    pub show: Option<Show>,
}

const DEFAULT_SEARCH_LIMIT: u32 = 20;

#[command]
pub async fn search_media(app: AppHandle, query: String) -> Result<Vec<SearchResult>, u16> {
    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;
    
    let mut params = HashMap::new();
    params.insert("query".to_string(), query);
    
    let result = client.req_api(
        &app, 
        API.search.text.method.as_str(), 
        API.search.text.uri.clone(), 
        Some(params), 
        None, 
        Some(DEFAULT_SEARCH_LIMIT),
        None, 
        true
    ).await;
    
    if let Ok(result) = result {
        let search_results = serde_json::from_value::<Vec<SearchResult>>(result).unwrap();
        Ok(search_results)
    } else {
        Err(result.unwrap_err())
    }
}
