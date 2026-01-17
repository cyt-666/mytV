# 500 错误修复总结

## 🔍 问题诊断

你遇到的 500 错误是因为：

### 根本原因
1. **未登录状态**：用户还没有登录（没有有效的 token）
2. **需要认证的 API**：`movies_recommand` 和 `shows_recommand` 需要用户登录才能访问
3. **未处理的错误**：代码中使用 `.unwrap()` 直接解析 API 响应，当 API 返回 401 时会 panic

### 错误链
```
未登录 
→ 调用需要认证的 API 
→ Trakt API 返回 401 
→ 代码尝试解析空响应 
→ .unwrap() panic 
→ 500 Internal Server Error
```

## ✅ 已实施的修复

### 1. 后端修复 (`src-tauri/src/trakt_api/recommand.rs`)

**改进前**：
```rust
let result = client.req_api(...).await;
if let Ok(result) = result {
    let movies = serde_json::from_value::<Vec<MovieRecommand>>(result).unwrap(); // ❌ 会 panic
    Ok(movies)
} else {
    Err(result.unwrap_err())
}
```

**改进后**：
```rust
// 1. 先检查是否已登录
if !client.authenticated {
    println!("movies_recommand: 用户未登录，返回 401");
    return Err(401);
}

// 2. 安全地处理响应解析
match result {
    Ok(value) => {
        match serde_json::from_value::<Vec<MovieRecommand>>(value) {
            Ok(movies) => Ok(movies),
            Err(e) => {
                println!("movies_recommand: 解析响应失败: {:?}", e);
                Err(500) // ✅ 返回错误码而不是 panic
            }
        }
    }
    Err(status) => Err(status)
}
```

**改进内容**：
- ✅ 添加登录状态检查
- ✅ 移除所有 `.unwrap()` 调用
- ✅ 使用 `match` 优雅处理错误
- ✅ 添加详细的日志输出

### 2. 影响的函数
修复了 4 个需要认证的函数：
- `movies_recommand`
- `shows_recommand`  
- `movies_recommand_page`
- `shows_recommand_page`

## 🎯 现在的行为

### 未登录时
1. 用户访问首页
2. 尝试加载"推荐电影"和"推荐剧集"标签
3. 后端检测到未登录，返回 `401`
4. 前端收到 401 错误（不会崩溃）
5. 控制台显示："需要登录才能查看推荐电影/剧集"

### 登录后
1. 用户登录获取 token
2. API 请求带上认证 token
3. 成功获取推荐内容
4. 正常显示推荐电影和剧集

## 📋 测试步骤

### 1. 重新启动应用
```bash
yarn tauri dev
```

### 2. 未登录状态测试
- ✅ 切换到"🔥 热门"标签 - 应该能正常加载（trending 不需要登录）
- ⚠️ 切换到"🎬 推荐电影"标签 - 会显示加载失败（需要登录）
- ⚠️ 切换到"📺 推荐剧集"标签 - 会显示加载失败（需要登录）

**预期日志**：
```
movies_recommand: 用户未登录，返回 401
加载推荐电影失败: 401
需要登录才能查看推荐电影
```

### 3. 登录后测试
1. 点击右上角"登录"按钮
2. 完成 Trakt OAuth 授权
3. 重新访问"推荐电影"和"推荐剧集"标签
4. 应该能正常加载内容

## 🔧 用户体验改进建议

当前行为：未登录时，推荐标签页会显示加载错误。

**建议改进**：

### 前端添加登录提示
```vue
<template>
  <div v-if="!isLoggedIn && activeTab === 'movies'" class="login-prompt">
    <a-empty description="需要登录才能查看个性化推荐">
      <a-button type="primary" @click="login">立即登录</a-button>
    </a-empty>
  </div>
  <MediaGrid v-else :items="recommendedMovies" ... />
</template>
```

### 或者：隐藏需要登录的标签页
```vue
<a-tab-pane key="movies" title="🎬 推荐电影" v-if="isLoggedIn">
  <!-- 内容 -->
</a-tab-pane>
```

## 📊 API 端点认证要求

| 端点 | 需要登录 | 说明 |
|------|---------|------|
| `/movies/trending` | ❌ | 公开的热门电影 |
| `/shows/trending` | ❌ | 公开的热门剧集 |
| `/search/*` | ❌ | 公开搜索 |
| `/recommendations/movies` | ✅ | 个性化推荐（基于用户历史） |
| `/recommendations/shows` | ✅ | 个性化推荐（基于用户历史） |
| `/users/settings` | ✅ | 用户资料 |
| `/users/*/watched` | ✅ | 观看历史 |
| `/users/*/collection` | ✅ | 收藏 |
| `/users/*/watchlist` | ✅ | 观看清单 |

## 🎬 下一步

现在应用应该不会再出现 500 错误了！

1. **未登录时**：
   - ✅ 可以浏览热门内容
   - ✅ 可以搜索
   - ✅ 可以查看详情
   - ⚠️ 推荐功能会提示需要登录

2. **登录后**：
   - ✅ 所有功能都能正常使用
   - ✅ 可以查看个性化推荐
   - ✅ 可以管理收藏和清单

建议：优化前端 UI，为未登录用户显示友好的登录提示，而不是错误信息。
