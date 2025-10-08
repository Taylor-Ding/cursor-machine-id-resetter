<template>
  <div class="app-container">
    <!-- 标题栏 -->
    <div class="title-bar" data-tauri-drag-region>
      <div class="title-bar-left">
        <RefreshCcw class="icon" :size="20" />
        <span class="title">{{ t('app.title') }}</span>
      </div>
      <div class="title-bar-right">
        <el-dropdown @command="handleLanguageChange">
          <span class="language-selector">
            <Globe :size="16" />
            <span class="ml-1">{{ currentLanguage }}</span>
          </span>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="zh_cn">简体中文</el-dropdown-item>
              <el-dropdown-item command="en">English</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </div>

    <!-- 主内容区 -->
    <div class="main-content">
      <MainView />
    </div>

    <!-- 状态栏 -->
    <div class="status-bar">
      <div class="status-left">
        <span class="status-text">{{ statusText }}</span>
      </div>
      <div class="status-right">
        <span class="version">v{{ appVersion }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { RefreshCcw, Globe } from 'lucide-vue-next'
import MainView from './views/MainView.vue'
import { useAppStore } from './stores/app'

const { t, locale } = useI18n()
const appStore = useAppStore()

const appVersion = ref('1.0.0')

const currentLanguage = computed(() => {
  return locale.value === 'zh_cn' ? '中文' : 'English'
})

const statusText = computed(() => appStore.statusText || t('status.ready'))

const handleLanguageChange = (lang: string) => {
  locale.value = lang
  localStorage.setItem('language', lang)
}
</script>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  width: 100vw;
  height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.title-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 40px;
  padding: 0 16px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  user-select: none;
}

.title-bar-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.icon {
  color: #409EFF;
}

.title {
  font-size: 14px;
  font-weight: 600;
  color: #333;
}

.title-bar-right {
  display: flex;
  align-items: center;
}

.language-selector {
  display: flex;
  align-items: center;
  padding: 4px 8px;
  cursor: pointer;
  font-size: 13px;
  color: #666;
  border-radius: 4px;
  transition: all 0.3s;
}

.language-selector:hover {
  background: rgba(64, 158, 255, 0.1);
  color: #409EFF;
}

.main-content {
  flex: 1;
  overflow: auto;
  padding: 20px;
}

.status-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 28px;
  padding: 0 16px;
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  border-top: 1px solid rgba(0, 0, 0, 0.1);
  font-size: 12px;
  color: #666;
}

.status-text {
  display: flex;
  align-items: center;
  gap: 4px;
}

.version {
  font-weight: 500;
  color: #999;
}
</style>

