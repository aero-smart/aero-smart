<template>
  <Transition
    enter-active-class="transition duration-200 ease-out"
    enter-from-class="opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="transition duration-150 ease-in"
    leave-from-class="opacity-100"
    leave-to-class="opacity-0"
  >
    <div v-if="isOpen" class="fixed inset-0 z-[100] flex items-center justify-center" role="dialog">
      <!-- Backdrop -->
      <div class="absolute inset-0 bg-black/50 backdrop-blur-sm" @click="close"></div>

      <!-- Modal Content -->
      <div
        class="bg-surface w-[600px] max-h-[90vh] flex flex-col rounded-2xl shadow-2xl border border-gray-200 overflow-hidden relative z-10 transform transition-all duration-300"
      >
        <!-- Header -->
        <div class="px-6 py-4 border-b border-gray-100 flex items-center justify-between bg-white">
          <h2 class="text-lg font-bold text-black flex items-center gap-2">
            <SettingsIcon class="w-5 h-5 text-black" />
            Settings
          </h2>
          <button
            @click="close"
            class="text-gray-400 hover:text-black transition-colors rounded-full p-1 hover:bg-gray-100"
          >
            <XIcon class="w-5 h-5" />
          </button>
        </div>

        <!-- Content -->
        <div class="flex-1 overflow-y-auto p-6 space-y-8 bg-white">
          <!-- Loading State -->
          <div v-if="loading" class="flex justify-center py-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-black"></div>
          </div>

          <div v-else class="space-y-8">
            <!-- Serial Configuration -->
            <section>
              <h3
                class="text-xs font-bold text-black uppercase tracking-wider mb-4 flex items-center gap-2"
              >
                <Cable class="w-4 h-4" />
                Serial Communication
              </h3>
              <div class="grid grid-cols-2 gap-4">
                <div class="col-span-2">
                  <label class="block text-xs font-medium text-gray-500 mb-1.5">Port</label>
                  <input
                    v-model="config.serial.port"
                    type="text"
                    class="w-full px-4 py-2.5 bg-gray-50 border border-gray-200 rounded-lg text-sm text-black focus:outline-none focus:ring-1 focus:ring-black focus:border-black transition-all"
                    placeholder="e.g. /dev/tty.usbmodem..."
                  />
                </div>
                <div>
                  <label class="block text-xs font-medium text-gray-500 mb-1.5">Baud Rate</label>
                  <input
                    v-model.number="config.serial.baud_rate"
                    type="number"
                    class="w-full px-4 py-2.5 bg-gray-50 border border-gray-200 rounded-lg text-sm text-black focus:outline-none focus:ring-1 focus:ring-black focus:border-black transition-all"
                  />
                </div>
                <div>
                  <label class="block text-xs font-medium text-gray-500 mb-1.5"
                    >Retry Interval (s)</label
                  >
                  <input
                    v-model.number="config.serial.retry_interval_secs"
                    type="number"
                    class="w-full px-4 py-2.5 bg-gray-50 border border-gray-200 rounded-lg text-sm text-black focus:outline-none focus:ring-1 focus:ring-black focus:border-black transition-all"
                  />
                </div>
              </div>
            </section>

            <div class="h-px bg-gray-100"></div>

            <!-- Server Configuration -->
            <section>
              <h3
                class="text-xs font-bold text-black uppercase tracking-wider mb-4 flex items-center gap-2"
              >
                <Network class="w-4 h-4" />
                WebSocket Server
              </h3>
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="block text-xs font-medium text-gray-500 mb-1.5">Host</label>
                  <input
                    v-model="config.server.host"
                    type="text"
                    class="w-full px-4 py-2.5 bg-gray-50 border border-gray-200 rounded-lg text-sm text-black focus:outline-none focus:ring-1 focus:ring-black focus:border-black transition-all"
                  />
                </div>
                <div>
                  <label class="block text-xs font-medium text-gray-500 mb-1.5">Port</label>
                  <input
                    v-model.number="config.server.port"
                    type="number"
                    class="w-full px-4 py-2.5 bg-gray-50 border border-gray-200 rounded-lg text-sm text-black focus:outline-none focus:ring-1 focus:ring-black focus:border-black transition-all"
                  />
                </div>
              </div>
            </section>

            <div class="h-px bg-gray-100"></div>

            <!-- Rules Configuration -->
            <section>
              <h3
                class="text-xs font-bold text-black uppercase tracking-wider mb-4 flex items-center gap-2"
              >
                <Sliders class="w-4 h-4" />
                Rules & Behavior
              </h3>
              <div class="space-y-3">
                <label
                  class="flex items-center justify-between p-3 border border-gray-100 rounded-lg hover:bg-gray-50 cursor-pointer transition-all group"
                >
                  <span class="text-sm font-medium text-black">Debug Mode</span>
                  <div class="relative inline-flex items-center cursor-pointer">
                    <input type="checkbox" v-model="config.rules.debug_mode" class="sr-only peer" />
                    <div
                      class="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-black"
                    ></div>
                  </div>
                </label>
                <label
                  class="flex items-center justify-between p-3 border border-gray-100 rounded-lg hover:bg-gray-50 cursor-pointer transition-all group"
                >
                  <span class="text-sm font-medium text-black">Show Onboarding</span>
                  <div class="relative inline-flex items-center cursor-pointer">
                    <input
                      type="checkbox"
                      v-model="config.rules.enable_onboarding"
                      class="sr-only peer"
                    />
                    <div
                      class="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-black"
                    ></div>
                  </div>
                </label>
              </div>
            </section>
          </div>
        </div>

        <!-- Footer -->
        <div class="px-6 py-4 border-t border-gray-100 bg-white flex items-center justify-between">
          <button
            @click="handleDelete"
            class="px-4 py-2 text-sm font-medium text-black border border-gray-200 hover:bg-gray-50 hover:border-black rounded-lg transition-all flex items-center gap-2 group"
          >
            <Trash2 class="w-4 h-4 group-hover:text-red-600 transition-colors" />
            <span>删除配置</span>
          </button>

          <div class="flex items-center gap-3">
            <button
              @click="close"
              class="px-5 py-2 text-sm font-medium text-black hover:bg-gray-100 rounded-lg transition-colors flex items-center gap-2"
            >
              取消
            </button>
            <button
              @click="handleSave"
              :disabled="saving"
              class="px-6 py-2 text-sm font-medium text-white bg-black hover:bg-gray-800 rounded-lg shadow-sm hover:shadow-md transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2 transform active:scale-95"
            >
              <span
                v-if="saving"
                class="animate-spin rounded-full h-3 w-3 border-b-2 border-white"
              ></span>
              <Check v-else class="w-4 h-4" />
              保存
            </button>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import {
  Settings as SettingsIcon,
  X as XIcon,
  Trash2,
  Check,
  Cable,
  Network,
  Sliders,
} from 'lucide-vue-next'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  isOpen: boolean
}>()

