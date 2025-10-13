<template>
  <div class="qoder-panel">
    <div class="panel-container">
      
      <!-- 警告提示 -->
      <div class="warning-card">
        <div class="warning-icon">
          <AlertTriangle :size="24" />
        </div>
        <div class="warning-content">
          <h3>{{ t('qoder.warning_title') }}</h3>
          <p>{{ t('qoder.warning_desc') }}</p>
        </div>
      </div>

      <!-- Qoder 信息卡片 -->
      <div class="info-card gradient-card">
        <div class="card-header">
          <div class="header-left">
            <Info :size="20" />
            <h3>{{ t('qoder.current_info') }}</h3>
          </div>
          <el-button
            type="primary"
            size="small"
            :icon="RefreshCw"
            @click="loadQoderInfo"
            :loading="loading"
          >
            {{ t('common.refresh') }}
          </el-button>
        </div>
        <div class="card-body">
          <div class="info-grid">
            <div class="info-item">
              <span class="label">{{ t('qoder.machine_id') }}:</span>
              <span class="value">{{ qoderInfo.machine_id || 'N/A' }}</span>
            </div>
            <div class="info-item">
              <span class="label">{{ t('qoder.status') }}:</span>
              <span 
                :class="['status-badge', qoderInfo.is_running ? 'running' : 'stopped']"
              >
                <Circle :size="8" :fill="qoderInfo.is_running ? '#52c41a' : '#8c8c8c'" />
                {{ qoderInfo.is_running ? t('qoder.running') : t('qoder.stopped') }}
              </span>
            </div>
            <div class="info-item">
              <span class="label">{{ t('qoder.installed') }}:</span>
              <span class="value">{{ qoderInfo.is_installed ? t('common.yes') : t('common.no') }}</span>
            </div>
            <div class="info-item">
              <span class="label">{{ t('qoder.process_count') }}:</span>
              <span class="value">{{ qoderInfo.process_count }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 操作模式选择 -->
      <div class="mode-selector">
        <div 
          v-for="mode in modes" 
          :key="mode.value"
          :class="['mode-card', { active: selectedMode === mode.value }]"
          @click="selectedMode = mode.value"
        >
          <component :is="mode.icon" :size="32" />
          <h4>{{ t(mode.title) }}</h4>
          <p>{{ t(mode.desc) }}</p>
          <div v-if="mode.badge" class="mode-badge">{{ t(mode.badge) }}</div>
        </div>
      </div>

      <!-- 完整重置选项 -->
      <div v-if="selectedMode === 'full'" class="options-card">
        <h3>{{ t('qoder.reset_options') }}</h3>
        <div class="options-info">
          <ul>
            <li>✅ {{ t('qoder.option_machine_id') }}</li>
            <li>✅ {{ t('qoder.option_telemetry') }}</li>
            <li>✅ {{ t('qoder.option_cache') }}</li>
            <li>✅ {{ t('qoder.option_identity') }}</li>
            <li>✅ {{ t('qoder.option_hardware') }}</li>
            <li>💾 {{ t('qoder.option_preserve') }}</li>
          </ul>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="action-buttons">
        <el-button
          v-if="qoderInfo.is_running"
          type="warning"
          size="large"
          :icon="PowerOff"
          @click="handleQuitQoder"
          :loading="quitting"
        >
          {{ t('qoder.quit_qoder') }}
        </el-button>

        <el-button
          type="primary"
          size="large"
          :icon="getActionIcon()"
          @click="handleReset"
          :loading="resetting"
          :disabled="!canReset()"
        >
          {{ getActionText() }}
        </el-button>
      </div>

      <!-- 统计信息（重置后显示） -->
      <div v-if="resetStats" class="stats-card">
        <h3>{{ t('qoder.reset_stats') }}</h3>
        <div class="stats-grid">
          <div class="stat-item">
            <FileX :size="20" />
            <span class="stat-value">{{ resetStats.files_removed }}</span>
            <span class="stat-label">{{ t('qoder.files_removed') }}</span>
          </div>
          <div class="stat-item">
            <FolderX :size="20" />
            <span class="stat-value">{{ resetStats.dirs_removed }}</span>
            <span class="stat-label">{{ t('qoder.dirs_removed') }}</span>
          </div>
          <div class="stat-item">
            <Database :size="20" />
            <span class="stat-value">{{ resetStats.telemetry_keys_cleared }}</span>
            <span class="stat-label">{{ t('qoder.keys_cleared') }}</span>
          </div>
        </div>
        <div v-if="resetStats.errors && resetStats.errors.length > 0" class="errors-list">
          <h4>{{ t('qoder.errors_encountered') }}:</h4>
          <ul>
            <li v-for="(error, index) in resetStats.errors" :key="index">{{ error }}</li>
          </ul>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { ElMessage, ElMessageBox } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import { 
  AlertTriangle, Info, RefreshCw, Circle, PowerOff,
  Trash2, Zap, Settings, FileX, FolderX, Database
} from 'lucide-vue-next'

const { t } = useI18n()

// 状态管理
const loading = ref(false)
const quitting = ref(false)
const resetting = ref(false)
const selectedMode = ref('full')

const qoderInfo = ref({
  machine_id: '',
  is_running: false,
  is_installed: false,
  process_count: 0
})

const resetStats = ref<any>(null)

// 模式配置
const modes = [
  {
    value: 'full',
    icon: Trash2,
    title: 'qoder.mode_full',
    desc: 'qoder.mode_full_desc',
    badge: null
  },
  {
    value: 'simple',
    icon: Settings,
    title: 'qoder.mode_simple',
    desc: 'qoder.mode_simple_desc',
    badge: null
  }
]

// 加载 Qoder 信息
const loadQoderInfo = async () => {
  loading.value = true
  try {
    const info = await invoke('get_qoder_info')
    qoderInfo.value = info as any
  } catch (error: any) {
    ElMessage.error(t('qoder.load_info_failed') + ': ' + error)
  } finally {
    loading.value = false
  }
}

// 关闭 Qoder
const handleQuitQoder = async () => {
  try {
    await ElMessageBox.confirm(
      t('qoder.quit_confirm_message'),
      t('qoder.quit_confirm_title'),
      {
        confirmButtonText: t('common.confirm'),
        cancelButtonText: t('common.cancel'),
        type: 'warning'
      }
    )
    
    quitting.value = true
    const success = await invoke('quit_qoder')
    
    if (success) {
      ElMessage.success(t('qoder.quit_success'))
      await loadQoderInfo()
    } else {
      ElMessage.warning(t('qoder.quit_failed'))
    }
  } catch (error: any) {
    if (error !== 'cancel') {
      ElMessage.error(t('qoder.quit_error') + ': ' + error)
    }
  } finally {
    quitting.value = false
  }
}

// 执行重置
const handleReset = async () => {
  try {
    await ElMessageBox.confirm(
      t('qoder.reset_confirm_message'),
      t('qoder.reset_confirm_title'),
      {
        confirmButtonText: t('common.confirm'),
        cancelButtonText: t('common.cancel'),
        type: 'warning',
        dangerouslyUseHTMLString: true
      }
    )
    
    resetting.value = true
    resetStats.value = null
    
    let result: any
    
    if (selectedMode.value === 'full') {
      result = await invoke('reset_qoder_full')
    } else if (selectedMode.value === 'simple') {
      result = await invoke('reset_qoder_machine_id')
    }
    
    if (result.success) {
      ElMessage.success({
        message: t('qoder.reset_success'),
        duration: 5000
      })
      
      if (result.stats) {
        resetStats.value = result.stats
      }
      
      await loadQoderInfo()
    } else {
      ElMessage.error(result.error || t('qoder.reset_failed'))
    }
  } catch (error: any) {
    if (error !== 'cancel') {
      ElMessage.error(t('qoder.reset_error') + ': ' + error)
    }
  } finally {
    resetting.value = false
  }
}

// 判断是否可以执行重置
const canReset = () => {
  if (!qoderInfo.value.is_installed) return false
  return !qoderInfo.value.is_running
}

// 获取操作图标
const getActionIcon = () => {
  return Trash2
}

// 获取操作文本
const getActionText = () => {
  if (selectedMode.value === 'full') {
    return t('qoder.reset_full')
  } else {
    return t('qoder.reset_simple')
  }
}

onMounted(() => {
  loadQoderInfo()
})
</script>

<style scoped>
.qoder-panel {
  padding: 24px;
  height: 100%;
  overflow-y: auto;
}

.panel-container {
  max-width: 900px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.warning-card {
  display: flex;
  gap: 16px;
  padding: 16px;
  background: linear-gradient(135deg, #fff3cd 0%, #ffe69c 100%);
  border: 1px solid #ffc107;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(255, 193, 7, 0.2);
}

.warning-icon {
  color: #ff9800;
  flex-shrink: 0;
}

.warning-content h3 {
  margin: 0 0 8px 0;
  font-size: 16px;
  color: #f57c00;
}

.warning-content p {
  margin: 0;
  font-size: 14px;
  color: #e65100;
  line-height: 1.6;
}

.gradient-card {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 16px;
  padding: 24px;
  color: white;
  box-shadow: 0 8px 24px rgba(102, 126, 234, 0.3);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.header-left h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
}

.card-body {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border-radius: 12px;
  padding: 16px;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.info-item .label {
  font-size: 13px;
  opacity: 0.9;
}

.info-item .value {
  font-size: 15px;
  font-weight: 600;
  font-family: 'Monaco', 'Courier New', monospace;
}

.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 13px;
  font-weight: 500;
}

.status-badge.running {
  background: rgba(82, 196, 26, 0.2);
  color: #52c41a;
}

.status-badge.stopped {
  background: rgba(140, 140, 140, 0.2);
  color: #8c8c8c;
}

.mode-selector {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.mode-card {
  position: relative;
  padding: 20px;
  background: white;
  border: 2px solid #e8eaed;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s;
  text-align: center;
}

.mode-card:hover {
  border-color: #667eea;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.2);
}

.mode-card.active {
  border-color: #667eea;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.mode-card h4 {
  margin: 12px 0 8px;
  font-size: 16px;
  color: #333;
}

.mode-card p {
  margin: 0;
  font-size: 13px;
  color: #666;
  line-height: 1.5;
}

.mode-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  padding: 2px 8px;
  background: linear-gradient(135deg, #ff6b6b 0%, #ff8e53 100%);
  color: white;
  font-size: 11px;
  font-weight: 600;
  border-radius: 8px;
}

.options-card {
  padding: 20px;
  background: white;
  border-radius: 12px;
  border: 1px solid #e8eaed;
}

.options-card h3 {
  margin: 0 0 16px;
  font-size: 16px;
  color: #333;
}

.options-info ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.options-info li {
  padding: 8px 0;
  color: #666;
  font-size: 14px;
}

.action-buttons {
  display: flex;
  gap: 12px;
  justify-content: center;
}

.action-buttons .el-button {
  min-width: 160px;
}

.stats-card {
  padding: 20px;
  background: white;
  border-radius: 12px;
  border: 1px solid #e8eaed;
}

.stats-card h3 {
  margin: 0 0 16px;
  font-size: 16px;
  color: #333;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-bottom: 16px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px;
  background: #f5f7fa;
  border-radius: 8px;
}

.stat-item .stat-value {
  font-size: 24px;
  font-weight: 700;
  color: #667eea;
}

.stat-item .stat-label {
  font-size: 13px;
  color: #666;
}

.errors-list {
  padding: 16px;
  background: #fff3cd;
  border-radius: 8px;
  border: 1px solid #ffc107;
}

.errors-list h4 {
  margin: 0 0 8px;
  font-size: 14px;
  color: #f57c00;
}

.errors-list ul {
  margin: 0;
  padding-left: 20px;
}

.errors-list li {
  font-size: 13px;
  color: #e65100;
  line-height: 1.6;
}

/* 滚动条美化 */
.qoder-panel::-webkit-scrollbar {
  width: 8px;
}

.qoder-panel::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

.qoder-panel::-webkit-scrollbar-thumb {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 4px;
}

.qoder-panel::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(135deg, #5568d3 0%, #6a3f8f 100%);
}
</style>
