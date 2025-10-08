<template>
  <div class="main-view">
    <div class="main-card">
      <!-- Tab 切换 - 直接放在顶部 -->
      <el-tabs v-model="activeTab" class="tabs">
        <!-- 重置标签页 -->
        <el-tab-pane :label="t('tabs.reset')" name="reset">
          <ResetPanel />
        </el-tab-pane>

        <!-- 恢复标签页 -->
        <el-tab-pane :label="t('tabs.restore')" name="restore">
          <RestorePanel />
        </el-tab-pane>

        <!-- 邮箱生成标签页 -->
        <el-tab-pane :label="t('tabs.email')" name="email">
          <EmailGeneratorPanel />
        </el-tab-pane>

        <!-- 日志标签页 -->
        <el-tab-pane :label="t('tabs.logs')" name="logs">
          <LogsPanel />
        </el-tab-pane>

        <!-- 设置标签页 -->
        <el-tab-pane :label="t('tabs.settings')" name="settings">
          <SettingsPanel />
        </el-tab-pane>
      </el-tabs>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import ResetPanel from '../components/ResetPanel.vue'
import RestorePanel from '../components/RestorePanel.vue'
import EmailGeneratorPanel from '../components/EmailGeneratorPanel.vue'
import LogsPanel from '../components/LogsPanel.vue'
import SettingsPanel from '../components/SettingsPanel.vue'

const { t } = useI18n()
const activeTab = ref('reset')
</script>

<style scoped>
.main-view {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: stretch;
  justify-content: center;
}

.main-card {
  width: 100%;
  max-width: 1200px;
  height: 100%;
  background: rgba(255, 255, 255, 0.98);
  backdrop-filter: blur(20px);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.tabs {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
}

:deep(.el-tabs) {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
}

:deep(.el-tabs__header) {
  flex-shrink: 0;
  margin: 0;
  padding: 0 24px;
  background: white;
  border-bottom: 2px solid #e8eaed;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
  z-index: 10;
}

:deep(.el-tabs__nav-wrap) {
  margin-bottom: 0;
}

:deep(.el-tabs__content) {
  flex: 1;
  overflow: hidden;
  padding: 0;
  position: relative;
  min-height: 0;
  background: #f5f7fa;
}

:deep(.el-tab-pane) {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 0;
}

:deep(.el-tabs__item) {
  font-size: 16px;
  font-weight: 500;
  padding: 0 24px;
  height: 56px;
  line-height: 56px;
  color: #606266;
  transition: all 0.3s;
}

:deep(.el-tabs__item:hover) {
  color: #667eea;
}

:deep(.el-tabs__item.is-active) {
  color: #667eea;
  font-weight: 600;
}

:deep(.el-tabs__active-bar) {
  background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
  height: 3px;
  border-radius: 3px 3px 0 0;
}

/* 滚动条美化 */
:deep(.el-tab-pane)::-webkit-scrollbar {
  width: 8px;
}

:deep(.el-tab-pane)::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

:deep(.el-tab-pane)::-webkit-scrollbar-thumb {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 4px;
}

:deep(.el-tab-pane)::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(135deg, #5568d3 0%, #6a3f8f 100%);
}
</style>

