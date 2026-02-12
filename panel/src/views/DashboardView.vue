<template>
  <div class="flex flex-col h-full p-4 gap-4 bg-[#f5f5f5] text-[#423d3c] font-sans">
    <!-- Main Content Grid -->
    <div class="flex-1 grid grid-cols-12 gap-4 min-h-0 items-stretch">
      <!-- Left Column (Wind Input + Wind Speed) -->
      <div class="col-span-3 flex flex-col gap-4 min-h-0 overflow-y-auto">
        <!-- Wind Input -->
        <div class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4">
          <div class="text-xs font-bold text-gray-700">{{ $t('dashboard.wind_input') }}</div>

          <div class="flex flex-col items-center gap-4 py-2">
            <!-- Digital Display -->
            <div class="relative w-full flex justify-center">
              <span class="text-6xl font-black text-gray-900 tracking-tighter">{{
                targetSpeed.toFixed(0)
              }}</span>
              <span class="absolute right-8 bottom-2 text-sm font-bold text-gray-400">m/s</span>
            </div>

            <!-- Controls Grid -->
            <div class="grid grid-cols-2 gap-x-12 gap-y-3 w-full px-4">
              <!-- Left Side (- Buttons) -->
              <div class="flex flex-col gap-2 items-end">
                <button
                  @click="adjustSpeed(1)"
                  class="w-12 h-12 rounded-xl bg-gray-100 hover:bg-gray-200 active:bg-gray-300 text-gray-900 font-bold text-lg flex items-center justify-center transition-colors"
                >
                  +1
                </button>
                <button
                  @click="adjustSpeed(5)"
                  class="w-12 h-12 rounded-xl bg-gray-100 hover:bg-gray-200 active:bg-gray-300 text-gray-900 font-bold text-lg flex items-center justify-center transition-colors"
                >
                  +5
                </button>
              </div>

              <!-- Right Side (+ Buttons) -->
              <div class="flex flex-col gap-2 items-start">
                <button
                  @click="adjustSpeed(-1)"
                  class="w-12 h-12 rounded-xl bg-gray-100 hover:bg-gray-200 active:bg-gray-300 text-gray-900 font-bold text-lg flex items-center justify-center transition-colors"
                >
                  -1
                </button>
                <button
                  @click="adjustSpeed(-5)"
                  class="w-12 h-12 rounded-xl bg-gray-100 hover:bg-gray-200 active:bg-gray-300 text-gray-900 font-bold text-lg flex items-center justify-center transition-colors"
                >
                  -5
                </button>
              </div>
            </div>

            <!-- Confirm Button -->
            <button
              @click="updateSpeed"
              class="w-full mt-2 bg-black text-white font-bold py-3 rounded-xl hover:bg-gray-800 active:scale-95 transition-all"
            >
              {{ $t('common.confirm') }}
            </button>
          </div>
        </div>

        <!-- Wind Speed Display -->
        <div
          class="bg-white rounded-2xl p-3 shadow-sm border border-white flex-1 flex flex-col gap-3 min-h-0"
        >
          <div class="text-xs font-bold text-gray-700">
            {{ $t('dashboard.wind_speed_display') }}
          </div>

          <!-- Gauge -->
          <div class="flex-1 relative min-h-[240px] w-full overflow-visible pb-2">
            <div ref="gaugeChartEl" class="absolute inset-0 w-full h-full z-0"></div>
          </div>

          <!-- Bottom Info -->
          <div class="grid grid-cols-2 gap-3 pt-3 border-t border-gray-100">
            <div>
              <span class="text-xl font-bold text-gray-900 block tracking-tight">19°</span>
              <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold">{{
                $t('dashboard.wind_direction')
              }}</span>
            </div>
            <div>
              <span class="text-xl font-bold text-gray-900 block tracking-tight">
                {{ stats.avgSpeed.toFixed(1) }} m/s
              </span>
              <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold">{{
                $t('dashboard.avg_speed')
              }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Middle Column (Pressure Monitor) -->
      <div class="col-span-5 flex flex-col gap-4 min-h-0 overflow-y-auto">
        <div
          class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4 h-full"
        >
          <div class="text-xs font-bold text-gray-700">
            {{ $t('dashboard.pressure_diff_monitor') }}
          </div>

          <!-- Big Number -->
          <div class="bg-gray-100 rounded-xl p-6 flex flex-col items-center justify-center">
            <div class="text-xs text-gray-500 font-medium mb-1">
              {{ $t('dashboard.current_pressure_diff') }}
            </div>
            <div class="flex items-baseline gap-2">
              <span class="text-4xl font-bold text-gray-800 tracking-tight">{{
                pressureDiff.toFixed(2)
              }}</span>
              <span class="text-lg text-gray-500 font-medium">Pa</span>
            </div>
          </div>

          <!-- Waveform Chart -->
          <div class="flex-1 min-h-0 bg-white border border-gray-100 rounded-xl p-2 flex flex-col">
            <div class="text-[10px] text-gray-400 mb-1 flex-shrink-0">
              {{ $t('dashboard.realtime_waveform') }}
            </div>
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
              <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold">{{
                $t('dashboard.max_value')
              }}</span>
            </div>
            <div>
              <span class="text-xl font-bold text-gray-900 block tracking-tight">{{
                stats.min.toFixed(1)
              }}</span>
              <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold">{{
                $t('dashboard.min_value')
              }}</span>
            </div>
            <div>
              <span class="text-xl font-bold text-gray-900 block tracking-tight">{{
                stats.avg.toFixed(1)
              }}</span>
              <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold">{{
                $t('dashboard.average')
              }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Column (Sensor Output + IMU) -->
      <div class="col-span-4 flex flex-col gap-4 min-h-0 overflow-y-auto">
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
              <div>
                <div class="flex items-baseline gap-1">
                  <span class="text-4xl font-black text-gray-900 tracking-tighter leading-none">{{
                    env.pressure.toFixed(0)
                  }}</span>
                  <span class="text-xs text-gray-500 font-bold mb-1">Pa</span>
                </div>
              </div>
            </div>

            <!-- Bottom Metrics Row -->
            <div class="grid grid-cols-3 gap-3 pt-3 border-t border-gray-100">
              <!-- Pressure 2 -->
              <div>
                <span class="text-xl font-bold text-gray-900 block tracking-tight">{{
                  (env.pressure - pressureDiff).toFixed(0)
                }}</span>
                <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold">{{
                  $t('dashboard.pressure_2')
                }}</span>
              </div>
              <!-- Speed 1 -->
              <div>
                <span class="text-xl font-bold text-gray-900 block tracking-tight">{{
                  airspeed.toFixed(2)
                }}</span>
                <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold">{{
                  $t('dashboard.speed_1')
                }}</span>
              </div>
              <!-- Speed 2 -->
              <div>
                <span class="text-xl font-bold text-gray-900 block tracking-tight">{{
                  (airspeed * 1.1).toFixed(2)
                }}</span>
                <span class="text-[10px] text-gray-400 uppercase tracking-wider font-semibold">{{
                  $t('dashboard.speed_2')
                }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- IMU Visualization -->
        <div
          class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4 flex-1"
        >
          <div class="text-xs font-bold text-gray-700">{{ $t('dashboard.imu_visualization') }}</div>
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
              <div class="text-[10px] text-gray-500 font-medium">{{ $t('dashboard.roll') }}</div>
              <div class="text-sm font-bold">{{ imu.attitude.roll.toFixed(1) }}°</div>
            </div>
            <div class="bg-gray-50 p-2 rounded-lg border border-gray-100">
              <div class="text-[10px] text-gray-500 font-medium">{{ $t('dashboard.pitch') }}</div>
              <div class="text-sm font-bold">{{ imu.attitude.pitch.toFixed(1) }}°</div>
            </div>
            <div class="bg-gray-50 p-2 rounded-lg border border-gray-100">
              <div class="text-[10px] text-gray-500 font-medium">{{ $t('dashboard.yaw') }}</div>
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
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const store = useDeviceStore()
const { imu, airspeed, pressureDiff, env, isConnected } = storeToRefs(store)

const currentTab = ref('Overview')
const targetSpeed = ref(0)

function adjustSpeed(delta: number) {
  let newSpeed = targetSpeed.value + delta
  if (newSpeed < 4) newSpeed = 4
  if (newSpeed > 34) newSpeed = 34
  targetSpeed.value = newSpeed
}

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
  dataZoom: [
    {
      type: 'inside',
      xAxisIndex: [0],
      filterMode: 'filter',
      zoomLock: false,
    },
  ],
  animation: false,
}