const emit = defineEmits(['update:isOpen', 'close'])

const loading = ref(true)
const saving = ref(false)

// Config Structure matching Rust AppConfig
interface AppConfig {
  serial: {
    port: string
    baud_rate: number
    handshake_timeout_secs: number
    retry_interval_secs: number
  }
  server: {
    port: number
    host: string
  }
  rules: {
    debug_mode: boolean
    enable_onboarding: boolean
  }
}

const config = ref<AppConfig>({
  serial: { port: '', baud_rate: 915200, handshake_timeout_secs: 2, retry_interval_secs: 5 },
  server: { port: 3000, host: '0.0.0.0' },
  rules: { debug_mode: false, enable_onboarding: true },
})

const loadConfig = async () => {
  loading.value = true
  try {
    const data = await invoke<AppConfig>('get_app_config')
    config.value = data
  } catch (error) {
    console.error('Failed to load config:', error)
  } finally {
    loading.value = false
  }
}

const handleSave = async () => {
  saving.value = true
  try {
    await invoke('save_app_config', { config: config.value })
    close()
  } catch (error) {
    console.error('Failed to save config:', error)
    alert('Failed to save configuration: ' + error)
  } finally {
    saving.value = false
  }
}

const handleDelete = async () => {
  if (confirm('确定要重置所有设置为默认值吗？此操作无法撤销。')) {
    config.value = {
      serial: {
        port: '/dev/tty.usbmodem1234',
        baud_rate: 915200,
        handshake_timeout_secs: 2,
        retry_interval_secs: 5,
      },
      server: { port: 3000, host: '0.0.0.0' },
      rules: { debug_mode: false, enable_onboarding: true },
    }
    await handleSave()
  }
}

const close = () => {
  emit('close')
  emit('update:isOpen', false)
}

watch(
  () => props.isOpen,
  (newVal) => {
    if (newVal) {
      loadConfig()
    }
  },
)
</script>

<style scoped>
/* Add any specific transitions if needed */
</style>
