<template>
  <span
    :class="[
      'inline-flex items-center px-2 py-0.5 rounded text-xs font-medium border',
      // Variants
      variant === 'default' && 'bg-gray-100 text-gray-800 border-gray-200',
      variant === 'primary' && 'bg-blue-50 text-blue-700 border-blue-200',
      variant === 'success' && 'bg-green-50 text-green-700 border-green-200',
      variant === 'warning' && 'bg-yellow-50 text-yellow-800 border-yellow-200',
      variant === 'danger' && 'bg-red-50 text-red-700 border-red-200',
      variant === 'outline' && 'bg-transparent border-gray-300 text-gray-600',

      // Sizes
      size === 'sm' && 'text-[10px] px-1.5',
      size === 'lg' && 'text-sm px-2.5 py-0.5',

      className,
    ]"
  >
    <span
      v-if="dot"
      :class="[
        'mr-1.5 h-1.5 w-1.5 rounded-full',
        variant === 'default' && 'bg-gray-400',
        variant === 'primary' && 'bg-blue-400',
        variant === 'success' && 'bg-green-400',
        variant === 'warning' && 'bg-yellow-400',
        variant === 'danger' && 'bg-red-400',
      ]"
    ></span>
    <slot />
  </span>
</template>

<script setup lang="ts">
import type { PropType } from 'vue'

defineProps({
  variant: {
    type: String as PropType<'default' | 'primary' | 'success' | 'warning' | 'danger' | 'outline'>,
    default: 'default',
  },
  size: {
    type: String as PropType<'sm' | 'default' | 'lg'>,
    default: 'default',
  },
  dot: {
    type: Boolean,
    default: false,
  },
  className: {
    type: String,
    default: '',
  },
})
</script>
