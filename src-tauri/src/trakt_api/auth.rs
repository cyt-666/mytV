use reqwest;
use tokio::sync::Mutex;

use serde_json::{json, Value};
use tauri::{command, Manager};
use tauri::{AppHandle, Emitter};
use tauri_plugin_oauth::{cancel, start_with_config, OauthConfig};
use tauri_plugin_opener::open_url;

use crate::app_conf::get_config;
use crate::token::Token;
use tauri_plugin_store::StoreExt;
use url::Url;

use log::{info, error};

use super::{ApiClient, API, TRAKT_API_HOST, TRAKT_HOST};

#[command]
pub async fn start_trakt_user_auth(app: AppHandle) -> Result<(), u16> {
    info!("Starting Trakt User Auth");
    let token_state = app.try_state::<Mutex<Token>>();
    if let Some(token_state) = token_state {
        let token = token_state.lock().await;
        if !token.access_token.is_empty() {
            info!("Token exists and is valid: {:?}", token);
            return Err(200);
        }
    }
    let mut url = Url::parse(format!("{}{}", TRAKT_HOST, API.auth.authorize.uri).as_str()).unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", &get_config().client_id);
    
    let conf_redirect = &get_config().redirect_uri;
    let final_redirect_uri = if conf_redirect.starts_with("http") {
        conf_redirect.clone()
    } else {
        format!(
            "http://localhost:{}{}",
            get_config().oauth_port,
            conf_redirect
        )
    };
    
    info!("Redirect URI: {}", final_redirect_uri);
    
    url.query_pairs_mut().append_pair("redirect_uri", &final_redirect_uri);
    
    url.query_pairs_mut().append_pair("response_type", "code");
    let url = url.to_string();
    info!("Opening URL: {}", url);
    let r = open_url(&url, None::<&str>);
    if r.is_err() {
        error!("Failed to open URL");
        return Err(400);
    }
    handle_oauth_callback("oauth/callback".to_string(), 4396, app).await
}

#[command]
pub async fn get_token(app: AppHandle, code: &str) -> Result<Token, String> {
    info!("Getting Token with code: {}", code);
    
    // 检查是否已有 Token
    if token_exists(&app) {
        let token_state = app.try_state::<Mutex<Token>>().unwrap();
        let token = token_state.lock().await;
        info!("Token already exists: {:?}", token);
        return Ok(token.clone());
    }

    let get_token_api = API.auth.get_token.clone();
    let url = format!("{}{}", TRAKT_API_HOST, get_token_api.uri);
    
    let mut request_body = get_token_api.body.unwrap();

    let conf = get_config();
    request_body["client_id"] = conf.client_id.clone().into();
    request_body["client_secret"] = conf.client_secret.clone().into();
    
    let redirect_uri_str = if conf.redirect_uri.starts_with("http") {
        conf.redirect_uri.clone()
    } else {
        format!("http://localhost:{}{}", conf.oauth_port, conf.redirect_uri)
    };
    
    request_body["redirect_uri"] = redirect_uri_str.clone().into();
    request_body["code"] = code.to_string().into();

    info!("Token Request URI: {}, Redirect URI: {}", url, redirect_uri_str);

    let client = if let Some(client) = app.try_state::<Mutex<ApiClient>>() {
        client
    } else {
        let client = ApiClient::new(&app);
        app.manage(Mutex::new(client));
        app.state::<Mutex<ApiClient>>()
    };

    let resp_body: Result<Value, u16> = {
        let mut client = client.lock().await;
        client.req_api(&app, "POST", url, None, Some(request_body), None, None, false).await
    }; 
    
    if let Ok(resp_body) = resp_body {
        info!("Token Response OK");
        let new_token = serde_json::from_value::<Token>(resp_body);
        match new_token {
            Ok(new_token) => {
                let store = app.store("app_data.json").unwrap();
                store.set("token", serde_json::to_value(&new_token).unwrap());
                if let Some(token) = app.try_state::<Mutex<Token>>() {
                    {
                        let mut token = token.lock().await;
                        token.access_token = new_token.access_token.clone();
                        token.refresh_token = new_token.refresh_token.clone();
                        token.created_at = new_token.created_at;
                        token.token_type = new_token.token_type.clone();
                        token.expires_in = new_token.expires_in;
                        token.scope = new_token.scope.clone();
                    }
                    {
                        let mut client = client.lock().await;
                        client.refresh_client(Some(new_token.clone()));
                    }
                } else {
                    app.manage(Mutex::new(new_token.clone()));
                }
                info!("Token saved successfully");
                return Ok(new_token);
            }
            Err(e) => {
                error!("Failed to parse token: {:?}", e);
                return Err(e.to_string());
            }
        }
    } else {
        let err_code = resp_body.err().unwrap();
        error!("Failed to get token, HTTP Error Code: {:?}", err_code);
    }
    Err("获取token失败".to_string())
}

