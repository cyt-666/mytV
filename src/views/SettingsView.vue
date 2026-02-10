<template>
  <div class="settings-view">
    <div class="page-container">
      <!-- 页面头部 -->
      <div class="page-header">
        <h1 class="page-title">设置</h1>
      </div>

      <!-- 设置区域 -->
        <div class="settings-container">
        <h2 class="section-title">应用偏好</h2>
        
        <div class="settings-card">
          <!-- 主题设置 -->
          <div class="setting-item">
            <div class="setting-icon">
              <icon-skin />
            </div>
            <div class="setting-content">
              <div class="setting-text">
                <div class="setting-title">外观主题</div>
                <div class="setting-desc">选择应用的主题外观</div>
              </div>
              <div class="setting-control">
                <a-radio-group v-model="themeMode" type="button" @change="setTheme">
                  <a-radio value="light">浅色</a-radio>
                  <a-radio value="dark">深色</a-radio>
                  <a-radio value="system">跟随系统</a-radio>
                </a-radio-group>
              </div>
            </div>
          </div>

          <!-- 日志级别 -->
          <div class="setting-item">
            <div class="setting-icon">
              <icon-bug />
            </div>
            <div class="setting-content">
              <div class="setting-text">
                <div class="setting-title">日志级别</div>
                <div class="setting-desc">控制应用程序输出日志的详细程度</div>
              </div>
              <div class="setting-control">
                <a-select 
                  v-model="logLevel" 
                  @change="handleLogLevelChange" 
                  :style="{width:'140px'}"
                  :trigger-props="{ autoFitPopupMinWidth: true }"
                >
                  <a-option value="error">Error</a-option>
                  <a-option value="warn">Warn</a-option>
                  <a-option value="info">Info</a-option>
                  <a-option value="debug">Debug</a-option>
                  <a-option value="trace">Trace</a-option>
                </a-select>
              </div>
            </div>
          </div>
        </div>

        <div class="settings-footer">
          MyTV v0.1.0
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { IconBug, IconSkin } from '@arco-design/web-vue/es/icon'
import { invoke } from '@tauri-apps/api/core'
import { Message } from '@arco-design/web-vue'
import { useTheme } from '../composables/useTheme'

const { themeMode, setTheme } = useTheme()
const logLevel = ref('info')

const loadSettings = async () => {
  try {
    const config = await invoke<{ log_level: string }>('get_app_config')
    logLevel.value = config.log_level || 'info'
  } catch (error) {
    console.error('Failed to load settings:', error)
  }
}

const handleLogLevelChange = async (value: any) => {
  try {
    await invoke('update_log_level', { level: value })
    Message.success('日志级别已更新，重启应用后生效')
  } catch (error) {
    console.error('Failed to update log level:', error)
    Message.error('更新日志级别失败')
  }
}

onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.settings-view {
  min-height: 100vh;
  padding-bottom: 40px;
}

.page-container {
  max-width: 800px; /* 限制最大宽度，让设置页更精致 */
  margin: 0 auto;
  padding: 0 20px;
}

.page-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 32px 0 24px;
}

.page-title {
  font-size: 32px;
  font-weight: 700;
  margin: 0;
  color: var(--glass-text);
  letter-spacing: -0.5px;
}

.section-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--glass-text-secondary);
  margin: 0 0 12px 12px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.settings-card {
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
  border-radius: 12px;
  overflow: hidden;
  box-shadow: var(--glass-shadow);
}

.setting-item {
  display: flex;
  align-items: center;
  padding: 16px 20px;
  gap: 16px;
  min-height: 72px;
}

.setting-item:not(:last-child) {
  border-bottom: 1px solid var(--glass-border);
}

.setting-icon {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  background: rgba(128, 128, 128, 0.1);
  color: var(--glass-text);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
}

.setting-content {
  flex: 1;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.setting-text {
  flex: 1;
  padding-right: 16px;
}

.setting-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--glass-text);
  margin-bottom: 4px;
}

.setting-desc {
  font-size: 13px;
  color: var(--glass-text-secondary);
  line-height: 1.4;
}

.settings-footer {
  margin-top: 32px;
  text-align: center;
  font-size: 12px;
  color: var(--glass-text-secondary);
  opacity: 0.6;
}

@media (max-width: 640px) {
  .page-title {
    font-size: 24px;
  }
  
  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    padding: 16px;
  }
  
  .setting-icon {
    display: none; /* 移动端简化显示 */
  }
  
  .setting-content {
    width: 100%;
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .setting-control {
    width: 100%;
  }
  
  .setting-control :deep(.arco-select) {
    width: 100% !important;
  }
}
</style>
