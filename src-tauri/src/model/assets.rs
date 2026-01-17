use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Images {
    #[serde(default)]
    pub poster: Vec<String>,
    #[serde(default)]
    pub fanart: Vec<String>,
    #[serde(default)]
    pub clearart: Vec<String>,
    #[serde(default)]
    pub logo: Vec<String>,
    #[serde(default)]
    pub banner: Vec<String>,
    #[serde(default)]
    pub thumb: Vec<String>,
    #[serde(default)]
    pub screenshot: Vec<String>,
}