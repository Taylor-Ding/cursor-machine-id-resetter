<template>
  <div class="reset-panel">
    <!-- 警告提示 - 更醒目的设计 -->
    <div class="warning-banner">
      <div class="warning-icon">
        <AlertCircle :size="24" />
      </div>
      <div class="warning-content">
        <h4 class="warning-title">{{ t('reset.warning_title') }}</h4>
        <p class="warning-desc">重置机器ID前，建议先关闭{{ ideName }}应用。重置完成后需要重启{{ ideName }}才能生效。同时需要使用管理员权限运行此工具，以避免权限问题导致重置失败。</p>
      </div>
    </div>

    <!-- 当前信息 - 卡片式设计 -->
    <div class="info-section">
      <div class="section-header">
        <div class="section-header-left">
          <div class="section-icon">
            <Info :size="20" />
          </div>
          <h3 class="section-title">{{ t('reset.current_info') }}</h3>
        </div>
      </div>
      
      <div class="info-cards">
        <div class="info-card gradient-blue">
          <div class="info-card-icon">
            <Cpu :size="28" />
          </div>
          <div class="info-card-content">
            <div class="info-card-label">{{ t('reset.machine_id') }}</div>
            <div class="info-card-value">{{ currentMachineId || t('common.loading') }}</div>
          </div>
        </div>

        <div class="info-card gradient-purple">
          <div class="info-card-icon">
            <Package :size="28" />
          </div>
          <div class="info-card-content">
            <div class="info-card-label">{{ ideName }} 版本</div>
            <div class="info-card-value">{{ cursorVersion || t('common.loading') }}</div>
          </div>
        </div>

        <div class="info-card gradient-green">
          <div class="info-card-icon">
            <Archive :size="28" />
          </div>
          <div class="info-card-content">
            <div class="info-card-label">{{ t('reset.backup_count') }}</div>
            <div class="info-card-value">{{ backupCount }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 重置选项 - 卡片式设计 -->
    <div class="options-section">
      <div class="section-header">
        <div class="section-header-left">
          <div class="section-icon">
            <Settings :size="20" />
          </div>
          <h3 class="section-title">{{ t('reset.options') }}</h3>
        </div>
      </div>

      <el-checkbox-group v-model="resetOptions" class="option-cards">
        <div 
          class="option-card" 
          :class="{ 'option-card-selected': resetOptions.includes('storage'), 'option-card-disabled': resetting }"
          @click="!resetting && toggleOption('storage')"
        >
          <div class="option-card-checkbox">
            <el-checkbox label="storage" :disabled="resetting" @click.stop />
          </div>
          <div class="option-card-icon">
            <Database :size="24" />
          </div>
          <div class="option-card-content">
            <div class="option-card-title">{{ t('reset.option_storage') }}</div>
          </div>
        </div>

        <div 
          class="option-card"
          :class="{ 'option-card-selected': resetOptions.includes('sqlite'), 'option-card-disabled': resetting }"
          @click="!resetting && toggleOption('sqlite')"
        >
          <div class="option-card-checkbox">
            <el-checkbox label="sqlite" :disabled="resetting" @click.stop />
          </div>
          <div class="option-card-icon">
            <HardDrive :size="24" />
          </div>
          <div class="option-card-content">
            <div class="option-card-title">{{ t('reset.option_sqlite') }}</div>
          </div>
        </div>

        <div 
          class="option-card"
          :class="{ 'option-card-selected': resetOptions.includes('system'), 'option-card-disabled': resetting }"
          @click="!resetting && toggleOption('system')"
        >
          <div class="option-card-checkbox">
            <el-checkbox label="system" :disabled="resetting" @click.stop />
          </div>
          <div class="option-card-icon">
            <Monitor :size="24" />
          </div>
          <div class="option-card-content">
            <div class="option-card-title">{{ t('reset.option_system') }}</div>
          </div>
        </div>

        <div 
          class="option-card"
          :class="{ 'option-card-selected': resetOptions.includes('patch'), 'option-card-disabled': resetting }"
          @click="!resetting && toggleOption('patch')"
        >
          <div class="option-card-checkbox">
            <el-checkbox label="patch" :disabled="resetting" @click.stop />
          </div>
          <div class="option-card-icon">
            <Zap :size="24" />
          </div>
          <div class="option-card-content">
            <div class="option-card-title">{{ t('reset.option_patch') }}</div>
          </div>
        </div>
      </el-checkbox-group>
    </div>

    <!-- 进度条 -->
    <div v-if="resetting" class="progress-section">
      <el-progress :percentage="progress" :status="progressStatus" :stroke-width="12" />
      <p class="progress-text">{{ progressText }}</p>
    </div>

    <!-- 结果显示 -->
    <transition name="fade">
      <div v-if="resetResult" class="result-section">
        <el-alert
          :title="resetResult.success ? t('reset.success_title') : t('reset.failed_title')"
          :type="resetResult.success ? 'success' : 'error'"
          :description="resetResult.message"
          show-icon
          closable
          @close="resetResult = null"
          class="result-alert"
        />
        
        <!-- 新ID值展示 -->
        <el-card v-if="resetResult.success && resetResult.newIds" class="new-ids-card" shadow="never">
          <template #header>
            <div class="card-header">
              <CheckCircle :size="18" class="header-icon" />
              <span class="header-title">新生成的机器 ID</span>
            </div>
          </template>
          
          <div class="id-list">
            <div v-for="(value, key) in resetResult.newIds" :key="key" class="id-item">
              <div class="id-label">{{ key }}</div>
              <div class="id-value">
                <code>{{ value }}</code>
                <el-button 
                  size="small" 
                  text 
                  @click="copyToClipboard(value)"
                  class="copy-btn"
                >
                  复制
                </el-button>
              </div>
            </div>
          </div>
        </el-card>
      </div>
    </transition>

    <!-- 操作按钮 -->
    <div class="action-section">
      <el-button
        type="primary"
        size="large"
        :loading="resetting"
        :disabled="resetOptions.length === 0"
        @click="handleReset"
        class="action-button action-button-primary"
      >
        <RefreshCcw :size="20" />
        <span>{{ resetting ? t('reset.resetting') : t('reset.start_reset') }}</span>
      </el-button>

      <el-button
        size="large"
        :disabled="resetting"
        @click="handleCheckStatus"
        class="action-button action-button-secondary"
      >
        <CheckCircle :size="20" />
        <span>{{ t('reset.check_status') }}</span>
      </el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { 
  RefreshCcw, Info, Settings, CheckCircle, AlertCircle,
  Cpu, Package, Archive, Database, HardDrive, Monitor, Zap
} from 'lucide-vue-next'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useAppStore } from '../stores/app'
import { useLogStore, type LogLevel } from '../stores/log'

