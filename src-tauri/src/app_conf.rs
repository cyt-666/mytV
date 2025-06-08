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
        if env::var("TAURI_ENV_DEBUG").unwrap_or_else(|_| "true".to_string()) == "true" {
            // 开发环境：从配置文件读取
            Self::load_from_file()
        } else {
            // 生产环境：不允许运行，必须提供外部配置
            panic!("生产环境必须通过外部方式提供配置，不能从文件读取敏感信息")
        }
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
