<template>
  <div class="settings-panel">
    <!-- 通用设置 -->
    <div class="settings-section">
      <div class="section-header">
        <div class="section-header-left">
          <div class="section-icon gradient-blue">
            <Settings :size="20" />
          </div>
          <h3 class="section-title">{{ t('settings.general') }}</h3>
        </div>
      </div>

      <div class="settings-cards">
        <div class="setting-card">
          <div class="setting-card-header">
            <div class="setting-info">
              <div class="setting-icon">
                <Archive :size="20" />
              </div>
              <div class="setting-content">
                <div class="setting-label">{{ t('settings.auto_backup') }}</div>
                <div class="setting-desc">{{ t('settings.auto_backup_desc') }}</div>
              </div>
            </div>
            <el-switch v-model="settings.autoBackup" @change="handleSettingChange" />
          </div>
        </div>

        <div class="setting-card">
          <div class="setting-card-header">
            <div class="setting-info">
              <div class="setting-icon">
                <Database :size="20" />
              </div>
              <div class="setting-content">
                <div class="setting-label">{{ t('settings.backup_limit') }}</div>
                <div class="setting-desc">{{ t('settings.backup_limit_desc') }}</div>
              </div>
            </div>
            <el-input-number
              v-model="settings.backupLimit"
              :min="1"
              :max="50"
              @change="handleSettingChange"
              size="small"
            />
          </div>
        </div>

        <div class="setting-card">
          <div class="setting-card-header">
            <div class="setting-info">
              <div class="setting-icon">
                <Power :size="20" />
              </div>
              <div class="setting-content">
                <div class="setting-label">{{ t('settings.close_cursor') }}</div>
                <div class="setting-desc">{{ t('settings.close_cursor_desc') }}</div>
              </div>
            </div>
            <el-switch v-model="settings.closeCursor" @change="handleSettingChange" />
          </div>
        </div>
      </div>
    </div>

    <!-- 路径设置 -->
    <div class="settings-section">
      <div class="section-header">
        <div class="section-header-left">
          <div class="section-icon gradient-purple">
            <FolderOpen :size="20" />
          </div>
          <h3 class="section-title">{{ t('settings.paths') }}</h3>
        </div>
      </div>

      <div class="settings-cards">
        <div class="setting-card">
          <div class="setting-label-row">
            <Folder :size="18" />
            <span>{{ t('settings.cursor_path') }}</span>
          </div>
          <div class="path-input-group">
            <el-input v-model="settings.cursorPath" readonly placeholder="选择路径..." />
            <el-button @click="handleBrowsePath('cursor')" class="browse-button">
              <FolderOpen :size="16" />
              <span>浏览</span>
            </el-button>
          </div>
        </div>

        <div class="setting-card">
          <div class="setting-label-row">
            <Folder :size="18" />
            <span>Windsurf 路径</span>
          </div>
          <div class="path-input-group">
            <el-input v-model="settings.windsurfPath" placeholder="输入或选择 Windsurf 数据路径..." />
            <el-button @click="handleBrowsePath('windsurf')" class="browse-button">
              <FolderOpen :size="16" />
              <span>浏览</span>
            </el-button>
          </div>
          <div class="path-hint">
            例如: ~/Library/Application Support/Windsurf (macOS)
          </div>
        </div>

        <div class="setting-card">
          <div class="setting-label-row">
            <Folder :size="18" />
            <span>Qoder 路径</span>
          </div>
          <div class="path-input-group">
            <el-input v-model="settings.qoderPath" placeholder="输入或选择 Qoder 数据路径..." />
            <el-button @click="handleBrowsePath('qoder')" class="browse-button">
              <FolderOpen :size="16" />
              <span>浏览</span>
            </el-button>
          </div>
          <div class="path-hint">
            例如: ~/Library/Application Support/Qoder (macOS)
          </div>
        </div>

        <div class="setting-card">
          <div class="setting-label-row">
            <HardDrive :size="18" />
            <span>{{ t('settings.backup_path') }}</span>
          </div>
          <div class="path-input-group">
            <el-input v-model="settings.backupPath" readonly placeholder="选择路径..." />
            <el-button @click="handleBrowsePath('backup')" class="browse-button">
              <FolderOpen :size="16" />
              <span>浏览</span>
            </el-button>
          </div>
        </div>
      </div>
    </div>

    <!-- 高级设置 -->
    <div class="settings-section">
      <div class="section-header">
        <div class="section-header-left">
          <div class="section-icon gradient-orange">
            <Sliders :size="20" />
          </div>
          <h3 class="section-title">{{ t('settings.advanced') }}</h3>
        </div>
      </div>

      <div class="settings-cards">
        <div class="setting-card">
          <div class="setting-card-header">
            <div class="setting-info">
              <div class="setting-icon">
                <Wrench :size="20" />
              </div>
              <div class="setting-content">
                <div class="setting-label">{{ t('settings.patch_workbench') }}</div>
                <div class="setting-desc">{{ t('settings.patch_workbench_desc') }}</div>
              </div>
            </div>
            <el-switch v-model="settings.patchWorkbench" @change="handleSettingChange" />
          </div>
        </div>

        <div class="setting-card">
          <div class="setting-card-header">
            <div class="setting-info">
              <div class="setting-icon">
                <RefreshCw :size="20" />
              </div>
              <div class="setting-content">
                <div class="setting-label">{{ t('settings.update_system_id') }}</div>
                <div class="setting-desc">{{ t('settings.update_system_id_desc') }}</div>
              </div>
            </div>
            <el-switch v-model="settings.updateSystemId" @change="handleSettingChange" />
          </div>
        </div>

        <div class="setting-card">
          <div class="setting-card-header">
            <div class="setting-info">
              <div class="setting-icon">
                <Bug :size="20" />
              </div>
              <div class="setting-content">
                <div class="setting-label">{{ t('settings.debug_mode') }}</div>
                <div class="setting-desc">{{ t('settings.debug_mode_desc') }}</div>
              </div>
            </div>
            <el-switch v-model="settings.debugMode" @change="handleSettingChange" />
          </div>
        </div>
      </div>
    </div>

    <!-- 关于 -->
    <div class="settings-section">
      <div class="section-header">
        <div class="section-header-left">
          <div class="section-icon gradient-green">
            <Info :size="20" />
          </div>
          <h3 class="section-title">{{ t('settings.about') }}</h3>
        </div>
      </div>

      <div class="about-card">
        <div class="about-grid">
          <div class="about-item">
            <div class="about-icon">
              <Sparkles :size="20" />
            </div>
            <div class="about-content">
              <div class="about-label">{{ t('settings.app_name') }}</div>
              <div class="about-value">IDE重置工具</div>
            </div>
          </div>

          <div class="about-item">
            <div class="about-icon">
              <Tag :size="20" />
            </div>
            <div class="about-content">
              <div class="about-label">{{ t('settings.version') }}</div>
              <div class="about-value">v1.0.5</div>
            </div>
          </div>

          <div class="about-item">
            <div class="about-icon">
              <Monitor :size="20" />
            </div>
            <div class="about-content">
              <div class="about-label">{{ t('settings.platform') }}</div>
              <div class="about-value">{{ platformName }}</div>
            </div>
          </div>

          <div class="about-item">
            <div class="about-icon">
              <Github :size="20" />
            </div>
            <div class="about-content">
              <div class="about-label">{{ t('settings.github') }}</div>
              <a
                href="https://github.com/Taylor-Ding/cursor-machine-id-resetter.git"
                target="_blank"
                class="about-link"
              >
                Taylor-Ding/cursor-machine-id-resetter
              </a>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="action-section">
      <el-button
        type="primary"
        size="large"
        @click="handleSave"
        class="action-button action-button-primary"
      >
        <Save :size="20" />
        <span>{{ t('common.save') }}</span>
      </el-button>

      <el-button
        size="large"
        @click="handleReset"
        class="action-button action-button-secondary"
      >
        <RotateCcw :size="20" />
        <span>{{ t('settings.reset_default') }}</span>
      </el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import {
  Settings, FolderOpen, Sliders, Info, Save, RotateCcw,
  Archive, Database, Power, Folder, HardDrive, Wrench,
  RefreshCw, Bug, Sparkles, Tag, Monitor, Github
} from 'lucide-vue-next'
import { invoke } from '@tauri-apps/api/core'
import { platform } from '@tauri-apps/plugin-os'
import { open } from '@tauri-apps/plugin-dialog'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useLogStore } from '../stores/log'

