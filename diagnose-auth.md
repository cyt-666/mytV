# Trakt 认证问题诊断指南

## 问题检查清单

### 1. 检查配置文件

确认 `src-tauri/app.conf.json` 文件存在且内容正确：

```bash
# 检查文件是否存在
ls src-tauri/app.conf.json

# 查看文件内容（注意：不要分享到公开环境）
cat src-tauri/app.conf.json
```

配置文件应该包含：
```json
{
  "client_id": "your_trakt_client_id",
  "client_secret": "your_trakt_client_secret",
  "redirect_uri": "/oauth/callback",
  "oauth_port": 4396
}
```

### 2. 验证 Trakt API 凭证

访问 https://trakt.tv/oauth/applications 检查：
- ✅ 应用是否存在
- ✅ Client ID 和 Client Secret 是否正确
- ✅ Redirect URI 是否包含 `http://localhost:4396/oauth/callback`

### 3. 检查端口占用

```bash
# Windows
netstat -ano | findstr :4396

# 如果端口被占用，可以修改 app.conf.json 中的 oauth_port
```

### 4. 查看运行日志

启动应用并查看控制台输出：

```bash
yarn tauri dev
```

关注以下日志：
- ✅ "开始用户认证:" - 表示 `start_trakt_user_auth` 被调用
- ✅ "请求URL" - 显示 API 请求地址
- ✅ "响应状态码" - 显示 API 响应
- ❌ "获取token失败" - token 获取失败
- ❌ "401" - 认证失败，token 可能过期

### 5. 常见错误码

| 错误码 | 含义 | 解决方案 |
|--------|------|----------|
| 200 | 已登录 | 正常，无需处理 |
| 400 | 请求错误 | 检查配置文件参数 |
| 401 | 认证失败 | Token 过期或无效，需要重新登录 |
| 404 | API 路径错误 | 检查 api.json 配置 |
| 500 | 服务器错误 | 网络问题或 Trakt API 故障 |

### 6. 手动测试 OAuth 流程

1. **启动 OAuth 认证**：
   - 点击应用中的"登录"按钮
   - 浏览器应该打开 Trakt 授权页面

2. **授权后**：
   - 浏览器跳转到 `http://localhost:4396/oauth/callback?code=xxxxx`
   - 应该看到"认证完成"页面

3. **检查 Token 存储**：
   - Token 保存在 `app_data.json` 中
   - 位置：`%APPDATA%/com.mytv.dev/app_data.json` (开发环境)

### 7. 清理并重新认证

```bash
# 删除存储的 token
# Windows: 删除 %APPDATA%/com.mytv.dev/app_data.json

# 重启应用
yarn tauri dev
```

## 调试步骤

### Step 1: 验证后端能正常启动

```bash
cd src-tauri
cargo check
cargo run
```

### Step 2: 测试前端能调用后端命令

在浏览器控制台执行：

```javascript
// 检查登录状态
await window.__TAURI__.core.invoke('check_login_status')

// 如果返回 false，尝试登录
await window.__TAURI__.core.invoke('start_trakt_user_auth')
```

### Step 3: 查看网络请求

打开 Tauri DevTools (Ctrl+Shift+I)，查看：
1. Network 标签 - 确认 API 请求是否发送
2. Console 标签 - 查看错误信息

## 可能的修复方案

### 方案 1: 重置 Token 存储

```rust
// src-tauri/src/trakt_api/auth.rs
// 在 check_login_status 中添加调试日志

#[command]
pub async fn check_login_status(app: AppHandle) -> bool {
    let exists = token_exists(&app);
    println!("Token exists: {}", exists); // 添加日志
    
    // 如果 token 存在但失效，尝试刷新
    if exists {
        let token_state = app.try_state::<Mutex<Token>>();
        if let Some(token_state) = token_state {
            let token = token_state.lock().await;
            println!("Current token: {:?}", token); // 添加日志
        }
    }
    
    exists
}
```

### 方案 2: 修复 Token 刷新逻辑

当前代码在 401 时会自动刷新 token，但如果刷新失败，需要清除无效 token：

```rust
// src-tauri/src/trakt_api.rs 第 214-224 行
if status == 401 {
    println!("401 - 尝试刷新token");
    let result = refresh_token(app).await;
    if let Ok(token) = result {
        println!("刷新token成功: {:?}", &token);
        self.refresh_client(Some(token));
        // TODO: 重试原始请求
    } else {
        println!("刷新token失败: {:?}", &result);
        let store = app.store("app_data.json").unwrap();
        let _ = store.delete("token");
        
        // 清除应用状态中的 token
        if let Some(token_state) = app.try_state::<Mutex<Token>>() {
            let mut token = token_state.lock().await;
            *token = Token {
                access_token: String::new(),
                token_type: String::new(),
                expires_in: 0,
                refresh_token: String::new(),
                scope: String::new(),
                created_at: 0,
            };
        }
    }
}
```

### 方案 3: 添加 Token 过期检查

```rust
// src-tauri/src/token.rs
impl Token {
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let expires_at = self.created_at + self.expires_in as u64;
        now >= expires_at
    }
}
```

## 快速修复命令

如果需要快速重置认证状态：

```bash
# 1. 停止应用
# 2. 清理 token 存储
# Windows:
del "%APPDATA%\com.mytv.dev\app_data.json"

# 3. 重启应用
yarn tauri dev
```

## 需要更多帮助？

如果上述步骤无法解决问题，请提供：
1. 完整的控制台日志输出
2. 浏览器 DevTools 中的错误信息
3. `check_login_status` 返回的值
4. 是否能成功打开 Trakt 授权页面
