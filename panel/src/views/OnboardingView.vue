<template>
  <div class="fixed inset-0 bg-white font-sans">
    <div class="absolute inset-0 bg-gradient-to-b from-white via-white to-gray-50"></div>
    <div class="relative h-full w-full flex items-center justify-center p-6">
      <div
        class="w-full max-w-xl rounded-3xl border border-gray-200 bg-white shadow-sm overflow-hidden flex flex-col max-h-full"
      >
        <!-- Header -->
        <div class="px-8 pt-8 pb-6 shrink-0">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-2xl bg-black flex items-center justify-center">
                <svg
                  width="18"
                  height="28"
                  viewBox="0 0 18 28"
                  fill="none"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    d="M12.5348 21.7806L8.66407 16.8297L4.74836 21.7806H0L9.75257 13.7466L0.247552 6.02771H5.0184L8.73157 10.7761L12.3097 6.02771H17.0581L12.3097 13.8591L17.3056 21.7806H12.5348Z"
                    fill="white"
                  />
                </svg>
              </div>
              <div class="leading-tight">
                <div class="text-sm font-semibold text-gray-900 tracking-tight">AeroSmart</div>
                <div class="text-xs text-gray-500">{{ $t('onboarding.subtitle') }}</div>
              </div>
            </div>

            <div class="flex items-center gap-2">
              <div
                v-for="(_, index) in steps"
                :key="index"
                class="h-1.5 w-8 rounded-full transition-colors"
                :class="
                  index === currentStep
                    ? 'bg-black'
                    : index < currentStep
                      ? 'bg-black/40'
                      : 'bg-gray-200'
                "
              ></div>
            </div>
          </div>
        </div>

        <!-- Scrollable Content Area -->
        <div class="px-8 pb-4 flex-1 flex flex-col min-h-0 overflow-hidden">
          <Transition
            enter-active-class="transition duration-300 ease-out"
            enter-from-class="opacity-0 translate-y-2"
            enter-to-class="opacity-100 translate-y-0"
            leave-active-class="transition duration-200 ease-in"
            leave-from-class="opacity-100"
            leave-to-class="opacity-0"
            mode="out-in"
          >
            <!-- Step 0: Language & Region -->
            <div v-if="currentStep === 0" key="step0" class="space-y-8 overflow-y-auto">
              <div>
                <h1 class="text-2xl font-semibold text-gray-900 tracking-tight">
                  {{ $t('onboarding.title') }}
                </h1>
                <p class="mt-2 text-sm text-gray-500">{{ $t('onboarding.subtitle') }}</p>
              </div>

              <div class="space-y-6">
                <div class="space-y-2">
                  <div class="text-xs font-semibold text-gray-500 uppercase tracking-wider">
                    {{ $t('onboarding.language') }}
                  </div>
                  <div class="rounded-2xl bg-gray-100 p-1 flex gap-1">
                    <button
                      @click="setLocale('en')"
                      class="flex-1 h-11 rounded-xl text-sm font-medium transition-all"
                      :class="
                        currentLocale === 'en'
                          ? 'bg-white shadow-sm text-gray-900'
                          : 'text-gray-600 hover:text-gray-900'
                      "
                    >
                      English
                    </button>
                    <button
                      @click="setLocale('zh')"
                      class="flex-1 h-11 rounded-xl text-sm font-medium transition-all"
                      :class="
                        currentLocale === 'zh'
                          ? 'bg-white shadow-sm text-gray-900'
                          : 'text-gray-600 hover:text-gray-900'
                      "
                    >
                      中文
                    </button>
                  </div>
                </div>

                <div class="space-y-2">
                  <div class="text-xs font-semibold text-gray-500 uppercase tracking-wider">
                    {{ $t('onboarding.region') }}
                  </div>
                  <div class="rounded-2xl bg-gray-100 p-1 grid grid-cols-3 gap-1">
                    <button
                      v-for="region in ['us', 'cn', 'eu']"
                      :key="region"
                      @click="setRegion(region)"
                      class="h-11 rounded-xl text-sm font-medium uppercase transition-all"
                      :class="
                        selectedRegion === region
                          ? 'bg-white shadow-sm text-gray-900'
                          : 'text-gray-600 hover:text-gray-900'
                      "
                    >
                      {{ region }}
                    </button>
                  </div>
                </div>
              </div>
            </div>

            <!-- Step 1: WiFi -->
            <div v-else-if="currentStep === 1" key="step1" class="flex flex-col h-full space-y-4">
              <div class="shrink-0">
                <div class="flex justify-between items-start">
                  <div>
                    <h1 class="text-2xl font-semibold text-gray-900 tracking-tight">
                      {{ $t('onboarding.wifi_title') }}
                    </h1>
                    <p class="mt-2 text-sm text-gray-500">{{ $t('onboarding.wifi_subtitle') }}</p>
                  </div>
                  <div class="flex items-center">
                    <button
                      @click="showSkipWifiModal = true"
                      class="mr-2 text-sm text-gray-500 hover:text-gray-900 font-medium px-3 py-2 rounded-lg hover:bg-gray-100 transition-colors"
                    >
                      {{ $t('onboarding.wifi_skip') }}
                    </button>
                    <button
                      @click="wifiStore.scan()"
                      :disabled="scanning"
                      class="p-2 rounded-full hover:bg-gray-100 disabled:opacity-50"
                    >
                      <RefreshCw
                        :class="[
                          'w-5 h-5',
                          scanning ? 'animate-spin text-blue-600' : 'text-gray-600',
                        ]"
                      />
                    </button>
                  </div>
                </div>

                <!-- Error Message -->
                <div v-if="wifiError" class="mt-4 p-3 bg-red-50 text-red-600 text-xs rounded-xl">
                  {{ wifiError }}
                </div>
              </div>

              <!-- Wifi List -->
              <div
                class="flex-1 rounded-2xl border border-gray-200 overflow-hidden bg-white overflow-y-auto min-h-0"
              >
                <div
                  v-if="networks.length === 0 && !scanning"
                  class="p-6 text-center text-gray-400 text-sm"
                >
                  No networks found
                </div>

                <button
                  v-for="wifi in networks"
                  :key="wifi.ssid"
                  @click="selectWifi(wifi)"
                  class="w-full px-4 py-3 flex items-center justify-between text-left transition-colors hover:bg-gray-50 border-b border-gray-100 last:border-0"
                  :class="selectedWifi?.ssid === wifi.ssid ? 'bg-gray-50' : ''"
                >
                  <div class="flex items-center gap-3 min-w-0">
                    <Wifi class="w-4 h-4 text-gray-500" />
                    <div class="min-w-0">
                      <div class="text-sm font-medium text-gray-900 truncate">{{ wifi.ssid }}</div>
                      <div class="text-[10px] text-gray-400 flex items-center gap-2">
                        <span>{{ wifi.security }}</span>
                        <span v-if="wifi.in_use" class="text-green-600 font-bold">Connected</span>
                      </div>
                    </div>
                  </div>
                  <div class="flex items-center gap-3">
                    <div class="text-xs text-gray-500 tabular-nums">{{ wifi.signal }}%</div>
                    <div class="w-5 text-right text-gray-900">
                      <span v-if="selectedWifi?.ssid === wifi.ssid" class="text-sm">✓</span>
                    </div>
                  </div>
                </button>
              </div>

              <!-- Connection Status -->
              <div class="shrink-0">
                <div
                  v-if="connecting || testing"
                  class="flex items-center justify-center gap-2 py-4 text-sm text-gray-600"
                >
                  <Loader2 class="w-4 h-4 animate-spin" />
                  <span>{{
                    connecting ? 'Connecting to WiFi...' : 'Testing Internet Connection...'
                  }}</span>
                </div>
                <div
                  v-else-if="testResult === true"
                  class="flex items-center justify-center gap-2 py-4 text-sm text-green-600 font-medium"
                >
                  <CheckCircle2 class="w-4 h-4" />
                  <span>Internet Connected (bilibili.com reachable)</span>
                </div>
                <div
                  v-else-if="testResult === false"
                  class="flex items-center justify-center gap-2 py-4 text-sm text-red-600 font-medium"
                >
                  <XCircle class="w-4 h-4" />
                  <span>Internet Unreachable</span>
                </div>
              </div>
            </div>

            <!-- Step 2: Terms -->
            <div v-else-if="currentStep === 2" key="step2" class="flex flex-col h-full space-y-6">
              <div class="shrink-0">
                <h1 class="text-2xl font-semibold text-gray-900 tracking-tight">
                  {{ $t('onboarding.terms_title') }}
                </h1>
                <p class="mt-2 text-sm text-gray-500">{{ $t('onboarding.terms_subtitle') }}</p>
              </div>

              <div
                class="flex-1 rounded-2xl border border-gray-200 bg-gray-50/50 p-4 overflow-y-auto min-h-0"
              >
                <div class="text-sm text-gray-500 text-center py-6">
                  {{ $t('onboarding.terms_empty') }}
                </div>
              </div>

              <label
                class="shrink-0 flex items-center gap-3 rounded-2xl border border-gray-200 bg-white px-4 py-3 cursor-pointer hover:bg-gray-50 transition-colors"
              >
                <input
                  v-model="acceptTerms"
                  type="checkbox"
                  class="w-4 h-4 rounded border-gray-300 text-black focus:ring-black"
                />
                <span class="text-sm font-medium text-gray-900">{{
                  $t('onboarding.terms_accept')
                }}</span>
              </label>
            </div>

            <!-- Step 3: Activation -->
            <div
              v-else-if="currentStep === 3"
              key="step3"
              class="flex flex-col h-full justify-center"
            >
              <div class="shrink-0 mb-6">
                <h1 class="text-2xl font-semibold text-gray-900 tracking-tight">
                  Device Activation
                </h1>
                <p class="mt-2 text-sm text-gray-500">Connect and synchronize with the hardware.</p>
              </div>

              <div class="flex-1 flex flex-col items-center justify-center space-y-6">
                <!-- Status Icon -->
                <div
                  class="w-20 h-20 rounded-full flex items-center justify-center transition-colors cursor-pointer select-none"
                  :class="{
                    'bg-gray-100': activationStatus === 'idle',
                    'bg-blue-50': activationStatus === 'activating',
                    'bg-green-50': activationStatus === 'success',
                    'bg-red-50': activationStatus === 'error',
                    'bg-yellow-50': activationStatus === 'skipped',
                  }"
                  @click="handleErrorClick"
                >
                  <Wifi v-if="activationStatus === 'idle'" class="w-10 h-10 text-gray-400" />
                  <Loader2
                    v-else-if="activationStatus === 'activating'"
                    class="w-10 h-10 text-blue-600 animate-spin"
                  />
                  <CheckCircle2
                    v-else-if="activationStatus === 'success'"
                    class="w-10 h-10 text-green-600"
                  />
                  <XCircle
                    v-else-if="activationStatus === 'error'"
                    class="w-10 h-10 text-red-600"
                  />
                  <CheckCircle2
                    v-else-if="activationStatus === 'skipped'"
                    class="w-10 h-10 text-yellow-600"
                  />
                </div>

                <!-- Status Text -->
                <div class="text-center space-y-2">
                  <h3 class="text-lg font-bold text-gray-900">
                    {{
                      activationStatus === 'idle'
                        ? 'Ready to Activate'
                        : activationStatus === 'activating'
                          ? 'Activating...'
                          : activationStatus === 'success'
                            ? 'Activation Successful'
                            : activationStatus === 'skipped'
                              ? 'Activation Skipped'
                              : 'Activation Failed'
                    }}
                  </h3>
                  <p class="text-sm text-gray-500 max-w-xs mx-auto">
                    {{
                      activationStatus === 'idle'
                        ? 'Click the button below to start the device activation process.'
                        : activationStatus === 'activating'
                          ? 'Connecting to serial port and syncing hardware clock...'
                          : activationStatus === 'success'
                            ? 'Device connected and synchronized. You can now finish setup.'
                            : activationStatus === 'skipped'
                              ? 'Device activation was skipped. You can finish setup now.'
                              : activationError || 'An unknown error occurred.'
                    }}
                  </p>
                </div>

                <!-- Action Button -->
                <button
                  v-if="activationStatus === 'idle' || activationStatus === 'error'"
                  @click="startActivation"
                  class="px-8 py-3 bg-black text-white rounded-xl font-bold hover:bg-gray-800 transition-colors shadow-lg shadow-gray-200"
                >
                  开始激活设备
                </button>
              </div>
            </div>
          </Transition>
        </div>

        <!-- Footer Actions -->
        <div
          class="px-8 py-6 border-t border-gray-100 bg-gray-50/40 flex items-center justify-between shrink-0"
        >
          <button
            v-if="currentStep > 0"
            @click="prevStep"
            class="h-10 px-4 rounded-full text-sm font-medium text-gray-700 hover:bg-gray-100 transition-colors"
          >
            {{ $t('common.back') }}
          </button>
          <div v-else class="h-10"></div>

          <button
            @click="nextStep"
            class="h-10 px-5 rounded-full bg-black text-white text-sm font-semibold hover:bg-gray-800 transition-colors disabled:opacity-50 disabled:hover:bg-black"
            :disabled="!canProceed"
          >
            <span class="inline-flex items-center gap-2">
              {{ currentStep === steps.length - 1 ? $t('common.finish') : $t('common.next') }}
              <ArrowRight class="w-4 h-4" />
            </span>
          </button>
        </div>
      </div>
    </div>

    <!-- WiFi Password Modal -->
    <div
      v-if="showWifiModal"
      class="fixed inset-0 bg-black/40 backdrop-blur-sm flex items-center justify-center z-40"
    >
      <div class="bg-white p-6 rounded-3xl shadow-xl w-80 flex flex-col gap-4">
        <h3 class="text-lg font-bold text-gray-900">Connect to {{ selectedWifi?.ssid }}</h3>
        <input
          v-model="wifiPassword"
          type="password"
          placeholder="Password"
          class="bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 text-sm outline-none focus:border-black focus:ring-1 focus:ring-black"
          @focus="keyboardStore.open(wifiPassword, (val) => (wifiPassword = val), confirmConnect)"
        />
        <div class="flex justify-end gap-2 mt-2">
          <button @click="closeWifiModal" class="text-sm text-gray-500 px-4 py-2 font-medium">
            Cancel
          </button>
          <button
            @click="confirmConnect"
            :disabled="connecting"
            class="text-sm bg-black text-white px-5 py-2 rounded-xl hover:bg-gray-800 disabled:opacity-50 font-bold"
          >
            {{ connecting ? 'Connecting...' : 'Connect' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Skip WiFi Confirmation Modal -->
    <div
      v-if="showSkipWifiModal"
      class="fixed inset-0 bg-black/40 backdrop-blur-sm flex items-center justify-center z-40"
    >
      <div class="bg-white p-6 rounded-3xl shadow-xl w-80 flex flex-col gap-4">
        <h3 class="text-lg font-bold text-gray-900">{{ $t('onboarding.skip_wifi_title') }}</h3>
        <p class="text-sm text-gray-500">
          {{ $t('onboarding.skip_wifi_message') }}
        </p>
        <div class="flex justify-end gap-2 mt-2">
          <button
            @click="showSkipWifiModal = false"
            class="text-sm text-gray-500 px-4 py-2 font-medium hover:bg-gray-50 rounded-xl"
          >
            {{ $t('common.cancel') }}
          </button>
          <button
            @click="confirmSkipWifi"
            class="text-sm bg-black text-white px-5 py-2 rounded-xl hover:bg-gray-800 font-bold"
          >
            {{ $t('common.confirm') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import {
  ArrowRight,
  Wifi,
  Gauge,
  Thermometer,
  Activity,
  RefreshCw,
  Loader2,
  CheckCircle2,
  XCircle,
} from 'lucide-vue-next'
import { useLocaleStore } from '@/stores/locale'
import { useWifiStore } from '@/stores/wifi'
import { storeToRefs } from 'pinia'
import { useKeyboardStore } from '@/stores/keyboard'
import type { WifiNetwork } from '@/api/wifi'

import { invoke } from '@tauri-apps/api/core'

const router = useRouter()
const localeStore = useLocaleStore()
const wifiStore = useWifiStore()
const keyboardStore = useKeyboardStore()

const { currentLocale } = storeToRefs(localeStore)
const { setLocale } = localeStore
const {
  networks,
  scanning,
  connecting,
  testing,
  testResult,
  error: wifiError,
} = storeToRefs(wifiStore)

interface AppConfig {
  serial: {
    port: string
    baud_rate: number
    handshake_timeout_secs: number
    retry_interval_secs: number
  }
  server: {
    port: number
    host: string
  }
  rules: {
    debug_mode: boolean
    enable_onboarding: boolean
  }
}

const currentStep = ref(0)
const selectedRegion = ref('us')
const selectedWifi = ref<WifiNetwork | null>(null)
const wifiPassword = ref('')
const loginForm = ref({ email: '', password: '', remember: false })
const acceptTerms = ref(false)
const activationStatus = ref<'idle' | 'activating' | 'success' | 'error' | 'skipped'>('idle')
const activationError = ref('')
const errorClickCount = ref(0)

const steps = ['Language & Region', 'WiFi', 'Terms', 'Activation']

// Wifi Logic
const showWifiModal = ref(false)
const showSkipWifiModal = ref(false)

onMounted(() => {
  // If we start at step 1, scan
  if (currentStep.value === 1) wifiStore.scan()
})

watch(currentStep, (newStep) => {
  if (newStep === 1) {
    wifiStore.scan()
  }
})

function selectWifi(wifi: WifiNetwork) {
  if (wifi.in_use) {
    selectedWifi.value = wifi
    // Already connected, maybe test connectivity?
    wifiStore.testConnection()
    return
  }

  selectedWifi.value = wifi
  wifiPassword.value = ''
  showWifiModal.value = true
}

function closeWifiModal() {
  showWifiModal.value = false
  wifiPassword.value = ''
  keyboardStore.close()
}

async function confirmConnect() {
  if (!selectedWifi.value) return

  keyboardStore.close() // Hide keyboard to show status
  try {
    await wifiStore.connect(selectedWifi.value.ssid, wifiPassword.value)
    showWifiModal.value = false
    // Auto test after connect
    await wifiStore.testConnection()
  } catch (e) {
    // Error is in store
  }
}

function confirmSkipWifi() {
  showSkipWifiModal.value = false
  currentStep.value += 2
}

async function startActivation() {
  activationStatus.value = 'activating'
  activationError.value = ''

  try {
    const startRes = await fetch('http://localhost:3000/api/activation/start', { method: 'POST' })
    if (!startRes.ok) throw new Error('Failed to start activation')

    // Poll status
    const poll = setInterval(async () => {
      try {
        if (activationStatus.value === 'skipped') {
          clearInterval(poll)
          return
        }
        const res = await fetch('http://localhost:3000/api/activation/status')
        const data = await res.json()
        console.log('Activation Status:', data)

        if (data === 'Active') {
          activationStatus.value = 'success'
          clearInterval(poll)
        } else if (typeof data === 'object' && 'Failed' in data) {
          activationStatus.value = 'error'
          activationError.value = data.Failed
          clearInterval(poll)
        }
      } catch (e) {
        console.error(e)
      }
    }, 1000)
  } catch (e) {
    activationStatus.value = 'error'
    activationError.value = String(e)
  }
}

const handleErrorClick = () => {
  if (activationStatus.value === 'error') {
    errorClickCount.value++
    if (errorClickCount.value >= 5) {
      activationStatus.value = 'skipped'
    }
  }
}

const canProceed = computed(() => {
  switch (currentStep.value) {
    case 0:
      return true
    case 1:
      // Require testResult to be true (internet connected)
      // Or if user insists on skipping? For now strict requirement as per user request "confirm network connection"
      return testResult.value === true
    case 2:
      return acceptTerms.value
    case 3:
      return activationStatus.value === 'success' || activationStatus.value === 'skipped'
    default:
      return false
  }
})

const setRegion = (region: string) => {
  selectedRegion.value = region
}

const prevStep = () => {
  if (currentStep.value > 0) {
    currentStep.value--
  }
}

const nextStep = async () => {
  if (currentStep.value < steps.length - 1 && canProceed.value) {
    currentStep.value++
  } else if (currentStep.value === steps.length - 1 && canProceed.value) {
    try {
      const config = await invoke<AppConfig>('get_app_config')
      config.rules.enable_onboarding = false
      await invoke('save_app_config', { config })
    } catch (e) {
      console.error('Failed to update config:', e)
    }

    setTimeout(() => {
      router.push('/')
    }, 500)
  }
}
</script>
