<template>
  <div :class="['flex flex-col gap-1.5', className]">
    <label v-if="label" class="text-xs font-medium text-gray-700">
      {{ label }}
    </label>
    <div class="relative">
      <input
        :class="[
          'flex h-9 w-full rounded-md border border-gray-200 bg-white px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-gray-400 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-gray-400 disabled:cursor-not-allowed disabled:opacity-50',
          error ? 'border-red-500 focus-visible:ring-red-500' : '',
          $slots.prefix ? 'pl-9' : '',
          $slots.suffix ? 'pr-9' : '',
        ]"
        v-bind="$attrs"
        :value="modelValue"
        @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      />
      <div v-if="$slots.prefix" class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">
        <slot name="prefix" />
      </div>
      <div v-if="$slots.suffix" class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400">
        <slot name="suffix" />
      </div>
    </div>
    <span v-if="error" class="text-[10px] text-red-500 font-medium">{{ error }}</span>
    <span v-else-if="hint" class="text-[10px] text-gray-500">{{ hint }}</span>
  </div>
</template>

<script setup lang="ts">
defineProps({
  modelValue: {
    type: [String, Number],
    default: '',
  },
  label: {
    type: String,
    default: '',
  },
  error: {
    type: String,
    default: '',
  },
  hint: {
    type: String,
    default: '',
  },
  className: {
    type: String,
    default: '',
  },
})

defineEmits(['update:modelValue'])
</script>