interface Props {
  ideType?: 'cursor' | 'windsurf'
}

const props = withDefaults(defineProps<Props>(), {
  ideType: 'cursor'
})

const { t } = useI18n()
const appStore = useAppStore()
const logStore = useLogStore()

// 根据 IDE 类型动态选择命令
const commands = computed(() => ({
  getMachineInfo: props.ideType === 'windsurf' ? 'get_windsurf_info' : 'get_machine_info',
  resetMachineId: props.ideType === 'windsurf' ? 'reset_windsurf_machine_id' : 'reset_machine_id',
  checkStatus: props.ideType === 'windsurf' ? 'check_windsurf_status' : 'check_cursor_status',
  quitIDE: props.ideType === 'windsurf' ? 'quit_windsurf' : 'quit_cursor'
}))

const ideName = computed(() => props.ideType === 'windsurf' ? 'Windsurf' : 'Cursor')

const currentMachineId = ref('')
const cursorVersion = ref('')
const backupCount = ref(0)
const resetOptions = ref(['storage', 'sqlite', 'system', 'patch'])
const resetting = ref(false)
const progress = ref(0)
const progressStatus = ref<'success' | 'exception' | 'warning' | ''>('')
const progressText = ref('')
const resetResult = ref<{ success: boolean; message: string; newIds?: Record<string, string> } | null>(null)

