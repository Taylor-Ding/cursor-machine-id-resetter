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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import {
  Info, Settings, Mail, Key, User, Copy, Eye, EyeOff, Sparkles, UserCheck
} from 'lucide-vue-next'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import { useLogStore } from '../stores/log'

const { t } = useI18n()
const logStore = useLogStore()

interface AccountInfo {
  email: string
  password: string
  first_name: string
  last_name: string
}

const form = ref({
  domain: '',
  passwordLength: 16,
  timestampLength: 4,
})

const accountInfo = ref<AccountInfo | null>(null)
const generating = ref(false)
const showPassword = ref(false)

onMounted(async () => {
  // 从设置中加载域名配置
  try {
    const settings = await invoke('get_settings')
    if (settings && settings.email_domain) {
      form.value.domain = settings.email_domain
    }
  } catch (error) {
    console.error('Failed to load settings:', error)
  }
})

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
  } catch (error: any) {
    console.error('Generate account failed:', error)
    logStore.addLog('error', `${t('email.generate_failed')}: ${error}`)
    ElMessage.error(t('email.generate_failed'))
  } finally {
    generating.value = false
  }
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
</style>

