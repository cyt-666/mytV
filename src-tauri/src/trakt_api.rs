pub mod auth;
pub mod recommand;
pub mod user;
pub mod movie;
pub mod shows;
pub mod search;
pub mod sync;
pub mod translation_cache; 

use auth::refresh_token;
use tauri::{AppHandle, Manager};
use reqwest::{Client, ClientBuilder, RequestBuilder};
use reqwest::header::{HeaderMap, HeaderValue};
use tauri_plugin_store::StoreExt;
use url::Url;
use std::time::Duration;
use crate::token::Token;
use crate::app_conf::get_config;
use serde_json::Value;
use std::collections::HashMap;
use lazy_static::lazy_static;
use serde::Deserialize;
use tokio::sync::Mutex;
use tokio::runtime::Runtime;

const TRAKT_API_HOST: &str = "https://api.trakt.tv";
const TRAKT_HOST: &str = "https://trakt.tv";
const API_MAP: &str = include_str!("../api.json");

// 使用 lazy_static 创建全局静态配置实例
lazy_static! {
    pub static ref API: Api = Api::new();
}

#[derive(Debug, Deserialize, Clone)]
pub struct Entry {
    pub uri: String,
    pub method: String,
    pub params: Option<HashMap<String, String>>,
    pub body: Option<Value>,
    pub content_type: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Api {
    pub auth: AuthApi,
    pub recommand: RecommandApi,
    pub search: SearchApi,
    pub user: UserApi,
    pub movie: MovieApi,
    pub shows: ShowApi,
    pub sync: SyncApi,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthApi {
    pub authorize: Entry,
    pub get_token: Entry,
    pub refresh_token: Entry,
    pub revoke_token: Entry,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RecommandApi {
    pub movies: Entry,
    pub shows: Entry,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SearchApi {
    pub text: Entry,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserApi {
    pub profile: Entry,
    pub watched: Entry,
    pub stats: Entry,
    pub collection: Entry,
    pub watchlist: Entry,
    pub history: Entry,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MovieApi {
    pub trending: Entry,
    pub details: Entry,
    pub trans: Entry,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ShowApi {
    pub trending: Entry,
    pub details: Entry,
    pub trans: Entry,
    pub seasons: Entry,
    pub season_trans: Entry,
    pub season_episodes: Entry,
    pub episode_details: Entry,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SyncApi {
    pub add_to_collection: Entry,
    pub remove_from_collection: Entry,
    pub add_to_watchlist: Entry,
    pub remove_from_watchlist: Entry,
    pub add_to_history: Entry,
}

impl Api {
    pub fn new() -> Self {
        serde_json::from_str::<Api>(API_MAP).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    authenticated: bool,
    client: Client,
}

impl ApiClient {
    pub fn new(app: &AppHandle) -> Self {
        let token = app.try_state::<Mutex<Token>>();
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("trakt-api-version", HeaderValue::from_static("2"));
        
        let client_id = get_config().client_id.clone();
        debug!("使用 Client ID (前8位): {}...", &client_id[..8.min(client_id.len())]);
        headers.insert("trakt-api-key", HeaderValue::from_str(&client_id).unwrap());
        
        let client = if let Some(token) = token {
            debug!("检测到 Token，使用认证模式");
            let rt = Runtime::new().unwrap();
            let token = rt.block_on(token.lock());
            headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", token.access_token).as_str()).unwrap());
            let client = ClientBuilder::new()
                    .connect_timeout(Duration::from_secs(5))
                    .default_headers(headers)
                    .build()
                    .unwrap();
            ApiClient { client, authenticated: true }
        } else {
            debug!("未检测到 Token，使用未认证模式");
            let client = ClientBuilder::new()
                .connect_timeout(Duration::from_secs(5))
                .default_headers(headers)
                .build()
                .unwrap();
            ApiClient { client, authenticated: false }
        };
        client
    }

    pub async fn req_api(&mut self, app: &AppHandle, method: &str, mut uri: String, params: Option<HashMap<String, String>>, body: Option<Value>, limit: Option<u32>, page: Option<u32>, images: bool) -> Result<Value, u16> {
        let mut url = if uri.starts_with("http") {
            Url::parse(uri.as_str()).unwrap()
        } else {
            if !uri.starts_with("/") {
                uri = format!("/{}", uri);
            }
            Url::parse(format!("{}{}", TRAKT_API_HOST, uri).as_str()).unwrap()
        };
        if images {
            url.query_pairs_mut().append_pair("extended", "images");
        }
        if let Some(limit) = limit {
            url.query_pairs_mut().append_pair("limit", limit.to_string().as_str());
        }
        if let Some(page) = page {
            url.query_pairs_mut().append_pair("page", page.to_string().as_str());
        }
        
        debug!("=== API 请求详情 ===");
        debug!("URL: {}", url.as_str());
        debug!("Method: {}", method);
        debug!("Authenticated: {}", self.authenticated);
        
        let method = method.to_lowercase();
        let mut req: RequestBuilder;
        match method.as_str() {
            "get" => {
                if let Some(params) = params {
                    for (key, value) in params {
                        url.query_pairs_mut().append_pair(key.as_str(), value.as_str());
                    }
                }
                req = self.client.get(url.clone())
                    .header("Content-Type", "application/json")
                    .header("trakt-api-version", "2")
                    .header("trakt-api-key", get_config().client_id.as_str())
                    .header("User-Agent", "MyTV/1.0");
            }
            "post" => {
                req = self.client.post(url.clone())
                    .header("Content-Type", "application/json")
                    .header("trakt-api-version", "2")
                    .header("trakt-api-key", get_config().client_id.as_str())
                    .header("User-Agent", "MyTV/1.0");
            }
            "put" => {
                req = self.client.put(url.clone())
                    .header("Content-Type", "application/json")
                    .header("trakt-api-version", "2")
                    .header("trakt-api-key", get_config().client_id.as_str())
                    .header("User-Agent", "MyTV/1.0");
            }
            "delete" => {
                req = self.client.delete(url.clone())
                    .header("Content-Type", "application/json")
                    .header("trakt-api-version", "2")
                    .header("trakt-api-key", get_config().client_id.as_str())
                    .header("User-Agent", "MyTV/1.0");
            }
            _ => {
                return Err(405);
            }
        }
        
        if self.authenticated {
            debug!("=== 添加认证头 ===");
            if let Some(token_state) = app.try_state::<Mutex<Token>>() {
                let token = token_state.lock().await;
                debug!("Token access_token (前20字符): {}...", &token.access_token[..20.min(token.access_token.len())]);
                req = req.header("Authorization", format!("Bearer {}", token.access_token));
            } else {
                warn!("警告: authenticated=true 但找不到 Token state!");
            }
        } else {
            debug!("=== 未认证模式，不添加 Authorization 头 ===");
        }
        if let Some(body) = body {
            let body = serde_json::to_string(&body).unwrap();
            debug!("请求体: {:?}", &body);
            req = req.body(body);
        }
        let req = req.build().unwrap();
        
        debug!("=== 请求 Headers ===");
        for (name, value) in req.headers() {
            debug!("{}: {:?}", name, value);
        }
        
        let resp = self.client.execute(req).await;
        match resp {
            Ok(resp) => {
                info!("请求URL {:?}，响应状态码 {:?}", url.as_str(), resp.status());
                if resp.status().is_success() {
                    let body = resp.text().await;
                    match body {
                        Ok(body) => {
                            Ok(serde_json::from_str(&body).unwrap())
                        }
                        Err(e) => {
                            error!("解析响应体失败: {:?}", e);
                            Err(500)
                        }
                    }
                } else {
                    let status = resp.status().as_u16();
                    if status == 401 {
                        warn!("收到 401 响应，尝试刷新 token");
                        let result = refresh_token(app).await;
                        if let Ok(token) = result {
                            info!("刷新token成功: {:?}", &token);
                            self.refresh_client(Some(token));
                            // Token 刷新成功，但不自动重试请求
                            // 让调用者处理重试逻辑
                        } else {
                            error!("刷新token失败: {:?}", &result);
                            // 清除存储中的无效 token
                            let store = app.store("app_data.json").unwrap();
                            let _ = store.delete("token");
                            
                            // 清除应用状态中的 token
                            if let Some(token_state) = app.try_state::<Mutex<Token>>() {
                                let mut t = token_state.lock().await;
                                *t = Token {
                                    access_token: String::new(),
                                    token_type: String::new(),
                                    expires_in: 0,
                                    refresh_token: String::new(),
                                    scope: String::new(),
                                    created_at: 0,
                                };
                            }
                        }
                    }
                    Err(status)
                }
            }
            Err(e) => {
                error!("请求失败: {:?}", e);
                Err(500)
            }
        }
    }

    pub fn refresh_client(&mut self, token: Option<Token>) {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("trakt-api-version", HeaderValue::from_static("2"));
        headers.insert("trakt-api-key", HeaderValue::from_str(&get_config().client_id).unwrap());
        
        if let Some(token) = token {
            headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", token.access_token).as_str()).unwrap());
            self.authenticated = true;
        } else {
            self.authenticated = false;
        }
        let client = ClientBuilder::new()
                    .connect_timeout(Duration::from_secs(5))
                    .default_headers(headers)
                    .build()
                    .unwrap();
        self.client = client;
    }
}
