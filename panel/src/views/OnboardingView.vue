<template>
  <div class="fixed inset-0 bg-white flex flex-col h-screen w-screen z-[9999]">
    <!-- Logo Container - Top Area -->
    <div class="flex items-center justify-center pt-8 px-4">
      <div class="relative w-24 h-24 flex items-center justify-center">
        <div class="absolute inset-0 bg-gray-100 rounded-full animate-pulse"></div>
        <svg
          width="60"
          height="90"
          viewBox="0 0 18 28"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          class="z-10 drop-shadow-xl"
        >
          <path
            d="M12.5348 21.7806L8.66407 16.8297L4.74836 21.7806H0L9.75257 13.7466L0.247552 6.02771H5.0184L8.73157 10.7761L12.3097 6.02771H17.0581L12.3097 13.8591L17.3056 21.7806H12.5348Z"
            fill="black"
          />
        </svg>
      </div>
    </div>

    <!-- Step Progress -->
    <div class="flex items-center justify-center gap-4 py-6 px-4">
      <div 
        v-for="(step, index) in steps" 
        :key="index"
        class="flex items-center"
      >
        <div 
          class="w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold transition-all duration-300"
          :class="{
            'bg-black text-white': currentStep <= index,
            'bg-gray-100 text-gray-400': currentStep > index
          }"
        >
          {{ index + 1 }}
        </div>
        <div 
          v-if="index < steps.length - 1"
          class="h-1 w-12 md:w-16 mx-2 transition-all duration-300"
          :class="{
            'bg-black': currentStep <= index,
            'bg-gray-200': currentStep > index
          }"
        ></div>
      </div>
    </div>

    <!-- Step Content -->
    <div class="flex-grow flex flex-col justify-center px-4 py-4 overflow-y-auto">
      <!-- Step 1: Language & Region Selection -->
      <div v-if="currentStep === 0" class="space-y-6 max-w-md mx-auto">
        <div class="text-center space-y-3">
          <h1 class="text-2xl font-bold text-black tracking-tight">{{ $t('onboarding.title') }}</h1>
          <p class="text-gray-500 text-base">{{ $t('onboarding.subtitle') }}</p>
        </div>

        <!-- Language Selection -->
        <div class="space-y-3">
          <h2 class="text-base font-semibold text-gray-700 text-center">{{ $t('onboarding.language') }}</h2>
          <div class="flex flex-wrap justify-center gap-3">
            <button
              @click="setLocale('en')"
              class="px-6 py-2 rounded-full border text-sm font-medium transition-all duration-200 w-40"
              :class="
                currentLocale === 'en'
                  ? 'bg-black text-white border-black'
                  : 'bg-white text-black border-gray-200 hover:border-black'
              "
            >
              English
            </button>
            <button
              @click="setLocale('zh')"
              class="px-6 py-2 rounded-full border text-sm font-medium transition-all duration-200 w-40"
              :class="
                currentLocale === 'zh'
                  ? 'bg-black text-white border-black'
                  : 'bg-white text-black border-gray-200 hover:border-black'
              "
            >
              中文
            </button>
          </div>
        </div>

        <!-- Region Selection -->
        <div class="space-y-3">
          <h2 class="text-base font-semibold text-gray-700 text-center">{{ $t('onboarding.region') }}</h2>
          <div class="flex flex-wrap justify-center gap-3">
            <button
              @click="setRegion('us')"
              class="px-6 py-2 rounded-full border text-sm font-medium transition-all duration-200 w-36"
              :class="
                selectedRegion === 'us'
                  ? 'bg-black text-white border-black'
                  : 'bg-white text-black border-gray-200 hover:border-black'
              "
            >
              US
            </button>
            <button
              @click="setRegion('cn')"
              class="px-6 py-2 rounded-full border text-sm font-medium transition-all duration-200 w-36"
              :class="
                selectedRegion === 'cn'
                  ? 'bg-black text-white border-black'
                  : 'bg-white text-black border-gray-200 hover:border-black'
              "
            >
              China
            </button>
            <button
              @click="setRegion('eu')"
              class="px-6 py-2 rounded-full border text-sm font-medium transition-all duration-200 w-36"
              :class="
                selectedRegion === 'eu'
                  ? 'bg-black text-white border-black'
                  : 'bg-white text-black border-gray-200 hover:border-black'
              "
            >
              EU
            </button>
          </div>
        </div>
      </div>

      <!-- Step 2: WiFi Selection -->
      <div v-if="currentStep === 1" class="space-y-3 max-w-md mx-auto">
        <div class="text-center space-y-2">
          <h1 class="text-xl font-bold text-black tracking-tight">{{ $t('onboarding.wifi_title') }}</h1>
          <p class="text-gray-500 text-sm">{{ $t('onboarding.wifi_subtitle') }}</p>
        </div>

        <div class="space-y-1.5">
          <div 
            v-for="wifi in availableWifi" 
            :key="wifi.ssid"
            @click="selectWifi(wifi)"
            class="p-2 rounded-xl border transition-all duration-200 cursor-pointer"
            :class="
              selectedWifi?.ssid === wifi.ssid
                ? 'bg-black text-white border-black'
                : 'bg-white text-black border-gray-200 hover:border-black'
            "
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <Wifi class="w-3 h-3" />
                <div>
                  <div class="font-medium text-sm">{{ wifi.ssid }}</div>
                  <div class="text-xs text-gray-400" v-if="selectedWifi?.ssid === wifi.ssid">
                    {{ $t('onboarding.wifi_selected') }}
                  </div>
                </div>
              </div>
              <div class="flex items-center gap-1">
                <div 
                  class="w-2 h-2 rounded-full"
                  :class="{
                    'bg-green-500': wifi.signal > 70,
                    'bg-yellow-500': wifi.signal > 40 && wifi.signal <= 70,
                    'bg-red-500': wifi.signal <= 40
                  }"
                ></div>
                <span class="text-xs">{{ wifi.signal }}%</span>
              </div>
            </div>
            
            <!-- Password input for selected WiFi -->
            <div v-if="selectedWifi?.ssid === wifi.ssid" class="mt-2 space-y-1.5">
              <h3 class="text-xs font-semibold text-gray-700">{{ $t('onboarding.wifi_password') }}</h3>
              <input 
                v-model="wifiPassword"
                type="password"
                class="w-full px-2 py-1.5 rounded-xl border border-gray-200 focus:outline-none focus:border-black transition-all duration-200 text-sm"
                placeholder="Enter WiFi password"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- Step 3: Login -->
      <div v-if="currentStep === 2" class="space-y-4 max-w-md mx-auto">
        <div class="text-center space-y-2">
          <h1 class="text-xl font-bold text-black tracking-tight">{{ $t('onboarding.login_title') }}</h1>
          <p class="text-gray-500 text-sm">{{ $t('onboarding.login_subtitle') }}</p>
        </div>

        <div class="space-y-2">
          <div class="space-y-1.5">
            <label class="text-xs font-semibold text-gray-700 block">{{ $t('onboarding.email') }}</label>
            <input 
              v-model="loginForm.email"
              type="email"
              class="w-full px-2 py-1.5 rounded-xl border border-gray-200 focus:outline-none focus:border-black transition-all duration-200 text-sm"
              placeholder="your.email@example.com"
            />
          </div>

          <div class="space-y-1.5">
            <label class="text-xs font-semibold text-gray-700 block">{{ $t('onboarding.password') }}</label>
            <input 
              v-model="loginForm.password"
              type="password"
              class="w-full px-2 py-1.5 rounded-xl border border-gray-200 focus:outline-none focus:border-black transition-all duration-200 text-sm"
              placeholder="Enter your password"
            />
          </div>

          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <input 
                v-model="loginForm.remember"
                type="checkbox"
                id="remember"
                class="w-3 h-3 rounded border-gray-300 text-black focus:ring-black"
              />
              <label for="remember" class="text-xs text-gray-600">{{ $t('onboarding.remember_me') }}</label>
            </div>
            <a href="#" class="text-xs text-black font-medium hover:underline">{{ $t('onboarding.forgot_password') }}</a>
          </div>
        </div>
      </div>

      <!-- Step 4: Terms & Conditions -->
      <div v-if="currentStep === 3" class="space-y-3 max-w-md mx-auto">
        <div class="text-center space-y-2">
          <h1 class="text-xl font-bold text-black tracking-tight">{{ $t('onboarding.terms_title') }}</h1>
          <p class="text-gray-500 text-sm">{{ $t('onboarding.terms_subtitle') }}</p>
        </div>

        <div class="p-3 rounded-xl border border-gray-200 bg-gray-50 h-32 overflow-y-auto">
          <!-- Empty content as requested -->
          <div class="text-center text-gray-400 py-4">
            <p class="text-sm">{{ $t('onboarding.terms_empty') }}</p>
          </div>
        </div>

        <div class="flex items-center gap-2">
          <input 
            v-model="acceptTerms"
            type="checkbox"
            id="terms"
            class="w-3 h-3 rounded border-gray-300 text-black focus:ring-black"
          />
          <label for="terms" class="text-xs text-gray-600">{{ $t('onboarding.terms_accept') }}</label>
        </div>
      </div>

      <!-- Step 5: Calibration -->
      <div v-if="currentStep === 4" class="space-y-4 max-w-md mx-auto">
        <div class="text-center space-y-2">
          <h1 class="text-xl font-bold text-black tracking-tight">{{ $t('onboarding.calibration_title') }}</h1>
          <p class="text-gray-500 text-sm">{{ $t('onboarding.calibration_subtitle') }}</p>
        </div>

        <div class="space-y-2">
          <div 
            v-for="(item, index) in calibrationItems" 
            :key="index"
            class="p-2 rounded-xl border transition-all duration-200"
            :class="{
              'bg-green-50 border-green-200 text-green-700': calibrationStatus[item.key],
              'bg-white border-gray-200 text-black': !calibrationStatus[item.key]
            }"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <component :is="item.icon" class="w-3 h-3" />
                <div>
                  <div class="font-medium text-sm">{{ item.title }}</div>
                  <div class="text-xs text-gray-400">{{ item.description }}</div>
                </div>
              </div>
              <button
                @click="calibrate(item.key)"
                class="px-2 py-1 rounded-full text-xs font-medium transition-all duration-200 min-w-[80px]"
                :class="{
                  'bg-black text-white hover:bg-gray-800': !calibrationStatus[item.key],
                  'bg-green-100 text-green-700 cursor-not-allowed': calibrationStatus[item.key]
                }"
                :disabled="calibrationStatus[item.key]"
              >
                {{ calibrationStatus[item.key] ? $t('onboarding.calibration_done') : $t('onboarding.calibration_start') }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Navigation Buttons - Bottom Area -->
    <div class="py-6 px-4 flex items-center justify-between">
      <button
        v-if="currentStep > 0"
        @click="prevStep"
        class="min-w-[120px] h-12 px-6 py-3 rounded-full border border-gray-200 text-black font-medium transition-all duration-200 hover:border-black hover:bg-gray-50 text-base flex items-center justify-center"
      >
        {{ $t('common.back') }}
      </button>
      <div v-else class="min-w-[120px]"></div>

      <button
        @click="nextStep"
        class="group relative min-w-[160px] h-12 px-8 py-3 bg-black text-white rounded-full font-medium text-base overflow-hidden transition-all duration-300 hover:shadow-lg hover:scale-105 active:scale-95 flex items-center justify-center gap-2"
        :disabled="!canProceed"
        :class="{
          'opacity-50 cursor-not-allowed': !canProceed,
          'opacity-100 cursor-pointer': canProceed
        }"
      >
        {{ currentStep === steps.length - 1 ? $t('common.finish') : $t('common.next') }}
        <ArrowRight class="w-5 h-5 group-hover:translate-x-1 transition-transform" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ArrowRight, Wifi, Gauge, Thermometer, Activity } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'
import { useLocaleStore } from '@/stores/locale'
import { storeToRefs } from 'pinia'

const router = useRouter()
const { t } = useI18n()
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
  // Simulate calibration process
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
    // Complete onboarding
    setTimeout(() => {
      router.push('/')
    }, 500)
  }
}
</script>

<style scoped>
/* Optional: Add custom animations if Tailwind utilities aren't enough */
@keyframes float {
  0%,
  100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}

/* Custom scrollbar for terms and conditions */
.p-6::-webkit-scrollbar {
  width: 6px;
}

.p-6::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 8px;
}

.p-6::-webkit-scrollbar-thumb {
  background: #888;
  border-radius: 8px;
}

.p-6::-webkit-scrollbar-thumb:hover {
  background: #555;
}
</style>