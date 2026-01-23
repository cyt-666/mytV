use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranslationData {
    pub title: Option<String>,
    pub overview: Option<String>,
    pub tagline: Option<String>,
    pub updated_at: u64, // 时间戳
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranslationCacheItem {
    pub data: Option<TranslationData>,
    pub updated_at: u64,
    pub expires_at: u64,
}

impl TranslationCacheItem {
    pub fn new(data: Option<TranslationData>, cache_duration_ms: u64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Self {
            data,
            updated_at: now,
            expires_at: now + cache_duration_ms,
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        now > self.expires_at
    }
}
