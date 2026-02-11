<template>
  <div class="flex flex-col h-full p-4 gap-4 bg-[#f5f5f5] text-[#423d3c] font-sans overflow-hidden">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <span class="w-1.5 h-7 bg-black rounded-full"></span>
        <div class="text-lg font-bold tracking-tight">Sensor Analysis</div>
        <span class="text-[10px] text-gray-500 bg-white border border-gray-200 rounded-full px-2 py-0.5">Live</span>
      </div>
      <div class="text-xs text-gray-400 font-medium">Auto Refresh</div>
    </div>

    <div class="flex-1 grid grid-cols-1 lg:grid-cols-12 gap-4 min-h-0 items-stretch">
      <div class="lg:col-span-5 flex flex-col gap-4 min-h-0 h-full">
        <div class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4 h-full">
          <div class="flex items-center justify-between">
            <div class="text-xs font-bold text-gray-700">Sensor Snapshot</div>
          </div>
          <div class="grid grid-cols-2 gap-3 content-start auto-rows-min">
            <div 
              v-for="metric in telemetryData" 
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
      <div class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4 min-h-0 h-full overflow-hidden">
          <div class="flex items-center justify-between">
            <div class="text-xs font-bold text-gray-700">AI Suggestions</div>
            <div class="flex items-center gap-2 text-[10px] text-gray-400">
              <Sparkles :size="12" />
              Guided
            </div>
          </div>
          <div class="bg-gray-50 rounded-xl p-3 flex items-center gap-3">
            <div class="w-8 h-8 rounded-lg bg-white border border-gray-100 flex items-center justify-center text-gray-600">
              <Sparkles :size="16" />
            </div>
            <input
              v-model="inputQuery"
              type="text"
              placeholder="输入提示词获取建议"
              class="flex-1 bg-transparent border-none outline-none text-sm text-gray-800 placeholder-gray-400"
              @keyup.enter="sendQuery"
            />
            <button
              class="px-3 py-1.5 rounded-lg bg-gray-900 text-white text-xs hover:bg-black flex items-center gap-1 disabled:opacity-40 disabled:hover:bg-gray-900"
              @click="sendQuery"
              :disabled="!inputQuery.trim()"
            >
              <Send :size="12" />
              发送
            </button>
          </div>
          <div class="flex-1 min-h-0 flex flex-col gap-4">
            <div class="bg-gray-50 border border-gray-100 rounded-xl p-4 min-h-[140px] flex-1">
              <div class="text-[10px] text-gray-400 uppercase tracking-wider">AI Output</div>
              <div class="mt-2 text-[13px] text-gray-700 leading-relaxed">{{ aiResult }}</div>
            </div>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 flex-1 auto-rows-fr">
              <button
                v-for="(item, idx) in pagedSuggestions"
                :key="idx"
                @click="applySuggestion(item.prompt)"
                class="text-left bg-gray-50 border border-gray-100 rounded-xl p-3 hover:bg-white hover:border-gray-200 transition-colors h-full"
              >
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-2">
                    <component :is="item.icon" :size="14" class="text-gray-500" />
                    <span class="text-[10px] text-gray-400 uppercase tracking-wider">{{ item.category }}</span>
                  </div>
                  <span class="text-[10px] text-gray-400">Apply</span>
                </div>
                <div class="mt-2 text-sm font-semibold text-gray-800">{{ item.title }}</div>
                <div class="text-[11px] text-gray-500">{{ item.detail }}</div>
              </button>
            </div>
            <div class="flex items-center justify-between text-[10px] text-gray-400">
              <button
                class="px-2 py-1 rounded-md border border-gray-200 hover:bg-gray-100 disabled:opacity-40 disabled:hover:bg-white"
                @click="prevPage"
                :disabled="currentPage === 1"
              >
                上一页
              </button>
              <span class="flex-1 text-center">第 {{ currentPage }} / {{ totalPages }} 页</span>
              <button
                class="px-2 py-1 rounded-md border border-gray-200 hover:bg-gray-100 disabled:opacity-40 disabled:hover:bg-white"
                @click="nextPage"
                :disabled="currentPage === totalPages"
              >
                下一页
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
import { Sparkles, Send, Wind, Gauge, Thermometer, Database, Zap, Activity } from 'lucide-vue-next'

const store = useDeviceStore()
const { airspeed, pressureDiff, env, battery, imu, lidar, acoustic } = storeToRefs(store)

const inputQuery = ref('')
const aiResult = ref('压差与气流曲线保持平稳，建议以 2 秒窗口平滑后再评估异常波动。')
const currentPage = ref(1)
const suggestionsPerPage = 4

type SensorOption = {
  id: string
  label: string
  sub: string
  icon: any
  getValue: () => string
}

const baseSensorIds = ['airspeed', 'pressure_diff', 'ambient', 'temperature', 'battery', 'imu_pitch']
const selectedSensorIds = ref<string[]>([...baseSensorIds])

