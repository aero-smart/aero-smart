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
<<<<<<< HEAD
    if ('__TAURI_INTERNALS__' in window) {
=======
    if (window.__TAURI_INTERNALS__) {
>>>>>>> d161b09ed0a35c904f723c5685a1faf4ad736a2b
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
