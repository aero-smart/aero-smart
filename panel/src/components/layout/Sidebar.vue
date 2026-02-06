<template>
  <aside class="w-20 bg-surface flex flex-col items-center py-6 gap-6 border-r border-slate-700 h-full z-50">
    <div class="mb-2">
      <!-- Logo or Brand -->
      <div class="w-12 h-12 bg-primary rounded-xl flex items-center justify-center text-white font-bold text-xl shadow-lg shadow-blue-500/20">
        AS
      </div>
    </div>
    
    <nav class="flex-1 flex flex-col gap-4 w-full px-2">
      <router-link 
        v-for="item in menuItems" 
        :key="item.path" 
        :to="item.path"
        class="flex flex-col items-center justify-center p-3 rounded-xl transition-all duration-200 text-slate-400 hover:text-white hover:bg-slate-700/50"
        active-class="bg-primary text-white shadow-lg shadow-blue-900/50 scale-105"
      >
        <component :is="item.icon" :size="24" stroke-width="2" />
        <span class="text-[10px] mt-1 font-medium tracking-wide">{{ item.label }}</span>
      </router-link>
    </nav>

    <div class="mt-auto flex flex-col gap-4 w-full px-2 mb-2">
       <div 
        class="flex flex-col items-center justify-center p-2 rounded-xl bg-slate-900/50 border border-slate-800"
        :class="isConnected ? 'text-success border-green-900/30' : 'text-danger border-red-900/30'"
       >
          <Activity :size="20" class="animate-pulse" v-if="isConnected" />
          <Activity :size="20" v-else />
          <span class="text-[9px] mt-1 font-mono uppercase">{{ isConnected ? 'LINK' : 'LOST' }}</span>
       </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { LayoutDashboard, Gamepad2, Activity, Settings, BarChart3 } from 'lucide-vue-next';
import { useDeviceStore } from '@/stores/device';
import { storeToRefs } from 'pinia';

const store = useDeviceStore();
const { isConnected } = storeToRefs(store);

const menuItems = [
  { label: 'Dash', path: '/', icon: LayoutDashboard },
  { label: 'Control', path: '/control', icon: Gamepad2 },
  { label: 'Analyze', path: '/analysis', icon: BarChart3 },
  { label: 'Config', path: '/settings', icon: Settings },
];
</script>