let unlistenLogMessage: UnlistenFn | null = null

const copyToClipboard = async (text: string) => {
  try {
    await navigator.clipboard.writeText(text)
    ElMessage.success('已复制到剪贴板')
  } catch (err) {
    ElMessage.error('复制失败')
  }
}

const toggleOption = (option: string) => {
  const index = resetOptions.value.indexOf(option)
  if (index > -1) {
    resetOptions.value.splice(index, 1)
  } else {
    resetOptions.value.push(option)
  }
}

onMounted(async () => {
  await loadCurrentInfo()
  
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

const loadCurrentInfo = async () => {
  try {
    const info = await invoke(commands.value.getMachineInfo)
    currentMachineId.value = info.machine_id
    cursorVersion.value = info.cursor_version
    backupCount.value = info.backup_count
  } catch (error) {
    console.error(`Failed to load ${ideName.value} info:`, error)
    logStore.addLog('error', `${ideName.value} ${t('reset.load_info_failed')}`)
  }
}

const handleReset = async () => {
  try {
    await ElMessageBox.confirm(
      `此操作将重置${ideName.value}机器ID。是否继续？`,
      '确认重置',
      {
        confirmButtonText: t('common.confirm'),
        cancelButtonText: t('common.cancel'),
        type: 'warning',
      }
    )

    resetting.value = true
    progress.value = 0
    progressStatus.value = ''
    resetResult.value = null

    appStore.setStatus(t('reset.resetting'))
    logStore.addLog('info', t('reset.start_reset'))

    // 先关闭 IDE
    logStore.addLog('info', `正在关闭 ${ideName.value}...`)
    try {
      await invoke(commands.value.quitIDE)
      logStore.addLog('success', `${ideName.value} 已关闭`)
      progress.value = 20
    } catch (error: any) {
      logStore.addLog('warning', `关闭 ${ideName.value} 失败: ${error}`)
      
      // 询问用户是否继续
      await ElMessageBox.confirm(
        `无法自动关闭 ${ideName.value}，请手动关闭后继续。是否继续重置？`,
        '关闭失败',
        {
          confirmButtonText: t('reset.continue_reset'),
          cancelButtonText: t('common.cancel'),
          type: 'warning',
        }
      )
    }

    // 调用Rust后端
    const result = await invoke(commands.value.resetMachineId, {
      options: resetOptions.value
    }) as any

    progress.value = 100
    progressStatus.value = 'success'
    
    // 显示新的 ID 值
    let idDetails = ''
    const newIds: Record<string, string> = {}
    
    if (result.new_ids) {
      for (const [key, value] of Object.entries(result.new_ids)) {
        newIds[key] = value as string
        logStore.addLog('success', `新 ID - ${key}: ${value}`)
      }
    }
    
    resetResult.value = {
      success: true,
      message: `机器ID已成功重置，请重启${ideName.value}应用`,
      newIds: Object.keys(newIds).length > 0 ? newIds : undefined
    }

    logStore.addLog('success', `${ideName.value}机器ID已成功重置`)
    ElMessage.success(`${ideName.value}机器ID已成功重置，请重启应用`)

    // 重新加载信息
    await loadCurrentInfo()

  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('Reset failed:', error)
      progressStatus.value = 'exception'
      
      // 检测是否是权限问题
      const errorMessage = error.message || error.toString() || ''
      const isPermissionError = 
        errorMessage.toLowerCase().includes('permission') ||
        errorMessage.toLowerCase().includes('access') ||
        errorMessage.toLowerCase().includes('denied') ||
        errorMessage.toLowerCase().includes('权限') ||
        errorMessage.toLowerCase().includes('拒绝')
      
      let displayMessage = error.message || t('reset.failed_message')
      
      if (isPermissionError) {
        displayMessage = t('reset.permission_error')
        logStore.addLog('error', t('reset.permission_error'))
        
        // 显示详细的权限错误提示
        ElMessageBox.alert(
          t('reset.permission_error_detail'),
          t('reset.permission_error_title'),
          {
            confirmButtonText: t('common.ok'),
            type: 'error',
            dangerouslyUseHTMLString: true
          }
        )
      } else {
        logStore.addLog('error', displayMessage)
        ElMessage.error(t('reset.failed_message'))
      }
      
      resetResult.value = {
        success: false,
        message: displayMessage
      }
    }
  } finally {
    resetting.value = false
    appStore.setStatus(t('status.ready'))
  }
}

const handleCheckStatus = async () => {
  try {
    const status = await invoke(commands.value.checkStatus) as any
    ElMessageBox.alert(
      status.message,
      `${ideName.value} ${t('reset.status_title')}`,
      {
        confirmButtonText: t('common.ok'),
        type: status.is_running ? 'warning' : 'info'
      }
    )
  } catch (error) {
    console.error(`Check ${ideName.value} status failed:`, error)
    ElMessage.error(t('reset.check_status_failed'))
  }
}
</script>

<style scoped>
.reset-panel {
  padding: 24px;
  min-height: 100%;
  box-sizing: border-box;
  background: transparent;
}

/* 警告横幅 */
.warning-banner {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  padding: 20px;
  background: linear-gradient(135deg, #fff3e0 0%, #ffe0b2 100%);
  border-radius: 12px;
  border-left: 4px solid #ff9800;
  margin-bottom: 32px;
  box-shadow: 0 2px 8px rgba(255, 152, 0, 0.1);
  animation: slideDown 0.5s ease-out;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.warning-icon {
  color: #f57c00;
  flex-shrink: 0;
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.1);
  }
}

.warning-content {
  flex: 1;
}

.warning-title {
  font-size: 16px;
  font-weight: 700;
  color: #e65100;
  margin: 0 0 8px 0;
}

.warning-desc {
  font-size: 14px;
  color: #ef6c00;
  margin: 0;
  line-height: 1.6;
}

/* 区块样式 */
.info-section,
.options-section {
  margin-bottom: 32px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.section-header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.section-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 10px;
  color: white;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.section-title {
  font-size: 18px;
  font-weight: 700;
  color: #2c3e50;
  margin: 0;
}

/* 信息卡片 */
.info-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 20px;
}

.info-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 24px;
  border-radius: 16px;
  background: white;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
}

