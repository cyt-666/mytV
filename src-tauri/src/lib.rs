mod app_conf;
mod token;
mod trakt_api;
mod model;
mod settings;
use tauri::Manager;
use tauri_plugin_store::StoreExt;
use token::Token;
use tokio::sync::Mutex;
use trakt_api::ApiClient;


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
        .plugin(tauri_plugin_log::Builder::new()
            .level(log_level)
            .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
            .build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_oauth::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
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
            trakt_api::movie::movie_details,
            trakt_api::movie::movie_translation,
            trakt_api::shows::show_trending,
            trakt_api::shows::show_trending_page,
            trakt_api::shows::show_popular_page,
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
        .setup(|app|{
            let store = app.store("app_data.json");
            if let Ok(store) = store {
                if store.has("token") {
                    let token = store.get("token").unwrap();
                    let token = serde_json::from_value::<Token>(token);
                    match token {
                        Ok(token) => {
                            app.manage(Mutex::new(token));
                        }
                        _ => {}
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
