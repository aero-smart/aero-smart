<template>
  <div :class="[
    'grid gap-4 md:gap-6',
    cols === 1 && 'grid-cols-1',
    cols === 2 && 'grid-cols-1 md:grid-cols-2',
    cols === 3 && 'grid-cols-1 md:grid-cols-2 lg:grid-cols-3',
    cols === 4 && 'grid-cols-1 md:grid-cols-2 lg:grid-cols-4',
    className
  ]">
    <slot />
  </div>
</template>

<script setup lang="ts">
defineProps({
  cols: {
    type: Number,
    default: 1
  },
  className: {
    type: String,
    default: ''
  }
})
</script>
