# 403 错误再次出现 - Client ID 问题

## 🔍 问题诊断

经过测试，发现：

```bash
curl "https://api.trakt.tv/movies/trending?limit=1" \
  -H "trakt-api-key: a7fafee8..." \
  → 返回 403 Forbidden
```

**结论：你的 Client ID 无效或已被 Trakt 禁用！**

## ❌ 为什么会这样？

可能的原因：

1. **Client ID 格式错误** - 复制时出错
2. **应用被禁用** - Trakt 后台禁用了应用
3. **应用被删除** - 之前的应用已不存在
4. **Client ID 已过期** - 长时间未使用被回收

## ✅ 解决方案

### 步骤 1: 检查 Trakt 应用状态

1. 访问：https://trakt.tv/oauth/applications
2. 登录你的 Trakt 账户
3. 检查应用列表：
   - ✅ 应用存在且状态正常
   - ❌ 应用不存在或被禁用

### 步骤 2: 创建新的 Trakt 应用

如果应用不存在或有问题，创建新应用：

1. **点击 "NEW APPLICATION"**

2. **填写应用信息**：
   ```
   Name: MyTV
   Description: Third-party Trakt client for desktop
   Redirect URI: http://localhost:4396/oauth/callback
   ```

3. **勾选权限** (Permissions)：
   - ☑ `/checkin` - Checkin access
   - ☑ `/scrobble` - Scrobble access  
   - ☑ `/recommendations` - Personal recommendations
   - ☑ `/sync/collection` - Collection management
   - ☑ `/sync/watchlist` - Watchlist management
   - ☑ `/sync/watched` - Watch history
   - ☑ `/comments` - Comments

4. **提交并获取凭证**

### 步骤 3: 更新配置文件

创建应用后，你会看到：

```
Client ID: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
Client Secret: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

**更新 `src-tauri/app.conf.json`**：

```json
{
  "client_id": "粘贴完整的新 Client ID（64位）",
  "client_secret": "粘贴完整的新 Client Secret（64位）",
  "redirect_uri": "/oauth/callback",
  "oauth_port": 4396
}
```

**重要**：
- ✅ 确保复制**完整的** Client ID（64 个字符）
- ✅ 不要有多余的空格或换行
- ✅ 使用双引号包裹字符串

### 步骤 4: 验证配置

使用验证脚本测试新的 Client ID：

```bash
# 在项目根目录运行
bash verify-api-key.sh
```

或手动测试：

```bash
# 替换 YOUR_NEW_CLIENT_ID 为你的新 Client ID
curl -i "https://api.trakt.tv/movies/trending?limit=1" \
  -H "Content-Type: application/json" \
  -H "trakt-api-version: 2" \
  -H "trakt-api-key: YOUR_NEW_CLIENT_ID"
```

**预期响应**：
```
HTTP/1.1 200 OK
```

### 步骤 5: 重启应用

```bash
# 停止当前应用 (Ctrl+C)
yarn tauri dev
```

**预期日志**：
```
使用 Client ID (前8位): xxxxxxxx...
未检测到 Token，使用未认证模式
请求URL "https://api.trakt.tv/movies/trending...", 响应状态码 200 ✅
```

## 📋 常见问题

### Q: 我的 Client ID 长度不是 64 位怎么办？

**A**: Trakt Client ID 应该是 64 个十六进制字符。如果不是：
- 检查是否复制完整
- 确认你复制的是 "Client ID" 而不是其他字段

### Q: 我创建了新应用但还是 403？

**A**: 检查：
1. Client ID 是否正确复制到 `app.conf.json`
2. JSON 格式是否正确（无语法错误）
3. 是否重启了应用
4. 使用 curl 测试 API 是否返回 200

### Q: 找不到之前的应用怎么办？

**A**: 直接创建新应用即可。旧应用的凭证不会影响新应用。

## 🔧 快速检查清单

- [ ] 访问 https://trakt.tv/oauth/applications
- [ ] 确认应用存在且状态正常
- [ ] Client ID 长度为 64 字符
- [ ] Client Secret 长度为 64 字符
- [ ] `app.conf.json` JSON 格式正确
- [ ] 使用 curl 测试返回 200
- [ ] 重启应用查看日志

## 💡 防止未来出现此问题

1. **备份凭证** - 将 Client ID 和 Secret 保存到密码管理器
2. **定期检查** - 确保 Trakt 应用状态正常
3. **不要分享** - 保密你的 Client ID 和 Secret

## 下一步

更新配置后，应用应该能正常工作：
- ✅ 加载热门内容
- ✅ 搜索功能
- ✅ 查看详情
- 🔐 登录后访问个人功能

如果更新配置后仍然 403，请提供：
1. 新 Client ID 的前 8 位字符
2. curl 测试的完整输出
3. 应用启动日志
