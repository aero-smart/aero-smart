<template>
  <aside
    class="w-[40px] bg-surface flex flex-col items-center py-0 h-full border-r border-gray-200 z-50 select-none justify-between"
  >
    <!-- Top: Brand/Logo (Frame 7) -->
    <div class="w-[40px] h-[40px] flex items-center justify-center mt-1">
      <svg
        width="18"
        height="28"
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
        class="w-[29px] h-[29px] rounded-[5px] flex items-center justify-center transition-all duration-200 group relative"
        :class="
          $route.path === item.path
            ? 'bg-nav-active text-text-primary'
            : 'text-text-secondary hover:text-text-primary hover:bg-gray-100'
        "
      >
        <component :is="item.icon" :size="15" stroke-width="2" />

        <!-- Tooltip -->
        <div
          class="absolute left-full ml-2 px-2 py-1 bg-gray-800 text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none z-50"
        >
          {{ item.label }}
        </div>
      </router-link>
    </div>

    <!-- Bottom: Settings (Icons 4) -->
    <div class="w-[40px] h-[40px] flex flex-col items-center justify-center mb-1">
      <router-link
        to="/settings"
        class="w-[29px] h-[29px] rounded-[5px] flex items-center justify-center text-text-secondary hover:text-text-primary hover:bg-gray-100 transition-all group relative"
        active-class="bg-nav-active text-text-primary"
      >
        <Settings :size="15" stroke-width="2" />
        <!-- Tooltip -->
        <div
          class="absolute left-full ml-2 px-2 py-1 bg-gray-800 text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none z-50"
        >
          Settings
        </div>
      </router-link>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { Compass, Sliders, Files, Settings } from 'lucide-vue-next'
import { useDeviceStore } from '@/stores/device'
import { storeToRefs } from 'pinia'

const store = useDeviceStore()
const { isConnected } = storeToRefs(store)

const menuItems = [
  { label: 'Overview', path: '/', icon: Compass },
  { label: 'Sensor', path: '/control', icon: Sliders },
  { label: 'Power', path: '/analysis', icon: Files },
]
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
