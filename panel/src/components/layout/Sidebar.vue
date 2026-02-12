<template>
  <aside
    class="w-[56px] bg-surface flex flex-col items-center py-0 h-full border-r border-gray-200 z-50 select-none justify-between"
  >
    <!-- Top: Brand/Logo (Frame 7) -->
    <div class="w-[56px] h-[56px] flex items-center justify-center mt-1">
      <svg
        width="20"
        height="30"
        viewBox="0 0 18 28"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M12.5348 21.7806L8.66407 16.8297L4.74836 21.7806H0L9.75257 13.7466L0.247552 6.02771H5.0184L8.73157 10.7761L12.3097 6.02771H17.0581L12.3097 13.8591L17.3056 21.7806H12.5348Z"
          fill="black"
        />
      </svg>
    </div>

    <!-- Middle: Navigation Group (Frame 38) -->
    <div
      class="bg-nav-group p-[2px] rounded-lg flex flex-col gap-0 shadow-sm border border-white/50"
    >
      <router-link
        v-for="item in menuItems"
        :key="item.path"
        :to="item.path"
        class="w-[36px] h-[36px] rounded-[7px] flex items-center justify-center transition-all duration-200 group relative"
        :class="
          $route.path === item.path
            ? 'bg-nav-active text-text-primary'
            : 'text-text-secondary hover:text-text-primary hover:bg-gray-100'
        "
      >
        <component :is="item.icon" :size="18" stroke-width="2" />

        <!-- Tooltip -->
        <div
          class="absolute left-full ml-2 px-2 py-1 bg-gray-800 text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none z-50"
        >
          {{ item.label }}
        </div>
      </router-link>
    </div>

    <!-- Bottom: Settings (Icons 4) -->
    <div class="flex flex-col items-center gap-1 mb-1">
      <!-- Serial Status Indicator -->
      <button
        @click="handleSerialClick"
        class="w-[36px] h-[10px] flex items-center justify-center group relative cursor-help"
      >
        <div
          class="w-1.5 h-1.5 rounded-full transition-all duration-300"
          :class="serialIndicatorClass"
        ></div>
        <!-- Tooltip -->
        <div
          class="absolute left-full ml-2 px-2 py-1 bg-gray-800 text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none z-50"
        >
          {{ serialTooltip }}
        </div>
      </button>

      <!-- WiFi Indicator -->
      <button
        @click="isWifiManagerOpen = true"
        class="w-[36px] h-[36px] rounded-[7px] flex items-center justify-center text-text-secondary hover:text-text-primary hover:bg-gray-100 transition-all group relative cursor-pointer"
        :class="{ 'bg-nav-active text-text-primary': isWifiManagerOpen }"
      >
        <component :is="wifiIcon" :size="18" stroke-width="2" :class="wifiColorClass" />
        <!-- Tooltip -->
        <div
          class="absolute left-full ml-2 px-2 py-1 bg-gray-800 text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none z-50"
        >
          {{ wifiTooltip }}
        </div>
      </button>

      <!-- Battery Indicator -->
      <div
        class="w-[36px] h-[36px] flex items-center justify-center text-text-secondary relative group"
      >
        <component :is="batteryIcon" :size="18" stroke-width="2" :class="batteryColorClass" />
        <!-- Tooltip -->
        <div
          class="absolute left-full ml-2 px-2 py-1 bg-gray-800 text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none z-50"
        >
          {{ batteryPercentage }}%
        </div>
      </div>

      <button
        @click="isSettingsOpen = true"
        class="w-[36px] h-[36px] rounded-[7px] flex items-center justify-center text-text-secondary hover:text-text-primary hover:bg-gray-100 transition-all group relative cursor-pointer"
        :class="{ 'bg-nav-active text-text-primary': isSettingsOpen }"
      >
        <Settings :size="18" stroke-width="2" />
        <!-- Tooltip -->
        <div
          class="absolute left-full ml-2 px-2 py-1 bg-gray-800 text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none z-50"
        >
          {{ $t('common.settings') }}
        </div>
      </button>
    </div>
  </aside>

  <SettingsModal v-model:isOpen="isSettingsOpen" @close="isSettingsOpen = false" />

  <!-- Wifi Manager Modal -->
  <Transition
    enter-active-class="transition duration-200 ease-out"
    enter-from-class="opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="transition duration-150 ease-in"
    leave-from-class="opacity-100"
    leave-to-class="opacity-0"
  >
    <div v-if="isWifiManagerOpen" class="fixed inset-0 z-[100] flex items-center justify-center">
      <div
        class="absolute inset-0 bg-black/50 backdrop-blur-sm"
        @click="isWifiManagerOpen = false"
      ></div>
      <div class="relative z-10 w-[400px]">
        <WifiManager @close="isWifiManagerOpen = false" />
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import {
  Compass,
  Sliders,
  Files,
  Settings,
  Battery,
  BatteryLow,
  BatteryMedium,
  BatteryFull,
  BatteryCharging,
  Wifi,
  WifiOff,
} from 'lucide-vue-next'
import { useDeviceStore } from '@/stores/device'
import { useWifiStore } from '@/stores/wifi'
import { useSerialStore } from '@/stores/serial'
import { storeToRefs } from 'pinia'
import { computed, ref, onMounted, onUnmounted } from 'vue'
import SettingsModal from '@/components/SettingsModal.vue'
import WifiManager from '@/components/WifiManager.vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const store = useDeviceStore()
const wifiStore = useWifiStore()
const serialStore = useSerialStore()
const { isConnected, battery } = storeToRefs(store)
const { status: wifiStatus, testResult: wifiTestResult } = storeToRefs(wifiStore)

