<template>
  <button
    :class="[
      'inline-flex items-center justify-center font-medium transition-all focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed',
      // Variants
      variant === 'primary' &&
        'bg-primary text-primary-foreground hover:bg-black/90 focus:ring-primary',
      variant === 'secondary' &&
        'bg-secondary text-secondary-foreground hover:bg-secondary/80 focus:ring-secondary',
      variant === 'outline' &&
        'border border-gray-200 bg-transparent hover:bg-gray-50 focus:ring-gray-200 text-gray-700',
      variant === 'ghost' && 'bg-transparent hover:bg-gray-100 text-gray-700 focus:ring-gray-200',
      variant === 'danger' && 'bg-danger text-white hover:bg-red-600 focus:ring-red-500',

      // Sizes
      size === 'sm' && 'h-8 px-3 text-xs rounded-md',
      size === 'default' && 'h-10 px-4 py-2 text-sm rounded-lg',
      size === 'lg' && 'h-12 px-8 text-base rounded-xl',
      size === 'icon' && 'h-10 w-10 p-0 rounded-lg',

      // Additional
      rounded ? 'rounded-full' : '',
      block ? 'w-full' : '',
      className,
    ]"
    :disabled="disabled || loading"
    v-bind="$attrs"
  >
    <svg
      v-if="loading"
      class="animate-spin -ml-1 mr-2 h-4 w-4"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      ></circle>
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      ></path>
    </svg>
    <slot name="icon-left" v-if="$slots['icon-left'] && !loading" />
    <span :class="{ 'ml-2': $slots['icon-left'] && !loading, 'mr-2': $slots['icon-right'] }">
      <slot />
    </span>
    <slot name="icon-right" />
  </button>
</template>

<script setup lang="ts">
import type { PropType } from 'vue'

defineProps({
  variant: {
    type: String as PropType<'primary' | 'secondary' | 'outline' | 'ghost' | 'danger'>,
    default: 'primary',
  },
  size: {
    type: String as PropType<'sm' | 'default' | 'lg' | 'icon'>,
    default: 'default',
  },
  disabled: {
    type: Boolean,
    default: false,
  },
  loading: {
    type: Boolean,
    default: false,
  },
  block: {
    type: Boolean,
    default: false,
  },
  rounded: {
    type: Boolean,
    default: false,
  },
  className: {
    type: String,
    default: '',
  },
})
</script>
