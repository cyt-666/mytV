mod db;
mod app_conf;
mod model;
mod settings;
mod token;
mod trakt_api;
use tauri::Manager;
use tauri_plugin_store::StoreExt;
use token::Token;
use tokio::sync::Mutex;
use trakt_api::ApiClient;
use db::cache;
use std::fs;
use std::path::Path;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_level = match app_conf::get_config().log_level.as_str() {
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    };

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log_level)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build(),
        )
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_oauth::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            trakt_api::auth::start_trakt_user_auth,
            trakt_api::auth::get_token,
            trakt_api::auth::check_login_status,
            trakt_api::auth::revoke_token,
            trakt_api::recommand::movies_recommand,
            trakt_api::recommand::shows_recommand,
            trakt_api::recommand::movies_recommand_page,
            trakt_api::recommand::shows_recommand_page,
            trakt_api::search::search_media,
            trakt_api::user::get_user_profile,
            trakt_api::user::get_user_stats,
            trakt_api::user::get_watched,
            trakt_api::user::get_collection,
            trakt_api::user::get_watchlist,
            trakt_api::user::get_history,
            trakt_api::sync::add_to_collection,
            trakt_api::sync::remove_from_collection,
            trakt_api::sync::add_to_watchlist,
            trakt_api::sync::remove_from_watchlist,
            trakt_api::sync::mark_as_watched,
            trakt_api::movie::movie_trending,
            trakt_api::movie::movie_trending_page,
            trakt_api::movie::movie_popular_page,
            trakt_api::movie::movie_anticipated,
            trakt_api::movie::movie_details,
            trakt_api::movie::movie_translation,
            trakt_api::shows::show_trending,
            trakt_api::shows::show_trending_page,
            trakt_api::shows::show_popular_page,
            trakt_api::shows::show_anticipated,
            trakt_api::shows::show_details,
            trakt_api::shows::show_translation,
            trakt_api::shows::show_seasons,
            trakt_api::shows::season_trans,
            trakt_api::shows::get_season_episodes,
            trakt_api::shows::get_episode_details,
            trakt_api::translation_cache::get_movie_translation_cached,
            trakt_api::translation_cache::get_show_translation_cached,
            trakt_api::translation_cache::get_season_translation_cached,
            trakt_api::translation_cache::clear_expired_translations,
            trakt_api::translation_cache::get_translation_cache_stats,
            trakt_api::calendars::get_calendar_movies,
            trakt_api::calendars::get_calendar_shows,
            trakt_api::calendars::get_calendar_new_shows,
            trakt_api::calendars::get_calendar_premieres,
            trakt_api::calendars::get_my_calendar_shows,
            trakt_api::progress::get_show_progress,
            trakt_api::progress::get_up_next,
            trakt_api::utils::get_proxied_image,
            settings::get_app_config,
            settings::update_log_level
        ])
        .setup(|app| {
            // 初始化后端使用的 DB pool
            let pool = tauri::async_runtime::block_on(async {
                match db::init_db_pool(&app.handle()).await {
                    Ok(pool) => {
                        app.manage(db::DbPool(pool.clone()));
                        log::info!("Backend DB pool initialized");
                        Some(pool)
                    }
                    Err(e) => {
                        log::error!("Failed to init backend DB pool: {}", e);
                        None
                    }
                }
            });
            
            // 尝试从 DB 恢复 Token
            let mut token_recovered = false;
            if let Some(ref pool) = pool {
                let token_res = tauri::async_runtime::block_on(async {
                    cache::get_config(pool, "token").await
                });
                
                if let Some(token_val) = token_res {
                     if let Ok(token) = serde_json::from_value::<Token>(token_val) {
                         app.manage(Mutex::new(token));
                         token_recovered = true;
                         log::info!("Token recovered from DB");
                     }
                }
            }

            // 降级：如果 DB 中没有，尝试从旧的 app_data.json 文件恢复 (平滑迁移)
            if !token_recovered {
                if let Ok(app_dir) = app.path().app_data_dir() {
                    let old_data_path = app_dir.join("app_data.json");
                    if old_data_path.exists() {
                        log::info!("Found old app_data.json at {:?}", old_data_path);
                        if let Ok(content) = fs::read_to_string(old_data_path) {
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                                if let Some(token_val) = json.get("token") {
                                    if let Ok(token) = serde_json::from_value::<Token>(token_val.clone()) {
                                        app.manage(Mutex::new(token));
                                        log::info!("Token successfully migrated from app_data.json");
                                        
                                        // 立即迁移到 DB
                                        if let Some(ref pool) = pool {
                                            let _ = tauri::async_runtime::block_on(async {
                                                let _ = cache::set_config(pool, "token", token_val).await;
                                                log::info!("Token saved to DB");
                                            });
                                        }
                                    } else {
                                        log::error!("Failed to parse token from app_data.json");
                                    }
                                }
                            }
                        }
                    } else {
                        log::info!("Old app_data.json not found, new user");
                    }
                }
            }
            
            let client = ApiClient::new(&app.handle());
            app.manage(Mutex::new(client));

            #[cfg(target_os = "windows")]
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_decorations(false);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
