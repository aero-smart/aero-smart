<template>
  <div class="flex flex-col h-full p-4 gap-4 bg-[#f5f5f5] text-[#423d3c] font-sans overflow-hidden">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <span class="w-1.5 h-7 bg-black rounded-full"></span>
        <div class="text-lg font-bold tracking-tight">Data Analysis</div>
        <span
          class="text-[10px] text-gray-500 bg-white border border-gray-200 rounded-full px-2 py-0.5"
          >{{ isRealtime ? 'Live' : 'History' }}</span
        >
      </div>
      <div class="flex items-center gap-4">
        <div
          class="flex items-center gap-2 bg-white rounded-lg p-1 border border-gray-200 shadow-sm"
        >
          <button
            @click="isRealtime = true"
            class="px-3 py-1 text-[11px] font-medium rounded-md transition-all"
            :class="
              isRealtime ? 'bg-gray-800 text-white shadow-sm' : 'text-gray-500 hover:bg-gray-50'
            "
          >
            Real-time
          </button>
          <button
            @click="isRealtime = false"
            class="px-3 py-1 text-[11px] font-medium rounded-md transition-all"
            :class="
              !isRealtime ? 'bg-gray-800 text-white shadow-sm' : 'text-gray-500 hover:bg-gray-50'
            "
          >
            History
          </button>
        </div>
        <button
          @click="exportData"
          class="flex items-center gap-2 px-3 py-1.5 bg-white border border-gray-200 rounded-lg text-[11px] font-semibold text-gray-700 hover:bg-gray-50 transition-colors shadow-sm"
        >
          <Download :size="14" />
          Export CSV
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex gap-4 min-h-0 overflow-hidden">
      <!-- Left: Control Panel -->
      <div class="w-72 flex flex-col gap-4 overflow-y-auto pr-1">
        <!-- Data Sources -->
        <div class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4">
          <div class="text-xs font-bold text-gray-700 flex items-center gap-2">
            <Database :size="14" class="text-gray-400" />
            Data Sources
          </div>
          <div class="flex flex-col gap-2">
            <label
              v-for="source in availableSources"
              :key="source.id"
              class="flex items-center justify-between p-2.5 rounded-xl border border-gray-100 bg-gray-50/50 hover:bg-gray-50 transition-colors cursor-pointer group"
            >
              <div class="flex items-center gap-3">
                <input
                  type="checkbox"
                  v-model="selectedSources"
                  :value="source.id"
                  class="w-3.5 h-3.5 rounded border-gray-300 text-gray-800 focus:ring-gray-800"
                />
                <div class="flex flex-col">
                  <span class="text-[11px] font-bold text-gray-700">{{ source.label }}</span>
                  <span class="text-[9px] text-gray-400 uppercase tracking-tighter">{{
                    source.unit
                  }}</span>
                </div>
              </div>
              <div
                class="w-2 h-2 rounded-full shadow-sm"
                :style="{ backgroundColor: source.color }"
              ></div>
            </label>
          </div>
        </div>

        <!-- Display Settings -->
        <div class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4">
          <div class="text-xs font-bold text-gray-700 flex items-center gap-2">
            <Sliders :size="14" class="text-gray-400" />
            Display Settings
          </div>
          <div class="flex flex-col gap-4">
            <!-- Time Range -->
            <div class="flex flex-col gap-1.5">
              <div class="flex justify-between text-[10px] text-gray-500 font-medium">
                <span>Time Range</span>
                <span class="text-gray-900 font-bold">{{ config.timeRange }}s</span>
              </div>
              <input
                type="range"
                v-model.number="config.timeRange"
                min="5"
                max="60"
                step="5"
                class="w-full h-1 bg-gray-100 rounded-lg appearance-none cursor-pointer accent-gray-800"
              />
            </div>
            <!-- Sampling Frequency -->
            <div class="flex flex-col gap-1.5">
              <div class="flex justify-between text-[10px] text-gray-500 font-medium">
                <span>Frequency</span>
                <span class="text-gray-900 font-bold">{{ config.frequency }}Hz</span>
              </div>
              <select
                v-model.number="config.frequency"
                class="w-full bg-gray-50 border border-gray-100 rounded-lg px-2 py-1.5 text-[11px] font-medium focus:outline-none focus:border-gray-300"
              >
                <option :value="10">10 Hz</option>
                <option :value="20">20 Hz</option>
                <option :value="50">50 Hz</option>
                <option :value="100">100 Hz</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Chart Style -->
        <div class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4">
          <div class="text-xs font-bold text-gray-700 flex items-center gap-2">
            <Palette :size="14" class="text-gray-400" />
            Chart Style
          </div>
          <div class="grid grid-cols-2 gap-3">
            <div class="flex flex-col gap-1.5">
              <span class="text-[10px] text-gray-500 font-medium">Line Width</span>
              <input
                type="number"
                v-model.number="config.lineWidth"
                min="1"
                max="5"
                class="w-full bg-gray-50 border border-gray-100 rounded-lg px-2 py-1.5 text-[11px] font-medium"
              />
            </div>
            <div class="flex flex-col gap-1.5">
              <span class="text-[10px] text-gray-500 font-medium">Show Grid</span>
              <div class="flex items-center h-full">
                <input
                  type="checkbox"
                  v-model="config.showGrid"
                  class="w-4 h-4 rounded border-gray-300 text-gray-800 focus:ring-gray-800"
                />
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Right: Waveform Display -->
      <div class="flex-1 bg-white rounded-2xl shadow-sm border border-white flex flex-col min-w-0">
        <!-- Toolbar -->
        <div class="p-3 border-b border-gray-100 flex items-center justify-between">
          <div class="flex items-center gap-2">
            <button
              @click="resetView"
              class="p-1.5 hover:bg-gray-50 rounded-lg text-gray-500 transition-colors"
              title="Reset View"
            >
              <RotateCcw :size="16" />
            </button>
            <div class="w-px h-4 bg-gray-100 mx-1"></div>
            <button
              @click="zoomIn"
              class="p-1.5 hover:bg-gray-50 rounded-lg text-gray-500 transition-colors"
              title="Zoom In"
            >
              <ZoomIn :size="16" />
            </button>
            <button
              @click="zoomOut"
              class="p-1.5 hover:bg-gray-50 rounded-lg text-gray-500 transition-colors"
              title="Zoom Out"
            >
              <ZoomOut :size="16" />
            </button>
          </div>
          <div class="flex items-center gap-4 text-[10px] text-gray-400 font-medium">
            <div
              v-for="source in activeSourceInfo"
              :key="source.id"
              class="flex items-center gap-1.5"
            >
              <span class="w-2 h-2 rounded-full" :style="{ backgroundColor: source.color }"></span>
              {{ source.label }}
            </div>
          </div>
        </div>

        <!-- Chart -->
        <div class="flex-1 relative min-h-0 w-full p-4">
          <div ref="chartEl" class="absolute inset-0 w-full h-full"></div>
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
import { Download, Database, Sliders, Palette, RotateCcw, ZoomIn, ZoomOut } from 'lucide-vue-next'

