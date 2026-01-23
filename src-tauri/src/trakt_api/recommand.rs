use crate::model::recommand::{MovieRecommand, ShowRecommand};
use crate::trakt_api::ApiClient;
use crate::trakt_api::API;
use tauri::{command, AppHandle, Manager};
use tokio::sync::Mutex;
use crate::db::{DbPool, cache};
use log::{info, error};

#[command]
pub async fn movies_recommand(app: AppHandle) -> Result<Vec<MovieRecommand>, u16> {
    let cache_key = "api_movies_recommand";

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<MovieRecommand>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;

    if !client.authenticated {
        println!("movies_recommand: 用户未登录，返回 401");
        return Err(401);
    }

    let params = API.recommand.movies.params.clone().unwrap();
    let result = client
        .req_api(
            &app,
            API.recommand.movies.method.as_str(),
            API.recommand.movies.uri.clone(),
            Some(params),
            None,
            Some(100),
            None,
            true,
        )
        .await;

    match result {
        Ok(value) => match serde_json::from_value::<Vec<MovieRecommand>>(value) {
            Ok(movies) => {
                if let Some(pool) = app.try_state::<DbPool>() {
                    // 对于推荐列表，由于默认直接转成Value了，这里需要将Vec重新转回去存
                    let val = serde_json::to_value(&movies).unwrap();
                    cache::set_api_response_cache(&pool.0, cache_key, &val).await;
                }
                Ok(movies)
            },
            Err(e) => {
                println!("movies_recommand: 解析响应失败: {:?}", e);
                Err(500)
            }
        },
        Err(status) => Err(status),
    }
}

#[command]
pub async fn shows_recommand(app: AppHandle) -> Result<Vec<ShowRecommand>, u16> {
    let cache_key = "api_shows_recommand";

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<ShowRecommand>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;

    if !client.authenticated {
        println!("shows_recommand: 用户未登录，返回 401");
        return Err(401);
    }

    let params = API.recommand.shows.params.clone().unwrap();
    let result = client
        .req_api(
            &app,
            API.recommand.shows.method.as_str(),
            API.recommand.shows.uri.clone(),
            Some(params),
            None,
            Some(100),
            None,
            true,
        )
        .await;

    match result {
        Ok(value) => match serde_json::from_value::<Vec<ShowRecommand>>(value) {
            Ok(shows) => {
                if let Some(pool) = app.try_state::<DbPool>() {
                    let val = serde_json::to_value(&shows).unwrap();
                    cache::set_api_response_cache(&pool.0, cache_key, &val).await;
                }
                Ok(shows)
            },
            Err(e) => {
                println!("shows_recommand: 解析响应失败: {:?}", e);
                Err(500)
            }
        },
        Err(status) => Err(status),
    }
}

#[command]
pub async fn movies_recommand_page(
    app: AppHandle,
    page: u32,
    limit: u32,
) -> Result<Vec<MovieRecommand>, u16> {
    let cache_key = format!("api_movies_recommand_p{}_l{}", page, limit);

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, &cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<MovieRecommand>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;

    if !client.authenticated {
        println!("movies_recommand_page: 用户未登录，返回 401");
        return Err(401);
    }

    let params = API.recommand.movies.params.clone().unwrap();
    let result = client
        .req_api(
            &app,
            API.recommand.movies.method.as_str(),
            API.recommand.movies.uri.clone(),
            Some(params),
            None,
            Some(limit),
            Some(page),
            true,
        )
        .await;

    match result {
        Ok(value) => match serde_json::from_value::<Vec<MovieRecommand>>(value) {
            Ok(movies) => {
                if let Some(pool) = app.try_state::<DbPool>() {
                    let val = serde_json::to_value(&movies).unwrap();
                    cache::set_api_response_cache(&pool.0, &cache_key, &val).await;
                }
                Ok(movies)
            },
            Err(e) => {
                println!("movies_recommand_page: 解析响应失败: {:?}", e);
                Err(500)
            }
        },
        Err(status) => Err(status),
    }
}

#[command]
pub async fn shows_recommand_page(
    app: AppHandle,
    page: u32,
    limit: u32,
) -> Result<Vec<ShowRecommand>, u16> {
    let cache_key = format!("api_shows_recommand_p{}_l{}", page, limit);

    if let Some(pool) = app.try_state::<DbPool>() {
        if let Some(json) = cache::get_api_response_cache(&pool.0, &cache_key).await {
            if let Ok(data) = serde_json::from_value::<Vec<ShowRecommand>>(json) {
                return Ok(data);
            }
        }
    }

    let client = app.state::<Mutex<ApiClient>>();
    let mut client = client.lock().await;

    if !client.authenticated {
        println!("shows_recommand_page: 用户未登录，返回 401");
        return Err(401);
    }

    let params = API.recommand.shows.params.clone().unwrap();
    let result = client
        .req_api(
            &app,
            API.recommand.shows.method.as_str(),
            API.recommand.shows.uri.clone(),
            Some(params),
            None,
            Some(limit),
            Some(page),
            true,
        )
        .await;

    match result {
        Ok(value) => match serde_json::from_value::<Vec<ShowRecommand>>(value) {
            Ok(shows) => {
                if let Some(pool) = app.try_state::<DbPool>() {
                    let val = serde_json::to_value(&shows).unwrap();
                    cache::set_api_response_cache(&pool.0, &cache_key, &val).await;
                }
                Ok(shows)
            },
            Err(e) => {
                println!("shows_recommand_page: 解析响应失败: {:?}", e);
                Err(500)
            }
        },
        Err(status) => Err(status),
    }
}
