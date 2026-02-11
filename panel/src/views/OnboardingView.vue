<template>
  <div class="fixed inset-0 bg-white flex flex-col items-center justify-center z-[9999]">
    <Transition
      appear
      enter-active-class="transition-all duration-1000 ease-out"
      enter-from-class="opacity-0 scale-90"
      enter-to-class="opacity-100 scale-100"
      leave-active-class="transition-all duration-500 ease-in"
      leave-from-class="opacity-100 scale-100"
      leave-to-class="opacity-0 scale-110"
    >
      <div v-if="show" class="flex flex-col items-center gap-12">
        <!-- Logo Container -->
        <div class="relative w-32 h-32 flex items-center justify-center">
          <div class="absolute inset-0 bg-gray-100 rounded-full animate-pulse"></div>
          <svg
            width="80"
            height="120"
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

        <!-- Title & Subtitle -->
        <div class="text-center space-y-4">
          <h1 class="text-4xl font-bold text-black tracking-tight">{{ $t('onboarding.title') }}</h1>
          <p class="text-gray-500 text-lg">{{ $t('onboarding.subtitle') }}</p>
        </div>

        <!-- Language Selection -->
        <div class="flex gap-4">
          <button
            @click="setLocale('en')"
            class="px-6 py-2 rounded-full border text-sm font-medium transition-all duration-200"
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
            class="px-6 py-2 rounded-full border text-sm font-medium transition-all duration-200"
            :class="
              currentLocale === 'zh'
                ? 'bg-black text-white border-black'
                : 'bg-white text-black border-gray-200 hover:border-black'
            "
          >
            中文
          </button>
        </div>

        <!-- Action Button -->
        <button
          @click="handleStart"
          class="group relative px-8 py-3 bg-black text-white rounded-full font-medium text-lg overflow-hidden transition-all duration-300 hover:shadow-lg hover:scale-105 active:scale-95"
        >
          <span class="relative flex items-center gap-2">
            {{ $t('common.start') }}
            <ArrowRight class="w-5 h-5 group-hover:translate-x-1 transition-transform" />
          </span>
        </button>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ArrowRight } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'
import { useLocaleStore } from '@/stores/locale'
import { storeToRefs } from 'pinia'

const router = useRouter()
const { t } = useI18n()
const localeStore = useLocaleStore()
const { currentLocale } = storeToRefs(localeStore)
const { setLocale } = localeStore

const show = ref(false)

onMounted(() => {
  // Trigger entry animation
  setTimeout(() => {
    show.value = true
  }, 100)
})

const handleStart = () => {
  show.value = false
  // Wait for exit animation
  setTimeout(() => {
    router.push('/')
  }, 500)
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
</style>