.info-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  opacity: 0.05;
  z-index: 0;
}

.info-card.gradient-blue::before {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.info-card.gradient-purple::before {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}

.info-card.gradient-green::before {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
}

.info-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
}

.info-card-icon {
  flex-shrink: 0;
  width: 56px;
  height: 56px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.9);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  position: relative;
  z-index: 1;
}

.gradient-blue .info-card-icon {
  color: #667eea;
}

.gradient-purple .info-card-icon {
  color: #f5576c;
}

.gradient-green .info-card-icon {
  color: #00f2fe;
}

.info-card-content {
  flex: 1;
  min-width: 0;
  position: relative;
  z-index: 1;
}

.info-card-label {
  font-size: 13px;
  color: #7f8c8d;
  margin-bottom: 6px;
  font-weight: 500;
}

.info-card-value {
  font-size: 15px;
  font-weight: 700;
  color: #2c3e50;
  word-break: break-all;
  line-height: 1.4;
}

/* 选项卡片 */
.option-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 16px;
}

.option-card {
  position: relative;
  padding: 20px;
  background: white;
  border: 2px solid #e8eaed;
  border-radius: 18px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
}

.option-card:hover:not(.option-card-disabled) {
  border-color: #667eea;
  box-shadow: 0 4px 16px rgba(102, 126, 234, 0.2);
  transform: translateY(-2px);
}

