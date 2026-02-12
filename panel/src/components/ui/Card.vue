<template>
  <div
    :class="[
      'bg-surface border border-gray-100/50 shadow-sm overflow-hidden transition-all',
      hoverable ? 'hover:shadow-md hover:-translate-y-0.5' : '',
      rounded ? 'rounded-2xl' : 'rounded-lg',
      className,
    ]"
  >
    <div
      v-if="$slots.header || title"
      class="px-4 py-3 border-b border-gray-100/50 flex items-center justify-between"
    >
      <h3 v-if="title" class="font-semibold text-gray-900 text-sm tracking-tight">{{ title }}</h3>
      <slot name="header" />
    </div>
    <div :class="['p-4', contentClass]">
      <slot />
    </div>
    <div v-if="$slots.footer" class="px-4 py-3 bg-gray-50/50 border-t border-gray-100/50">
      <slot name="footer" />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  title: {
    type: String,
    default: '',
  },
  hoverable: {
    type: Boolean,
    default: false,
  },
  rounded: {
    type: Boolean,
    default: true,
  },
  className: {
    type: String,
    default: '',
  },
  contentClass: {
    type: String,
    default: '',
  },
})
</script>
