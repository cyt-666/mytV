use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserProfile {
    pub user: User,
    pub account: Account,
    pub connections: Connections,
    pub sharing_text: SharingText,
    pub limits: Limits,
    pub permissions: Permissions,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    pub private: bool,
    pub name: String,
    pub vip: bool,
    pub vip_ep: bool,
    pub ids: UserIds,
    pub joined_at: String,
    pub location: Option<String>,
    pub about: Option<String>,
    pub gender: Option<String>,
    pub age: Option<u32>,
    pub images: UserImages,
    pub vip_og: bool,
    pub vip_years: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserIds {
    pub slug: String,
    pub uuid: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserImages {
    pub avatar: UserAvatar,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserAvatar {
    pub full: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub timezone: String,
    pub date_format: String,
    pub time_24hr: bool,
    pub cover_image: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Connections {
    pub facebook: bool,
    pub twitter: bool,
    pub mastodon: bool,
    pub google: bool,
    pub tumblr: bool,
    pub medium: bool,
    pub slack: bool,
    pub apple: bool,
    pub dropbox: bool,
    pub microsoft: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SharingText {
    pub watching: String,
    pub watched: String,
    pub rated: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Limits {
    pub list: ListLimit,
    pub watchlist: WatchlistLimit,
    pub favorites: FavoritesLimit,
    pub search: SearchLimit,
    pub collection: CollectionLimit,
    pub notes: NotesLimit,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListLimit {
    pub count: u32,
    pub item_count: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WatchlistLimit {
    pub item_count: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FavoritesLimit {
    pub item_count: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchLimit {
    pub recent_count: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CollectionLimit {
    pub item_count: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NotesLimit {
    pub item_count: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Permissions {
    pub commenting: bool,
    pub liking: bool,
    pub following: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stats {
    pub movies: MovieStats,
    pub shows: ShowStats,
    pub seasons: SeasonStats,
    pub episodes: EpisodeStats,
    pub network: NetworkStats,
    pub ratings: RatingStats,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MovieStats {
    pub plays: u32,
    pub watched: u32,
    pub minutes: u32,
    pub collected: u32,
    pub ratings: u32,
    pub comments: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShowStats {
    pub watched: u32,
    pub collected: u32,
    pub ratings: u32,
    pub comments: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SeasonStats {
    pub ratings: u32,
    pub comments: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EpisodeStats {
    pub plays: u32,
    pub watched: u32,
    pub minutes: u32,
    pub collected: u32,
    pub ratings: u32,
    pub comments: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NetworkStats {
    pub friends: u32,
    pub followers: u32,
    pub following: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RatingStats {
    pub total: u32,
    pub distribution: RatingDistribution,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RatingDistribution {
    #[serde(rename = "1")]
    pub one: u32,
    #[serde(rename = "2")]
    pub two: u32,
    #[serde(rename = "3")]
    pub three: u32,
    #[serde(rename = "4")]
    pub four: u32,
    #[serde(rename = "5")]
    pub five: u32,
    #[serde(rename = "6")]
    pub six: u32,
    #[serde(rename = "7")]
    pub seven: u32,
    #[serde(rename = "8")]
    pub eight: u32,
    #[serde(rename = "9")]
    pub nine: u32,
    #[serde(rename = "10")]
    pub ten: u32,
} 