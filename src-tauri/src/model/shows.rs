use crate::model::assets::Images;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowTrending {
    pub watchers: u32,
    pub show: Show,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowAnticipated {
    pub list_count: u32,
    pub show: Show,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Show {
    pub title: String,
    pub year: Option<u32>,
    pub ids: ShowIds,
    #[serde(default)]
    pub images: Images,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowIds {
    pub trakt: u32,
    pub slug: String,
    pub tvdb: Option<u32>,
    pub imdb: Option<String>,
    pub tmdb: Option<u32>,
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

// Episode 结构体 (用于 history API)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Episode {
    pub season: u32,
    pub number: u32,
    pub title: Option<String>,
    pub ids: EpisodeIds,
    pub runtime: Option<u32>,
    pub overview: Option<String>,
    pub rating: Option<f32>,
    pub votes: Option<u32>,
    pub first_aired: Option<String>,
    #[serde(default)]
    pub images: Option<crate::model::assets::Images>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EpisodeIds {
    pub trakt: u32,
    pub tvdb: Option<u32>,
    pub imdb: Option<String>,
    pub tmdb: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowProgress {
    pub aired: u32,
    pub completed: u32,
    pub last_watched_at: Option<String>,
    pub reset_at: Option<String>,
    pub next_episode: Option<Episode>,
    pub last_episode: Option<Episode>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpNextItem {
    pub show: Show,
    pub next_episode: Episode,
    pub progress: ShowProgressSummary,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowProgressSummary {
    pub aired: u32,
    pub completed: u32,
    pub last_watched_at: Option<String>,
}
