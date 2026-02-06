<template>
  <div class="flex flex-col h-full p-4 gap-4">
    <!-- Top Tabs -->
    <div class="flex gap-2 mb-2">
       <button 
         v-for="tab in tabs" 
         :key="tab"
         @click="currentTab = tab"
         class="px-6 py-2 rounded-full text-sm font-medium transition-all"
         :class="currentTab === tab ? 'bg-primary text-white shadow-lg shadow-blue-500/30' : 'bg-surface text-slate-400 hover:bg-slate-700'"
       >
         {{ tab }}
       </button>
    </div>

    <!-- Content -->
    <div class="flex-1 grid grid-cols-12 gap-4 min-h-0">
      <!-- Left Column: Flight Data -->
      <div class="col-span-3 flex flex-col gap-4">
        <!-- Airspeed Card -->
        <div class="bg-surface rounded-2xl p-4 flex flex-col items-center justify-center flex-1 border border-slate-700/50 shadow-sm">
           <span class="text-slate-400 text-xs uppercase tracking-wider mb-2 font-medium">Airspeed</span>
           <div class="text-4xl font-bold font-mono text-white tracking-tighter">{{ airspeed.toFixed(1) }}</div>
           <span class="text-slate-500 text-xs mt-1">m/s</span>
        </div>
        
        <!-- Battery Card -->
        <div class="bg-surface rounded-2xl p-4 flex flex-col gap-3 border border-slate-700/50 shadow-sm">
           <div class="flex justify-between items-center">
             <span class="text-slate-400 text-xs uppercase tracking-wider font-medium">Battery</span>
             <span class="text-xs font-mono font-bold" :class="battery.soc > 20 ? 'text-success' : 'text-danger'">{{ battery.soc.toFixed(0) }}%</span>
           </div>
           <div class="w-full bg-slate-900 h-2 rounded-full overflow-hidden shadow-inner">
             <div class="h-full bg-gradient-to-r from-green-500 to-emerald-400 transition-all duration-500" :style="{ width: `${battery.soc}%` }"></div>
           </div>
           <div class="flex justify-between text-xs text-slate-500 font-mono">
             <span>{{ battery.voltage.toFixed(1) }}V</span>
             <span>3S Lipo</span>
           </div>
        </div>
      </div>

      <!-- Center Column: Attitude (Placeholder for WebGL/Canvas) -->
      <div class="col-span-6 bg-slate-900 rounded-2xl border border-slate-800 relative overflow-hidden flex items-center justify-center shadow-inner">
          <div class="absolute inset-0 bg-[radial-gradient(circle_at_center,_var(--tw-gradient-stops))] from-blue-900/10 to-transparent"></div>
          <!-- Simple CSS Horizon -->
          <div class="relative w-48 h-48 rounded-full border-4 border-slate-700 overflow-hidden bg-sky-600/20 shadow-2xl">
             <div class="absolute inset-0 bg-amber-600/30 translate-y-1/2 transition-transform duration-100 ease-out origin-center"
                  :style="{ transform: `rotate(${-imu.attitude.roll}deg) translateY(${imu.attitude.pitch * 2}px)` }">
             </div>
             <div class="absolute inset-0 flex items-center justify-center pointer-events-none">
                <div class="w-full h-[2px] bg-white/30 shadow-sm"></div>
                <div class="absolute h-full w-[2px] bg-white/30 shadow-sm"></div>
                <!-- Center Dot -->
                <div class="absolute w-2 h-2 bg-white rounded-full shadow-lg"></div>
             </div>
          </div>
          <div class="absolute bottom-4 text-center">
             <div class="text-xs text-slate-400 uppercase tracking-widest mb-1 font-bold opacity-50">Attitude</div>
             <div class="font-mono text-xs text-slate-500">R: {{ imu.attitude.roll.toFixed(1) }}° P: {{ imu.attitude.pitch.toFixed(1) }}°</div>
          </div>
      </div>

      <!-- Right Column: Environment & Status -->
      <div class="col-span-3 flex flex-col gap-4">
         <div class="bg-surface rounded-2xl p-4 flex-1 border border-slate-700/50 flex flex-col gap-4 shadow-sm">
            <div class="flex items-center justify-between border-b border-slate-700/50 pb-2">
              <span class="text-xs text-slate-400 uppercase font-medium">Temp</span>
              <span class="font-mono text-white font-bold">{{ env.temperature.toFixed(1) }}°C</span>
            </div>
            <div class="flex items-center justify-between border-b border-slate-700/50 pb-2">
              <span class="text-xs text-slate-400 uppercase font-medium">Press</span>
              <span class="font-mono text-white font-bold">{{ (env.pressure / 100).toFixed(0) }} hPa</span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-xs text-slate-400 uppercase font-medium">Lidar</span>
              <span class="font-mono text-primary font-bold">{{ lidar.distance }} cm</span>
            </div>
         </div>
         
         <div class="bg-surface rounded-2xl p-4 border border-slate-700/50 shadow-sm flex flex-col justify-center">
            <div class="text-xs text-slate-400 uppercase mb-2 text-center font-medium">System Status</div>
            <div class="px-3 py-2 rounded-lg bg-slate-900 text-center font-bold text-sm tracking-wide border border-slate-800"
                 :class="{
                   'text-green-400 shadow-[0_0_10px_rgba(74,222,128,0.2)]': status === 'Running',
                   'text-yellow-400': status === 'Initializing',
                   'text-blue-400': status === 'Idle',
                   'text-red-400': status === 'Error'
                 }">
               {{ status }}
            </div>
         </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useDeviceStore } from '@/stores/device';
import { storeToRefs } from 'pinia';

const store = useDeviceStore();
const { airspeed, battery, imu, env, lidar, status } = storeToRefs(store);

const tabs = ['Overview', 'Sensors', 'Power'];
const currentTab = ref('Overview');
</script>
