use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConf {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub oauth_port: u16,
}

impl AppConf {
    fn new() -> Self {
        let client_id = option_env!("TRAKT_CLIENT_ID");
        let client_secret = option_env!("TRAKT_CLIENT_SECRET");
        let redirect_uri = option_env!("TRAKT_REDIRECT_URI");
        let oauth_port = option_env!("TRAKT_OAUTH_PORT");

        if let (Some(id), Some(secret), Some(uri)) = (client_id, client_secret, redirect_uri) {
            return AppConf {
                client_id: id.to_string(),
                client_secret: secret.to_string(),
                redirect_uri: uri.to_string(),
                oauth_port: oauth_port.and_then(|p| p.parse().ok()).unwrap_or(4396),
            };
        }

        Self::load_from_file()
    }

    fn load_from_file() -> Self {
        let config_path = Path::new("app.conf.json");

        if !config_path.exists() {
            panic!("配置文件 app.conf.json 不存在");
        }

        let content = fs::read_to_string(config_path).expect("无法读取配置文件");

        serde_json::from_str::<AppConf>(&content).expect("无法解析配置文件")
    }
}

// 使用 lazy_static 创建全局静态配置实例
lazy_static! {
    pub static ref APP_CONFIG: AppConf = AppConf::new();
}

// 提供一个便捷的获取配置的函数
pub fn get_config() -> &'static AppConf {
    &APP_CONFIG
}
