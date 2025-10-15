<template>
  <div class="email-generator-panel">
    <!-- 说明信息 -->
    <div class="info-banner">
      <div class="info-icon">
        <Info :size="24" />
      </div>
      <div class="info-content">
        <h4 class="info-title">{{ t('email.info_title') }}</h4>
        <p class="info-desc">{{ t('email.info_desc') }}</p>
      </div>
    </div>

    <!-- 配置区域 -->
    <div class="config-section">
      <div class="section-header">
        <div class="section-header-left">
          <div class="section-icon">
            <Settings :size="20" />
          </div>
          <h3 class="section-title">{{ t('email.config_title') }}</h3>
        </div>
      </div>

      <div class="config-form">
        <el-form :model="form" label-width="120px">
          <el-form-item :label="t('email.domain')" required>
            <el-input
              v-model="form.domain"
              :placeholder="t('email.domain_placeholder')"
              clearable
            >
              <template #prepend>@</template>
            </el-input>
          </el-form-item>

          <el-form-item :label="t('email.password_length')">
            <el-slider
              v-model="form.passwordLength"
              :min="8"
              :max="32"
              :step="1"
              show-stops
              show-input
            />
          </el-form-item>

          <el-form-item :label="t('email.timestamp_length')">
            <el-slider
              v-model="form.timestampLength"
              :min="0"
              :max="8"
              :step="1"
              show-stops
              show-input
            />
            <div class="form-hint">{{ t('email.timestamp_hint') }}</div>
          </el-form-item>
        </el-form>
      </div>
    </div>

    <!-- 生成的账号信息 -->
    <transition name="fade">
      <div v-if="accountInfo" class="result-section">
        <el-card class="account-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <UserCheck :size="18" class="header-icon" />
              <span class="header-title">{{ t('email.account_info') }}</span>
            </div>
          </template>

          <div class="account-info-grid">
            <div class="info-item">
              <div class="info-label">
                <Mail :size="16" />
                <span>{{ t('email.email') }}</span>
              </div>
              <div class="info-value">
                <code>{{ accountInfo.email }}</code>
                <el-button size="small" text @click="copyToClipboard(accountInfo.email)">
                  <Copy :size="14" />
                </el-button>
              </div>
            </div>

            <div class="info-item">
              <div class="info-label">
                <Key :size="16" />
                <span>{{ t('email.password') }}</span>
              </div>
              <div class="info-value">
                <code :class="{ 'blur-text': !showPassword }">{{ accountInfo.password }}</code>
                <el-button size="small" text @click="showPassword = !showPassword">
                  <Eye v-if="!showPassword" :size="14" />
                  <EyeOff v-else :size="14" />
                </el-button>
                <el-button size="small" text @click="copyToClipboard(accountInfo.password)">
                  <Copy :size="14" />
                </el-button>
              </div>
            </div>

            <div class="info-item">
              <div class="info-label">
                <User :size="16" />
                <span>{{ t('email.first_name') }}</span>
              </div>
              <div class="info-value">
                <code>{{ accountInfo.first_name }}</code>
                <el-button size="small" text @click="copyToClipboard(accountInfo.first_name)">
                  <Copy :size="14" />
                </el-button>
              </div>
            </div>

            <div class="info-item">
              <div class="info-label">
                <User :size="16" />
                <span>{{ t('email.last_name') }}</span>
              </div>
              <div class="info-value">
                <code>{{ accountInfo.last_name }}</code>
                <el-button size="small" text @click="copyToClipboard(accountInfo.last_name)">
                  <Copy :size="14" />
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
        :loading="generating"
        :disabled="!form.domain"
        @click="handleGenerate"
        class="action-button action-button-primary"
      >
        <Sparkles :size="20" />
        <span>{{ generating ? t('email.generating') : t('email.generate') }}</span>
      </el-button>
    </div>

    <!-- 历史记录区域 -->
    <div v-if="history.length > 0" class="history-section">
      <div class="section-header">
        <div class="section-header-left">
          <div class="section-icon section-icon-history">
            <History :size="20" />
          </div>
          <h3 class="section-title">{{ t('email.history_title') }}</h3>
          <span class="history-count">{{ history.length }}</span>
        </div>
        <el-button
          size="small"
          type="danger"
          text
          @click="handleClearHistory"
          :disabled="history.length === 0"
        >
          <Trash2 :size="16" />
          <span>{{ t('email.clear_history') }}</span>
        </el-button>
      </div>

      <div class="history-list">
        <transition-group name="list">
          <el-card
            v-for="item in history"
            :key="item.id"
            class="history-card"
            :class="{ 'is-expanded': item.expanded }"
            shadow="hover"
          >
            <div class="history-item">
              <div class="history-header" @click="toggleHistoryItem(item.id)">
                <div class="history-summary">
                  <div class="history-time">
                    <Clock :size="14" />
                    <span>{{ formatDate(item.created_at) }}</span>
                  </div>
                  <div class="history-email-preview">
                    <Mail :size="14" />
                    <code>{{ item.account_info.email }}</code>
                  </div>
                </div>
                <div class="history-actions" @click.stop>
                  <el-button
                    size="small"
                    text
                    @click="toggleHistoryItem(item.id)"
                    class="expand-btn"
                  >
                    <ChevronDown v-if="!item.expanded" :size="16" />
                    <ChevronUp v-else :size="16" />
                  </el-button>
                  <el-button
                    size="small"
                    type="danger"
                    text
                    @click="handleDeleteHistory(item.id)"
                  >
                    <X :size="16" />
                  </el-button>
                </div>
              </div>

              <transition name="expand">
                <div v-if="item.expanded" class="history-content">
                  <div class="history-info-item">
                    <div class="history-label">
                      <Mail :size="14" />
                      <span>{{ t('email.email') }}</span>
                    </div>
                    <div class="history-value">
                      <code>{{ item.account_info.email }}</code>
                      <el-button size="small" text @click="copyToClipboard(item.account_info.email)">
                        <Copy :size="14" />
                      </el-button>
                    </div>
                  </div>

                  <div class="history-info-item">
                    <div class="history-label">
                      <Key :size="14" />
                      <span>{{ t('email.password') }}</span>
                    </div>
                    <div class="history-value">
                      <code>{{ item.account_info.password }}</code>
                      <el-button size="small" text @click="copyToClipboard(item.account_info.password)">
                        <Copy :size="14" />
                      </el-button>
                    </div>
                  </div>

                  <div class="history-info-item">
                    <div class="history-label">
                      <User :size="14" />
                      <span>{{ t('email.name') }}</span>
                    </div>
                    <div class="history-value">
                      <code>{{ item.account_info.first_name }} {{ item.account_info.last_name }}</code>
                      <el-button size="small" text @click="copyToClipboard(item.account_info.first_name)">
                        <Copy :size="14" />
                      </el-button>
                    </div>
                  </div>
                </div>
              </transition>
            </div>
          </el-card>
        </transition-group>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import {
  Info, Settings, Mail, Key, User, Copy, Eye, EyeOff, Sparkles, UserCheck,
  History, Trash2, Clock, X, ChevronDown, ChevronUp
} from 'lucide-vue-next'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useLogStore } from '../stores/log'

