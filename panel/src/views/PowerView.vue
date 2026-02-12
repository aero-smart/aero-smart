<template>
  <div class="flex h-full bg-[#f9f9fb] font-sans overflow-hidden">
    <!-- Sidebar -->
    <div class="w-64 flex flex-col border-r border-gray-200 bg-[#fbfbfd]/80 backdrop-blur-xl">
      <!-- Sidebar Header -->
      <div class="h-14 px-4 flex items-center justify-between border-b border-gray-200/50">
        <div class="flex items-center gap-2">
          <Database :size="16" class="text-gray-500" />
          <span class="text-xs font-semibold text-gray-900 tracking-tight">{{ $t('analysis.data_sources') }}</span>
        </div>
      </div>

      <!-- Data Sources List -->
      <div class="flex-1 overflow-y-auto p-2 space-y-1">
        <button
          v-for="source in availableSources"
          :key="source.id"
          @click="toggleSource(source.id)"
          class="w-full flex items-center justify-between px-3 py-2.5 rounded-lg transition-all duration-200 text-left group relative overflow-hidden"
          :class="selectedSources.includes(source.id) ? 'bg-white shadow-sm ring-1 ring-black/5' : 'hover:bg-black/5'"
        >
          <!-- Active State Indicator Background (Optional, if we want a colored tint) -->
          <div 
            v-if="selectedSources.includes(source.id)"
            class="absolute inset-0 opacity-[0.03]"
            :style="{ backgroundColor: source.color }"
          ></div>

          <div class="flex items-center gap-3 relative z-10">
            <div 
              class="w-2.5 h-2.5 rounded-full shadow-sm transition-all duration-300 border border-white/50"
              :class="selectedSources.includes(source.id) ? 'scale-100' : 'scale-90 opacity-60 grayscale'"
              :style="{ backgroundColor: source.color }"
            ></div>
            <span 
              class="text-[13px] font-medium transition-colors duration-200" 
              :class="selectedSources.includes(source.id) ? 'text-gray-900' : 'text-gray-600'"
            >
              {{ source.label }}
            </span>
          </div>
          
          <span 
            class="relative z-10 text-[10px] font-semibold px-2 py-0.5 rounded-md border transition-all duration-200"
            :style="selectedSources.includes(source.id) ? { 
              color: source.color, 
              backgroundColor: `${source.color}10`,
              borderColor: `${source.color}20`
            } : {
              color: '#9ca3af',
              backgroundColor: '#f9fafb',
              borderColor: '#f3f4f6'
            }"
          >
            {{ source.unit }}
          </span>
        </button>
      </div>

      <!-- Sidebar Footer (Display Settings) -->
      <div class="p-4 border-t border-gray-200 bg-gray-50/50 backdrop-blur-sm">
        <div class="flex items-center gap-2 mb-4">
          <Sliders :size="14" class="text-gray-400" />
          <span class="text-[11px] font-semibold text-gray-500 uppercase tracking-wider">{{ $t('analysis.display_settings') }}</span>
        </div>
        
        <div class="space-y-5">
          <!-- Time Range -->
          <div class="space-y-2.5">
            <div class="flex justify-between items-center text-[11px]">
              <span class="text-gray-500 font-medium">{{ $t('analysis.time_range') }}</span>
              <span class="text-gray-900 font-bold bg-white px-2 py-0.5 rounded-md border border-gray-200 shadow-sm min-w-[32px] text-center">{{ config.timeRange }}s</span>
            </div>
            <div class="relative h-4 flex items-center">
              <input
                type="range"
                v-model.number="config.timeRange"
                min="5"
                max="60"
                step="5"
                class="w-full h-1 bg-gray-200 rounded-full appearance-none cursor-pointer accent-gray-800 z-10 relative"
              />
              <div class="absolute inset-0 flex justify-between px-0.5 pointer-events-none">
                <div v-for="n in 12" :key="n" class="w-0.5 h-0.5 rounded-full bg-gray-300 mt-2"></div>
              </div>
            </div>
          </div>

          <!-- Frequency -->
          <div class="space-y-2">
            <div class="flex justify-between items-center text-[11px]">
              <span class="text-gray-500 font-medium">{{ $t('analysis.frequency') }}</span>
            </div>
            <div class="relative group">
              <select
                v-model.number="config.frequency"
                class="w-full bg-white border border-gray-200 rounded-lg pl-3 pr-8 py-1.5 text-[12px] font-medium text-gray-700 focus:outline-none focus:ring-1 focus:ring-gray-300 shadow-sm appearance-none transition-all group-hover:border-gray-300"
              >
                <option :value="10">10 Hz</option>
                <option :value="20">20 Hz</option>
                <option :value="50">50 Hz</option>
                <option :value="100">100 Hz</option>
              </select>
