<template>
  <!-- Backdrop for closing -->
  <div v-if="show" class="fixed inset-0 z-[99] bg-transparent" @click="$emit('close')"></div>

  <div
    class="fixed inset-x-0 bottom-0 z-[100] bg-gray-900 pb-2 pt-1 shadow-[0_-4px_20px_rgba(0,0,0,0.3)] transition-transform duration-200 select-none"
    :class="{ 'translate-y-full': !show }"
    @click.stop
  >
    <!-- Close Handle/Bar -->
    <div class="flex justify-center mb-1 h-6 items-center w-full" @click="$emit('close')">
      <div class="w-12 h-1 bg-gray-600 rounded-full"></div>
    </div>

    <div class="flex flex-col w-full px-1 gap-1.5">
      <!-- Row 1: QWERTY / Numbers -->
      <div class="flex gap-1.5 w-full justify-center">
        <button v-for="key in row1" :key="key" class="key-btn flex-1" @click="handleInput(key)">
          {{ getKeyDisplay(key) }}
        </button>
      </div>

      <!-- Row 2: ASDFGH / Symbols 1 -->
      <div class="flex gap-1.5 w-full justify-center px-[4%]">
        <button v-for="key in row2" :key="key" class="key-btn flex-1" @click="handleInput(key)">
          {{ getKeyDisplay(key) }}
        </button>
      </div>

      <!-- Row 3: Shift + ZXCVBNM + Backspace / Symbols 2 -->
      <div class="flex gap-1.5 w-full justify-center">
        <!-- Shift / Symbol Switch -->
        <button
          class="key-btn special bg-gray-700 w-[14%]"
          :class="{
            'bg-gray-200 text-gray-900': isShift && !isSymbolMode,
            'bg-gray-700 text-white': !isShift || isSymbolMode,
          }"
          @click="handleShiftOrSymbol"
        >
          <template v-if="!isSymbolMode">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path d="M10 12h4" />
              <path d="M12 14v-4" />
              <path d="m18 9-6-6-6 6" />
              <path d="M6 9v3a6 6 0 0 0 12 0V9" />
            </svg>
          </template>
          <template v-else>
            <span class="text-sm font-bold">{{ isSymbolPage2 ? '1/2' : '2/2' }}</span>
          </template>
        </button>

        <!-- Keys -->
        <div class="flex flex-1 gap-1.5">
          <button v-for="key in row3" :key="key" class="key-btn flex-1" @click="handleInput(key)">
            {{ getKeyDisplay(key) }}
          </button>
        </div>

        <!-- Backspace -->
        <button
          class="key-btn special bg-gray-700 active:bg-gray-600 w-[14%]"
          @click="handleBackspace"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="M21 4H8l-7 8 7 8h13a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2z" />
            <line x1="18" x2="12" y1="9" y2="15" />
            <line x1="12" x2="18" y1="9" y2="15" />
          </svg>
        </button>
      </div>

      <!-- Row 4: Mode + Space + Enter -->
      <div class="flex gap-1.5 w-full justify-center mt-1">
        <button
          class="key-btn special bg-gray-700 w-[14%] text-sm font-bold"
          @click="toggleSymbolMode"
        >
          {{ isSymbolMode ? 'ABC' : '?123' }}
        </button>

        <button
          class="key-btn special bg-gray-700 w-[10%] text-sm font-bold"
          @click="handleInput(',')"
        >
          ,
        </button>

        <button class="key-btn flex-1 bg-gray-700 active:bg-gray-600" @click="handleInput(' ')">
          Space
        </button>

        <button
          class="key-btn special bg-gray-700 w-[10%] text-sm font-bold"
          @click="handleInput('.')"
        >
          .
        </button>

        <button
          class="key-btn special bg-blue-600 text-white active:bg-blue-700 w-[14%]"
          @click="$emit('enter')"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <polyline points="9 10 4 15 9 20" />
            <path d="M20 4v7a4 4 0 0 1-4 4H4" />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

const props = defineProps<{
  show: boolean
  modelValue: string
}>()

const emit = defineEmits(['update:modelValue', 'close', 'enter'])

const isShift = ref(false)
const isSymbolMode = ref(false)
const isSymbolPage2 = ref(false)

// Layout Definitions
const qwerty = {
  row1: ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
  row2: ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
  row3: ['z', 'x', 'c', 'v', 'b', 'n', 'm'],
}

const symbols1 = {
  row1: ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
  row2: ['@', '#', '$', '_', '&', '-', '+', '(', ')', '/'],
  row3: ['*', '"', "'", ':', ';', '!', '?'],
}

const symbols2 = {
  row1: ['~', '`', '|', '•', '√', 'π', '÷', '×', '¶', '∆'],
  row2: ['£', '¢', '€', '¥', '^', '°', '=', '{', '}', '\\'],
  row3: ['%', '©', '®', '™', '✓', '[', ']'],
}

// Computed Rows based on current mode
const row1 = computed(() => {
  if (!isSymbolMode.value) return qwerty.row1
  return isSymbolPage2.value ? symbols2.row1 : symbols1.row1
})

const row2 = computed(() => {
  if (!isSymbolMode.value) return qwerty.row2
  return isSymbolPage2.value ? symbols2.row2 : symbols1.row2
})

const row3 = computed(() => {
  if (!isSymbolMode.value) return qwerty.row3
  return isSymbolPage2.value ? symbols2.row3 : symbols1.row3
})

function getKeyDisplay(key: string) {
  if (!isSymbolMode.value && isShift.value) {
    return key.toUpperCase()
  }
  return key
}

function handleInput(key: string) {
  let char = key
  if (!isSymbolMode.value && isShift.value) {
    char = char.toUpperCase()
    // Auto-disable shift after one char unless caps lock (not implemented yet)
    // isShift.value = false
  }
  emit('update:modelValue', props.modelValue + char)
}

function handleBackspace() {
  if (props.modelValue.length > 0) {
    emit('update:modelValue', props.modelValue.slice(0, -1))
  }
}

function toggleSymbolMode() {
  isSymbolMode.value = !isSymbolMode.value
  isSymbolPage2.value = false // Reset to page 1 when entering symbol mode
}

function handleShiftOrSymbol() {
  if (isSymbolMode.value) {
    isSymbolPage2.value = !isSymbolPage2.value
  } else {
    isShift.value = !isShift.value
  }
}
</script>

<style scoped>
.key-btn {
  @apply h-14 bg-gray-800 text-white rounded-[6px] flex items-center justify-center text-xl font-medium shadow-[0_1px_0_rgba(0,0,0,0.3)] transition-all active:bg-gray-600 active:translate-y-[1px] active:shadow-none select-none touch-manipulation;
}
.special {
  @apply bg-gray-700 active:bg-gray-600;
}
</style>