const { t } = useI18n()
const logStore = useLogStore()

interface AccountInfo {
  email: string
  password: string
  first_name: string
  last_name: string
}

interface HistoryItem {
  id: string
  account_info: AccountInfo
  created_at: string
  domain: string
  expanded?: boolean
}

const form = ref({
  domain: '',
  passwordLength: 16,
  timestampLength: 4,
})

const accountInfo = ref<AccountInfo | null>(null)
const generating = ref(false)
const showPassword = ref(false)
const history = ref<HistoryItem[]>([])

onMounted(async () => {
  // 从设置中加载域名配置
  try {
    const settings = await invoke('get_settings') as any
    if (settings && settings.emailDomain) {
      form.value.domain = settings.emailDomain
      logStore.addLog('info', `已加载域名配置: ${settings.emailDomain}`)
    }
  } catch (error) {
    console.error('Failed to load settings:', error)
  }

  // 加载历史记录
  await loadHistory()
})

const loadHistory = async () => {
  try {
    const items = await invoke('get_account_history') as HistoryItem[]
    // 默认所有记录都是折叠状态
    history.value = items.map(item => ({ ...item, expanded: false }))
    logStore.addLog('info', `已加载 ${items.length} 条历史记录`)
  } catch (error: any) {
    console.error('Failed to load history:', error)
    logStore.addLog('error', `加载历史记录失败: ${error}`)
  }
}

