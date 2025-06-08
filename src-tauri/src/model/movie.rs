use serde::{Serialize, Deserialize};
use crate::model::assets::Images;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieTrending {
    pub watchers: u32,
    pub movie: Movie,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Movie {
    pub title: String,
    pub year: u32,
    pub ids: MovieIds,
    pub images: Images
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieIds{
    pub trakt: u32,
    pub slug: String,
    pub imdb: String,
    pub tmdb: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieDetails {
    pub title: String,
    pub year: u32,
    pub ids: MovieIds,
    pub tagline: Option<String>,
    pub overview: Option<String>,
    pub released: Option<String>,
    pub runtime: Option<u32>,
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
    pub certification: Option<String>,
    pub original_title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieTranslation {
    pub title: Option<String>,
    pub overview: Option<String>,
    pub tagline: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
}

// 电影翻译信息列表的类型别名
pub type MovieTranslations = Vec<MovieTranslation>;


