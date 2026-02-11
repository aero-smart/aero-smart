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
          Settings
        </div>
      </button>
    </div>
  </aside>

  <SettingsModal v-model:isOpen="isSettingsOpen" @close="isSettingsOpen = false" />
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
} from 'lucide-vue-next'
import { useDeviceStore } from '@/stores/device'
import { storeToRefs } from 'pinia'
import { computed, ref } from 'vue'
import SettingsModal from '@/components/SettingsModal.vue'

const store = useDeviceStore()
const { isConnected, battery } = storeToRefs(store)

const isSettingsOpen = ref(false)

const menuItems = [
  { label: 'Overview', path: '/', icon: Compass },
  { label: 'Sensor', path: '/control', icon: Sliders },
  { label: 'Data', path: '/analysis', icon: Files },
]

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
