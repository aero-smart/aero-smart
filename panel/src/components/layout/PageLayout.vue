<template>
  <div class="flex flex-col h-full bg-page text-primary font-sans overflow-hidden">
    <!-- Header -->
    <header
      v-if="$slots.header || title"
      class="flex-none px-4 py-3 border-b border-gray-100 flex items-center justify-between bg-white/80 backdrop-blur sticky top-0 z-10"
    >
      <div class="flex items-center gap-3">
        <slot name="title-prefix" />
        <h1 v-if="title" class="text-lg font-bold tracking-tight text-gray-900">{{ title }}</h1>
        <slot name="title-suffix" />
      </div>
      <div class="flex items-center gap-3">
        <slot name="actions" />
      </div>
    </header>

    <!-- Main Content -->
    <main class="flex-1 min-h-0 overflow-y-auto p-4 md:p-6">
      <div class="mx-auto max-w-7xl h-full flex flex-col">
        <slot />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
defineProps({
  title: {
    type: String,
    default: '',
  },
})
</script>