const { t } = useI18n()
const logStore = useLogStore()

interface Settings {
  autoBackup: boolean
  backupLimit: number
  closeCursor: boolean
  cursorPath: string
  windsurfPath: string
  qoderPath: string
  backupPath: string
  patchWorkbench: boolean
  updateSystemId: boolean
  debugMode: boolean
  emailDomain: string
}

const settings = ref<Settings>({
  autoBackup: true,
  backupLimit: 10,
  closeCursor: true,
  cursorPath: '',
  windsurfPath: '',
  qoderPath: '',
  backupPath: '',
  patchWorkbench: true,
  updateSystemId: true,
  debugMode: false,
  emailDomain: ''
})

const platformKey = ref<string>('')

const platformName = computed(() => {
  const names: Record<string, string> = {
    windows: 'Windows',
    macos: 'macOS',
    linux: 'Linux'
  }
  const p = platformKey.value
  return (names[p] ?? (p || 'Unknown'))
})

onMounted(async () => {
  await loadSettings()
  try {
    platformKey.value = await platform()
  } catch (e) {
    console.error('Failed to get platform:', e)
  }
})

const loadSettings = async () => {
  try {
    const savedSettings = await invoke('get_settings')
    Object.assign(settings.value, savedSettings)
  } catch (error) {
    console.error('Failed to load settings:', error)
    logStore.addLog('error', t('settings.load_failed'))
  }
}