<<<<<<< Updated upstream
              <div class="absolute inset-y-0 right-0 flex items-center px-2 pointer-events-none text-gray-400 group-hover:text-gray-600 transition-colors">
                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
=======
            </div>
          </div>
        </div>

        <!-- Chart Style -->
        <div class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4">
          <div class="text-xs font-bold text-gray-700 flex items-center gap-2">
            <Palette :size="14" class="text-gray-400" />
            {{ $t('analysis.chart_style') }}
          </div>
          <div class="grid grid-cols-2 gap-3">
            <div class="flex flex-col gap-1.5">
              <span class="text-[10px] text-gray-500 font-medium">{{
                $t('analysis.line_width')
              }}</span>
              <input
                type="number"
                v-model.number="config.lineWidth"
                min="1"
                max="5"
                class="w-full bg-gray-50 border border-gray-100 rounded-lg px-2 py-1.5 text-[11px] font-medium"
              />
            </div>
            <div class="flex flex-col gap-1.5">
              <span class="text-[10px] text-gray-500 font-medium">{{
                $t('analysis.show_grid')
              }}</span>
              <div class="flex items-center h-full">
                <input
                  type="checkbox"
                  v-model="config.showGrid"
                  class="w-4 h-4 rounded border-gray-300 text-gray-800 focus:ring-gray-800"
                />