pub async fn refresh_token(app: &AppHandle) -> Result<Token, u16> {
    let token = app.try_state::<Mutex<Token>>();
    if token.is_none() {
        return Err(401);
    }
    let token = token.unwrap();
    let mut token = token.lock().await;
    
    let conf = get_config();
    let redirect_uri_str = if conf.redirect_uri.starts_with("http") {
        conf.redirect_uri.clone()
    } else {
        format!("http://localhost:{}{}", conf.oauth_port, conf.redirect_uri)
    };

    let body = json!({
        "refresh_token": token.refresh_token.clone(),
        "client_id": conf.client_id.clone(),
        "client_secret": conf.client_secret.clone(),
        "redirect_uri": redirect_uri_str,
        "grant_type": "refresh_token".to_string(),
    });
    let result = reqwest::Client::new()
        .post(format!("{}{}", TRAKT_API_HOST, API.auth.refresh_token.uri))
        .header("Content-Type", "application/json")
        .header("User-Agent", "MyTV/1.0")
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await;
    match result {
        Ok(result) => {
            if !result.status().is_success() {
                return Err(result.status().as_u16());
            }
            let result = result.text().await.unwrap();
            let new_token = serde_json::from_str::<Token>(&result).unwrap();
            token.access_token = new_token.access_token.clone();
            token.refresh_token = new_token.refresh_token.clone();
            token.created_at = new_token.created_at;
            token.token_type = new_token.token_type.clone();
            token.expires_in = new_token.expires_in;
            token.scope = new_token.scope.clone();
            let store = app.store("app_data.json").unwrap();
            let _ = store.delete("token");
            let _ = store.set("token", serde_json::to_value(&new_token).unwrap());
            Ok(new_token)
        }
        Err(_) => {
            Err(500)
        }
    }
}

fn token_exists(app: &AppHandle) -> bool {
    let store = app.store("app_data.json").unwrap();
    let token = store.get("token");
    if token.is_some() {
        return true;
    }
    false
}

#[command]
pub async fn check_login_status(app: AppHandle) -> bool {
    let exists = token_exists(&app);
    
    if !exists {
        return false;
    }
    
    // 检查 token 是否过期
    let token_state = app.try_state::<Mutex<Token>>();
    if let Some(token_state) = token_state {
        let token = token_state.lock().await;
        if token.is_expired() {
            println!("Token 已过期，尝试刷新");
            drop(token); // 释放锁
            
            let refresh_result = refresh_token(&app).await;
            if refresh_result.is_ok() {
                println!("Token 刷新成功");
                return true;
            } else {
                println!("Token 刷新失败，需要重新登录");
                // 清除无效 token
                let store = app.store("app_data.json").unwrap();
                let _ = store.delete("token");
                return false;
            }
        }
    }
    
    exists
}

#[command]
pub async fn revoke_token(app: AppHandle) -> Result<(), String> {
    let token_state = app.try_state::<Mutex<Token>>();
    if let Some(token_state) = token_state {
        let token = token_state.lock().await;
        
        // 调用API撤销token
        let mut revoke_body = API.auth.revoke_token.body.clone().unwrap();
        revoke_body["token"] = token.access_token.clone().into();
        revoke_body["client_id"] = get_config().client_id.clone().into();
        revoke_body["client_secret"] = get_config().client_secret.clone().into();
        
        let result = reqwest::Client::new()
            .post(format!("{}{}", TRAKT_API_HOST, API.auth.revoke_token.uri))
            .header("Content-Type", "application/json")
            .header("User-Agent", "MyTV/1.0")
            .body(serde_json::to_string(&revoke_body).unwrap())
            .send()
            .await;
            
        // 无论API调用是否成功，都清除本地token
        let store = app.store("app_data.json").unwrap();
        let _ = store.delete("token");
        
        // 从应用状态中移除token
        drop(token); // 释放锁
        let _ = app.state::<Mutex<Token>>().try_lock().map(|mut t| {
            *t = Token {
                access_token: String::new(),
                token_type: String::new(),
                expires_in: 0,
                refresh_token: String::new(),
                scope: String::new(),
                created_at: 0,
            };
        });
        
        // 重置 ApiClient 的认证状态
        if let Some(client_state) = app.try_state::<Mutex<ApiClient>>() {
            if let Ok(mut client) = client_state.try_lock() {
                client.refresh_client(None);
            }
        }
        
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Ok(()) // 即使API失败也算成功，因为本地已清除
                }
            }
            Err(_) => Ok(()) // 即使网络失败也算成功，因为本地已清除
        }
    } else {
        Ok(()) // 没有token也算成功
    }
}

const web_page: &str = "<!DOCTYPE html>
<html lang=\"zh\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>认证完成</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            background: linear-gradient(to right, #6a11cb, #2575fc);
            color: white;
            text-align: center;
        }
        .container {
            background: rgba(255, 255, 255, 0.2);
            padding: 30px;
            border-radius: 10px;
            box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
        }
        .btn {
            margin-top: 20px;
            padding: 10px 20px;
            font-size: 16px;
            background: white;
            border: none;
            border-radius: 5px;
            cursor: pointer;
        }
    </style>
</head>
<body>

<div class=\"container\">
    <h1>🎉 认证完成 🎉</h1>
    <p>您的认证已成功完成，可以安全关闭此页面。</p>
</div>


</body>
</html>";

async fn handle_oauth_callback(
    red_url: String,
    port: u16,
    app: AppHandle,
) -> Result<(), u16> {
    let config = OauthConfig {
        ports: Some(vec![port]),
        response: Some(web_page.into()),
    };

    let _ = start_with_config(config, move |url| {
        info!("Received callback URL: {}", url);
        let url_instance = Url::parse(&url);
        if let Ok(url_instance) = url_instance {
            let request_path = url_instance.path();
            let app_conf = get_config();
            
            let is_match = if app_conf.redirect_uri.starts_with("http") {
                if let Ok(conf_url) = Url::parse(&app_conf.redirect_uri) {
                    conf_url.path() == request_path
                } else {
                    false
                }
            } else {
                request_path == app_conf.redirect_uri
            };

            info!("Callback match: {} (Request: {}, Config: {})", is_match, request_path, app_conf.redirect_uri);

            if is_match {
                let code = url_instance.query_pairs().find(|(key, _)| key == "code");
                if let Some((_, code)) = code {
                    app.emit("oauth-callback", &code).unwrap();
                    let _ = cancel(port);
                    return;
                }
            }
        }
        app.emit("oauth-callback", &"").unwrap();
        let _ = cancel(port);
    });
    Ok(())
}
