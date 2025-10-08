/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{vue,js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                primary: {
                    DEFAULT: '#409EFF',
                    dark: '#337ECC',
                    light: '#79BBFF'
                },
                success: '#67C23A',
                warning: '#E6A23C',
                danger: '#F56C6C',
                info: '#909399'
            }
        },
    },
    plugins: [],
}

