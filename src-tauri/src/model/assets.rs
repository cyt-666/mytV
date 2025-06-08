use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Images {
    pub poster: Vec<String>,
    pub fanart: Vec<String>,
    pub clearart: Vec<String>,
    pub logo: Vec<String>,
    pub banner: Vec<String>,
    pub thumb: Vec<String>,
}