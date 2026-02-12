import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue(), vueJsx(), vueDevTools()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  server: {
    host: true,
    port: 5173,
    strictPort: true,
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes('node_modules')) {
            if (id.includes('echarts')) {
              return 'echarts'
            }
            if (id.includes('lucide-vue-next')) {
              return 'icons'
            }
            if (
              id.includes('vue') ||
              id.includes('pinia') ||
              id.includes('vue-router') ||
              id.includes('vue-i18n')
            ) {
              return 'vue-vendor'
            }
          }
        },
      },
    },
    chunkSizeWarningLimit: 1200,
  },
})
