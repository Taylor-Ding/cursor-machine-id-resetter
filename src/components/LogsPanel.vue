<template>
  <div class="logs-panel">
    <div class="logs-header">
      <div class="header-left">
        <h3 class="section-title">
          <FileText :size="18" />
          <span>{{ t('logs.title') }}</span>
        </h3>
        <el-tag size="small" type="info">{{ logs.length }} {{ t('logs.items') }}</el-tag>
      </div>
      
      <div class="header-right">
        <el-select v-model="filterLevel" size="small" style="width: 120px" @change="handleFilterChange">
          <el-option :label="t('logs.all')" value="all" />
          <el-option :label="t('logs.info')" value="info" />
          <el-option :label="t('logs.success')" value="success" />
          <el-option :label="t('logs.warning')" value="warning" />
          <el-option :label="t('logs.error')" value="error" />
        </el-select>
        
        <el-button size="small" @click="handleExport">
          <Download :size="16" class="mr-1" />
          {{ t('logs.export') }}
        </el-button>
        
        <el-button size="small" type="danger" @click="handleClear">
          <Trash2 :size="16" class="mr-1" />
          {{ t('logs.clear') }}
        </el-button>
      </div>
    </div>

    <div class="logs-content" ref="logsContainer">
      <div v-if="filteredLogs.length === 0" class="empty-state">
        <FileText :size="64" class="empty-icon" />
        <p class="empty-text">{{ t('logs.empty') }}</p>
      </div>

      <div v-else class="log-list">
        <div
          v-for="log in filteredLogs"
          :key="log.id"
          :class="['log-item', `log-${log.level}`]"
        >
          <div class="log-header">
            <component :is="getLogIcon(log.level)" :size="16" class="log-icon" />
            <span class="log-time">{{ formatTime(log.timestamp) }}</span>
            <el-tag :type="getLogType(log.level)" size="small">
              {{ t(`logs.${log.level}`) }}
            </el-tag>
          </div>
          <div class="log-message">{{ log.message }}</div>
        </div>
      </div>
    </div>

    <div class="logs-footer">
      <el-checkbox v-model="autoScroll">
        {{ t('logs.auto_scroll') }}
      </el-checkbox>
      <span class="footer-info">
        {{ t('logs.showing') }}: {{ filteredLogs.length }} / {{ logs.length }}
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { FileText, Download, Trash2, Info, CheckCircle, AlertTriangle, XCircle } from 'lucide-vue-next'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useLogStore, type LogLevel } from '../stores/log'
import { save } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'

const { t } = useI18n()
const logStore = useLogStore()

const logsContainer = ref<HTMLElement | null>(null)
const filterLevel = ref<LogLevel | 'all'>('all')
const autoScroll = ref(true)

const logs = computed(() => logStore.logs)

const filteredLogs = computed(() => {
  if (filterLevel.value === 'all') {
    return logs.value
  }
  return logs.value.filter(log => log.level === filterLevel.value)
})

const getLogIcon = (level: LogLevel) => {
  const icons = {
    info: Info,
    success: CheckCircle,
    warning: AlertTriangle,
    error: XCircle
  }
  return icons[level] || Info
}

const getLogType = (level: LogLevel) => {
  const types = {
    info: 'info',
    success: 'success',
    warning: 'warning',
    error: 'danger'
  }
  return types[level] || 'info'
}

const formatTime = (timestamp: number): string => {
  return new Date(timestamp).toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

const handleFilterChange = () => {
  scrollToBottom()
}

const handleExport = async () => {
  try {
    const filePath = await save({
      defaultPath: `cursor-reset-logs-${Date.now()}.txt`,
      filters: [{
        name: 'Text',
        extensions: ['txt']
      }]
    })

    if (filePath) {
      const content = filteredLogs.value
        .map(log => `[${formatTime(log.timestamp)}] [${log.level.toUpperCase()}] ${log.message}`)
        .join('\n')

      await writeTextFile(filePath, content)
      ElMessage.success(t('logs.export_success'))
    }
  } catch (error) {
    console.error('Export failed:', error)
    ElMessage.error(t('logs.export_failed'))
  }
}

const handleClear = async () => {
  try {
    await ElMessageBox.confirm(
      t('logs.clear_confirm'),
      t('common.warning'),
      {
        confirmButtonText: t('common.confirm'),
        cancelButtonText: t('common.cancel'),
        type: 'warning',
      }
    )

    logStore.clearLogs()
    ElMessage.success(t('logs.clear_success'))
  } catch (error) {
    // 用户取消
  }
}

const scrollToBottom = () => {
  if (autoScroll.value && logsContainer.value) {
    nextTick(() => {
      if (logsContainer.value) {
        logsContainer.value.scrollTop = logsContainer.value.scrollHeight
      }
    })
  }
}

watch(() => logs.value.length, () => {
  scrollToBottom()
})
</script>

<style scoped>
.logs-panel {
  display: flex;
  flex-direction: column;
  min-height: 100%;
  padding: 24px;
  box-sizing: border-box;
  background: transparent;
}

.logs-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.header-right {
  display: flex;
  gap: 8px;
}

.logs-content {
  flex: 1;
  overflow-y: auto;
  border: 1px solid #e4e7ed;
  border-radius: 8px;
  background: #f7f8fa;
  padding: 12px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
}

.empty-icon {
  color: #dcdfe6;
  margin-bottom: 16px;
}

.empty-text {
  font-size: 14px;
  color: #909399;
}

.log-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.log-item {
  padding: 12px;
  background: white;
  border-radius: 6px;
  border-left: 3px solid;
  animation: fadeIn 0.3s ease-in-out;
}

.log-item.log-info {
  border-left-color: #909399;
}

.log-item.log-success {
  border-left-color: #67c23a;
}

.log-item.log-warning {
  border-left-color: #e6a23c;
}

.log-item.log-error {
  border-left-color: #f56c6c;
}

.log-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 6px;
}

.log-icon {
  flex-shrink: 0;
}

.log-time {
  font-size: 12px;
  color: #909399;
  font-family: 'Monaco', 'Courier New', monospace;
}

.log-message {
  font-size: 13px;
  color: #606266;
  line-height: 1.6;
  word-break: break-word;
}

.logs-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid #e4e7ed;
}

.footer-info {
  font-size: 12px;
  color: #909399;
}

.mr-1 {
  margin-right: 4px;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>

