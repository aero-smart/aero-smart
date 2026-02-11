<template>
  <div class="flex flex-col h-full p-4 gap-4 bg-[#f5f5f5] text-[#423d3c] font-sans">
    <!-- Top Tabs -->
    <div class="flex items-center gap-1 bg-white w-fit p-1.5 rounded-full border border-gray-200 shadow-[0_10px_24px_rgba(0,0,0,0.12)]">
      <button
        v-for="tab in ['Overview', 'Sensor', 'Power']"
        :key="tab"
        @click="currentTab = tab"
        class="px-4 py-1.5 text-[11px] font-semibold rounded-full transition-all duration-200"
        :class="
<<<<<<< HEAD
          currentTab === tab
            ? 'bg-black text-white shadow-[0_10px_22px_rgba(0,0,0,0.35)]'
            : 'text-gray-500 hover:text-gray-700 hover:bg-gray-100'
=======
          currentTab === tab ? 'bg-white text-black shadow-sm' : 'text-gray-500 hover:text-gray-700'
>>>>>>> d161b09ed0a35c904f723c5685a1faf4ad736a2b
        "
      >
        {{ tab }}
      </button>
    </div>

    <!-- Main Content Grid -->
    <div class="flex-1 grid grid-cols-12 gap-4 min-h-0">
      <!-- Left Column (Wind Input + Wind Speed) -->
      <div class="col-span-3 flex flex-col gap-4">
        <!-- Wind Input -->
        <div class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4">
          <div class="text-xs font-bold text-gray-700">Wind Input</div>

          <div class="flex flex-col gap-2">
            <div class="flex justify-between items-center">
              <span class="text-[11px] text-gray-500 font-medium">Target Speed</span>
              <div class="flex items-baseline gap-1">
                <input
                  type="number"
                  v-model.number="targetSpeed"
                  @change="updateSpeed"
                  class="w-12 text-right bg-transparent border-b border-gray-300 text-sm font-bold focus:outline-none focus:border-black transition-colors"
                  placeholder="0"
                />
                <span class="text-xs text-gray-400">m/s</span>
              </div>
            </div>
          </div>

          <div class="bg-gray-100 rounded-lg p-3 flex flex-col gap-2">
            <div class="flex items-center justify-between">
              <div class="text-[11px] text-gray-500 font-medium">Current Speed</div>
              <div class="flex items-baseline gap-1">
                <span class="text-xl font-bold text-gray-800">{{ airspeed.toFixed(1) }}</span>
                <span class="text-xs text-gray-500">m/s</span>
              </div>
            </div>
            <!-- Segmented Progress Bar (Monochrome) -->
            <div class="flex gap-[2px] h-4 w-full">
              <div
                v-for="i in 30"
                :key="i"
                class="flex-1 rounded-sm transition-colors duration-200"
                :class="i <= (airspeed / 30) * 30 ? 'bg-gray-800' : 'bg-gray-300'"
              ></div>
            </div>
          </div>
        </div>

        <!-- Wind Speed Display -->
<<<<<<< HEAD
        <div class="bg-white rounded-2xl p-3 shadow-sm border border-white flex-1 flex flex-col gap-3">
           <div class="text-xs font-bold text-gray-700">Wind Speed Display</div>
           
           <!-- Gauge -->
           <div class="relative w-full flex-1 min-h-[160px] aspect-[4/3] bg-gray-100 rounded-xl overflow-hidden">
              <div ref="gaugeChartEl" class="absolute inset-0 w-full h-full z-0"></div>
           </div>
=======
        <div
          class="bg-white rounded-2xl p-3 shadow-sm border border-white flex-1 flex flex-col gap-3"
        >
          <div class="text-xs font-bold text-gray-700">Wind Speed Display</div>
>>>>>>> d161b09ed0a35c904f723c5685a1faf4ad736a2b

          <!-- Gauge -->
          <div class="flex-1 bg-gray-100 rounded-xl relative min-h-[120px] w-full overflow-hidden">
            <div ref="gaugeChartEl" class="absolute inset-0 w-full h-full z-0"></div>
          </div>

          <!-- History Trend -->
          <div class="h-24 bg-white border border-gray-100 rounded-xl p-2 flex flex-col">
            <div class="text-[10px] text-gray-400 mb-1 flex-shrink-0">History Trend</div>
            <div class="flex-1 relative min-h-0 w-full">
              <div ref="trendChartEl" class="absolute inset-0 w-full h-full"></div>
            </div>
          </div>

          <!-- Bottom Info -->
          <div class="grid grid-cols-2 gap-3">
            <div class="bg-gray-50 p-2 rounded-lg border border-gray-100">
              <span class="text-[10px] text-gray-500 block">Wind Direction</span>
              <span class="text-sm font-bold text-gray-800">19°</span>
            </div>
            <div class="bg-gray-50 p-2 rounded-lg border border-gray-100">
              <span class="text-[10px] text-gray-500 block">Avg Wind Speed</span>
              <span class="text-sm font-bold text-gray-800"
                >{{ stats.avgSpeed.toFixed(1) }} m/s</span
              >
            </div>
          </div>
        </div>
      </div>

      <!-- Middle Column (Pressure Monitor) -->
      <div class="col-span-5 flex flex-col gap-4">
