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
        <div class="px-8 pb-4 flex-1 overflow-y-auto">
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
            <div v-if="currentStep === 0" key="step0" class="space-y-8">
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
            <div v-else-if="currentStep === 1" key="step1" class="space-y-6 pb-20">
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
              <div v-if="wifiError" class="p-3 bg-red-50 text-red-600 text-xs rounded-xl">
                {{ wifiError }}
              </div>

              <!-- Wifi List -->
              <div
                class="rounded-2xl border border-gray-200 overflow-hidden bg-white max-h-[300px] overflow-y-auto"
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

            <!-- Step 2: Login -->
            <div v-else-if="currentStep === 2" key="step2" class="space-y-6 pb-20">
              <div>
                <h1 class="text-2xl font-semibold text-gray-900 tracking-tight">
                  {{ $t('onboarding.login_title') }}
                </h1>
                <p class="mt-2 text-sm text-gray-500">{{ $t('onboarding.login_subtitle') }}</p>
              </div>

              <div class="space-y-4">
                <div class="space-y-2">
                  <div class="text-xs font-semibold text-gray-500 uppercase tracking-wider">
                    {{ $t('onboarding.email') }}
                  </div>
                  <input
                    v-model="loginForm.email"
                    type="email"
                    class="w-full h-12 px-4 rounded-2xl border border-gray-200 bg-white text-sm text-gray-900 placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-black/10 focus:border-black transition"
                    placeholder="name@example.com"
                    @focus="showKeyboardFor('email')"
                  />
                </div>

                <div class="space-y-2">
                  <div class="text-xs font-semibold text-gray-500 uppercase tracking-wider">
                    {{ $t('onboarding.password') }}
                  </div>
                  <input
                    v-model="loginForm.password"
                    type="password"
                    class="w-full h-12 px-4 rounded-2xl border border-gray-200 bg-white text-sm text-gray-900 placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-black/10 focus:border-black transition"
                    placeholder="••••••••"
                    @focus="showKeyboardFor('loginPassword')"
                  />
                </div>

                <div class="flex items-center justify-between pt-1">
                  <label class="flex items-center gap-2 cursor-pointer">
                    <input
                      v-model="loginForm.remember"
                      type="checkbox"
                      class="w-4 h-4 rounded border-gray-300 text-black focus:ring-black"
                    />
                    <span class="text-sm text-gray-600">{{ $t('onboarding.remember_me') }}</span>
                  </label>
                  <a href="#" class="text-sm text-gray-900 font-medium hover:underline">{{
                    $t('onboarding.forgot_password')
                  }}</a>
                </div>
              </div>
            </div>

            <!-- Step 3: Terms -->
            <div v-else-if="currentStep === 3" key="step3" class="space-y-6">
              <div>
                <h1 class="text-2xl font-semibold text-gray-900 tracking-tight">
                  {{ $t('onboarding.terms_title') }}
                </h1>
                <p class="mt-2 text-sm text-gray-500">{{ $t('onboarding.terms_subtitle') }}</p>
              </div>

              <div
                class="rounded-2xl border border-gray-200 bg-gray-50/50 p-4 h-40 overflow-y-auto"
              >
                <div class="text-sm text-gray-500 text-center py-6">
                  {{ $t('onboarding.terms_empty') }}
                </div>
              </div>

              <label
                class="flex items-center gap-3 rounded-2xl border border-gray-200 bg-white px-4 py-3 cursor-pointer hover:bg-gray-50 transition-colors"
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

            <!-- Step 4: Calibration -->
            <div v-else-if="currentStep === 4" key="step4" class="space-y-6">
              <div>
                <h1 class="text-2xl font-semibold text-gray-900 tracking-tight">
                  {{ $t('onboarding.calibration_title') }}
                </h1>
                <p class="mt-2 text-sm text-gray-500">
                  {{ $t('onboarding.calibration_subtitle') }}
                </p>
              </div>

              <div class="space-y-3">
                <div
                  v-for="(item, index) in calibrationItems"
                  :key="index"
                  class="rounded-2xl border border-gray-200 bg-white px-4 py-4 flex items-center justify-between"
                >
                  <div class="flex items-center gap-3 min-w-0">
                    <div
                      class="w-10 h-10 rounded-2xl flex items-center justify-center"
                      :class="
                        calibrationStatus[item.key]
                          ? 'bg-green-50 text-green-700'
                          : 'bg-gray-100 text-gray-600'
                      "
                    >
                      <component :is="item.icon" class="w-5 h-5" />
                    </div>
                    <div class="min-w-0">
                      <div class="text-sm font-semibold text-gray-900 truncate">
                        {{ item.title }}
                      </div>
                      <div class="text-xs text-gray-500 truncate">{{ item.description }}</div>
                    </div>
                  </div>

                  <button
                    @click="calibrate(item.key)"
                    class="shrink-0 h-9 px-4 rounded-full text-xs font-semibold transition-colors"
                    :class="
                      calibrationStatus[item.key]
                        ? 'bg-gray-100 text-gray-500 cursor-default'
                        : 'bg-black text-white hover:bg-gray-800'
                    "
                    :disabled="calibrationStatus[item.key]"
                  >
                    {{
                      calibrationStatus[item.key]
                        ? $t('onboarding.calibration_done')
                        : $t('onboarding.calibration_start')
                    }}
                  </button>
                </div>
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
          @focus="showKeyboardFor('wifiPassword')"
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

    <!-- Virtual Keyboard -->
    <VirtualKeyboard
      :show="showKeyboard"
      :model-value="keyboardValue"
      @update:model-value="handleKeyboardInput"
      @close="showKeyboard = false"
      @enter="handleKeyboardEnter"
    />
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
import VirtualKeyboard from '@/components/VirtualKeyboard.vue'
import type { WifiNetwork } from '@/api/wifi'

