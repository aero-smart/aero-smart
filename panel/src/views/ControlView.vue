<template>
  <div class="flex flex-col h-full p-4 gap-4 bg-[#f5f5f5] text-[#423d3c] font-sans overflow-hidden">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <span class="w-1.5 h-7 bg-black rounded-full"></span>
        <div class="text-lg font-bold tracking-tight">{{ $t('control.sensor_analysis') }}</div>
        <span
          class="text-[10px] text-gray-500 bg-white border border-gray-200 rounded-full px-2 py-0.5"
          >{{ $t('common.live') }}</span
        >
      </div>
      <div class="text-xs text-gray-400 font-medium">{{ $t('common.auto_refresh') }}</div>
    </div>

    <div class="flex-1 grid grid-cols-1 lg:grid-cols-12 gap-4 min-h-0 items-stretch">
      <div class="lg:col-span-5 flex flex-col gap-4 min-h-0 h-full">
        <div
          class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4 h-full"
        >
          <div class="flex items-center justify-between">
            <div class="text-xs font-bold text-gray-700">{{ $t('control.sensor_snapshot') }}</div>
            <div class="flex items-center gap-2">
              <span class="text-[10px] text-gray-400 font-medium"
                >{{ currentSensorPage }} / {{ totalSensorPages }}</span
              >
              <div class="flex items-center gap-1">
                <button
                  class="p-1 rounded-md border border-gray-200 hover:bg-gray-100 disabled:opacity-40 disabled:hover:bg-white transition-colors"
                  @click="prevSensorPage"
                  :disabled="currentSensorPage === 1"
                >
                  <ChevronLeft :size="14" />
                </button>
                <button
                  class="p-1 rounded-md border border-gray-200 hover:bg-gray-100 disabled:opacity-40 disabled:hover:bg-white transition-colors"
                  @click="nextSensorPage"
                  :disabled="currentSensorPage === totalSensorPages"
                >
                  <ChevronRight :size="14" />
                </button>
              </div>
            </div>
          </div>
          <div class="grid grid-cols-2 grid-rows-4 gap-3 flex-1 overflow-y-auto min-h-0">
            <div
              v-for="metric in pagedTelemetryData"
              :key="metric.id"
              class="bg-gray-50 rounded-xl p-3 border border-gray-100 flex flex-col justify-between"
            >
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-1">
                  <span class="text-gray-400 text-xs cursor-move">⋮⋮</span>
                  <div class="text-[11px] text-gray-500 font-medium">{{ metric.label }}</div>
                </div>
                <component :is="metric.icon" :size="14" class="text-gray-400" />
              </div>
              <div>
                <div class="mt-2 text-lg font-bold text-gray-800">{{ metric.value }}</div>
                <div class="text-[10px] text-gray-400">{{ metric.sub }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="lg:col-span-7 flex flex-col gap-4 min-h-0 h-full">
        <div
          class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4 min-h-0 h-full"
        >
          <div class="flex items-center justify-between">
            <div class="text-xs font-bold text-gray-700">{{ $t('control.ai_suggestions') }}</div>
            <div class="flex items-center gap-2 text-[10px] text-gray-400">
              <Sparkles :size="12" />
              {{ $t('control.guided') }}
            </div>
          </div>
          <div class="flex-1 min-h-0 flex flex-col gap-4 overflow-visible">
            <div class="bg-gray-50 border border-gray-100 rounded-xl p-3 min-h-[100px]">
              <div class="text-[10px] text-gray-400 uppercase tracking-wider">
                {{ $t('control.ai_output') }}
              </div>
              <div class="mt-2 text-[12px] text-gray-700 leading-relaxed">{{ aiResult }}</div>
            </div>
            <div class="flex items-center justify-between text-[10px] text-gray-400">
              <span class="text-xs font-semibold text-gray-600">{{ $t('control.presets') }}</span>
              <div class="flex items-center gap-2">
                <span class="text-[10px]">{{
                  $t('common.page_info', { current: currentPage, total: totalPages })
                }}</span>
                <div class="flex items-center gap-1">
                  <button
                    class="p-1 rounded-md border border-gray-200 hover:bg-gray-100 disabled:opacity-40 disabled:hover:bg-white transition-colors"
                    @click="prevPage"
                    :disabled="currentPage === 1"
                  >
                    <ChevronLeft :size="14" />
                  </button>
                  <button
                    class="p-1 rounded-md border border-gray-200 hover:bg-gray-100 disabled:opacity-40 disabled:hover:bg-white transition-colors"
                    @click="nextPage"
                    :disabled="currentPage === totalPages"
                  >
                    <ChevronRight :size="14" />
                  </button>
                </div>
              </div>
            </div>
            <div class="grid grid-cols-2 grid-rows-4 gap-2 flex-1 overflow-y-auto min-h-0">
              <button
                v-for="(item, idx) in pagedSuggestions"
                :key="idx"
                @click="applySuggestion(item.prompt)"
                class="text-left bg-gray-50 border border-gray-100 rounded-lg p-2 hover:bg-white hover:border-gray-200 transition-colors h-full flex flex-col justify-start"
              >
                <div class="flex items-center justify-between w-full">
                  <div class="flex items-center gap-1.5">
                    <component :is="item.icon" :size="12" class="text-gray-500" />
                    <span class="text-[9px] text-gray-400 uppercase tracking-wider">{{
                      item.category
                    }}</span>
                  </div>
                </div>
                <div class="mt-1 text-xs font-semibold text-gray-800 leading-tight truncate">
                  {{ item.title }}
                </div>
                <div class="text-[10px] text-gray-500 mt-0.5 leading-tight truncate w-full">
                  {{ item.detail }}
                </div>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { storeToRefs } from 'pinia'
import {
  Sparkles,
  Wind,
  Gauge,
  Thermometer,
  Database,
  Zap,
  Activity,
  ChevronLeft,
  ChevronRight,
} from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const store = useDeviceStore()
const { airspeed, pressureDiff, env, battery, imu, lidar, acoustic } = storeToRefs(store)

const aiResult = ref(t('control.ai_default_result'))
const currentPage = ref(1)
const suggestionsPerPage = 8

// Sensor Pagination
const currentSensorPage = ref(1)
const sensorsPerPage = 8

type SensorOption = {
  id: string
  label: string
  sub: string
  icon: any
  getValue: () => string
}

const baseSensorIds = [
  'airspeed',
  'pressure_diff',
  'ambient',
  'temperature',
  'battery',
  'imu_pitch',
]
const selectedSensorIds = ref<string[]>([...baseSensorIds])

const baseSensorOptions = computed<SensorOption[]>(() => [
  {
    id: 'airspeed',
    label: t('control.sensors.airspeed'),
    sub: t('control.subs.pitot_a'),
    icon: Wind,
    getValue: () => `${airspeed.value.toFixed(2)} m/s`,
  },
  {
    id: 'pressure_diff',
    label: t('control.sensors.pressure_diff'),
    sub: t('control.subs.differential'),
    icon: Gauge,
    getValue: () => `${(pressureDiff.value / 1000).toFixed(3)} kPa`,
  },
  {
    id: 'ambient',
    label: t('control.sensors.ambient'),
    sub: t('control.subs.barometer'),
    icon: Database,
    getValue: () => `${(env.value.pressure / 100).toFixed(1)} hPa`,
  },
  {
    id: 'temperature',
    label: t('control.sensors.temperature'),
    sub: t('control.subs.internal'),
    icon: Thermometer,
    getValue: () => `${env.value.temperature.toFixed(1)}°C`,
  },
  {
    id: 'battery',
    label: t('control.sensors.battery'),
    sub: t('control.subs.main_pack'),
    icon: Zap,
    getValue: () => `${battery.value.voltage.toFixed(2)} V`,
  },
  {
    id: 'imu_pitch',
    label: t('control.sensors.imu_pitch'),
    sub: t('control.subs.attitude'),
    icon: Activity,
    getValue: () => `${imu.value.attitude.pitch.toFixed(1)}°`,
  },
])

const extraSensorOptions = computed<SensorOption[]>(() => [
  {
    id: 'humidity',
    label: t('control.sensors.humidity'),
    sub: t('control.subs.environment'),
    icon: Database,
    getValue: () => `${env.value.humidity.toFixed(1)}%`,
  },
  {
    id: 'batterySoc',
    label: t('control.sensors.battery_soc'),
    sub: t('control.subs.main_pack'),
    icon: Zap,
    getValue: () => `${battery.value.soc.toFixed(0)}%`,
  },
  {
    id: 'imuRoll',
    label: t('control.sensors.imu_roll'),
    sub: t('control.subs.attitude'),
    icon: Activity,
    getValue: () => `${imu.value.attitude.roll.toFixed(1)}°`,
  },
  {
    id: 'imuYaw',
    label: t('control.sensors.imu_yaw'),
    sub: t('control.subs.attitude'),
    icon: Activity,
    getValue: () => `${imu.value.attitude.yaw.toFixed(1)}°`,
  },
  {
    id: 'lidar',
    label: t('control.sensors.lidar'),
    sub: t('control.subs.range'),
    icon: Gauge,
    getValue: () => `${lidar.value.distance.toFixed(0)} cm`,
  },
  {
    id: 'acoustic',
    label: t('control.sensors.acoustic'),
    sub: t('control.subs.audio'),
    icon: Wind,
    getValue: () => `${acoustic.value.spl.toFixed(1)} dB`,
  },
])

const allSensorOptions = computed(() => [...baseSensorOptions.value, ...extraSensorOptions.value])

const totalSensorPages = computed(() => Math.ceil(allSensorOptions.value.length / sensorsPerPage))

const pagedTelemetryData = computed(() => {
  const start = (currentSensorPage.value - 1) * sensorsPerPage
  const end = start + sensorsPerPage
  return allSensorOptions.value.slice(start, end).map((item) => ({
    id: item.id,
    label: item.label,
    sub: item.sub,
    value: item.getValue(),
    icon: item.icon,
  }))
})

function prevSensorPage() {
  if (currentSensorPage.value > 1) {
    currentSensorPage.value--
  }
}

function nextSensorPage() {
  if (currentSensorPage.value < totalSensorPages.value) {
    currentSensorPage.value++
  }
}

const suggestions = computed(() => [
  {
    icon: Wind,
    category: t('control.suggestions.flow.category'),
    title: t('control.suggestions.flow.title'),
    detail: t('control.suggestions.flow.detail'),
    prompt: t('control.suggestions.flow.prompt'),
  },
  {
    icon: Gauge,
    category: t('control.suggestions.pressure.category'),
    title: t('control.suggestions.pressure.title'),
    detail: t('control.suggestions.pressure.detail'),
    prompt: t('control.suggestions.pressure.prompt'),
  },
  {
    icon: Zap,
    category: t('control.suggestions.power.category'),
    title: t('control.suggestions.power.title'),
    detail: t('control.suggestions.power.detail'),
    prompt: t('control.suggestions.power.prompt'),
  },
  {
    icon: Activity,
    category: t('control.suggestions.imu.category'),
    title: t('control.suggestions.imu.title'),
    detail: t('control.suggestions.imu.detail'),
    prompt: t('control.suggestions.imu.prompt'),
  },
  {
    icon: Thermometer,
    category: t('control.suggestions.thermal.category'),
    title: t('control.suggestions.thermal.title'),
    detail: t('control.suggestions.thermal.detail'),
    prompt: t('control.suggestions.thermal.prompt'),
  },
  {
    icon: Database,
    category: t('control.suggestions.calibration.category'),
    title: t('control.suggestions.calibration.title'),
    detail: t('control.suggestions.calibration.detail'),
    prompt: t('control.suggestions.calibration.prompt'),
  },
  {
    icon: Gauge,
    category: t('control.suggestions.lidar.category'),
    title: t('control.suggestions.lidar.title'),
    detail: t('control.suggestions.lidar.detail'),
    prompt: t('control.suggestions.lidar.prompt'),
  },
  {
    icon: Wind,
    category: t('control.suggestions.acoustic.category'),
    title: t('control.suggestions.acoustic.title'),
    detail: t('control.suggestions.acoustic.detail'),
    prompt: t('control.suggestions.acoustic.prompt'),
  },
])

const totalPages = computed(() =>
  Math.max(1, Math.ceil(suggestions.value.length / suggestionsPerPage)),
)

const pagedSuggestions = computed(() => {
  const start = (currentPage.value - 1) * suggestionsPerPage
  return suggestions.value.slice(start, start + suggestionsPerPage)
})

function applySuggestion(prompt: string) {
  aiResult.value = `${prompt}${t('control.ai_apply_suffix')}`
}

function prevPage() {
  if (currentPage.value > 1) currentPage.value -= 1
}

function nextPage() {
  if (currentPage.value < totalPages.value) currentPage.value += 1
}
</script>
