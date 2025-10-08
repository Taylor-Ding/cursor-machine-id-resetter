<template>
  <div class="restore-panel">
    <el-alert
      :title="t('restore.info_title')"
      type="info"
      :description="t('restore.info_desc')"
      show-icon
      :closable="false"
      class="mb-4"
    />

    <!-- 备份列表 -->
    <div class="backup-list-section">
      <div class="section-header">
        <h3 class="section-title">
          <Database :size="18" />
          <span>{{ t('restore.backup_list') }}</span>
        </h3>
        <el-button size="small" @click="loadBackups" :loading="loading">
          <RefreshCw :size="16" class="mr-1" />
          {{ t('common.refresh') }}
        </el-button>
      </div>

      <div v-if="loading" class="loading-state">
        <el-skeleton :rows="5" animated />
      </div>

      <div v-else-if="backups.length === 0" class="empty-state">
        <Inbox :size="64" class="empty-icon" />
        <p class="empty-text">{{ t('restore.no_backups') }}</p>
      </div>

      <el-table
        v-else
        :data="backups"
        stripe
        highlight-current-row
        @current-change="handleSelectionChange"
        class="backup-table"
      >
        <el-table-column type="index" width="60" :label="t('common.index')" />
        
        <el-table-column prop="timestamp" :label="t('restore.backup_time')" width="200">
          <template #default="{ row }">
            <div class="time-cell">
              <Clock :size="14" class="time-icon" />
              <span>{{ formatTime(row.timestamp) }}</span>
            </div>
          </template>
        </el-table-column>

        <el-table-column prop="machine_id" :label="t('restore.machine_id')" min-width="200">
          <template #default="{ row }">
            <el-tooltip :content="row.machine_id" placement="top">
              <span class="machine-id-text">{{ truncateId(row.machine_id) }}</span>
            </el-tooltip>
          </template>
        </el-table-column>

        <el-table-column prop="size" :label="t('restore.size')" width="120">
          <template #default="{ row }">
            {{ formatSize(row.size) }}
          </template>
        </el-table-column>

        <el-table-column :label="t('common.actions')" width="200" fixed="right">
          <template #default="{ row }">
            <el-button size="small" type="primary" @click="handleRestore(row)">
              <Upload :size="14" class="mr-1" />
              {{ t('restore.restore') }}
            </el-button>
            <el-button size="small" type="danger" @click="handleDelete(row)">
              <Trash2 :size="14" />
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- 选中的备份详情 -->
    <el-card v-if="selectedBackup" class="backup-detail" shadow="never">
      <template #header>
        <div class="card-header">
          <span>{{ t('restore.backup_detail') }}</span>
        </div>
      </template>
      
      <div class="detail-content">
        <div class="detail-item">
          <span class="detail-label">{{ t('restore.backup_time') }}:</span>
          <span class="detail-value">{{ formatTime(selectedBackup.timestamp) }}</span>
        </div>
        <div class="detail-item">
          <span class="detail-label">{{ t('restore.machine_id') }}:</span>
          <span class="detail-value">{{ selectedBackup.machine_id }}</span>
        </div>
        <div class="detail-item">
          <span class="detail-label">{{ t('restore.size') }}:</span>
          <span class="detail-value">{{ formatSize(selectedBackup.size) }}</span>
        </div>
        <div class="detail-item">
          <span class="detail-label">{{ t('restore.path') }}:</span>
          <span class="detail-value">{{ selectedBackup.path }}</span>
        </div>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { Database, RefreshCw, Clock, Upload, Trash2, Inbox } from 'lucide-vue-next'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useLogStore, type LogLevel } from '../stores/log'

const { t } = useI18n()
const logStore = useLogStore()

interface Backup {
  id: string
  timestamp: number
  machine_id: string
  size: number
  path: string
}

const backups = ref<Backup[]>([])
const selectedBackup = ref<Backup | null>(null)
const loading = ref(false)

