import { defineStore } from 'pinia'
import { ref } from 'vue'

export type LogLevel = 'info' | 'success' | 'warning' | 'error'

export interface LogItem {
    id: string
    timestamp: number
    level: LogLevel
    message: string
}

export const useLogStore = defineStore('log', () => {
    const logs = ref<LogItem[]>([])

    const addLog = (level: LogLevel, message: string) => {
        const log: LogItem = {
            id: `${Date.now()}-${Math.random()}`,
            timestamp: Date.now(),
            level,
            message
        }
        logs.value.push(log)

        // 限制日志数量，最多保留1000条
        if (logs.value.length > 1000) {
            logs.value = logs.value.slice(-1000)
        }
    }

    const clearLogs = () => {
        logs.value = []
    }

    return {
        logs,
        addLog,
        clearLogs
    }
})