const handleSettingChange = () => {
  // 实时保存设置
  handleSave(false)
}

const handleSave = async (showMessage = true) => {
  try {
    await invoke('save_settings', {
      settings: settings.value
    })

    if (showMessage) {
      ElMessage.success(t('settings.save_success'))
      logStore.addLog('success', t('settings.save_success'))
    }
  } catch (error) {
    console.error('Failed to save settings:', error)
    ElMessage.error(t('settings.save_failed'))
    logStore.addLog('error', t('settings.save_failed'))
  }
}

const handleReset = async () => {
  try {
    await ElMessageBox.confirm(
      t('settings.reset_confirm'),
      t('common.warning'),
      {
        confirmButtonText: t('common.confirm'),
        cancelButtonText: t('common.cancel'),
        type: 'warning',
      }
    )

    const defaultSettings = await invoke('get_default_settings')
    Object.assign(settings.value, defaultSettings)
    await handleSave()

  } catch (error) {
    // 用户取消
  }
}

const handleBrowsePath = async (type: 'cursor' | 'backup') => {
  try {
    console.log('Opening dialog for type:', type)

    const selected = await open({
      directory: true,
      multiple: false,
      title: type === 'cursor' ? t('settings.select_cursor_path') : t('settings.select_backup_path')
    })

    console.log('Dialog result:', selected)

    if (selected) {
      if (type === 'cursor') {
        settings.value.cursorPath = selected as string
        console.log('Updated cursor path:', selected)
      } else {
        settings.value.backupPath = selected as string
        console.log('Updated backup path:', selected)
      }
      await handleSave(false)
      ElMessage.success(t('settings.path_updated'))
    }
  } catch (error) {
    console.error('Failed to browse path:', error)
    ElMessage.error(t('settings.browse_failed'))
    logStore.addLog('error', `Failed to browse ${type} path: ${error}`)
  }
}
</script>

