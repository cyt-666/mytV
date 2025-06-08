# 用户认证状态管理

## 使用方法

在任何需要访问用户信息的组件中：

```vue
<script setup lang="ts">
import { useAuth } from '../composables/useAuth'

const { userInfo, isLoggedIn, refreshUserInfo } = useAuth()

// userInfo: 当前用户信息 (User | null)
// isLoggedIn: 是否已登录 (boolean)
// refreshUserInfo: 刷新用户信息的函数
</script>

<template>
  <div v-if="isLoggedIn">
    <h1>欢迎, {{ userInfo?.username }}!</h1>
    <p>{{ userInfo?.name }}</p>
  </div>
  <div v-else>
    <p>请先登录</p>
  </div>
</template>
```

## 优势

- ✅ 避免重复API请求
- ✅ 全局状态同步
- ✅ 类型安全
- ✅ 自动更新

## 注意事项

- 只能在 AppLayout 的子组件中使用
- userInfo 会在登录成功后自动填充
- 退出登录时会自动清空 