<<<<<<< HEAD
        <div class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4 h-full">
           <div class="text-xs font-bold text-gray-700">Pressure Difference Monitor</div>
           
           <!-- Big Number -->
           <div class="relative overflow-hidden rounded-2xl p-6 bg-gradient-to-br from-white via-gray-50 to-gray-100 border border-gray-200/70 shadow-[0_6px_20px_rgba(0,0,0,0.06)] flex flex-col items-center justify-center gap-2">
             <div class="absolute inset-0 pointer-events-none">
               <div class="absolute -top-12 -left-12 h-28 w-28 rounded-full bg-white/70 blur-2xl"></div>
               <div class="absolute -bottom-12 -right-12 h-32 w-32 rounded-full bg-white/60 blur-2xl"></div>
             </div>
             <div class="relative z-10 text-[10px] uppercase tracking-[0.2em] text-gray-400 font-semibold">Current Pressure Diff</div>
             <div class="relative z-10 flex items-end gap-3">
               <span class="text-5xl font-semibold text-gray-900 tracking-tight tabular-nums">{{ pressureDiff.toFixed(2) }}</span>
               <span class="text-[11px] text-gray-500 font-semibold px-2 py-0.5 rounded-full bg-white/70 border border-gray-200">Pa</span>
             </div>
           </div>
=======
        <div
          class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4 h-full"
        >
          <div class="text-xs font-bold text-gray-700">Pressure Difference Monitor</div>
>>>>>>> d161b09ed0a35c904f723c5685a1faf4ad736a2b

          <!-- Big Number -->
          <div class="bg-gray-100 rounded-xl p-6 flex flex-col items-center justify-center">
            <div class="text-xs text-gray-500 font-medium mb-1">Current Pressure Diff</div>
            <div class="flex items-baseline gap-2">
              <span class="text-4xl font-bold text-gray-800 tracking-tight">{{
                pressureDiff.toFixed(2)
              }}</span>
              <span class="text-lg text-gray-500 font-medium">Pa</span>
            </div>
          </div>

          <!-- Waveform Chart -->
          <div class="flex-1 min-h-0 bg-white border border-gray-100 rounded-xl p-2 flex flex-col">
            <div class="text-[10px] text-gray-400 mb-1 flex-shrink-0">Real-time Waveform</div>
            <div class="flex-1 relative min-h-0 w-full">
              <div ref="waveformChartEl" class="absolute inset-0 w-full h-full"></div>
            </div>
          </div>

          <!-- Data Stats -->
          <div class="grid grid-cols-3 gap-3 pt-3 border-t border-gray-100">
            <div>
              <span class="text-xl font-bold text-gray-900 block tracking-tight">{{
                stats.max.toFixed(1)
              }}</span>
              <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold"
                >Max Value</span
              >
            </div>
            <div>
              <span class="text-xl font-bold text-gray-900 block tracking-tight">{{
                stats.min.toFixed(1)
              }}</span>
              <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold"
                >Min Value</span
              >
            </div>
            <div>
              <span class="text-xl font-bold text-gray-900 block tracking-tight">{{
                stats.avg.toFixed(1)
              }}</span>
              <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold"
                >Average</span
              >
            </div>
          </div>
        </div>
      </div>

      <!-- Right Column (Sensor Output + IMU) -->
      <div class="col-span-4 flex flex-col gap-4">
        <!-- Sensor Output (Refactored to match Reference Card Layout) -->
        <div
          class="bg-white rounded-2xl p-4 shadow-sm border border-white relative overflow-hidden"
        >
          <!-- Decorative Gradient Background (Subtle B&W) -->
          <div
            class="absolute inset-0 bg-gradient-to-br from-gray-50 to-white opacity-50 z-0 pointer-events-none"
          ></div>

          <div class="relative z-10 flex flex-col gap-4">
            <!-- Header -->
            <div class="text-xs font-bold text-gray-700">Sensor Output</div>

            <!-- Main Big Metric (Pressure 1) -->
            <div class="flex flex-col gap-2">
