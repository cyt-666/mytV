# Trakt Token 认证问题修复总结

## 已实施的修复

### 1. 添加 Token 过期检查 (`src-tauri/src/token.rs`)

```rust
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

**作用**：在每次检查登录状态时，自动判断 token 是否过期。

### 2. 改进登录状态检查 (`src-tauri/src/trakt_api/auth.rs`)

```rust
#[command]
pub async fn check_login_status(app: AppHandle) -> bool {
    let exists = token_exists(&app);
    
    if !exists {
        return false;
    }
    
    let token_state = app.try_state::<Mutex<Token>>();
    if let Some(token_state) = token_state {
        let token = token_state.lock().await;
        if token.is_expired() {
            println!("Token 已过期，尝试刷新");
            drop(token);
            
            let refresh_result = refresh_token(&app).await;
            if refresh_result.is_ok() {
                println!("Token 刷新成功");
                return true;
            } else {
                println!("Token 刷新失败，需要重新登录");
                let store = app.store("app_data.json").unwrap();
                let _ = store.delete("token");
                return false;
            }
        }
    }
    
    exists
}
```

**作用**：
- 检查 token 是否存在
- 如果存在但已过期，自动尝试刷新
- 如果刷新失败，清除无效 token，要求用户重新登录

### 3. 增强 401 错误处理 (`src-tauri/src/trakt_api.rs`)

```rust
if status == 401 {
    println!("收到 401 响应，尝试刷新 token");
    let result = refresh_token(app).await;
    if let Ok(token) = result {
        println!("刷新token成功: {:?}", &token);
        self.refresh_client(Some(token));
    } else {
        println!("刷新token失败: {:?}", &result);
        let store = app.store("app_data.json").unwrap();
        let _ = store.delete("token");
        
        if let Some(token_state) = app.try_state::<Mutex<Token>>() {
            let mut t = token_state.lock().await;
            *t = Token {
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

**作用**：
- 收到 401 错误时尝试刷新 token
- 刷新失败时彻底清除无效 token（包括存储和内存状态）
- 防止后续请求继续使用无效 token

## 如何测试修复

### 1. 编译并运行
```bash
yarn tauri dev
```

### 2. 测试场景

#### 场景 1：首次登录
1. 点击"登录"按钮
2. 浏览器打开 Trakt 授权页面
3. 授权后应该能正常获取 token
4. 控制台应该显示："获取到token:"

#### 场景 2：Token 过期自动刷新
1. 等待 token 过期（默认 3 个月）
2. 访问任何需要认证的页面
3. 系统应该自动刷新 token
4. 控制台显示："Token 已过期，尝试刷新" → "Token 刷新成功"

#### 场景 3：刷新失败需要重新登录
1. 如果 refresh_token 也失效
2. 系统清除无效 token
3. 控制台显示："Token 刷新失败，需要重新登录"
4. 用户被要求重新登录

## 常见问题排查

### 问题 1：仍然无法登录

**检查**：
```bash
# 确认配置文件存在
ls src-tauri/app.conf.json

# 查看配置内容（注意保密）
cat src-tauri/app.conf.json
```

**配置文件应该包含**：
```json
{
  "client_id": "your_trakt_client_id",
  "client_secret": "your_trakt_client_secret",
  "redirect_uri": "/oauth/callback",
  "oauth_port": 4396
}
```

### 问题 2：端口被占用

**检查端口**：
```bash
netstat -ano | findstr :4396
```

**解决方案**：修改 `app.conf.json` 中的 `oauth_port` 为其他端口（如 4397）

### 问题 3：401 错误循环

**原因**：refresh_token 也失效了

**解决方案**：
```bash
# 删除旧 token
del "%APPDATA%\com.mytv.dev\app_data.json"

# 重启应用
yarn tauri dev

# 重新登录
```

### 问题 4：控制台无日志输出

**检查**：
1. 确认运行的是 `yarn tauri dev` 而不是 `yarn dev`
2. 查看 Tauri 终端窗口（黑色窗口）而不是浏览器控制台

## 预期日志输出

### 正常登录流程：
```
开始用户认证: Ok(())
请求URL "https://api.trakt.tv/oauth/token", 响应状态码 200
获取到token: Token { access_token: "...", ... }
请求URL "https://api.trakt.tv/users/settings", 响应状态码 200
```

### Token 过期自动刷新：
```
Token 已过期，尝试刷新
刷新token成功
```

### Token 失效需要重新登录：
```
Token 已过期，尝试刷新
刷新token失败，需要重新登录
```

## 需要额外帮助？

如果问题仍未解决，请提供以下信息：
1. **完整的控制台日志** - 从启动到错误发生的所有输出
2. **浏览器 DevTools 日志** - 按 Ctrl+Shift+I 打开
3. **具体错误信息** - 截图或复制错误文本
4. **测试步骤** - 你具体做了什么操作导致错误

## 下一步优化建议

1. **添加用户友好的错误提示**
   - 在前端显示 token 过期提示
   - 引导用户重新登录

2. **自动重试机制**
   - Token 刷新成功后自动重试原始请求
   
3. **Token 预刷新**
   - 在 token 即将过期前主动刷新

4. **错误监控**
   - 记录认证失败原因
   - 统计分析常见问题