const toggleHistoryItem = (id: string) => {
  const item = history.value.find(h => h.id === id)
  if (item) {
    item.expanded = !item.expanded
  }
}

const copyToClipboard = async (text: string) => {
  try {
    await navigator.clipboard.writeText(text)
    ElMessage.success(t('common.copy_success'))
  } catch (err) {
    ElMessage.error(t('common.copy_failed'))
  }
}

const handleGenerate = async () => {
  if (!form.value.domain) {
    ElMessage.warning(t('email.domain_required'))
    return
  }

  generating.value = true
  logStore.addLog('info', t('email.generating'))

  try {
    const result = await invoke('generate_account', {
      domain: form.value.domain,
      passwordLength: form.value.passwordLength,
      timestampLength: form.value.timestampLength,
    }) as AccountInfo

    accountInfo.value = result
    showPassword.value = false

    logStore.addLog('success', `${t('email.generated_email')}: ${result.email}`)
    ElMessage.success(t('email.generate_success'))

    // 重新加载历史记录
    await loadHistory()

    // 保存域名到设置，下次自动加载
    try {
      const settings = await invoke('get_settings') as any
      settings.emailDomain = form.value.domain
      await invoke('save_settings', { settings })
      logStore.addLog('info', `已保存域名配置: ${form.value.domain}`)
    } catch (saveError) {
      console.error('Failed to save domain:', saveError)
      // 不影响主流程，不抛出错误
    }
  } catch (error: any) {
    console.error('Generate account failed:', error)
    logStore.addLog('error', `${t('email.generate_failed')}: ${error}`)
    ElMessage.error(t('email.generate_failed'))
  } finally {
    generating.value = false
  }
}

const handleDeleteHistory = async (id: string) => {
  try {
    await ElMessageBox.confirm(
      t('email.delete_history_confirm'),
      t('common.warning'),
      {
        confirmButtonText: t('common.confirm'),
        cancelButtonText: t('common.cancel'),
        type: 'warning',
      }
    )

    await invoke('delete_account_history', { id })
    ElMessage.success(t('email.delete_history_success'))
    await loadHistory()
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('Failed to delete history:', error)
      ElMessage.error(t('email.delete_history_failed'))
    }
  }
}

const handleClearHistory = async () => {
  try {
    await ElMessageBox.confirm(
      t('email.clear_history_confirm'),
      t('common.warning'),
      {
        confirmButtonText: t('common.confirm'),
        cancelButtonText: t('common.cancel'),
        type: 'warning',
      }
    )

    await invoke('clear_account_history')
    ElMessage.success(t('email.clear_history_success'))
    await loadHistory()
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('Failed to clear history:', error)
      ElMessage.error(t('email.clear_history_failed'))
    }
  }
}

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  })
}
</script>

<style scoped>
.email-generator-panel {
  padding: 24px;
  min-height: 100%;
  box-sizing: border-box;
  background: transparent;
}

