<template>
  <div class="flex flex-col h-full p-4 gap-4 bg-[#f5f5f5] text-[#423d3c] font-sans">
    <div class="text-xl font-bold text-gray-800">Settings</div>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <WifiManager />
      <!-- Other settings can go here -->
    </div>
  </div>
</template>

<script setup lang="ts">
import WifiManager from '@/components/WifiManager.vue'
</script>
