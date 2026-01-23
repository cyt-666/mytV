use crate::model::recommand::{MovieRecommand, ShowRecommand};
use crate::trakt_api::ApiClient;
use crate::trakt_api::API;
use tauri::{command, AppHandle, Manager};
use tokio::sync::Mutex;

#[command]
pub async fn movies_recommand(app: AppHandle) -> Result<Vec<MovieRecommand>, u16> {
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
            Ok(movies) => Ok(movies),
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
            Ok(shows) => Ok(shows),
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
            Ok(movies) => Ok(movies),
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
            Ok(shows) => Ok(shows),
            Err(e) => {
                println!("shows_recommand_page: 解析响应失败: {:?}", e);
                Err(500)
            }
        },
        Err(status) => Err(status),
    }
}
