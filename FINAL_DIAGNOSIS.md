# 最终诊断 - 403 错误分析

## ✅ 已确认的事实

1. **Client ID 有效** - 使用 curl 测试返回 200 OK
2. **配置文件正确** - `app.conf.json` 格式正确，64 位十六进制
3. **代码逻辑已修复** - `refresh_client` 现在正确设置所有 headers

## 🔍 现在需要你提供的信息

为了进一步诊断，请**重新启动应用**并提供**完整的控制台日志**：

```bash
yarn tauri dev
```

### 需要关注的日志输出

启动后，应该看到类似这样的日志：

```
使用 Client ID (前8位): a7fafee8...
未检测到 Token，使用未认证模式

=== API 请求详情 ===
URL: https://api.trakt.tv/movies/trending?extended=images&limit=40&page=1
Method: GET
Authenticated: false
请求URL "https://api.trakt.tv/movies/trending...", 响应状态码 ???
```

**请告诉我**：
1. 响应状态码是多少？（200 还是 403？）
2. 是否看到 "未检测到 Token，使用未认证模式"？
3. Authenticated 的值是 true 还是 false？

## 🤔 可能的原因分析

### 情况 1: 每次请求都是 403

**可能原因**：
- HTTP 客户端没有正确发送 headers
- 代理或防火墙拦截

**测试方法**：
使用 Wireshark 或 Fiddler 抓包查看实际发送的 HTTP headers

### 情况 2: 第一次 200，后续 403

**可能原因**：
- `ApiClient` 被错误地重新创建，丢失了 headers
- `refresh_client` 在某些情况下被调用但没有正确设置 headers

**测试方法**：
查看日志中是否多次出现 "使用 Client ID" 或 "检测到 Token"

### 情况 3: 只有特定 API 返回 403

**可能原因**：
- 某些 API 需要额外的权限或 headers
- API 路径错误

**测试方法**：
对比哪些 API 成功，哪些失败

## 📋 调试步骤

### 步骤 1: 清理并重新启动

```bash
# 停止所有 Rust 进程
taskkill /F /IM mytv.exe

# 清理构建
cd src-tauri
cargo clean

# 重新构建并运行
cd ..
yarn tauri dev
```

### 步骤 2: 查看详细日志

在 Tauri 终端窗口（黑色窗口）中，你应该看到：

```
使用 Client ID (前8位): a7fafee8...
未检测到 Token，使用未认证模式

=== API 请求详情 ===
URL: ...
Method: ...
Authenticated: ...
请求URL "...", 响应状态码 ...
```

### 步骤 3: 对比成功和失败的请求

如果有些请求成功（200），有些失败（403），对比它们的区别：
- URL 是否不同？
- Authenticated 状态是否不同？
- 是否在不同的时间点发生？

### 步骤 4: 测试单个 API

在浏览器控制台测试：

```javascript
// 测试 trending API（不需要登录）
await window.__TAURI__.core.invoke('movie_trending')

// 测试 recommand API（需要登录）
await window.__TAURI__.core.invoke('movies_recommand')
```

查看返回的错误码。

## 🔧 临时解决方案

如果确实无法解决 403 问题，可以：

1. **使用其他公开 API** - 暂时不使用需要特殊权限的 API
2. **降级功能** - 关闭推荐功能，只使用热门和搜索
3. **切换网络** - 尝试不同的网络连接（排除代理/防火墙问题）

## 下一步

请提供：
1. 重新启动后的**完整控制台日志**（从启动到出现 403 的所有输出）
2. 浏览器 DevTools 的 Console 输出
3. 具体哪个功能/标签页触发了 403

有了这些信息，我可以精确定位问题所在。
