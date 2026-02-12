<template>
  <div class="fixed inset-0 bg-white font-sans">
    <div class="absolute inset-0 bg-gradient-to-b from-white via-white to-gray-50"></div>
    <div class="relative h-full w-full flex items-center justify-center p-6">
      <div class="w-full max-w-xl rounded-3xl border border-gray-200 bg-white shadow-sm overflow-hidden">
        <div class="px-8 pt-8 pb-6">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-2xl bg-black flex items-center justify-center">
                <svg width="18" height="28" viewBox="0 0 18 28" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M12.5348 21.7806L8.66407 16.8297L4.74836 21.7806H0L9.75257 13.7466L0.247552 6.02771H5.0184L8.73157 10.7761L12.3097 6.02771H17.0581L12.3097 13.8591L17.3056 21.7806H12.5348Z" fill="white" />
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
                :class="index === currentStep ? 'bg-black' : index < currentStep ? 'bg-black/40' : 'bg-gray-200'"
              ></div>
            </div>
          </div>

          <div class="mt-6">
            <div class="text-xs font-semibold text-gray-500">
              {{ $t('onboarding.title') }}
            </div>
          </div>
        </div>

        <div class="px-8 pb-8">
          <Transition
            enter-active-class="transition duration-300 ease-out"
            enter-from-class="opacity-0 translate-y-2"
            enter-to-class="opacity-100 translate-y-0"
            leave-active-class="transition duration-200 ease-in"
            leave-from-class="opacity-100"
            leave-to-class="opacity-0"
            mode="out-in"
          >
            <div v-if="currentStep === 0" key="step0" class="space-y-8">
              <div>
                <h1 class="text-2xl font-semibold text-gray-900 tracking-tight">{{ $t('onboarding.title') }}</h1>
                <p class="mt-2 text-sm text-gray-500">{{ $t('onboarding.subtitle') }}</p>
              </div>

              <div class="space-y-6">
                <div class="space-y-2">
                  <div class="text-xs font-semibold text-gray-500 uppercase tracking-wider">{{ $t('onboarding.language') }}</div>
                  <div class="rounded-2xl bg-gray-100 p-1 flex gap-1">
                    <button
                      @click="setLocale('en')"
                      class="flex-1 h-11 rounded-xl text-sm font-medium transition-all"
                      :class="currentLocale === 'en' ? 'bg-white shadow-sm text-gray-900' : 'text-gray-600 hover:text-gray-900'"
                    >
                      English
                    </button>
                    <button
                      @click="setLocale('zh')"
                      class="flex-1 h-11 rounded-xl text-sm font-medium transition-all"
                      :class="currentLocale === 'zh' ? 'bg-white shadow-sm text-gray-900' : 'text-gray-600 hover:text-gray-900'"
                    >
                      中文
                    </button>
                  </div>
                </div>

                <div class="space-y-2">
                  <div class="text-xs font-semibold text-gray-500 uppercase tracking-wider">{{ $t('onboarding.region') }}</div>
                  <div class="rounded-2xl bg-gray-100 p-1 grid grid-cols-3 gap-1">
                    <button
                      v-for="region in ['us', 'cn', 'eu']"
                      :key="region"
                      @click="setRegion(region)"
                      class="h-11 rounded-xl text-sm font-medium uppercase transition-all"
                      :class="selectedRegion === region ? 'bg-white shadow-sm text-gray-900' : 'text-gray-600 hover:text-gray-900'"
                    >
                      {{ region }}
                    </button>
                  </div>
                </div>
              </div>
            </div>

            <div v-else-if="currentStep === 1" key="step1" class="space-y-6">
              <div>
                <h1 class="text-2xl font-semibold text-gray-900 tracking-tight">{{ $t('onboarding.wifi_title') }}</h1>
                <p class="mt-2 text-sm text-gray-500">{{ $t('onboarding.wifi_subtitle') }}</p>
              </div>

              <div class="rounded-2xl border border-gray-200 overflow-hidden bg-white">
                <button
                  v-for="wifi in availableWifi"
                  :key="wifi.ssid"
                  @click="selectWifi(wifi)"
                  class="w-full px-4 py-3 flex items-center justify-between text-left transition-colors hover:bg-gray-50"
                  :class="selectedWifi?.ssid === wifi.ssid ? 'bg-gray-50' : ''"
                >
                  <div class="flex items-center gap-3 min-w-0">
                    <Wifi class="w-4 h-4 text-gray-500" />
                    <div class="min-w-0">
                      <div class="text-sm font-medium text-gray-900 truncate">{{ wifi.ssid }}</div>
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

              <div v-if="selectedWifi" class="space-y-2">
                <div class="text-xs font-semibold text-gray-500 uppercase tracking-wider">{{ $t('onboarding.wifi_password') }}</div>
                <input
                  v-model="wifiPassword"
                  type="password"
                  class="w-full h-12 px-4 rounded-2xl border border-gray-200 bg-white text-sm text-gray-900 placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-black/10 focus:border-black transition"
                  placeholder="••••••••"
                />
              </div>
            </div>

            <div v-else-if="currentStep === 2" key="step2" class="space-y-6">
              <div>
                <h1 class="text-2xl font-semibold text-gray-900 tracking-tight">{{ $t('onboarding.login_title') }}</h1>
                <p class="mt-2 text-sm text-gray-500">{{ $t('onboarding.login_subtitle') }}</p>
              </div>

              <div class="space-y-4">
                <div class="space-y-2">
                  <div class="text-xs font-semibold text-gray-500 uppercase tracking-wider">{{ $t('onboarding.email') }}</div>
                  <input
                    v-model="loginForm.email"
                    type="email"
                    class="w-full h-12 px-4 rounded-2xl border border-gray-200 bg-white text-sm text-gray-900 placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-black/10 focus:border-black transition"
                    placeholder="name@example.com"
                  />
                </div>

                <div class="space-y-2">
                  <div class="text-xs font-semibold text-gray-500 uppercase tracking-wider">{{ $t('onboarding.password') }}</div>
                  <input
                    v-model="loginForm.password"
                    type="password"
                    class="w-full h-12 px-4 rounded-2xl border border-gray-200 bg-white text-sm text-gray-900 placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-black/10 focus:border-black transition"
                    placeholder="••••••••"
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
                  <a href="#" class="text-sm text-gray-900 font-medium hover:underline">{{ $t('onboarding.forgot_password') }}</a>
                </div>
              </div>
            </div>

            <div v-else-if="currentStep === 3" key="step3" class="space-y-6">
              <div>
                <h1 class="text-2xl font-semibold text-gray-900 tracking-tight">{{ $t('onboarding.terms_title') }}</h1>
                <p class="mt-2 text-sm text-gray-500">{{ $t('onboarding.terms_subtitle') }}</p>
              </div>

              <div class="rounded-2xl border border-gray-200 bg-gray-50/50 p-4 h-40 overflow-y-auto">
                <div class="text-sm text-gray-500 text-center py-6">
                  {{ $t('onboarding.terms_empty') }}
                </div>
              </div>

              <label class="flex items-center gap-3 rounded-2xl border border-gray-200 bg-white px-4 py-3 cursor-pointer hover:bg-gray-50 transition-colors">
                <input
                  v-model="acceptTerms"
                  type="checkbox"
                  class="w-4 h-4 rounded border-gray-300 text-black focus:ring-black"
                />
                <span class="text-sm font-medium text-gray-900">{{ $t('onboarding.terms_accept') }}</span>
              </label>
            </div>

            <div v-else-if="currentStep === 4" key="step4" class="space-y-6">
              <div>
                <h1 class="text-2xl font-semibold text-gray-900 tracking-tight">{{ $t('onboarding.calibration_title') }}</h1>
                <p class="mt-2 text-sm text-gray-500">{{ $t('onboarding.calibration_subtitle') }}</p>
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
                      :class="calibrationStatus[item.key] ? 'bg-green-50 text-green-700' : 'bg-gray-100 text-gray-600'"
                    >
                      <component :is="item.icon" class="w-5 h-5" />
                    </div>
                    <div class="min-w-0">
                      <div class="text-sm font-semibold text-gray-900 truncate">{{ item.title }}</div>
                      <div class="text-xs text-gray-500 truncate">{{ item.description }}</div>
                    </div>
                  </div>

                  <button
                    @click="calibrate(item.key)"
                    class="shrink-0 h-9 px-4 rounded-full text-xs font-semibold transition-colors"
                    :class="calibrationStatus[item.key] ? 'bg-gray-100 text-gray-500 cursor-default' : 'bg-black text-white hover:bg-gray-800'"
                    :disabled="calibrationStatus[item.key]"
                  >
                    {{ calibrationStatus[item.key] ? $t('onboarding.calibration_done') : $t('onboarding.calibration_start') }}
                  </button>
                </div>
              </div>
            </div>
          </Transition>
        </div>

        <div class="px-8 py-6 border-t border-gray-100 bg-gray-50/40 flex items-center justify-between">
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
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { ArrowRight, Wifi, Gauge, Thermometer, Activity } from 'lucide-vue-next'
import { useLocaleStore } from '@/stores/locale'
import { storeToRefs } from 'pinia'