>>>>>>> Stashed changes
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex flex-col min-w-0 bg-white">
      <!-- Toolbar -->
      <div class="h-14 px-4 border-b border-gray-100 flex items-center justify-between bg-white/80 backdrop-blur sticky top-0 z-10">
        <div class="flex items-center gap-4">
          <div class="flex items-center gap-3">
            <h1 class="text-lg font-bold text-gray-900 tracking-tight">{{ $t('analysis.title') }}</h1>
            <span 
              class="flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[10px] font-semibold border transition-colors"
              :class="isRealtime ? 'bg-green-50 text-green-700 border-green-200' : 'bg-gray-50 text-gray-600 border-gray-200'"
            >
              <span class="w-1.5 h-1.5 rounded-full" :class="isRealtime ? 'bg-green-500 animate-pulse' : 'bg-gray-400'"></span>
              {{ isRealtime ? $t('common.live') : $t('common.history') }}
            </span>
          </div>
          
          <div class="h-4 w-px bg-gray-200 mx-2"></div>

          <!-- View Controls -->
          <div class="flex items-center bg-gray-100/50 p-0.5 rounded-lg border border-gray-200/50">
            <button
              @click="isRealtime = true"
              class="px-3 py-1 text-[11px] font-medium rounded-md transition-all"
              :class="isRealtime ? 'bg-white text-gray-900 shadow-sm ring-1 ring-black/5' : 'text-gray-500 hover:text-gray-700'"
            >
              {{ $t('common.real_time') }}
            </button>
            <button
              @click="isRealtime = false"
              class="px-3 py-1 text-[11px] font-medium rounded-md transition-all"
              :class="!isRealtime ? 'bg-white text-gray-900 shadow-sm ring-1 ring-black/5' : 'text-gray-500 hover:text-gray-700'"
            >
              {{ $t('common.history') }}
            </button>
          </div>
        </div>

        <div class="flex items-center gap-3">
          <!-- Chart Actions -->
          <div class="flex items-center gap-1">
            <button @click="resetView" class="p-1.5 text-gray-400 hover:text-gray-700 hover:bg-gray-50 rounded-md transition-colors" title="Reset View">
              <RotateCcw :size="15" />
            </button>
            <button @click="zoomIn" class="p-1.5 text-gray-400 hover:text-gray-700 hover:bg-gray-50 rounded-md transition-colors" title="Zoom In">
              <ZoomIn :size="15" />
            </button>
            <button @click="zoomOut" class="p-1.5 text-gray-400 hover:text-gray-700 hover:bg-gray-50 rounded-md transition-colors" title="Zoom Out">
              <ZoomOut :size="15" />
            </button>
          </div>
          
          <div class="h-4 w-px bg-gray-200"></div>

          <button
            @click="exportData"
            class="flex items-center gap-2 px-3 py-1.5 bg-white border border-gray-200 rounded-lg text-[11px] font-semibold text-gray-700 hover:bg-gray-50 hover:border-gray-300 transition-all shadow-sm active:scale-[0.98]"
          >
            <Download :size="14" />
            {{ $t('common.export_csv') }}
          </button>
        </div>
      </div>

      <!-- Chart Area -->
      <div class="flex-1 relative flex flex-col min-h-0 bg-white">
        <!-- ECharts Container -->
        <div class="flex-1 w-full p-0">
          <div ref="chartEl" class="w-full h-full"></div>
        </div>

        <!-- Dashboard Panel (Bottom) -->
        <div class="h-28 bg-gray-50/50 border-t border-gray-200/60 p-4 flex gap-4 backdrop-blur-sm z-10 items-center justify-between">
          <!-- Prev Button -->
          <button 
            v-if="totalPages > 1"
            @click="prevPage" 
            :disabled="currentPage === 1"
            class="flex-shrink-0 p-1.5 rounded-lg border border-gray-200 bg-white shadow-sm hover:bg-gray-50 disabled:opacity-40 disabled:hover:bg-white transition-all active:scale-95"
          >
            <ChevronLeft :size="16" class="text-gray-600" />
          </button>

          <div ref="dashboardContainerRef" class="flex-1 flex gap-3 overflow-hidden h-full">
            <div
              v-for="source in paginatedSources"
              :key="source.id"
              class="flex-shrink-0 min-w-[150px] bg-gray-50 rounded-xl border border-gray-100 p-3 flex flex-col justify-between group flex-1"
            >
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-1.5">
                  <span class="w-1.5 h-1.5 rounded-full" :style="{ backgroundColor: source.color }"></span>
                  <div class="text-[11px] text-gray-500 font-medium truncate">{{ source.label }}</div>
                </div>
                <component :is="getSourceIcon(source.id)" class="text-gray-400 w-3.5 h-3.5 transition-colors" />
              </div>
              
              <div>
                <div class="mt-2 text-lg font-bold text-gray-800 leading-none">
                  {{ getCurrentValue(source.id) }} <span class="text-xs font-normal text-gray-500 ml-0.5">{{ source.unit }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Next Button -->
          <button 
            v-if="totalPages > 1"
            @click="nextPage" 
            :disabled="currentPage === totalPages"
            class="flex-shrink-0 p-1.5 rounded-lg border border-gray-200 bg-white shadow-sm hover:bg-gray-50 disabled:opacity-40 disabled:hover:bg-white transition-all active:scale-95"
          >
            <ChevronRight :size="16" class="text-gray-600" />
          </button>
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
import { 
  Database, 
  Sliders, 
  Download, 
  RotateCcw, 
  ZoomIn, 
  ZoomOut,
  Wind,
  Gauge,
  Thermometer,
  Zap,
  Activity,
  ChevronLeft,
  ChevronRight
} from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
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

const availableSources = computed(() => [
  { id: 'airspeed', label: t('control.sensors.airspeed'), unit: 'm/s', color: '#1f2937' },
  { id: 'pressureDiff', label: t('control.sensors.pressure_diff'), unit: 'Pa', color: '#4b5563' },
  { id: 'env_pressure', label: t('control.sensors.ambient'), unit: 'Pa', color: '#9ca3af' },
  { id: 'env_temp', label: t('control.sensors.temperature'), unit: '°C', color: '#6b7280' },
  { id: 'battery_v', label: t('control.sensors.battery'), unit: 'V', color: '#374151' },
  { id: 'imu_roll', label: t('control.sensors.imu_roll'), unit: '°', color: '#111827' },
  { id: 'imu_pitch', label: t('control.sensors.imu_pitch'), unit: '°', color: '#1f2937' },
])

const toggleSource = (id: string) => {
  if (selectedSources.value.includes(id)) {
    selectedSources.value = selectedSources.value.filter((s) => s !== id)
  } else {
    selectedSources.value.push(id)
  }
}

const activeSourceInfo = computed(() =>
  availableSources.value.filter((s) => selectedSources.value.includes(s.id)),
)

const getCurrentValue = (id: string) => {
  switch (id) {
    case 'airspeed': return airspeed.value.toFixed(2)
    case 'pressureDiff': return pressureDiff.value.toFixed(2)
    case 'env_pressure': return env.value.pressure.toFixed(0)
    case 'env_temp': return env.value.temperature.toFixed(1)
    case 'battery_v': return battery.value.voltage.toFixed(2)
    case 'imu_roll': return imu.value.attitude.roll.toFixed(1)
    case 'imu_pitch': return imu.value.attitude.pitch.toFixed(1)
    default: return '0'
  }
}

const getSourceIcon = (id: string) => {
  switch (id) {
    case 'airspeed': return Wind
    case 'pressureDiff': return Gauge
    case 'env_pressure': return Gauge
    case 'env_temp': return Thermometer
    case 'battery_v': return Zap
    case 'imu_roll': 
    case 'imu_pitch': return Activity
    default: return Activity
  }
}

// Pagination Logic
const dashboardContainerRef = ref<HTMLElement | null>(null)
const itemsPerPage = ref(4)
const currentPage = ref(1)

const totalPages = computed(() => Math.ceil(activeSourceInfo.value.length / itemsPerPage.value))

const paginatedSources = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage.value
  return activeSourceInfo.value.slice(start, start + itemsPerPage.value)
})