<<<<<<< HEAD
               <div>
                 <div class="flex items-baseline gap-1">
                   <span class="text-4xl font-black text-gray-900 tracking-tighter leading-none">{{ (env.pressure).toFixed(0) }}</span>
                   <span class="text-xs text-gray-500 font-bold mb-1">Pa</span>
                 </div>
               </div>
=======
              <div>
                <div class="flex items-baseline gap-1">
                  <span class="text-4xl font-black text-gray-900 tracking-tighter leading-none">{{
                    env.pressure.toFixed(0)
                  }}</span>
                  <span class="text-xs text-gray-500 font-bold mb-1">Pa</span>
                </div>
                <span class="text-[10px] text-gray-400 font-medium mt-1 block"
                  >Main Pressure (Abs)</span
                >
              </div>

              <!-- Decorative Bar Visual -->
              <div class="flex gap-1 h-8 items-end opacity-20 select-none">
                <div class="flex-1 bg-black rounded-sm h-[40%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[70%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[50%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[80%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[60%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[90%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[45%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[75%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[55%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[85%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[65%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[95%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[40%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[70%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[50%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[80%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[60%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[90%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[50%]"></div>
                <div class="flex-1 bg-black rounded-sm h-[70%]"></div>
              </div>
>>>>>>> d161b09ed0a35c904f723c5685a1faf4ad736a2b
            </div>

            <!-- Bottom Metrics Row -->
            <div class="grid grid-cols-3 gap-3 pt-3 border-t border-gray-100">
              <!-- Pressure 2 -->
              <div>
                <span class="text-xl font-bold text-gray-900 block tracking-tight">{{
                  (env.pressure - pressureDiff).toFixed(0)
                }}</span>
                <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold"
                  >Pressure 2</span
                >
              </div>
              <!-- Speed 1 -->
              <div>
                <span class="text-xl font-bold text-gray-900 block tracking-tight">{{
                  airspeed.toFixed(2)
                }}</span>
                <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold"
                  >Speed 1</span
                >
              </div>
              <!-- Speed 2 -->
              <div>
                <span class="text-xl font-bold text-gray-900 block tracking-tight">{{
                  (airspeed * 1.1).toFixed(2)
                }}</span>
                <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold"
                  >Speed 2</span
                >
              </div>
            </div>
          </div>
        </div>

        <!-- IMU Visualization -->
        <div
          class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4 flex-1"
        >
          <div class="text-xs font-bold text-gray-700">IMU Visualization</div>
          <div class="flex-1 flex items-center justify-center perspective-container">
            <div class="cube" :style="cubeStyle">
              <div class="face front"></div>
              <div class="face back"></div>
              <div class="face right"></div>
              <div class="face left"></div>
              <div class="face top"></div>
              <div class="face bottom"></div>
            </div>
          </div>
          <div class="grid grid-cols-3 gap-2">
            <div class="bg-gray-50 p-2 rounded-lg border border-gray-100">
              <div class="text-[10px] text-gray-500 font-medium">Roll</div>
              <div class="text-sm font-bold">{{ imu.attitude.roll.toFixed(1) }}°</div>
            </div>
            <div class="bg-gray-50 p-2 rounded-lg border border-gray-100">
              <div class="text-[10px] text-gray-500 font-medium">Pitch</div>
              <div class="text-sm font-bold">{{ imu.attitude.pitch.toFixed(1) }}°</div>
            </div>
            <div class="bg-gray-50 p-2 rounded-lg border border-gray-100">
              <div class="text-[10px] text-gray-500 font-medium">Yaw</div>
              <div class="text-sm font-bold">{{ imu.attitude.yaw.toFixed(1) }}°</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { storeToRefs } from 'pinia'
import * as echarts from 'echarts'

const store = useDeviceStore()
const { imu, airspeed, pressureDiff, env, isConnected } = storeToRefs(store)

const currentTab = ref('Overview')
const targetSpeed = ref(0)

// Simulation for Demo
let simTimer: number | null = null

function startSimulation() {
  if (simTimer) return
  simTimer = window.setInterval(() => {
    if (isConnected.value) return

    // Simulate Pressure Diff (sine wave + noise)
    const time = Date.now() / 1000
    pressureDiff.value = 30 + Math.sin(time) * 10 + (Math.random() - 0.5) * 2

    // Simulate Airspeed (correlated with pressure)
    airspeed.value = 5 + Math.sin(time * 0.5) * 2 + (Math.random() - 0.5) * 0.5

    // Simulate IMU
    imu.value.attitude.roll = Math.sin(time * 0.5) * 10
    imu.value.attitude.pitch = Math.cos(time * 0.3) * 10
    imu.value.attitude.yaw += 0.1
  }, 100)
}