const store = useDeviceStore()
const { airspeed, pressureDiff, env, battery, imu } = storeToRefs(store)

const isRealtime = ref(true)
const selectedSources = ref<string[]>(['airspeed', 'pressureDiff'])
const chartEl = ref<HTMLElement | null>(null)
let chart: echarts.ECharts | null = null

const config = ref({
  timeRange: 30, // seconds
  frequency: 20, // Hz
  lineWidth: 2,
  showGrid: true,
  smooth: true,
})

const availableSources = [
  { id: 'airspeed', label: 'Airspeed', unit: 'm/s', color: '#1f2937' },
  { id: 'pressureDiff', label: 'Pressure Diff', unit: 'Pa', color: '#4b5563' },
  { id: 'env_pressure', label: 'Ambient Pressure', unit: 'Pa', color: '#9ca3af' },
  { id: 'env_temp', label: 'Temperature', unit: '°C', color: '#6b7280' },
  { id: 'battery_v', label: 'Voltage', unit: 'V', color: '#374151' },
  { id: 'imu_roll', label: 'Roll', unit: '°', color: '#111827' },
  { id: 'imu_pitch', label: 'Pitch', unit: '°', color: '#1f2937' },
]

const activeSourceInfo = computed(() =>
  availableSources.filter((s) => selectedSources.value.includes(s.id)),
)

