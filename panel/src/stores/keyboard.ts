import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useKeyboardStore = defineStore('keyboard', () => {
  const isVisible = ref(false)
  const currentValue = ref('')
  const inputCallback = ref<((val: string) => void) | null>(null)
  const enterCallback = ref<(() => void) | null>(null)

  // Track the active element to update its value directly if needed
  const activeElement = ref<HTMLInputElement | HTMLTextAreaElement | null>(null)

  function open(
    initialValue: string,
    onInput: (val: string) => void,
    onEnter?: () => void,
    element?: HTMLInputElement | HTMLTextAreaElement,
  ) {
    currentValue.value = initialValue
    inputCallback.value = onInput
    enterCallback.value = onEnter || null
    activeElement.value = element || null
    isVisible.value = true
  }

  function close() {
    isVisible.value = false
    activeElement.value = null
    // Don't clear callbacks immediately to avoid race conditions if needed,
    // but usually safe to clear or just ignore.
  }

  function handleInput(val: string) {
    currentValue.value = val
    if (inputCallback.value) {
      inputCallback.value(val)
    }
    // Also update DOM element directly if available to ensure sync
    if (activeElement.value) {
      activeElement.value.value = val
      activeElement.value.dispatchEvent(new Event('input'))
    }
  }

  function handleEnter() {
    if (enterCallback.value) {
      enterCallback.value()
    }
    close()
  }

  return {
    isVisible,
    currentValue,
    open,
    close,
    handleInput,
    handleEnter,
  }
})
