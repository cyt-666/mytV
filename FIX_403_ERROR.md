# 403 Forbidden 错误修复指南

## 问题原因

你遇到的 `403 Forbidden` 错误是因为：

**Trakt API 拒绝了你的请求，这通常意味着你的 API Key 配置有问题。**

从日志可以看到：
- `false` - 表示没有检测到登录状态（没有 token）
- `403` - API 拒绝请求（API Key 无效或缺失）

## 立即检查

### 1. 检查配置文件是否存在

```bash
# 查看配置文件
cat src-tauri/app.conf.json
```

**如果文件不存在**，你需要创建它。

### 2. 验证 API Key 配置

配置文件应该长这样：

```json
{
  "client_id": "你的实际client_id",
  "client_secret": "你的实际client_secret",
  "redirect_uri": "/oauth/callback",
  "oauth_port": 4396
}
```

**重要**：
- `client_id` 和 `client_secret` 必须是真实有效的值
- **不能是空字符串或占位符**

### 3. 获取有效的 Trakt API 凭证

1. **访问 Trakt 开发者页面**：
   - 网址：https://trakt.tv/oauth/applications
   
2. **如果没有应用，创建新应用**：
   - 点击 "NEW APPLICATION"
   - 填写信息：
     - **Name**: MyTV
     - **Description**: Third-party Trakt client
     - **Redirect URI**: `http://localhost:4396/oauth/callback`
     - **Permissions**: 勾选所有需要的权限
   
3. **创建后获取凭证**：
   - 复制 **Client ID**
   - 复制 **Client Secret**

4. **更新配置文件**：
   ```json
   {
     "client_id": "粘贴你的Client ID",
     "client_secret": "粘贴你的Client Secret",
     "redirect_uri": "/oauth/callback",
     "oauth_port": 4396
   }
   ```

## 测试 API Key 是否有效

创建一个测试脚本来验证 API Key：

```bash
# 替换 YOUR_CLIENT_ID 为你的实际 Client ID
curl -i "https://api.trakt.tv/movies/trending?limit=1" \
  -H "Content-Type: application/json" \
  -H "trakt-api-version: 2" \
  -H "trakt-api-key: YOUR_CLIENT_ID"
```

**预期响应**：
- ✅ **200 OK** - API Key 有效
- ❌ **403 Forbidden** - API Key 无效或缺失
- ❌ **401 Unauthorized** - 需要认证的端点（trending 不需要）

## 快速修复步骤

### 步骤 1: 创建配置文件

在 `src-tauri/` 目录下创建或编辑 `app.conf.json`：

```bash
# 创建配置文件
cd src-tauri
nano app.conf.json
```

### 步骤 2: 填入有效凭证

```json
{
  "client_id": "你从Trakt获取的Client ID",
  "client_secret": "你从Trakt获取的Client Secret",
  "redirect_uri": "/oauth/callback",
  "oauth_port": 4396
}
```

### 步骤 3: 重启应用

```bash
# 停止当前运行的应用 (Ctrl+C)
# 重新启动
yarn tauri dev
```

### 步骤 4: 验证日志

启动后应该看到：
- ✅ 没有 "403" 错误
- ✅ 请求返回 "200" 状态码
- ✅ 能够正常加载数据

## 常见错误

### 错误 1: 配置文件不存在

**症状**：
```
thread 'main' panicked at '配置文件 app.conf.json 不存在'
```

**解决**：按照上面步骤创建配置文件。

### 错误 2: JSON 格式错误

**症状**：
```
thread 'main' panicked at '无法解析配置文件'
```

**解决**：检查 JSON 格式，确保：
- 所有字符串用双引号
- 键值对用逗号分隔
- 最后一个键值对后不要有逗号

### 错误 3: API Key 失效

**症状**：即使有配置文件，仍然返回 403

**解决**：
1. 访问 https://trakt.tv/oauth/applications
2. 检查应用状态是否正常
3. 重新生成 Client Secret
4. 更新配置文件

## 安全提示

⚠️ **重要**：`app.conf.json` 包含敏感信息，已被添加到 `.gitignore`。

**不要**：
- ❌ 将配置文件提交到 Git
- ❌ 分享配置文件截图
- ❌ 在公开环境泄露 Client Secret

**应该**：
- ✅ 保密你的 Client ID 和 Client Secret
- ✅ 使用环境变量（生产环境）
- ✅ 定期轮换密钥

## 配置文件位置

配置文件应该在：
```
MyTV/
└── src-tauri/
    └── app.conf.json  <-- 这里
```

**注意**：配置文件在 `src-tauri/` 目录下，**不是**项目根目录。

## 验证配置是否生效

启动应用后，检查控制台日志：

```
# 应该看到（如果已登录）：
true

# 或者（如果未登录）：
false

# 然后应该看到成功的请求：
请求URL "https://api.trakt.tv/movies/trending...", 响应状态码 200
```

## 下一步

配置好 API Key 后：
1. 重启应用
2. 点击"登录"进行 OAuth 认证
3. 授权后就可以正常使用所有功能了

如果仍有问题，请提供：
- `app.conf.json` 文件是否存在
- Client ID 的前几位字符（用于验证格式）
- 完整的控制台日志