<style scoped>
.settings-panel {
  padding: 24px;
  min-height: 100%;
  box-sizing: border-box;
  background: transparent;
}

/* 区块样式 */
.settings-section {
  margin-bottom: 32px;
}

.settings-section:last-child {
  margin-bottom: 0;
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
  border-radius: 10px;
  color: white;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.section-icon.gradient-blue {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.section-icon.gradient-purple {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}

.section-icon.gradient-orange {
  background: linear-gradient(135deg, #fa709a 0%, #fee140 100%);
}

.section-icon.gradient-green {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
}

.section-title {
  font-size: 18px;
  font-weight: 700;
  color: #2c3e50;
  margin: 0;
}

/* 设置卡片 */
.settings-cards {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.setting-card {
  padding: 20px;
  background: white;
  border: 2px solid #e8eaed;
  border-radius: 12px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.setting-card:hover {
  border-color: #667eea;
  box-shadow: 0 4px 16px rgba(102, 126, 234, 0.15);
  transform: translateY(-2px);
}

.setting-card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
}

.setting-info {
  display: flex;
  align-items: center;
  gap: 16px;
  flex: 1;
  min-width: 0;
}

.setting-icon {
  flex-shrink: 0;
  width: 44px;
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 10px;
  background: linear-gradient(135deg, #f5f7fa 0%, #e8eaed 100%);
  color: #667eea;
}

.setting-content {
  flex: 1;
  min-width: 0;
}

.setting-label {
  font-size: 15px;
  font-weight: 600;
  color: #2c3e50;
  margin-bottom: 4px;
}

.setting-desc {
  font-size: 13px;
  color: #7f8c8d;
  line-height: 1.5;
}

/* 路径输入 */
.setting-label-row {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 15px;
  font-weight: 600;
  color: #2c3e50;
  margin-bottom: 12px;
}

.path-input-group {
  display: flex;
  gap: 12px;
  align-items: center;
}

.path-input-group :deep(.el-input) {
  flex: 1;
}

.path-input-group :deep(.el-input__wrapper) {
  border-radius: 8px;
  border: 2px solid #e8eaed;
  transition: all 0.3s;
}

.path-input-group :deep(.el-input__wrapper:hover) {
  border-color: #667eea;
}

.path-hint {
  font-size: 12px;
  color: #7f8c8d;
  margin-top: 8px;
  padding-left: 4px;
  font-style: italic;
}

.browse-button {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 20px;
  border-radius: 8px;
  border: 2px solid #667eea;
  color: #667eea;
  background: white;
  font-weight: 600;
  transition: all 0.3s;
}

.browse-button:hover {
  background: rgba(102, 126, 234, 0.1);
  border-color: #5568d3;
  color: #5568d3;
}

/* 关于卡片 */
.about-card {
  padding: 24px;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.05) 0%, rgba(118, 75, 162, 0.05) 100%);
  border: 2px solid #e8eaed;
  border-radius: 12px;
}

.about-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 20px;
}

.about-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background: white;
  border-radius: 10px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  transition: all 0.3s;
}

.about-item:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
}

.about-icon {
  flex-shrink: 0;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.about-content {
  flex: 1;
  min-width: 0;
}

.about-label {
  font-size: 13px;
  color: #7f8c8d;
  margin-bottom: 4px;
}

.about-value {
  font-size: 15px;
  font-weight: 600;
  color: #2c3e50;
}

.about-link {
  font-size: 15px;
  font-weight: 600;
  color: #667eea;
  text-decoration: none;
  transition: color 0.3s;
}

.about-link:hover {
  color: #5568d3;
  text-decoration: underline;
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
:deep(.el-switch.is-checked .el-switch__core) {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

:deep(.el-input-number) {
  border-radius: 8px;
}

:deep(.el-input-number .el-input__wrapper) {
  border-radius: 8px;
  border: 2px solid #e8eaed;
  transition: all 0.3s;
}

:deep(.el-input-number .el-input__wrapper:hover) {
  border-color: #667eea;
}
</style>

