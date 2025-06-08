use serde::{Serialize, Deserialize};
use crate::model::assets::Images;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowTrending {
    pub watchers: u32,
    pub show: Show,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Show {
    pub title: String,
    pub year: u32,
    pub ids: ShowIds,
    pub images: Images
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowIds {
    pub trakt: u32,
    pub slug: String,
    pub tvdb: u32,
    pub imdb: String,
    pub tmdb: u32,
    pub tvrage: Option<Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowDetails {
    pub title: String,
    pub year: u32,
    pub ids: ShowIds,
    pub tagline: Option<String>,
    pub overview: Option<String>,
    pub first_aired: Option<String>,
    pub airs: Option<ShowAirs>,
    pub runtime: Option<u32>,
    pub certification: Option<String>,
    pub network: Option<String>,
    pub country: Option<String>,
    pub updated_at: Option<String>,
    pub trailer: Option<String>,
    pub homepage: Option<String>,
    pub status: Option<String>,
    pub rating: Option<f32>,
    pub votes: Option<u32>,
    pub comment_count: Option<u32>,
    pub languages: Option<Vec<String>>,
    pub available_translations: Option<Vec<String>>,
    pub genres: Option<Vec<String>>,
    pub aired_episodes: Option<u32>,
    pub original_title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowAirs {
    pub day: Option<String>,
    pub time: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowTranslation {
    pub title: Option<String>,
    pub overview: Option<String>,
    pub tagline: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
}

// 电视剧翻译信息列表的类型别名
pub type ShowTranslations = Vec<ShowTranslation>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Season {
    pub number: u32,
    pub ids: SeasonIds,
    pub rating: Option<f32>,
    pub votes: Option<u32>,
    pub episode_count: Option<u32>,
    pub aired_episodes: Option<u32>,
    pub title: Option<String>,
    pub overview: Option<String>,
    pub first_aired: Option<String>,
    pub updated_at: Option<String>,
    pub network: Option<String>,
    pub original_title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeasonIds {
    pub trakt: u32,
    pub tvdb: Option<u32>,
    pub tmdb: Option<u32>,
}

// 季度列表的类型别名
pub type Seasons = Vec<Season>;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeasonTranslation {
    pub title: Option<String>,
    pub overview: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
}

pub type SeasonTranslations = Vec<SeasonTranslation>;
