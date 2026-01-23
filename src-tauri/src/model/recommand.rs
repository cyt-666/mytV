use crate::model::assets::Images;
use crate::model::movie::MovieIds;
use crate::model::shows::ShowIds;
use crate::model::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieRecommand {
    pub title: String,
    pub year: u32,
    pub ids: MovieIds,
    pub images: Images,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowRecommand {
    pub title: String,
    pub year: u32,
    pub favorited_by: Vec<User>,
    pub ids: ShowIds,
    pub images: Images,
}