// Data buffers for real-time display
const maxPoints = computed(() => config.value.timeRange * config.value.frequency)
const dataBuffers = ref<Record<string, { time: number; value: number }[]>>({
  airspeed: [],
  pressureDiff: [],
  env_pressure: [],
  env_temp: [],
  battery_v: [],
  imu_roll: [],
  imu_pitch: [],
})

function updateBuffers() {
  if (!isRealtime.value) return

  const now = Date.now()
  const newData = {
    airspeed: airspeed.value,
    pressureDiff: pressureDiff.value,
    env_pressure: env.value.pressure,
    env_temp: env.value.temperature,
    battery_v: battery.value.voltage,
    imu_roll: imu.value.attitude.roll,
    imu_pitch: imu.value.attitude.pitch,
  }

  Object.keys(dataBuffers.value).forEach((key) => {
    const buffer = dataBuffers.value[key]
    if (buffer) {
      buffer.push({ time: now, value: (newData as any)[key] })
      if (buffer.length > maxPoints.value) {
        buffer.shift()
      }
    }
  })

  updateChart()
}

let updateTimer: number | null = null

function initChart() {
  if (!chartEl.value) return
  chart = echarts.init(chartEl.value)

  const option: echarts.EChartsOption = {
    tooltip: {
      trigger: 'axis',
      backgroundColor: 'rgba(255, 255, 255, 0.9)',
      borderWidth: 0,
      textStyle: { color: '#1f2937', fontSize: 11 },
      shadowBlur: 10,
      shadowColor: 'rgba(0, 0, 0, 0.1)',
      axisPointer: {
        type: 'cross',
        lineStyle: { color: '#e5e7eb', type: 'dashed' },
        crossStyle: { color: '#e5e7eb' },
      },
    },
    legend: { show: false },
    grid: {
      top: 20,
      left: 40,
      right: 20,
      bottom: 40,
      containLabel: true,
    },
    xAxis: {
      type: 'time',
      axisLine: { show: false },
      axisTick: { show: false },
      axisLabel: { color: '#9ca3af', fontSize: 10 },
      splitLine: {
        show: config.value.showGrid,
        lineStyle: { color: '#f3f4f6', type: 'dashed' },
      },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisTick: { show: false },
      axisLabel: { color: '#9ca3af', fontSize: 10 },
      splitLine: {
        show: config.value.showGrid,
        lineStyle: { color: '#f3f4f6', type: 'dashed' },
      },
    },
    dataZoom: [
      { type: 'inside', throttle: 20 },
      {
        type: 'slider',
        height: 12,
        bottom: 5,
        borderColor: 'transparent',
        backgroundColor: '#f9fafb',
        fillerColor: 'rgba(31, 41, 55, 0.05)',
        handleStyle: { color: '#1f2937' },
        textStyle: { show: false },
      },
    ],
    series: selectedSources.value.map((id) => {
      const info = availableSources.find((s) => s.id === id)!
      return {
        id,
        name: info.label,
        type: 'line',
        showSymbol: false,
        smooth: config.value.smooth,
        lineStyle: { width: config.value.lineWidth, color: info.color },
        data: [],
      }
    }),
    animation: false,
  }

  chart.setOption(option)
}