// Stats
const stats = ref({
  max: 0,
  min: 0,
  avg: 0,
  avgSpeed: 0,
})

const pressureHistory = ref<{ value: number; time: string }[]>([])
const speedHistory = ref<{ value: number; time: string }[]>([])
const maxHistoryLength = 100

// Chart Elements
const waveformChartEl = ref<HTMLElement | null>(null)
const gaugeChartEl = ref<HTMLElement | null>(null)
const trendChartEl = ref<HTMLElement | null>(null)

let waveformChart: echarts.ECharts | null = null
let gaugeChart: echarts.ECharts | null = null
let trendChart: echarts.ECharts | null = null
let resizeObserver: ResizeObserver | null = null

// Cube Style for IMU
const cubeStyle = computed(() => {
  const { roll, pitch, yaw } = imu.value.attitude
  return {
    transform: `rotateX(${-pitch}deg) rotateY(${yaw}deg) rotateZ(${-roll}deg)`,
  }
})

function updateSpeed() {
  store.setThrottle(targetSpeed.value)
}

// Chart Options
const commonChartOptions = {
  grid: { top: 15, right: 30, bottom: 25, left: 45 },
  xAxis: {
    type: 'category',
    show: true,
    axisLabel: { fontSize: 10, color: '#999' },
    boundaryGap: false,
  },
  yAxis: {
    type: 'value',
    splitLine: { show: true, lineStyle: { type: 'dashed', color: '#eee' } },
    axisLabel: { fontSize: 11, color: '#666', margin: 10 },
  },
  animation: false,
}

onMounted(() => {
  initCharts()
  requestAnimationFrame(() => {
    handleResize()
  })
  window.addEventListener('resize', handleResize)
  if ('ResizeObserver' in window) {
    resizeObserver = new ResizeObserver(() => {
      handleResize()
    })
    if (waveformChartEl.value) resizeObserver.observe(waveformChartEl.value)
    if (gaugeChartEl.value) resizeObserver.observe(gaugeChartEl.value)
    if (trendChartEl.value) resizeObserver.observe(trendChartEl.value)
  }
  // Start simulation if not connected
  if (!isConnected.value) {
    startSimulation()
  }
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
  resizeObserver?.disconnect()
  if (simTimer) clearInterval(simTimer)
  waveformChart?.dispose()
  gaugeChart?.dispose()
  trendChart?.dispose()
})

function handleResize() {
  waveformChart?.resize()
  gaugeChart?.resize()
  trendChart?.resize()
}

