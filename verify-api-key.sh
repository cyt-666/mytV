#!/bin/bash

# Trakt API Client ID 验证脚本

echo "========================================="
echo "Trakt API Client ID 验证"
echo "========================================="
echo ""

# 读取配置文件
if [ ! -f "src-tauri/app.conf.json" ]; then
    echo "❌ 错误：配置文件 src-tauri/app.conf.json 不存在"
    exit 1
fi

echo "✅ 配置文件存在"
echo ""

# 提取 Client ID（需要安装 jq 或使用 Python）
if command -v python3 &> /dev/null; then
    CLIENT_ID=$(python3 -c "import json; print(json.load(open('src-tauri/app.conf.json'))['client_id'])")
    echo "Client ID (前16位): ${CLIENT_ID:0:16}..."
    echo "Client ID 长度: ${#CLIENT_ID}"
    echo ""
else
    echo "⚠️  无法读取 Client ID，请手动检查"
    echo ""
fi

# 测试 API 访问
echo "========================================="
echo "测试 Trakt API 访问"
echo "========================================="
echo ""

if [ -n "$CLIENT_ID" ]; then
    echo "测试 1: 使用你的 Client ID 访问公开 API"
    echo "----------------------------------------"
    RESPONSE=$(curl -s -w "\nHTTP_CODE:%{http_code}" \
        "https://api.trakt.tv/movies/trending?limit=1" \
        -H "Content-Type: application/json" \
        -H "trakt-api-version: 2" \
        -H "trakt-api-key: $CLIENT_ID")
    
    HTTP_CODE=$(echo "$RESPONSE" | grep "HTTP_CODE" | cut -d':' -f2)
    
    if [ "$HTTP_CODE" = "200" ]; then
        echo "✅ 成功 (200 OK)"
        echo "你的 Client ID 有效！"
    elif [ "$HTTP_CODE" = "403" ]; then
        echo "❌ 失败 (403 Forbidden)"
        echo ""
        echo "可能的原因："
        echo "1. Client ID 无效或已被禁用"
        echo "2. Trakt 应用未激活"
        echo "3. Client ID 格式错误"
        echo ""
        echo "解决方案："
        echo "1. 访问 https://trakt.tv/oauth/applications"
        echo "2. 检查你的应用状态"
        echo "3. 如果应用不存在，创建新应用"
        echo "4. 复制新的 Client ID 和 Client Secret"
        echo "5. 更新 src-tauri/app.conf.json"
    else
        echo "⚠️  意外响应 ($HTTP_CODE)"
        echo "响应内容:"
        echo "$RESPONSE" | grep -v "HTTP_CODE"
    fi
else
    echo "⚠️  无法提取 Client ID，请手动测试"
    echo ""
    echo "手动测试命令："
    echo 'curl -i "https://api.trakt.tv/movies/trending?limit=1" \'
    echo '  -H "Content-Type: application/json" \'
    echo '  -H "trakt-api-version: 2" \'
    echo '  -H "trakt-api-key: YOUR_CLIENT_ID"'
fi

echo ""
echo "========================================="
echo "检查完成"
echo "========================================="
