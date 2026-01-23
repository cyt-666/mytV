pub const FULL_SCHEMA: &str = "
-- 媒体详情缓存
CREATE TABLE IF NOT EXISTS media_cache (
    id TEXT PRIMARY KEY,
    media_type TEXT NOT NULL,
    trakt_id INTEGER NOT NULL,
    data TEXT NOT NULL,
    updated_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
);

-- 列表请求缓存
CREATE TABLE IF NOT EXISTS api_response_cache (
    key TEXT PRIMARY KEY,
    data TEXT NOT NULL,
    updated_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
);

-- 用户数据缓存
CREATE TABLE IF NOT EXISTS user_data_cache (
    key TEXT PRIMARY KEY,
    data TEXT NOT NULL,
    updated_at INTEGER NOT NULL,
    is_dirty BOOLEAN DEFAULT 0
);

-- 翻译缓存
CREATE TABLE IF NOT EXISTS translation_cache (
    id TEXT PRIMARY KEY,
    title TEXT,
    overview TEXT,
    tagline TEXT,
    updated_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
);

-- 应用配置表
CREATE TABLE IF NOT EXISTS app_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_media_expires ON media_cache(expires_at);
CREATE INDEX IF NOT EXISTS idx_api_expires ON api_response_cache(expires_at);
";
