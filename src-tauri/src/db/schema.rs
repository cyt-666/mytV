pub const SCHEMA_V1: &str = "
-- 媒体详情缓存 (Movie/Show/Season/Episode Details)
CREATE TABLE IF NOT EXISTS media_cache (
    id TEXT PRIMARY KEY,          -- 格式: {type}_{trakt_id} (e.g., 'movie_12345', 'show_67890', 'season_67890_1')
    media_type TEXT NOT NULL,     -- 'movie', 'show', 'season', 'episode'
    trakt_id INTEGER NOT NULL,
    data TEXT NOT NULL,           -- JSON 字符串
    updated_at INTEGER NOT NULL,  -- 时间戳 (ms)
    expires_at INTEGER NOT NULL   -- 过期时间戳 (ms)
);

-- 列表请求缓存 (Trending, Popular, Recommendations)
CREATE TABLE IF NOT EXISTS api_response_cache (
    key TEXT PRIMARY KEY,         -- 请求的唯一标识 (e.g., 'trending_movies_page1')
    data TEXT NOT NULL,           -- JSON 字符串
    updated_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
);

-- 用户数据缓存 (Watchlist, History, Collection)
CREATE TABLE IF NOT EXISTS user_data_cache (
    key TEXT PRIMARY KEY,         -- e.g., 'watchlist_movies_{username}', 'history_{username}'
    data TEXT NOT NULL,           -- JSON 字符串 (完整列表)
    updated_at INTEGER NOT NULL,  -- 上次同步时间
    is_dirty BOOLEAN DEFAULT 0    -- 是否需要后台更新 (前端操作导致的数据变更)
);

-- 翻译缓存 (迁移自原 JSON 文件)
CREATE TABLE IF NOT EXISTS translation_cache (
    id TEXT PRIMARY KEY,          -- 格式: {type}_{id} (e.g., 'movie_12345')
    title TEXT,
    overview TEXT,
    tagline TEXT,
    updated_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
);

-- 应用配置表 (迁移自 app_data.json 和 settings.json)
CREATE TABLE IF NOT EXISTS app_config (
    key TEXT PRIMARY KEY,         -- e.g., 'token', 'theme', 'language'
    value TEXT NOT NULL,          -- JSON 字符串
    updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_media_expires ON media_cache(expires_at);
CREATE INDEX IF NOT EXISTS idx_api_expires ON api_response_cache(expires_at);
";