const isSettingsOpen = ref(false)
const isWifiManagerOpen = ref(false)
let wifiInterval: number | null = null

onMounted(() => {
  // Initial check
  wifiStore.updateStatus()
  wifiStore.testConnection()
  serialStore.startPolling()

  // Poll every 5 seconds
  wifiInterval = window.setInterval(() => {
    wifiStore.updateStatus()
    if (wifiStatus.value.connected) {
      wifiStore.testConnection()
    }
  }, 5000)
})

onUnmounted(() => {
  if (wifiInterval) {
    clearInterval(wifiInterval)
  }
  serialStore.stopPolling()
})

const serialIndicatorClass = computed(() => {
  const s = serialStore.statusString
  if (s === 'Active') return 'bg-green-500 shadow-[0_0_4px_rgba(34,197,94,0.4)]'
  if (['Handshaking', 'WaitingForFirstMessage'].includes(s)) return 'bg-blue-500 animate-pulse shadow-[0_0_4px_rgba(59,130,246,0.4)]'
  return 'bg-red-500 animate-pulse shadow-[0_0_4px_rgba(239,68,68,0.4)]'
})

const serialTooltip = computed(() => {
  const s = serialStore.statusString
  if (s === 'Active') return 'Serial: Connected'
  if (['Handshaking', 'WaitingForFirstMessage'].includes(s)) return `Serial: ${s}`
  if (s === 'Failed' && serialStore.error) return `Serial Error: ${serialStore.error}`
  return `Serial: ${s}`
})

const handleSerialClick = () => {
  const s = serialStore.statusString
  // Red or Blue status
  if (
    s === 'Failed' ||
    s === 'Idle' ||
    s === 'Connecting' ||
    ['Handshaking', 'WaitingForFirstMessage'].includes(s)
  ) {
    if (confirm('Restart serial handshake?')) {
      serialStore.restart()
    }
  }
}

const menuItems = computed(() => [
  { label: t('nav.overview'), path: '/', icon: Compass },
  { label: t('nav.sensor'), path: '/control', icon: Sliders },
  { label: t('nav.data'), path: '/analysis', icon: Files },
])

const batteryPercentage = computed(() => Math.round(battery.value.soc))

const batteryIcon = computed(() => {
  const soc = batteryPercentage.value
  // Assuming negative value implies charging if we had that data, but for now just levels
  if (soc >= 90) return BatteryFull
  if (soc >= 50) return BatteryMedium
  if (soc >= 20) return BatteryLow
  return Battery // Use default/empty-ish for very low
})

const batteryColorClass = computed(() => {
  const soc = batteryPercentage.value
  if (soc >= 50) return 'text-green-500'
  if (soc >= 20) return 'text-yellow-500'
  return 'text-red-500'
})

const wifiIcon = computed(() => {
  // If test passes, force Wifi icon (no slash)
  if (wifiTestResult.value === true) return Wifi
  return wifiStatus.value.connected ? Wifi : WifiOff
})

const wifiColorClass = computed(() => {
  // Requirement 1: If internet test passes, mark as green regardless of wifiStatus.connected check
  // (Assuming testResult implies connection is working)
  if (wifiTestResult.value === true) return 'text-green-500'
  
  if (!wifiStatus.value.connected) return 'text-red-500'
  // Connected but no internet or testing
  return 'text-yellow-500'
})

const wifiTooltip = computed(() => {
  if (!wifiStatus.value.connected) return 'Disconnected'
  const status = wifiTestResult.value ? 'Online' : 'Offline'
  return `${wifiStatus.value.ssid || 'WiFi'} (${status})`
})
</script>

<style scoped>
/* Ensure the layout distributes space correctly */
aside {
  /* This aligns with Figma's vertical layout */
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
}
</style>
