// 图片类型
export interface MovieImages {
  poster: string[];
  fanart?: string[];
  banner?: string[];
  thumb?: string[];
  logo?: string[];
  clearart?: string[];
  screenshot?: string[];
}

// 基础影视作品接口
export interface BaseMedia {
  title: string;
  year?: number;
  ids?: {
    trakt: number;
    slug: string;
    tmdb?: number;
    imdb?: string;
    tvdb?: number;
    tvrage?: number[];
  };
  images: MovieImages;
  overview?: string;
  rating?: number;
  votes?: number;
  genres?: string[];
  runtime?: number;
  released?: string;
  trailer?: string;
  media_type?: 'movie' | 'show' | 'season';
}

// 电影类型
export interface Movie extends BaseMedia {
  tagline?: string;
  watchers?: number;
}

// 电影详细信息
export interface MovieDetails {
  title: string;
  year: number;
  ids: {
    trakt: number;
    slug: string;
    imdb?: string;
    tmdb?: number;
  };
  tagline?: string;
  overview?: string;
  released?: string;
  runtime?: number;
  country?: string;
  updated_at?: string;
  trailer?: string;
  homepage?: string;
  status?: string;
  rating?: number;
  votes?: number;
  comment_count?: number;
  languages?: string[];
  available_translations?: string[];
  genres?: string[];
  certification?: string;
  original_title?: string;
}

// 电影翻译信息
export interface MovieTranslation {
  title: string;
  overview: string;
  tagline: string;
  language: string;
  country: string;
}

// 电影翻译信息列表
export type MovieTranslations = MovieTranslation[];

export interface MovieTrending {
  watchers?: number;
  movie?: Movie;
}

export interface MovieAnticipated {
  list_count: number;
  movie: Movie;
}

// 电视剧类型
export interface Show extends BaseMedia {
  watchers?: number;
  favorited_by?: User[];
  status?: string;
  network?: string;
  country?: string;
  language?: string;
  aired_episodes?: number;
  total_episodes?: number;
  seasons?: number;
  latestSeason?: number;
}

export interface ShowTrending {
  watchers?: number;
  show?: Show;
}

export interface ShowAnticipated {
  list_count: number;
  show: Show;
}

// 电视剧详细信息
export interface ShowDetails {
  title: string;
  year: number;
  ids: {
    trakt: number;
    slug: string;
    tvdb?: number;
    imdb?: string;
    tmdb?: number;
    tvrage?: number[];
  };
  tagline?: string;
  overview?: string;
  first_aired?: string;
  airs?: {
    day?: string;
    time?: string;
    timezone?: string;
  };
  runtime?: number;
  certification?: string;
  network?: string;
  country?: string;
  updated_at?: string;
  trailer?: string;
  homepage?: string;
  status?: string;
  rating?: number;
  votes?: number;
  comment_count?: number;
  languages?: string[];
  available_translations?: string[];
  genres?: string[];
  aired_episodes?: number;
  original_title?: string;
}

// 电视剧翻译信息
export interface ShowTranslation {
  title: string;
  overview: string;
  tagline: string;
  language: string;
  country: string;
}

// 电视剧翻译信息列表
export type ShowTranslations = ShowTranslation[];

// 季度信息
export interface Season {
  number: number;
  ids: {
    trakt: number;
    tvdb?: number;
    tmdb?: number;
  };
  rating?: number;
  votes?: number;
  episode_count?: number;
  aired_episodes?: number;
  title?: string;
  overview?: string;
  first_aired?: string;
  updated_at?: string;
  network?: string;
  original_title?: string;
}

// 季度列表
export type Seasons = Season[];

// 季度翻译信息
export interface SeasonTranslation {
  title?: string;
  overview?: string;
  language?: string;
  country?: string;
}

// 季度翻译信息列表
export type SeasonTranslations = SeasonTranslation[];