let unlistenLogMessage: UnlistenFn | null = null

onMounted(async () => {
  await loadBackups()
  
  // 监听后端日志事件
  unlistenLogMessage = await listen<{ level: string; message: string; timestamp: number }>('log-message', (event) => {
    const { level, message } = event.payload
    logStore.addLog(level as LogLevel, message)
  })
})

onUnmounted(() => {
  // 清理事件监听
  if (unlistenLogMessage) {
    unlistenLogMessage()
  }
})

const loadBackups = async () => {
  loading.value = true
  try {
    const result = await invoke('get_backups')
    backups.value = result as Backup[]
    logStore.addLog('info', t('restore.backups_loaded', { count: backups.value.length }))
  } catch (error) {
    console.error('Failed to load backups:', error)
    ElMessage.error(t('restore.load_failed'))
    logStore.addLog('error', t('restore.load_failed'))
  } finally {
    loading.value = false
  }
}

const handleSelectionChange = (backup: Backup | null) => {
  selectedBackup.value = backup
}

const handleRestore = async (backup: Backup) => {
  try {
    await ElMessageBox.confirm(
      t('restore.confirm_message', { time: formatTime(backup.timestamp) }),
      t('restore.confirm_title'),
      {
        confirmButtonText: t('common.confirm'),
        cancelButtonText: t('common.cancel'),
        type: 'warning',
      }
    )

    const result = await invoke('restore_from_backup', {
      backupId: backup.id
    })

    ElMessage.success(t('restore.success'))
    logStore.addLog('success', t('restore.success'))

  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('Restore failed:', error)
      ElMessage.error(t('restore.failed'))
      logStore.addLog('error', t('restore.failed'))
    }
  }
}

const handleDelete = async (backup: Backup) => {
  try {
    await ElMessageBox.confirm(
      t('restore.delete_confirm'),
      t('common.warning'),
      {
        confirmButtonText: t('common.confirm'),
        cancelButtonText: t('common.cancel'),
        type: 'warning',
      }
    )

    await invoke('delete_backup', {
      backupId: backup.id
    })

    ElMessage.success(t('restore.delete_success'))
    logStore.addLog('info', t('restore.delete_success'))
    await loadBackups()

  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('Delete failed:', error)
      ElMessage.error(t('restore.delete_failed'))
    }
  }
}

const formatTime = (timestamp: number): string => {
  return new Date(timestamp).toLocaleString()
}

const formatSize = (bytes: number): string => {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(2) + ' MB'
}

const truncateId = (id: string): string => {
  if (id.length <= 16) return id
  return id.substring(0, 8) + '...' + id.substring(id.length - 8)
}
</script>

<style scoped>
.restore-panel {
  padding: 24px;
  min-height: 100%;
  box-sizing: border-box;
  background: transparent;
}

.backup-list-section {
  margin-bottom: 20px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
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

.loading-state,
.empty-state {
  padding: 40px;
  text-align: center;
}

.empty-icon {
  color: #dcdfe6;
  margin-bottom: 16px;
}

.empty-text {
  font-size: 14px;
  color: #909399;
}

.backup-table {
  width: 100%;
}

.time-cell {
  display: flex;
  align-items: center;
  gap: 6px;
}

.time-icon {
  color: #909399;
}

.machine-id-text {
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  color: #606266;
}

.backup-detail {
  margin-top: 20px;
}

.detail-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.detail-item {
  display: flex;
  padding: 8px 0;
  border-bottom: 1px solid #f0f0f0;
}

.detail-item:last-child {
  border-bottom: none;
}

.detail-label {
  flex: 0 0 140px;
  font-weight: 600;
  color: #606266;
}

.detail-value {
  flex: 1;
  color: #303133;
  word-break: break-all;
}

.mr-1 {
  margin-right: 4px;
}

.mb-4 {
  margin-bottom: 16px;
}
</style>

