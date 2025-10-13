<template>
  <div class="ide-selector">
    <div class="selector-header">
      <h3>{{ $t('selectIDE') }}</h3>
    </div>
    <div class="ide-buttons">
      <button
        v-for="ide in ides"
        :key="ide.type"
        @click="selectIDE(ide.type)"
        :class="['ide-button', { active: modelValue === ide.type }]"
        class="group"
      >
        <div class="ide-icon-wrapper">
          <svg
            v-if="ide.type === 'cursor'"
            class="ide-icon"
            viewBox="0 0 24 24"
            fill="currentColor"
          >
            <path d="M12 2L2 7v10c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V7l-10-5z"/>
          </svg>
          <svg
            v-else-if="ide.type === 'windsurf'"
            class="ide-icon"
            viewBox="0 0 24 24"
            fill="currentColor"
          >
            <path d="M14 2l6.5 6.5L22 10l-8.5 8.5L13 20l-6-6 8-10z M6 22v-4l4 4H6z"/>
          </svg>
        </div>
        <div class="ide-info">
          <span class="ide-name">{{ ide.name }}</span>
          <span class="ide-status" v-if="modelValue === ide.type">
            ✓ {{ $t('selected') }}
          </span>
        </div>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue'

interface IDE {
  type: 'cursor' | 'windsurf'
  name: string
}

const props = defineProps<{
  modelValue: 'cursor' | 'windsurf'
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: 'cursor' | 'windsurf'): void
}>()

const ides: IDE[] = [
  { type: 'cursor', name: 'Cursor' },
  { type: 'windsurf', name: 'Windsurf' }
]

function selectIDE(type: 'cursor' | 'windsurf') {
  emit('update:modelValue', type)
}
</script>

<style scoped lang="css">
.ide-selector {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
  margin-bottom: 24px;
}

.selector-header h3 {
  color: white;
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 16px 0;
  text-align: center;
}

.ide-buttons {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}

.ide-button {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  background: rgba(255, 255, 255, 0.95);
  border: 2px solid transparent;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

.ide-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  background: white;
}

.ide-button.active {
  border-color: #667eea;
  background: white;
  box-shadow: 0 8px 32px rgba(102, 126, 234, 0.3);
}

.ide-icon-wrapper {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  flex-shrink: 0;
}

.ide-button.active .ide-icon-wrapper {
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% {
    transform: scale(1);
    box-shadow: 0 0 0 0 rgba(102, 126, 234, 0.7);
  }
  50% {
    transform: scale(1.05);
    box-shadow: 0 0 0 8px rgba(102, 126, 234, 0);
  }
}

.ide-icon {
  width: 28px;
  height: 28px;
  color: white;
}

.ide-info {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  flex: 1;
}

.ide-name {
  font-size: 18px;
  font-weight: 600;
  color: #2d3748;
  margin-bottom: 4px;
}

.ide-status {
  font-size: 12px;
  color: #667eea;
  font-weight: 500;
}
</style>