// 缓存的翻译数据结构
export interface TranslationData {
  title?: string;
  overview?: string;
  tagline?: string;
  updated_at: number;
}

// 剧集信息
export interface Episode {
  season: number;
  number: number;
  title: string;
  ids: {
    trakt: number;
    tvdb?: number;
    tmdb?: number;
    imdb?: string;
  };
  overview?: string;
  rating?: number;
  votes?: number;
  runtime?: number;
  first_aired?: string;
  images?: MovieImages;
}

// 用户观看状态
export interface WatchedStatus {
  media_id: number;
  media_type: 'movie' | 'show' | 'episode';
  watched_at?: string;
  progress?: number; // 0-100
}

// 用户收藏
export interface UserCollection {
  media_id: number;
  media_type: 'movie' | 'show';
  collected_at: string;
}

// 用户观看清单
export interface UserWatchlist {
  media_id: number;
  media_type: 'movie' | 'show';
  listed_at: string;
}

// 用户信息
export interface User {
  username: string;
  private: boolean;
  name?: string;
  vip: boolean;
  vip_ep: boolean;
  ids: {
    slug: string;
    uuid: string;
  };
  description?: string;
  email?: string;
  location?: string;
  website?: string;
  twitter?: string;
  joined_at: string;
  last_login_at: string;
  images?: {
    avatar: {
      full: string;
    };
  };
}

// 用户统计
export interface UserStats {
  movies: {
    plays: number;
    watched: number;
    minutes: number;
    collected: number;
    ratings: number;
    comments: number;
  };
  shows: {
    watched: number;
    collected: number;
    ratings: number;
    comments: number;
  };
  seasons: {
    ratings: number;
    comments: number;
  };
  episodes: {
    plays: number;
    watched: number;
    minutes: number;
    collected: number;
    ratings: number;
    comments: number;
  };
  network: {
    friends: number;
    followers: number;
    following: number;
  };
  ratings: {
    total: number;
    distribution: {
      "1": number;
      "2": number;
      "3": number;
      "4": number;
      "5": number;
      "6": number;
      "7": number;
      "8": number;
      "9": number;
      "10": number;
    };
  };
}

// API返回类型
export type MoviesRecommendResponse = Movie[];
export type ShowsRecommendResponse = Show[];
export type MovieTrendingResponse = MovieTrending[];
export type ShowTrendingResponse = ShowTrending[];
export type SearchResponse = {
  type: 'movie' | 'show';
  score: number;
  movie?: Movie;
  show?: Show;
}[];

export interface CalendarMovie {
  released?: string;
  movie: Movie;
}

export interface CalendarShow {
  first_aired?: string;
  episode?: Episode;
  show: Show;
}

export interface ShowProgress {
  aired: number;
  completed: number;
  last_watched_at?: string;
  reset_at?: string;
  next_episode?: Episode;
  last_episode?: Episode;
}

export interface ShowProgressSummary {
  aired: number;
  completed: number;
  last_watched_at?: string;
}

export interface UpNextItem {
  show: Show;
  next_episode: Episode;
  progress: ShowProgressSummary;
}

// 电影推荐返回类型（保持向后兼容）
export interface MovieRecommend extends Movie {}

// Token接口
export interface Token {
  access_token: string
  token_type: string
  expires_in: number
  refresh_token: string
  scope: string
  created_at: number
}

export interface MovieWatched {
  watcher_count: number
  play_count: number
  collected_count: number
  movie: Movie
}

export interface MovieCollected {
  collected_count: number
  movie: Movie
}

export interface ShowWatched {
  watcher_count: number
  play_count: number
  collected_count: number
  show: Show
}

export interface ShowCollected {
  collected_count: number
  show: Show
}

export type MovieWatchedResponse = MovieWatched[]
export type MovieCollectedResponse = MovieCollected[]
export type ShowWatchedResponse = ShowWatched[]
export type ShowCollectedResponse = ShowCollected[]