const baseSensorOptions = computed<SensorOption[]>(() => [
  { id: 'airspeed', label: 'Airspeed', sub: 'Pitot A', icon: Wind, getValue: () => `${airspeed.value.toFixed(2)} m/s` },
  { id: 'pressure_diff', label: 'Pressure Diff', sub: 'Differential', icon: Gauge, getValue: () => `${pressureDiff.value.toFixed(1)} Pa` },
  { id: 'ambient', label: 'Ambient', sub: 'Barometer', icon: Database, getValue: () => `${(env.value.pressure / 100).toFixed(1)} hPa` },
  { id: 'temperature', label: 'Temperature', sub: 'Internal', icon: Thermometer, getValue: () => `${env.value.temperature.toFixed(1)}°C` },
  { id: 'battery', label: 'Battery', sub: 'Main Pack', icon: Zap, getValue: () => `${battery.value.voltage.toFixed(2)} V` },
  { id: 'imu_pitch', label: 'IMU Pitch', sub: 'Attitude', icon: Activity, getValue: () => `${imu.value.attitude.pitch.toFixed(1)}°` }
])

const extraSensorOptions: SensorOption[] = [
  { id: 'humidity', label: 'Humidity', sub: 'Environment', icon: Database, getValue: () => `${env.value.humidity.toFixed(1)}%` },
  { id: 'batterySoc', label: 'Battery SOC', sub: 'Main Pack', icon: Zap, getValue: () => `${battery.value.soc.toFixed(0)}%` },
  { id: 'imuRoll', label: 'IMU Roll', sub: 'Attitude', icon: Activity, getValue: () => `${imu.value.attitude.roll.toFixed(1)}°` },
  { id: 'imuYaw', label: 'IMU Yaw', sub: 'Attitude', icon: Activity, getValue: () => `${imu.value.attitude.yaw.toFixed(1)}°` },
  { id: 'lidar', label: 'Lidar', sub: 'Range', icon: Gauge, getValue: () => `${lidar.value.distance.toFixed(0)} cm` },
  { id: 'acoustic', label: 'Acoustic SPL', sub: 'Audio', icon: Wind, getValue: () => `${acoustic.value.spl.toFixed(1)} dB` }
]

const telemetryData = computed(() =>
  baseSensorOptions.value.slice(0, 6).map(item => ({
    id: item.id,
    label: item.label,
    sub: item.sub,
    value: item.getValue(),
    icon: item.icon
  }))
)

const suggestions = [
  {
    icon: Wind,
    category: 'Flow',
    title: '稳定气流质量',
    detail: '检查压差波动并调整滤波参数',
    prompt: '根据当前压差评估气流稳定性'
  },
  {
    icon: Gauge,
    category: 'Pressure',
    title: '优化管路响应',
    detail: '评估管路延迟并校准传感器',
    prompt: '给出压差传感器校准建议'
  },
  {
    icon: Zap,
    category: 'Power',
    title: '降低能耗波动',
    detail: '分析电流峰值与负载关系',
    prompt: '分析当前电池负载与功耗'
  },
  {
    icon: Activity,
    category: 'IMU',
    title: '抑制振动噪声',
    detail: '检查 IMU 姿态抖动与频谱',
    prompt: '分析 IMU 振动噪声来源'
  },
  {
    icon: Thermometer,
    category: 'Thermal',
    title: '评估温升状态',
    detail: '关注温度斜率与环境漂移',
    prompt: '分析当前温度变化趋势'
  },
  {
    icon: Database,
    category: 'Calibration',
    title: '检查基准零点',
    detail: '对比历史零漂并更新标定',
    prompt: '给出传感器零点校准建议'
  },
  {
    icon: Gauge,
    category: 'Lidar',
    title: '优化测距稳定性',
    detail: '观察量测噪声并提高采样一致性',
    prompt: '分析测距数据的波动来源'
  },
  {
    icon: Wind,
    category: 'Acoustic',
    title: '分析噪声能量',
    detail: '评估声压级与气流相关性',
    prompt: '分析当前声学噪声的主要特征'
  }
]

const totalPages = computed(() =>
  Math.max(1, Math.ceil(suggestions.length / suggestionsPerPage))
)

const pagedSuggestions = computed(() => {
  const start = (currentPage.value - 1) * suggestionsPerPage
  return suggestions.slice(start, start + suggestionsPerPage)
})

function applySuggestion(prompt: string) {
  inputQuery.value = prompt
  sendQuery()
}

function prevPage() {
  if (currentPage.value > 1) currentPage.value -= 1
}

function nextPage() {
  if (currentPage.value < totalPages.value) currentPage.value += 1
}

function sendQuery() {
  if (!inputQuery.value.trim()) return
  aiResult.value = `已接收：${inputQuery.value}。建议结合当前传感器趋势判断是否需要进一步实验。`
  inputQuery.value = ''
}

</script>
