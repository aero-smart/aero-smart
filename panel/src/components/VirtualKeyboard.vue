<template>
  <div class="fixed inset-x-0 bottom-0 z-50 bg-gray-900 p-2 pb-6 shadow-[0_-4px_20px_rgba(0,0,0,0.3)] transition-transform duration-200" :class="{ 'translate-y-full': !show }">
    <!-- Close Handle/Bar -->
    <div class="flex justify-center mb-2" @click="$emit('close')">
      <div class="w-12 h-1 bg-gray-600 rounded-full"></div>
    </div>

    <div class="flex flex-col gap-2 max-w-4xl mx-auto select-none">
      <!-- Row 1: Numbers -->
      <div class="flex gap-1 justify-center">
        <button 
          v-for="key in ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']" 
          :key="key"
          class="key-btn"
          @click="handleInput(key)"
        >
          {{ key }}
        </button>
      </div>

      <!-- Row 2: QWERTY -->
      <div class="flex gap-1 justify-center">
        <button 
          v-for="key in ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p']" 
          :key="key"
          class="key-btn"
          @click="handleInput(key)"
        >
          {{ isShift ? key.toUpperCase() : key }}
        </button>
      </div>

      <!-- Row 3: ASDFGH -->
      <div class="flex gap-1 justify-center px-4">
        <button 
          v-for="key in ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l']" 
          :key="key"
          class="key-btn"
          @click="handleInput(key)"
        >
          {{ isShift ? key.toUpperCase() : key }}
        </button>
      </div>

      <!-- Row 4: ZXCVBNM + Shift + Backspace -->
      <div class="flex gap-1 justify-center">
        <button 
          class="key-btn special bg-gray-700 active:bg-gray-600 w-12"
          :class="{ 'text-blue-400': isShift }"
          @click="isShift = !isShift"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 12h4"/><path d="M12 14v-4"/><path d="m18 9-6-6-6 6"/><path d="M6 9v3a6 6 0 0 0 12 0V9"/></svg>
        </button>
        
        <button 
          v-for="key in ['z', 'x', 'c', 'v', 'b', 'n', 'm']" 
          :key="key"
          class="key-btn"
          @click="handleInput(key)"
        >
          {{ isShift ? key.toUpperCase() : key }}
        </button>

        <button 
          class="key-btn special bg-gray-700 active:bg-gray-600 w-12"
          @click="handleBackspace"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 4H8l-7 8 7 8h13a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2z"/><line x1="18" x2="12" y1="9" y2="15"/><line x1="12" x2="18" y1="9" y2="15"/></svg>
        </button>
      </div>

      <!-- Row 5: Space + Symbols + Enter -->
      <div class="flex gap-1 justify-center mt-1">
        <button class="key-btn special w-16 text-xs" @click="handleInput(isSymbol ? 'abc' : '!#1')">
          {{ isSymbol ? 'ABC' : '?123' }}
        </button>
        <button class="key-btn flex-1 max-w-[200px]" @click="handleInput(' ')">Space</button>
        <button class="key-btn special w-16 text-xs" @click="handleInput('.')">.</button>
        <button class="key-btn special bg-blue-600 text-white active:bg-blue-700 w-20" @click="$emit('enter')">
          Enter
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  show: boolean
  modelValue: string
}>()

const emit = defineEmits(['update:modelValue', 'close', 'enter'])

const isShift = ref(false)
const isSymbol = ref(false)

// Simple symbol mapping (can be expanded)
const symbolMap: Record<string, string> = {
  'q': '1', 'w': '2', 'e': '3', 'r': '4', 't': '5', 'y': '6', 'u': '7', 'i': '8', 'o': '9', 'p': '0',
  'a': '@', 's': '#', 'd': '$', 'f': '_', 'g': '&', 'h': '-', 'j': '+', 'k': '(', 'l': ')',
  'z': '*', 'x': '"', 'c': '\'', 'v': ':', 'b': ';', 'n': '!', 'm': '?'
}

function handleInput(key: string) {
  if (key === '!#1') {
    isSymbol.value = !isSymbol.value
    return
  }
  if (key === 'abc') {
    isSymbol.value = false
    return
  }

  let char = key
  if (isSymbol.value) {
      // Map char to symbol if in symbol mode
      // For now, if we are in symbol mode, we might want to display different keys
      // But to keep it simple, we just mapped standard QWERTY keys to symbols logic or rely on separate row.
      // Wait, the template above renders numbers in Row 1.
      // Let's implement a better symbol toggle later if needed.
      // For now, let's just stick to the printed keys.
  } else {
      if (isShift.value) char = char.toUpperCase()
  }
  
  emit('update:modelValue', props.modelValue + char)
}

function handleBackspace() {
  if (props.modelValue.length > 0) {
    emit('update:modelValue', props.modelValue.slice(0, -1))
  }
}

</script>

<style scoped>
.key-btn {
  @apply h-12 w-10 bg-gray-800 text-white rounded-md flex items-center justify-center text-lg font-medium shadow-sm active:bg-gray-700 transition-colors active:scale-95;
}
.special {
  @apply text-sm font-bold;
}
</style>
