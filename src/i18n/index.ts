import { createI18n } from 'vue-i18n'
import zh_cn from './locales/zh_cn.json'
import en from './locales/en.json'

const messages = {
    zh_cn,
    en
}

// 检测系统语言
const getDefaultLocale = (): string => {
    const saved = localStorage.getItem('language')
    if (saved) return saved

    const browserLang = navigator.language.toLowerCase()
    if (browserLang.startsWith('zh')) {
        return 'zh_cn'
    }
    return 'en'
}

export const i18n = createI18n({
    legacy: false,
    locale: getDefaultLocale(),
    fallbackLocale: 'en',
    messages
})