const router = useRouter()
const localeStore = useLocaleStore()
const { currentLocale } = storeToRefs(localeStore)
const { setLocale } = localeStore

const currentStep = ref(0)
const selectedRegion = ref('us')
const selectedWifi = ref<any>(null)
const wifiPassword = ref('')
const loginForm = ref({ email: '', password: '', remember: false })
const acceptTerms = ref(false)
const calibrationStatus = ref<{ [key: string]: boolean }>({ environment: false, temperature: false, engine: false })

const steps = ['Language & Region', 'WiFi', 'Login', 'Terms', 'Calibration']

const availableWifi = ref([
  { ssid: 'Home WiFi', signal: 90 },
  { ssid: 'Office Network', signal: 75 },
  { ssid: 'Public WiFi', signal: 45 },
  { ssid: 'Mobile Hotspot', signal: 60 }
])

const calibrationItems = [
  { key: 'environment', title: 'Environment Pressure', description: 'Calibrate ambient pressure sensor', icon: Gauge },
  { key: 'temperature', title: 'Temperature', description: 'Calibrate temperature sensor', icon: Thermometer },
  { key: 'engine', title: 'Engine', description: 'Calibrate engine sensors', icon: Activity }
]

const canProceed = computed(() => {
  switch (currentStep.value) {
    case 0:
      return true
    case 1:
      return !!selectedWifi.value
    case 2:
      return loginForm.value.email && loginForm.value.password
    case 3:
      return acceptTerms.value
    case 4:
      return Object.values(calibrationStatus.value).every(status => status)
    default:
      return false
  }
})

const setRegion = (region: string) => {
  selectedRegion.value = region
}

const selectWifi = (wifi: any) => {
  selectedWifi.value = wifi
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
