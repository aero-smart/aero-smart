<template>
  <div :class="['w-full mx-auto px-4 sm:px-6 lg:px-8', className]" :style="{ maxWidth: maxWidth }">
    <slot />
  </div>
</template>

<script setup lang="ts">
defineProps({
  maxWidth: {
    type: String,
    default: '1280px', // max-w-7xl equivalent
  },
  className: {
    type: String,
    default: '',
  },
})
</script>
