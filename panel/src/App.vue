<script setup lang="ts">
import { RouterView } from 'vue-router'
import { useDeviceStore } from '@/stores/device'
import { onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

const store = useDeviceStore()

onMounted(async () => {
  store.connect()
  try {
    // Only try to set fullscreen if we are in a Tauri environment
    if (window.__TAURI_INTERNALS__) {
       await getCurrentWindow().setFullscreen(true)
    }
  } catch (e) {
    console.error('Failed to set fullscreen:', e)
  }
})
</script>

<template>
  <RouterView />
</template>
