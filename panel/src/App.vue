<script setup lang="ts">
import { RouterView } from 'vue-router'
import { useDeviceStore } from '@/stores/device'
import { onMounted } from 'vue'

const store = useDeviceStore()

onMounted(() => {
  store.connect()
})
</script>

<template>
  <RouterView />
</template>
