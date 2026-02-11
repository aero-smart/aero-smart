<script setup lang="ts">
import { RouterView, useRouter } from 'vue-router'
import { useDeviceStore } from '@/stores/device'
import { onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'

const store = useDeviceStore()
const router = useRouter()

// Config interface matching Rust struct
interface AppConfig {
  rules: {
    enable_onboarding: boolean
  }
}

onMounted(async () => {
  store.connect()

  try {
    // Only try to set fullscreen if we are in a Tauri environment
    if (window.__TAURI_INTERNALS__) {
      await getCurrentWindow().setFullscreen(true)

      // Check onboarding config
      try {
        const config = await invoke<AppConfig>('get_app_config')
        if (config.rules.enable_onboarding) {
          router.push('/onboarding')
        }
      } catch (e) {
        console.error('Failed to get app config:', e)
      }
    }
  } catch (e) {
    console.error('Failed to set fullscreen:', e)
  }
})
</script>

<template>
  <RouterView />
</template>