function updateChart() {
  if (!chart) return

  const series = selectedSources.value.map((id) => {
    const buffer = dataBuffers.value[id] || []
    return {
      id,
      data: buffer.map((d) => [d.time, d.value]),
    }
  })

  chart.setOption({ series })
}

function resetView() {
  chart?.dispatchAction({
    type: 'dataZoom',
    start: 0,
    end: 100,
  })
}

function zoomIn() {
  chart?.dispatchAction({
    type: 'dataZoom',
    start: 20,
    end: 80,
  })
}

function zoomOut() {
  chart?.dispatchAction({
    type: 'dataZoom',
    start: 0,
    end: 100,
  })
}

function exportData() {
  if (selectedSources.value.length === 0) return

  const primarySourceId = selectedSources.value[0] as string
  const primaryBuffer = (dataBuffers.value as any)[primarySourceId] || []

  const headers = ['Timestamp', ...selectedSources.value].join(',')
  const rows = primaryBuffer.map((_: any, idx: number) => {
    const time = primaryBuffer[idx].time
    const vals = selectedSources.value.map((id) => {
      const buffer = (dataBuffers.value as any)[id]
      return buffer && buffer[idx] ? buffer[idx].value : 0
    })
    return [new Date(time).toISOString(), ...vals].join(',')
  })

  const csvContent = [headers, ...rows].join('\n')
  const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' })
  const link = document.createElement('a')
  link.href = URL.createObjectURL(blob)
  link.setAttribute('download', `aero_data_${new Date().getTime()}.csv`)
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
}

watch(
  selectedSources,
  (newSources) => {
    if (!chart) return

    // Update series in chart
    const currentOption = chart.getOption() as any
    const currentSeriesIds = currentOption.series.map((s: any) => s.id)

    // Add new series
    newSources.forEach((id) => {
      if (!currentSeriesIds.includes(id)) {
        const info = availableSources.find((s) => s.id === id)!
        const buffer = dataBuffers.value[id] || []
        chart!.setOption({
          series: [
            {
              id,
              name: info.label,
              type: 'line',
              showSymbol: false,
              smooth: config.value.smooth,
              lineStyle: { width: config.value.lineWidth, color: info.color },
              data: buffer.map((d) => [d.time, d.value]),
            },
          ],
        })
      }
    })

    // Remove unused series
    currentSeriesIds.forEach((id: string) => {
      if (!newSources.includes(id)) {
        chart!.setOption({
          series: [{ id, data: [] }], // Effectively hide it or remove it
        })
      }
    })
  },
  { deep: true },
)

watch(
  () => config.value.showGrid,
  (val) => {
    chart?.setOption({
      xAxis: { splitLine: { show: val } },
      yAxis: { splitLine: { show: val } },
    })
  },
)

watch(
  () => config.value.lineWidth,
  (val) => {
    const series = selectedSources.value.map((id) => ({
      id,
      lineStyle: { width: val },
    }))
    chart?.setOption({ series })
  },
)

onMounted(() => {
  initChart()
  updateTimer = window.setInterval(updateBuffers, 1000 / config.value.frequency)
  window.addEventListener('resize', () => chart?.resize())
})

onUnmounted(() => {
  if (updateTimer) clearInterval(updateTimer)
  chart?.dispose()
})
</script>

<style scoped>
/* Custom range input styling */
input[type='range']::-webkit-slider-thumb {
  appearance: none;
  width: 12px;
  height: 12px;
  background: #1f2937;
  border-radius: 50%;
  cursor: pointer;
  border: 2px solid white;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

/* Scrollbar styling */
.overflow-y-auto::-webkit-scrollbar {
  width: 4px;
}
.overflow-y-auto::-webkit-scrollbar-track {
  background: transparent;
}
.overflow-y-auto::-webkit-scrollbar-thumb {
  background: #e5e7eb;
  border-radius: 10px;
}
.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: #d1d5db;
}
</style>