function initCharts() {
  if (waveformChartEl.value) {
    waveformChart = echarts.init(waveformChartEl.value)
    waveformChart.setOption({
      ...commonChartOptions,
      series: [
        {
          data: [],
          type: 'line',
          smooth: true,
          showSymbol: false,
          lineStyle: { width: 2, color: '#333333' },
          areaStyle: {
            color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
              { offset: 0, color: 'rgba(0,0,0,0.1)' },
              { offset: 1, color: 'rgba(0,0,0,0.0)' },
            ]),
          },
        },
      ],
    })
  }

  if (gaugeChartEl.value) {
    gaugeChart = echarts.init(gaugeChartEl.value)
    gaugeChart.setOption({
<<<<<<< HEAD
      series: [{
        type: 'gauge',
        center: ['50%', '60%'],
        radius: '90%',
        startAngle: 200,
        endAngle: -20,
        min: 0,
        max: 30,
        splitNumber: 6,
        itemStyle: { color: '#333' },
        progress: { show: true, width: 8 },
        pointer: { show: true, length: '60%', width: 4 },
        axisLine: { lineStyle: { width: 8, color: [[1, '#e5e7eb']] } },
        axisTick: { distance: -12, length: 4, lineStyle: { color: '#999', width: 1 } },
        splitLine: { distance: -12, length: 8, lineStyle: { color: '#999', width: 2 } },
        axisLabel: { distance: -16, color: '#666', fontSize: 10 },
        anchor: { show: true, showAbove: true, size: 8, itemStyle: { borderWidth: 2, borderColor: '#333' } },
        detail: {
          valueAnimation: true,
          fontSize: 30,
          fontWeight: 'bold',
          color: '#1f2937',
          offsetCenter: [0, '40%'],
          formatter: (val: number) => `{value|${val.toFixed(1)}}\n{unit|m/s}`,
          rich: {
            value: {
              fontSize: 30,
              fontWeight: 'bold',
              color: '#1f2937',
              lineHeight: 30
=======
      series: [
        {
          type: 'gauge',
          center: ['50%', '55%'],
          radius: '100%',
          startAngle: 200,
          endAngle: -20,
          min: 0,
          max: 30,
          splitNumber: 6,
          itemStyle: { color: '#333' },
          progress: { show: true, width: 8 },
          pointer: { show: true, length: '60%', width: 4 },
          axisLine: { lineStyle: { width: 8, color: [[1, '#e5e7eb']] } },
          axisTick: { distance: -12, length: 4, lineStyle: { color: '#999', width: 1 } },
          splitLine: { distance: -12, length: 8, lineStyle: { color: '#999', width: 2 } },
          axisLabel: { distance: -16, color: '#666', fontSize: 10 },
          anchor: {
            show: true,
            showAbove: true,
            size: 8,
            itemStyle: { borderWidth: 2, borderColor: '#333' },
          },
          detail: {
            valueAnimation: true,
            fontSize: 30,
            fontWeight: 'bold',
            color: '#1f2937',
            offsetCenter: [0, '40%'],
            formatter: (val: number) => `{value|${val.toFixed(1)}}\n{unit|m/s}`,
            rich: {
              value: {
                fontSize: 30,
                fontWeight: 'bold',
                color: '#1f2937',
                lineHeight: 30,
              },
              unit: {
                fontSize: 12,
                color: '#6b7280',
                lineHeight: 20,
                padding: [5, 0, 0, 0],
              },
>>>>>>> d161b09ed0a35c904f723c5685a1faf4ad736a2b
            },
          },
          data: [{ value: 0 }],
        },
      ],
    })
  }

  if (trendChartEl.value) {
    trendChart = echarts.init(trendChartEl.value)
    trendChart.setOption({
      ...commonChartOptions,
      series: [
        {
          data: [],
          type: 'line',
          smooth: true,
          showSymbol: false,
          lineStyle: { width: 1.5, color: '#666' },
        },
      ],
    })
  }
}

// Update Loops
watch([pressureDiff, airspeed], () => {
  const p = pressureDiff.value
  const s = airspeed.value

  // Update History
  const now = new Date().toLocaleTimeString('en-US', { hour12: false })
  pressureHistory.value.push({ value: p, time: now })
  speedHistory.value.push({ value: s, time: now })

  if (pressureHistory.value.length > maxHistoryLength) pressureHistory.value.shift()
  if (speedHistory.value.length > maxHistoryLength) speedHistory.value.shift()

  // Update Stats
  const pValues = pressureHistory.value.map((d) => d.value)
  const sValues = speedHistory.value.map((d) => d.value)

  stats.value.max = Math.max(...pValues)
  stats.value.min = Math.min(...pValues)
  stats.value.avg = pValues.reduce((a, b) => a + b, 0) / pValues.length
  stats.value.avgSpeed = sValues.reduce((a, b) => a + b, 0) / sValues.length

  // Update Charts
  waveformChart?.setOption({
    xAxis: { data: pressureHistory.value.map((d) => d.time) },
    series: [{ data: pValues }],
  })
  gaugeChart?.setOption({ series: [{ data: [{ value: s }] }] })
  trendChart?.setOption({
    xAxis: { data: speedHistory.value.map((d) => d.time) },
    series: [{ data: sValues }],
  })
})
</script>

<style scoped>
.perspective-container {
  perspective: 800px;
}

.cube {
  width: 100px;
  height: 100px;
  position: relative;
  transform-style: preserve-3d;
  transition: transform 0.1s linear;
}

.face {
  position: absolute;
  width: 100px;
  height: 100px;
  background: rgba(255, 255, 255, 0.9);
  border: 2px solid #333;
  opacity: 0.8;
}

<<<<<<< HEAD
.front  { transform: rotateY(0deg) translateZ(50px); }
.back   { transform: rotateY(180deg) translateZ(50px); }
.right  { transform: rotateY(90deg) translateZ(50px); }
.left   { transform: rotateY(-90deg) translateZ(50px); }
.top    { transform: rotateX(90deg) translateZ(50px); }
.bottom { transform: rotateX(-90deg) translateZ(50px); }
=======
.front {
  transform: rotateY(0deg) translateZ(50px);
}
.back {
  transform: rotateY(180deg) translateZ(50px);
}
.right {
  transform: rotateY(90deg) translateZ(50px);
}
.left {
  transform: rotateY(-90deg) translateZ(50px);
}
.top {
  transform: rotateX(90deg) translateZ(50px);
}
.bottom {
  transform: rotateX(-90deg) translateZ(50px);
}
>>>>>>> d161b09ed0a35c904f723c5685a1faf4ad736a2b
</style>