.option-card-selected {
  border-color: #667eea;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.05) 0%, rgba(118, 75, 162, 0.05) 100%);
  box-shadow: 0 4px 16px rgba(102, 126, 234, 0.15);
}

.option-card-disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.option-card-checkbox {
  position: absolute;
  top: 12px;
  right: 12px;
}

.option-card-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  margin-bottom: 12px;
  border-radius: 14px;
  background: linear-gradient(135deg, #f5f7fa 0%, #e8eaed 100%);
  color: #667eea;
  transition: all 0.3s;
}

.option-card-selected .option-card-icon {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.option-card-content {
  text-align: left;
}

.option-card-title {
  font-size: 15px;
  font-weight: 600;
  color: #2c3e50;
}

/* 进度条 */
.progress-section {
  margin: 32px 0;
  padding: 24px;
  background: linear-gradient(135deg, #f5f7fa 0%, #e8eaed 100%);
  border-radius: 12px;
  animation: fadeIn 0.3s ease-in;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.progress-text {
  text-align: center;
  margin-top: 12px;
  font-size: 14px;
  color: #5a6c7d;
  font-weight: 500;
}

/* 结果显示区域 */
.result-section {
  margin-bottom: 24px;
}

.result-alert {
  margin-bottom: 16px;
}

/* 新ID卡片 */
.new-ids-card {
  border: 2px solid #67c23a;
  border-radius: 12px;
  overflow: hidden;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
  color: #67c23a;
}

.header-icon {
  color: #67c23a;
}

.header-title {
  font-size: 16px;
}

.id-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.id-item {
  padding: 12px;
  background: #f5f7fa;
  border-radius: 8px;
  transition: all 0.2s;
}

.id-item:hover {
  background: #e8eaed;
}

.id-label {
  font-size: 13px;
  color: #606266;
  font-weight: 600;
  margin-bottom: 6px;
}

.id-value {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.id-value code {
  flex: 1;
  padding: 8px 12px;
  background: white;
  border: 1px solid #dcdfe6;
  border-radius: 6px;
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 12px;
  color: #303133;
  word-break: break-all;
  line-height: 1.5;
}

.copy-btn {
  flex-shrink: 0;
  color: #409eff;
  font-size: 13px;
}

.copy-btn:hover {
  color: #66b1ff;
}

.fade-enter-active,
.fade-leave-active {
  transition: all 0.3s ease;
}

.fade-enter-from {
  opacity: 0;
  transform: translateY(-10px);
}

.fade-leave-to {
  opacity: 0;
  transform: translateY(10px);
}

/* 操作按钮 */
.action-section {
  display: flex;
  gap: 16px;
  margin-top: 32px;
  padding-top: 32px;
  border-top: 2px solid #f0f2f5;
}

.action-button {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 16px 32px;
  font-size: 16px;
  font-weight: 600;
  border-radius: 12px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.action-button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.15);
}

.action-button:active:not(:disabled) {
  transform: translateY(0);
}

.action-button-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  color: white;
}

.action-button-primary:hover:not(:disabled) {
  background: linear-gradient(135deg, #5568d3 0%, #6a3f8f 100%);
}

.action-button-secondary {
  background: white;
  border: 2px solid #667eea;
  color: #667eea;
}

.action-button-secondary:hover:not(:disabled) {
  background: rgba(102, 126, 234, 0.05);
  border-color: #5568d3;
  color: #5568d3;
}

/* Element Plus 样式覆盖 */
:deep(.el-progress__text) {
  font-weight: 600;
  color: #667eea !important;
}

:deep(.el-progress-bar__inner) {
  background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
}

:deep(.el-checkbox__inner) {
  border-radius: 4px;
}

:deep(.el-checkbox__input.is-checked .el-checkbox__inner) {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-color: #667eea;
}
</style>

