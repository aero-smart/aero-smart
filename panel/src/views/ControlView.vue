<template>
  <div class="flex flex-col h-full p-6 gap-6">
     <div class="flex items-center justify-between mb-4">
        <h2 class="text-xl font-bold text-white tracking-tight flex items-center gap-3">
           <span class="w-2 h-8 bg-primary rounded-full"></span>
           Manual Control
        </h2>
        <div class="flex gap-2">
           <span class="text-xs text-slate-500 uppercase self-center mr-2 font-medium">Mode</span>
           <div class="px-3 py-1 bg-primary text-white text-xs font-bold rounded-md shadow-lg shadow-blue-500/20">MANUAL</div>
        </div>
     </div>

     <div class="grid grid-cols-2 gap-8 h-full">
        <!-- Throttle Control -->
        <div class="bg-surface rounded-2xl p-6 border border-slate-700/50 flex flex-col items-center gap-6 shadow-lg">
           <div class="w-full flex justify-between items-center">
              <span class="text-slate-400 font-medium uppercase text-sm tracking-wider">EDF Throttle</span>
              <span class="text-3xl font-mono font-bold text-primary">{{ controls.throttle }}</span>
           </div>
           
           <!-- Vertical Slider Container -->
           <div class="flex-1 w-32 bg-slate-900 rounded-3xl relative overflow-hidden touch-none shadow-inner border border-slate-800">
              <input 
                type="range" 
                min="0" 
                max="255" 
                v-model.number="controls.throttle" 
                @input="updateThrottle"
                class="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10"
                orient="vertical"
              />
              <div class="absolute bottom-0 left-0 w-full bg-gradient-to-t from-blue-600 via-cyan-500 to-cyan-400 transition-all duration-75"
                   :style="{ height: `${(controls.throttle / 255) * 100}%` }">
                   <!-- Glow effect -->
                   <div class="absolute top-0 left-0 w-full h-2 bg-white/50 blur-sm"></div>
              </div>
              
              <!-- Ticks -->
              <div class="absolute inset-0 flex flex-col justify-between py-6 px-4 pointer-events-none">
                 <div class="w-full h-[1px] bg-white/10"></div>
                 <div class="w-full h-[1px] bg-white/10"></div>
                 <div class="w-full h-[1px] bg-white/10"></div>
                 <div class="w-full h-[1px] bg-white/10"></div>
                 <div class="w-full h-[1px] bg-white/10"></div>
              </div>
           </div>
        </div>

        <!-- Servo Control -->
        <div class="bg-surface rounded-2xl p-6 border border-slate-700/50 flex flex-col gap-8 shadow-lg">
           <div class="w-full flex justify-between items-center">
              <span class="text-slate-400 font-medium uppercase text-sm tracking-wider">Servo Angle</span>
              <span class="text-3xl font-mono font-bold text-warning">{{ controls.servo }}°</span>
           </div>

           <!-- Knob / Horizontal Slider Container -->
           <div class="flex-1 flex flex-col justify-center gap-8">
              <div class="w-full relative h-16 bg-slate-900 rounded-full flex items-center px-2 shadow-inner border border-slate-800">
                 <input 
                    type="range" 
                    min="0" 
                    max="180" 
                    v-model.number="controls.servo"
                    @input="updateServo"
                    class="w-full z-10 opacity-0 absolute inset-0 cursor-pointer"
                 />
                 <!-- Track Fill -->
                 <div class="absolute left-2 top-2 bottom-2 bg-slate-800 rounded-full pointer-events-none" :style="{ right: `calc(100% - ${(controls.servo / 180) * 100}%)` }"></div>
                 
                 <!-- Thumb -->
                 <div class="absolute h-10 w-10 bg-warning rounded-full shadow-lg shadow-yellow-500/30 transition-all duration-75 pointer-events-none flex items-center justify-center border-4 border-slate-800"
                      :style="{ left: `calc(${(controls.servo / 180) * 100}% - 20px)` }">
                 </div>
              </div>
              
              <!-- Presets -->
              <div class="grid grid-cols-3 gap-4">
                  <button @click="setServo(0)" class="py-4 bg-slate-800 rounded-xl text-slate-400 font-bold hover:bg-slate-700 hover:text-white transition-all active:scale-95 shadow-sm border border-slate-700">0°</button>
                  <button @click="setServo(90)" class="py-4 bg-slate-800 rounded-xl text-slate-400 font-bold hover:bg-slate-700 hover:text-white transition-all active:scale-95 shadow-sm border border-slate-700">90°</button>
                  <button @click="setServo(180)" class="py-4 bg-slate-800 rounded-xl text-slate-400 font-bold hover:bg-slate-700 hover:text-white transition-all active:scale-95 shadow-sm border border-slate-700">180°</button>
              </div>
           </div>
        </div>
     </div>
  </div>
</template>

<script setup lang="ts">
import { useDeviceStore } from '@/stores/device';
import { storeToRefs } from 'pinia';

const store = useDeviceStore();
const { controls } = storeToRefs(store);

function updateThrottle() {
   store.setThrottle(controls.value.throttle);
}

function updateServo() {
   store.setServo(controls.value.servo);
}

function setServo(val: number) {
   controls.value.servo = val;
   updateServo();
}
</script>