const router = useRouter()
const localeStore = useLocaleStore()
const wifiStore = useWifiStore()

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

const currentStep = ref(0)
const selectedRegion = ref('us')
const selectedWifi = ref<WifiNetwork | null>(null)
const wifiPassword = ref('')
const loginForm = ref({ email: '', password: '', remember: false })
const acceptTerms = ref(false)
const calibrationStatus = ref<{ [key: string]: boolean }>({
  environment: false,
  temperature: false,
  engine: false,
})

const steps = ['Language & Region', 'WiFi', 'Login', 'Terms', 'Calibration']

// Keyboard State
const showKeyboard = ref(false)
const activeInput = ref<string | null>(null)

const keyboardValue = computed(() => {
  if (activeInput.value === 'wifiPassword') return wifiPassword.value
  if (activeInput.value === 'email') return loginForm.value.email
  if (activeInput.value === 'loginPassword') return loginForm.value.password
  return ''
})

function showKeyboardFor(field: string) {
  activeInput.value = field
  showKeyboard.value = true
}

function handleKeyboardInput(val: string) {
  if (activeInput.value === 'wifiPassword') wifiPassword.value = val
  else if (activeInput.value === 'email') loginForm.value.email = val
  else if (activeInput.value === 'loginPassword') loginForm.value.password = val
}

function handleKeyboardEnter() {
  showKeyboard.value = false
  if (activeInput.value === 'wifiPassword' && showWifiModal.value) {
    confirmConnect()
  }
}

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
  showKeyboard.value = false
  activeInput.value = null
}

async function confirmConnect() {
  if (!selectedWifi.value) return

  showKeyboard.value = false // Hide keyboard to show status
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

const calibrationItems = [
  {
    key: 'environment',
    title: 'Environment Pressure',
    description: 'Calibrate ambient pressure sensor',
    icon: Gauge,
  },
  {
    key: 'temperature',
    title: 'Temperature',
    description: 'Calibrate temperature sensor',
    icon: Thermometer,
  },
  { key: 'engine', title: 'Engine', description: 'Calibrate engine sensors', icon: Activity },
]

const canProceed = computed(() => {
  switch (currentStep.value) {
    case 0:
      return true
    case 1:
      // Require testResult to be true (internet connected)
      // Or if user insists on skipping? For now strict requirement as per user request "confirm network connection"
      return testResult.value === true
    case 2:
      return loginForm.value.email && loginForm.value.password
    case 3:
      return acceptTerms.value
    case 4:
      return Object.values(calibrationStatus.value).every((status) => status)
    default:
      return false
  }
})

const setRegion = (region: string) => {
  selectedRegion.value = region
}

const calibrate = (key: string) => {
  setTimeout(() => {
    calibrationStatus.value[key] = true
  }, 1000)
}

const prevStep = () => {
  if (currentStep.value > 0) {
    currentStep.value--
  }
}

const nextStep = () => {
  if (currentStep.value < steps.length - 1 && canProceed.value) {
    currentStep.value++
  } else if (currentStep.value === steps.length - 1 && canProceed.value) {
    setTimeout(() => {
      router.push('/')
    }, 500)
  }
}
</script>