const nextPage = () => {
  if (currentPage.value < totalPages.value) currentPage.value++
}

const prevPage = () => {
  if (currentPage.value > 1) currentPage.value--
}

watch(activeSourceInfo, () => {
  if (currentPage.value > totalPages.value && totalPages.value > 0) {
    currentPage.value = totalPages.value
  }
})

let resizeObserver: ResizeObserver | null = null

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
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: 'transparent',
      borderWidth: 0,
      textStyle: { color: '#374151', fontSize: 12, fontWeight: 500 },
      shadowBlur: 12,
      shadowColor: 'rgba(0, 0, 0, 0.08)',
      padding: [8, 12],
      axisPointer: {
        type: 'line',
        lineStyle: { color: '#e5e7eb', width: 1, type: 'solid' },
      },
      formatter: (params: any) => {
        let result = `<div class="mb-1 text-xs text-gray-400 font-medium">${params[0].axisValueLabel}</div>`
        params.forEach((item: any) => {
          const color = item.color
          result += `
            <div class="flex items-center justify-between gap-4 text-xs">
              <div class="flex items-center gap-1.5">
                <span class="w-1.5 h-1.5 rounded-full" style="background-color: ${color}"></span>
                <span class="text-gray-600">${item.seriesName}</span>
              </div>
              <span class="font-bold font-mono" style="color: ${color}">${item.value[1].toFixed(2)}</span>
            </div>
          `
        })
        return result
      }
    },
    legend: { show: false },
    grid: {
      top: 30,
      left: 20,
      right: 20,
      bottom: 60,
      containLabel: true,
      borderWidth: 0,
    },
    xAxis: {
      type: 'time',
      axisLine: { show: false },
      axisTick: { show: false },
      axisLabel: { color: '#9ca3af', fontSize: 10, margin: 12 },
      splitLine: {
        show: config.value.showGrid,
        lineStyle: { color: '#f3f4f6', type: 'dashed' },
      },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisTick: { show: false },
      axisLabel: { color: '#9ca3af', fontSize: 10, margin: 12 },
      splitLine: {
        show: config.value.showGrid,
        lineStyle: { color: '#f3f4f6', type: 'dashed' },
      },
    },
    dataZoom: [
      { type: 'inside', throttle: 20 },
      {
        type: 'slider',
        height: 16,
        bottom: 20,
        borderColor: 'transparent',
        backgroundColor: '#f9fafb',
        fillerColor: 'rgba(31, 41, 55, 0.05)',
        handleStyle: { color: '#1f2937', opacity: 0.8 },
        moveHandleStyle: { color: '#1f2937', opacity: 0.8 },
        dataBackground: {
          lineStyle: { opacity: 0 },
          areaStyle: { opacity: 0 }
        },
        selectedDataBackground: {
          lineStyle: { opacity: 0 },
          areaStyle: { opacity: 0 }
        },
        textStyle: { show: false },
        showDetail: false, // Hide the detail text on the sides
      },
    ],
    series: selectedSources.value.map((id) => {
      const info = availableSources.value.find((s) => s.id === id)!
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
        const info = availableSources.value.find((s) => s.id === id)!
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

  if (dashboardContainerRef.value) {
    resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const width = entry.contentRect.width
        // card min-width 150 + gap 12 ~ 162
        // We use floor to ensure they fit without scrolling
        const count = Math.floor((width + 12) / 162) 
        itemsPerPage.value = Math.max(1, count)
        // Adjust page if current page becomes invalid due to resize
        if (currentPage.value > Math.ceil(activeSourceInfo.value.length / itemsPerPage.value)) {
           currentPage.value = Math.max(1, Math.ceil(activeSourceInfo.value.length / itemsPerPage.value))
        }
      }
    })
    resizeObserver.observe(dashboardContainerRef.value)
  }
})

onUnmounted(() => {
  if (updateTimer) clearInterval(updateTimer)
  chart?.dispose()
  resizeObserver?.disconnect()
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
