import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useAppStore = defineStore('app', () => {
    const statusText = ref('')
    const loading = ref(false)
    const currentView = ref('reset')

    const setStatus = (text: string) => {
        statusText.value = text
    }

    const setLoading = (value: boolean) => {
        loading.value = value
    }

    const setCurrentView = (view: string) => {
        currentView.value = view
    }

    return {
        statusText,
        loading,
        currentView,
        setStatus,
        setLoading,
        setCurrentView
    }
})

