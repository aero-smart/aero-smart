<script setup lang="ts">
import { RouterView } from 'vue-router'
import { useDeviceStore } from '@/stores/device'
import { onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

const store = useDeviceStore()

onMounted(async () => {
  store.connect()
  try {
    await getCurrentWindow().setFullscreen(true)
  } catch (e) {
    console.error('Failed to set fullscreen:', e)
  }
})
</script>

<template>
  <RouterView />
</template>
