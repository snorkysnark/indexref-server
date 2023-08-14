import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vitejs.dev/config/
export default defineConfig({
    base: "/static/",

    plugins: [react()],
    server: {
        port: 3000,
    },
    build: {
        manifest: true,
        rollupOptions: {
            input: 'src/main.tsx'
        }
    },

    css: {
        modules: {
            localsConvention: "camelCaseOnly"
        }
    }
})
