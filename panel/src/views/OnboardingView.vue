<template>
  <div class="fixed inset-0 bg-[#f2f2f7] flex items-center justify-center overflow-hidden font-sans">
    <!-- Abstract Background Orbs -->
    <div class="absolute top-[-20%] left-[-10%] w-[600px] h-[600px] bg-blue-400/20 rounded-full blur-[100px] pointer-events-none"></div>
    <div class="absolute bottom-[-20%] right-[-10%] w-[600px] h-[600px] bg-purple-400/20 rounded-full blur-[100px] pointer-events-none"></div>

    <!-- Main Card -->
    <div class="relative w-full max-w-[800px] h-[600px] bg-white/80 backdrop-blur-2xl rounded-[2rem] shadow-2xl border border-white/50 flex overflow-hidden">
      
      <!-- Left Sidebar (Progress & Info) -->
      <div class="w-[280px] bg-gray-50/50 border-r border-gray-200/50 p-8 flex flex-col justify-between backdrop-blur-sm">
        <div>
          <!-- Logo -->
          <div class="w-10 h-10 bg-black rounded-xl flex items-center justify-center mb-8 shadow-lg">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="white" stroke-width="2" stroke-linejoin="round"/>
              <path d="M2 17L12 22L22 17" stroke="white" stroke-width="2" stroke-linejoin="round"/>
              <path d="M2 12L12 17L22 12" stroke="white" stroke-width="2" stroke-linejoin="round"/>
            </svg>
          </div>
          
          <!-- Steps -->
          <div class="space-y-6">
            <div 
              v-for="(step, index) in steps" 
              :key="index"
              class="flex items-center gap-3 transition-all duration-300"
              :class="currentStep === index ? 'opacity-100 translate-x-1' : 'opacity-40'"
            >
              <div 
                class="w-6 h-6 rounded-full flex items-center justify-center text-[10px] font-bold border transition-all duration-300"
                :class="currentStep === index 
                  ? 'bg-black text-white border-black scale-110 shadow-md' 
                  : (index < currentStep ? 'bg-green-500 text-white border-green-500' : 'bg-transparent text-gray-500 border-gray-300')"
              >
                <template v-if="index < currentStep">✓</template>
                <template v-else>{{ index + 1 }}</template>
              </div>
              <span class="text-sm font-medium tracking-tight text-gray-900">{{ step }}</span>
            </div>
          </div>
        </div>

        <!-- Help Link -->
        <div class="text-xs text-gray-400 font-medium">
          {{ $t('common.need_help') || 'Need help?' }}
        </div>
      </div>

      <!-- Right Content Area -->
      <div class="flex-1 flex flex-col relative bg-white/40">
        <!-- Content Transition -->
        <div class="flex-1 overflow-y-auto overflow-x-hidden p-10 scrollbar-hide">
          <Transition
            enter-active-class="transition-all duration-500 ease-[cubic-bezier(0.25,1,0.5,1)]"
            enter-from-class="opacity-0 translate-x-8 scale-95"
            enter-to-class="opacity-100 translate-x-0 scale-100"
            leave-active-class="transition-all duration-300 ease-in"
            leave-from-class="opacity-100 translate-x-0"
            leave-to-class="opacity-0 -translate-x-4"
            mode="out-in"
          >
            <!-- Step 1: Language & Region -->
            <div v-if="currentStep === 0" key="step0" class="h-full flex flex-col justify-center max-w-md mx-auto">
              <div class="mb-8">
                <h1 class="text-3xl font-bold text-gray-900 mb-2 tracking-tight">{{ $t('onboarding.title') }}</h1>
                <p class="text-gray-500">{{ $t('onboarding.subtitle') }}</p>
              </div>

              <div class="space-y-8">
                <!-- Language -->
                <div class="space-y-3">
                  <label class="text-xs font-semibold text-gray-500 uppercase tracking-wider pl-1">{{ $t('onboarding.language') }}</label>
                  <div class="grid grid-cols-2 gap-4">
                    <button
                      v-for="lang in [{code: 'en', label: 'English'}, {code: 'zh', label: '中文'}]"
                      :key="lang.code"
                      @click="setLocale(lang.code)"
                      class="h-14 rounded-2xl border flex items-center justify-center font-medium transition-all duration-200"
                      :class="currentLocale === lang.code 
                        ? 'bg-black text-white border-black shadow-lg scale-[1.02]' 
                        : 'bg-white text-gray-900 border-gray-200 hover:border-gray-300 hover:bg-gray-50'"
                    >
                      {{ lang.label }}
                    </button>
                  </div>
                </div>

                <!-- Region -->
                <div class="space-y-3">
                  <label class="text-xs font-semibold text-gray-500 uppercase tracking-wider pl-1">{{ $t('onboarding.region') }}</label>
                  <div class="grid grid-cols-3 gap-3">
                    <button
                      v-for="region in ['us', 'cn', 'eu']"
                      :key="region"
                      @click="setRegion(region)"
                      class="h-12 rounded-xl border text-sm font-medium transition-all duration-200 uppercase"
                      :class="selectedRegion === region 
                        ? 'bg-black text-white border-black shadow-md' 
                        : 'bg-white text-gray-700 border-gray-200 hover:border-gray-300 hover:bg-gray-50'"
                    >
                      {{ region }}
                    </button>
                  </div>
                </div>
              </div>
            </div>

            <!-- Step 2: WiFi -->
            <div v-else-if="currentStep === 1" key="step1" class="h-full flex flex-col max-w-md mx-auto pt-4">
              <div class="mb-6">
                <h1 class="text-2xl font-bold text-gray-900 mb-2 tracking-tight">{{ $t('onboarding.wifi_title') }}</h1>
                <p class="text-gray-500 text-sm">{{ $t('onboarding.wifi_subtitle') }}</p>
              </div>

              <div class="bg-white/60 rounded-2xl border border-gray-200/60 overflow-hidden shadow-sm backdrop-blur-sm">
                <div 
                  v-for="(wifi, idx) in availableWifi" 
                  :key="wifi.ssid"
                  @click="selectWifi(wifi)"
                  class="p-4 flex items-center justify-between cursor-pointer transition-colors hover:bg-black/5"
                  :class="[
                    idx !== availableWifi.length - 1 ? 'border-b border-gray-100' : '',
                    selectedWifi?.ssid === wifi.ssid ? 'bg-blue-50/50' : ''
                  ]"
                >
                  <div class="flex items-center gap-3">
                    <Wifi class="w-5 h-5 text-gray-600" />
                    <div>
                      <div class="font-medium text-gray-900 text-sm">{{ wifi.ssid }}</div>
                    </div>
                  </div>
                  <div class="flex items-center gap-3">
                    <div class="flex gap-0.5 items-end h-3">
                       <div class="w-1 bg-gray-300 rounded-sm" :class="wifi.signal > 20 ? 'h-1.5 bg-gray-800' : 'h-1.5'"></div>
                       <div class="w-1 bg-gray-300 rounded-sm" :class="wifi.signal > 40 ? 'h-2 bg-gray-800' : 'h-2'"></div>
                       <div class="w-1 bg-gray-300 rounded-sm" :class="wifi.signal > 70 ? 'h-3 bg-gray-800' : 'h-3'"></div>
                    </div>
                    <div v-if="selectedWifi?.ssid === wifi.ssid" class="text-blue-600">
                      <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                      </svg>
                    </div>
                  </div>
                </div>
              </div>

              <Transition
                enter-active-class="transition-all duration-300 ease-out"
                enter-from-class="opacity-0 -translate-y-2"
                enter-to-class="opacity-100 translate-y-0"
              >
                <div v-if="selectedWifi" class="mt-6 space-y-2">
                  <label class="text-xs font-semibold text-gray-500 uppercase tracking-wider pl-1">{{ $t('onboarding.wifi_password') }}</label>
                  <input 
                    v-model="wifiPassword"
                    type="password"
                    class="w-full h-12 px-4 rounded-xl border border-gray-200 bg-white focus:outline-none focus:ring-2 focus:ring-black/5 focus:border-black transition-all text-sm shadow-sm"
                    placeholder="Enter password..."
                    autofocus
                  />
                </div>
              </Transition>
            </div>

            <!-- Step 3: Login -->
            <div v-else-if="currentStep === 2" key="step2" class="h-full flex flex-col justify-center max-w-md mx-auto">
              <div class="mb-8 text-center">
                <div class="w-16 h-16 bg-gray-100 rounded-full mx-auto mb-4 flex items-center justify-center">
                  <svg class="w-8 h-8 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                  </svg>
                </div>
                <h1 class="text-2xl font-bold text-gray-900 mb-2 tracking-tight">{{ $t('onboarding.login_title') }}</h1>
                <p class="text-gray-500 text-sm">{{ $t('onboarding.login_subtitle') }}</p>
              </div>

              <div class="space-y-4">
                <div class="space-y-1.5">
                  <label class="text-xs font-semibold text-gray-500 uppercase tracking-wider pl-1">{{ $t('onboarding.email') }}</label>
                  <input 
                    v-model="loginForm.email"
                    type="email"
                    class="w-full h-12 px-4 rounded-xl border border-gray-200 bg-white focus:outline-none focus:ring-2 focus:ring-black/5 focus:border-black transition-all text-sm shadow-sm"
                    placeholder="name@example.com"
                  />
                </div>

                <div class="space-y-1.5">
                  <label class="text-xs font-semibold text-gray-500 uppercase tracking-wider pl-1">{{ $t('onboarding.password') }}</label>
                  <input 
                    v-model="loginForm.password"
                    type="password"
                    class="w-full h-12 px-4 rounded-xl border border-gray-200 bg-white focus:outline-none focus:ring-2 focus:ring-black/5 focus:border-black transition-all text-sm shadow-sm"
                    placeholder="••••••••"
                  />
                </div>

                <div class="flex items-center justify-between pt-2">
                  <label class="flex items-center gap-2 cursor-pointer">
                    <input 
                      v-model="loginForm.remember"
                      type="checkbox"
                      class="w-4 h-4 rounded border-gray-300 text-black focus:ring-black transition-colors"
                    />
                    <span class="text-sm text-gray-600">{{ $t('onboarding.remember_me') }}</span>
                  </label>
                  <a href="#" class="text-sm text-black font-medium hover:underline">{{ $t('onboarding.forgot_password') }}</a>
                </div>
              </div>
            </div>

            <!-- Step 4: Terms -->
            <div v-else-if="currentStep === 3" key="step3" class="h-full flex flex-col max-w-md mx-auto pt-4">
              <div class="mb-6">
                <h1 class="text-2xl font-bold text-gray-900 mb-2 tracking-tight">{{ $t('onboarding.terms_title') }}</h1>
                <p class="text-gray-500 text-sm">{{ $t('onboarding.terms_subtitle') }}</p>
              </div>

              <div class="flex-1 bg-white rounded-2xl border border-gray-200 p-6 overflow-y-auto shadow-inner mb-6 text-sm leading-relaxed text-gray-600">
                <p class="mb-4">1. Acceptance of Terms</p>
                <p class="mb-4">By accessing and using this service, you accept and agree to be bound by the terms and provision of this agreement.</p>
                <p class="mb-4">2. Use License</p>
                <p class="mb-4">Permission is granted to temporarily download one copy of the materials (information or software) on AeroSmart's website for personal, non-commercial transitory viewing only.</p>
                <p class="mb-4">3. Disclaimer</p>
                <p class="mb-4">The materials on AeroSmart's website are provided on an 'as is' basis. AeroSmart makes no warranties, expressed or implied, and hereby disclaims and negates all other warranties including, without limitation, implied warranties or conditions of merchantability, fitness for a particular purpose, or non-infringement of intellectual property or other violation of rights.</p>
                <p>{{ $t('onboarding.terms_empty') }}</p>
              </div>

              <label class="flex items-center gap-3 p-4 bg-gray-50 rounded-xl border border-gray-200 cursor-pointer transition-colors hover:bg-gray-100 hover:border-gray-300">
                <input 
                  v-model="acceptTerms"
                  type="checkbox"
                  class="w-5 h-5 rounded border-gray-300 text-black focus:ring-black transition-all"
                />
                <span class="text-sm font-medium text-gray-900">{{ $t('onboarding.terms_accept') }}</span>
              </label>
            </div>

            <!-- Step 5: Calibration -->
            <div v-else-if="currentStep === 4" key="step4" class="h-full flex flex-col max-w-md mx-auto pt-4">
              <div class="mb-6">
                <h1 class="text-2xl font-bold text-gray-900 mb-2 tracking-tight">{{ $t('onboarding.calibration_title') }}</h1>
                <p class="text-gray-500 text-sm">{{ $t('onboarding.calibration_subtitle') }}</p>
              </div>

              <div class="space-y-4">
                <div 
                  v-for="(item, index) in calibrationItems" 
                  :key="index"
                  class="group relative bg-white rounded-2xl border p-4 transition-all duration-300 hover:shadow-md"
                  :class="calibrationStatus[item.key] ? 'border-green-200 bg-green-50/30' : 'border-gray-200'"
                >
                  <div class="flex items-center justify-between relative z-10">
                    <div class="flex items-center gap-4">
                      <div 
                        class="w-10 h-10 rounded-full flex items-center justify-center transition-colors duration-300"
                        :class="calibrationStatus[item.key] ? 'bg-green-100 text-green-600' : 'bg-gray-100 text-gray-500'"
                      >
                        <component :is="item.icon" class="w-5 h-5" />
                      </div>
                      <div>
                        <div class="font-bold text-gray-900 text-sm">{{ item.title }}</div>
                        <div class="text-xs text-gray-500">{{ item.description }}</div>
                      </div>
                    </div>
                    
                    <button
                      @click="calibrate(item.key)"
                      class="px-4 py-2 rounded-full text-xs font-bold transition-all duration-300"
                      :class="calibrationStatus[item.key] 
                        ? 'bg-transparent text-green-600 cursor-default' 
                        : 'bg-black text-white hover:bg-gray-800 hover:scale-105 active:scale-95 shadow-sm'"
                      :disabled="calibrationStatus[item.key]"
                    >
                      <div class="flex items-center gap-1">
                        <span v-if="calibrationStatus[item.key]" class="text-lg">✓</span>
                        <span>{{ calibrationStatus[item.key] ? $t('onboarding.calibration_done') : $t('onboarding.calibration_start') }}</span>
                      </div>
                    </button>
                  </div>
                  
                  <!-- Success background effect -->
                  <div 
                    class="absolute inset-0 bg-green-50/50 rounded-2xl transition-opacity duration-500 pointer-events-none"
                    :class="calibrationStatus[item.key] ? 'opacity-100' : 'opacity-0'"
                  ></div>
                </div>
              </div>
            </div>
          </Transition>
        </div>

        <!-- Bottom Actions -->
        <div class="h-[88px] px-10 border-t border-gray-100 flex items-center justify-between bg-white/50 backdrop-blur-md">
          <button
            v-if="currentStep > 0"
            @click="prevStep"
            class="px-6 py-2.5 rounded-full text-sm font-medium text-gray-500 hover:text-gray-900 hover:bg-gray-100 transition-all"
          >
            {{ $t('common.back') }}
          </button>
          <div v-else></div>

          <button
            @click="nextStep"
            class="relative overflow-hidden group px-8 py-3 rounded-full bg-black text-white font-medium text-sm transition-all duration-300 hover:shadow-lg hover:shadow-black/20 hover:-translate-y-0.5 disabled:opacity-50 disabled:hover:translate-y-0 disabled:hover:shadow-none"
            :disabled="!canProceed"
          >
            <span class="relative z-10 flex items-center gap-2">
              {{ currentStep === steps.length - 1 ? $t('common.finish') : $t('common.next') }}
              <ArrowRight class="w-4 h-4 transition-transform group-hover:translate-x-1" />
            </span>
            <div class="absolute inset-0 bg-gray-800 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
          </button>
        </div>
      </div>
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