/* 信息横幅 */
.info-banner {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  padding: 20px;
  background: linear-gradient(135deg, #e3f2fd 0%, #bbdefb 100%);
  border-radius: 12px;
  border-left: 4px solid #2196f3;
  margin-bottom: 32px;
  box-shadow: 0 2px 8px rgba(33, 150, 243, 0.1);
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

.info-icon {
  color: #1976d2;
  flex-shrink: 0;
}

.info-content {
  flex: 1;
}

.info-title {
  font-size: 16px;
  font-weight: 700;
  color: #0d47a1;
  margin: 0 0 8px 0;
}

.info-desc {
  font-size: 14px;
  color: #1565c0;
  margin: 0;
  line-height: 1.6;
}

/* 区块样式 */
.config-section {
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

/* 配置表单 */
.config-form {
  padding: 24px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
}

.form-hint {
  margin-top: 8px;
  font-size: 12px;
  color: #909399;
}

/* 账号信息卡片 */
.result-section {
  margin-bottom: 24px;
}

.account-card {
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

.account-info-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 16px;
}

.info-item {
  padding: 16px;
  background: #f5f7fa;
  border-radius: 8px;
  transition: all 0.2s;
}

.info-item:hover {
  background: #ecf5ff;
}

.info-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: #606266;
  font-weight: 600;
  margin-bottom: 8px;
}

.info-value {
  display: flex;
  align-items: center;
  gap: 8px;
}

.info-value code {
  flex: 1;
  padding: 8px 12px;
  background: white;
  border: 1px solid #dcdfe6;
  border-radius: 6px;
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  color: #303133;
  word-break: break-all;
  line-height: 1.5;
}

.blur-text {
  filter: blur(4px);
  user-select: none;
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

/* Element Plus 样式覆盖 */
:deep(.el-input-group__prepend) {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  font-weight: 600;
}

:deep(.el-slider__button) {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: 2px solid white;
}

:deep(.el-slider__bar) {
  background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
}

/* 历史记录区域 */
.history-section {
  margin-top: 32px;
}

.section-icon-history {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}

.history-count {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 24px;
  height: 24px;
  padding: 0 8px;
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  color: white;
  font-size: 12px;
  font-weight: 700;
  border-radius: 12px;
  margin-left: 8px;
}

.history-list {
  display: grid;
  grid-template-columns: 1fr;
  gap: 16px;
  margin-top: 20px;
}

.history-card {
  border-radius: 12px;
  overflow: hidden;
  transition: all 0.3s;
  cursor: pointer;
}

.history-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15) !important;
}

.history-card.is-expanded {
  border-color: #667eea;
}

.history-item {
  display: flex;
  flex-direction: column;
}

.history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 12px;
  border-bottom: 1px solid #e4e7ed;
  transition: all 0.2s;
}

.history-header:hover {
  background: #f5f7fa;
  margin: -20px -20px 0 -20px;
  padding: 20px 20px 12px 20px;
}

.history-summary {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.history-time {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: #909399;
}

.history-email-preview {
  display: flex;
  align-items: center;
  gap: 8px;
}

.history-email-preview code {
  padding: 4px 8px;
  background: #f5f7fa;
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  color: #303133;
  font-weight: 500;
}

.history-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.expand-btn {
  color: #909399;
  transition: all 0.2s;
}

.expand-btn:hover {
  color: #667eea;
  transform: scale(1.1);
}

.history-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding-top: 16px;
}

.history-info-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.history-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: #909399;
  font-weight: 600;
}

.history-value {
  display: flex;
  align-items: center;
  gap: 8px;
}

.history-value code {
  flex: 1;
  padding: 6px 10px;
  background: #f5f7fa;
  border: 1px solid #e4e7ed;
  border-radius: 6px;
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 12px;
  color: #303133;
  word-break: break-all;
  line-height: 1.5;
}

/* 列表过渡动画 */
.list-enter-active,
.list-leave-active {
  transition: all 0.3s ease;
}

.list-enter-from {
  opacity: 0;
  transform: translateX(-20px);
}

.list-leave-to {
  opacity: 0;
  transform: translateX(20px);
}

.list-move {
  transition: transform 0.3s ease;
}

/* 展开/折叠动画 */
.expand-enter-active,
.expand-leave-active {
  transition: all 0.3s ease;
  overflow: hidden;
}

.expand-enter-from,
.expand-leave-to {
  opacity: 0;
  max-height: 0;
  padding-top: 0;
}

.expand-enter-to,
.expand-leave-from {
  opacity: 1;
  max-height: 500px;
  padding-top: 16px;
}
</style>