onMounted(() => {
  initCharts()
  window.addEventListener('resize', handleResize)
  // Start simulation if not connected
  if (!isConnected.value) {
    startSimulation()
  }
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
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
      series: [
        {
          type: 'gauge',
          center: ['50%', '55%'],
          radius: '85%',
          startAngle: 200,
          endAngle: -20,
          min: 0,
          max: 30,
          splitNumber: 6,
          itemStyle: { color: '#333' },
          progress: { show: true, width: 8 },
          pointer: { show: true, length: '55%', width: 4 },
          axisLine: { lineStyle: { width: 8, color: [[1, '#e5e7eb']] } },
          axisTick: { distance: -10, length: 4, lineStyle: { color: '#999', width: 1 } },
          splitLine: { distance: -10, length: 8, lineStyle: { color: '#999', width: 2 } },
          axisLabel: { distance: -12, color: '#666', fontSize: 8 },
          anchor: {
            show: true,
            showAbove: true,
            size: 6,
            itemStyle: { borderWidth: 2, borderColor: '#333' },
          },
          detail: {
            valueAnimation: true,
            fontSize: 24,
            fontWeight: 'bold',
            color: '#1f2937',
            offsetCenter: [0, '40%'],
            formatter: (val: number) => `{value|${val.toFixed(1)}}\n{unit|m/s}`,
            rich: {
              value: {
                fontSize: 24,
                fontWeight: 'bold',
                color: '#1f2937',
                lineHeight: 24,
              },
              unit: {
                fontSize: 10,
                color: '#6b7280',
                lineHeight: 14,
                padding: [5, 0, 0, 0],
              },
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
</style>
