import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [vue()],
    resolve: {
        alias: {
            '@': path.resolve(__dirname, './src')
        }
    },
    clearScreen: false,
    server: {
        host: '127.0.0.1',  // 使用 IPv4 避免权限问题
        port: 5173,  // 使用 Vite 默认端口
        strictPort: false,  // 如果端口被占用，自动尝试下一个
        watch: {
            ignored: ['**/src-tauri/**']
        }
    }
})

