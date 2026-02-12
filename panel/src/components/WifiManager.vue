<template>
  <div class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4 h-[500px]">
    <div class="flex justify-between items-center shrink-0">
      <div class="text-sm font-bold text-gray-900">{{ $t('wifi.title') }}</div>
      <button @click="$emit('close')" class="p-1 rounded-full hover:bg-gray-100 text-gray-500">
        <X class="w-4 h-4" />
      </button>
    </div>

    <div class="flex justify-between items-center shrink-0">
      <div v-if="activeConnection.connected" class="flex flex-col gap-0.5">
        <div class="flex items-center gap-2">
          <span class="text-xs text-green-600 font-bold flex items-center gap-1">
            <CheckCircle2 class="w-3 h-3" />
            {{ activeConnection.ssid || $t('wifi.unknown_ssid') }}
          </span>
          <button
            @click="handleDisconnect"
            :disabled="connecting"
            class="text-[10px] bg-red-50 text-red-600 px-1.5 py-0.5 rounded hover:bg-red-100 disabled:opacity-50"
          >
            {{ $t('wifi.disconnect') }}
          </button>
        </div>
        <!-- Requirement 2: Show IPv4 Address -->
        <div class="text-[10px] text-gray-400 font-mono ml-4">
          {{ $t('wifi.ip_prefix') }}{{ activeConnection.ip || $t('wifi.obtaining_ip') }}
        </div>
      </div>
      <div v-else class="text-xs text-gray-500">{{ $t('wifi.not_connected') }}</div>

      <button
        @click="handleScan"
        :disabled="scanning"
        class="bg-gray-900 text-white text-xs px-3 py-1.5 rounded-lg hover:bg-gray-800 disabled:opacity-50 transition-colors flex items-center gap-1.5"
      >
        <RefreshCw :class="['w-3 h-3', scanning ? 'animate-spin' : '']" />
        {{ scanning ? $t('wifi.scanning') : $t('wifi.scan') }}
      </button>
    </div>

    <div v-if="error" class="text-xs text-red-500 bg-red-50 p-2 rounded-lg shrink-0">
      {{ error }}
    </div>

    <!-- Network List -->
    <div class="flex-1 overflow-y-auto min-h-0 -mx-2 px-2">
      <div v-if="networks.length > 0" class="flex flex-col gap-1">
        <button
          v-for="net in networks"
          :key="net.ssid"
          class="w-full flex justify-between items-center p-3 rounded-xl hover:bg-gray-50 border border-transparent hover:border-gray-200 transition-all text-left group"
          :class="{ 'bg-gray-50 border-gray-200': net.in_use }"
          @click="promptConnect(net)"
        >
          <div class="flex items-center gap-3 min-w-0">
            <Wifi class="w-4 h-4 text-gray-500 group-hover:text-gray-900" />
            <div class="flex flex-col min-w-0">
              <span class="text-sm font-medium text-gray-900 truncate">{{ net.ssid }}</span>
              <div class="flex items-center gap-2 text-[10px] text-gray-500">
                <span>{{ net.security }}</span>
                <span
                  v-if="net.in_use"
                  class="text-green-600 font-bold bg-green-50 px-1.5 py-0.5 rounded-full"
                  >{{ $t('wifi.connected') }}</span
                >
              </div>
            </div>
          </div>
          <div class="text-xs font-medium text-gray-400 tabular-nums">{{ net.signal }}%</div>
        </button>
      </div>
      <div
        v-else-if="!scanning"
        class="flex flex-col items-center justify-center h-full text-gray-400 gap-2"
      >
        <WifiOff class="w-8 h-8 opacity-20" />
        <span class="text-xs">{{ $t('wifi.no_networks') }}</span>
      </div>
    </div>

    <!-- Password Modal -->
    <div
      v-if="showPasswordModal"
      class="absolute inset-0 bg-white/95 backdrop-blur-sm flex items-center justify-center z-50 rounded-2xl"
    >
      <div class="w-64 flex flex-col gap-4">
        <div class="text-center">
          <div class="text-sm font-bold text-gray-900">{{ $t('wifi.connect_title') }}</div>
          <div class="text-xs text-gray-500 mt-1">{{ selectedSsid }}</div>
        </div>

        <input
          v-model="password"
          type="password"
          :placeholder="$t('wifi.password_placeholder')"
          class="bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 text-sm outline-none focus:border-black focus:ring-1 focus:ring-black w-full"
          @focus="keyboardStore.open(password, (val) => (password = val), confirmConnect)"
        />

        <div class="flex gap-2 mt-2">
          <button
            @click="closeModal"
            class="flex-1 text-xs font-medium text-gray-600 bg-gray-100 hover:bg-gray-200 py-2.5 rounded-xl transition-colors"
          >
            {{ $t('common.cancel') }}
          </button>
          <button
            @click="confirmConnect"
            :disabled="connecting"
            class="flex-1 text-xs font-bold bg-black text-white py-2.5 rounded-xl hover:bg-gray-800 disabled:opacity-50 transition-colors flex items-center justify-center gap-2"
          >
            <Loader2 v-if="connecting" class="w-3 h-3 animate-spin" />
            {{ connecting ? $t('wifi.connecting') : $t('wifi.connect') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useWifiStore } from '@/stores/wifi'
import { useKeyboardStore } from '@/stores/keyboard'
import { storeToRefs } from 'pinia'
import type { WifiNetwork } from '@/api/wifi'
import { X, RefreshCw, Wifi, WifiOff, CheckCircle2, Loader2 } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'

defineEmits(['close'])

const { t } = useI18n()
const store = useWifiStore()
const keyboardStore = useKeyboardStore()
const { networks, status, scanning, connecting, error } = storeToRefs(store)

const showPasswordModal = ref(false)
const selectedSsid = ref('')
const password = ref('')

// Compute active connection state by merging store status and scan results
const activeConnection = computed(() => {
  // Priority 1: Status from backend says connected or has IP
  if (status.value.connected || status.value.ip) {
    return {
      connected: true,
      ssid: status.value.ssid,
      ip: status.value.ip,
    }
  }

  // Priority 2: Scan list shows an in-use network
  const connectedNet = networks.value.find((n) => n.in_use)
  if (connectedNet) {
    return {
      connected: true,
      ssid: connectedNet.ssid,
      ip: null, // Scan result doesn't provide IP
    }
  }

  return {
    connected: false,
    ssid: null,
    ip: null,
  }
})

onMounted(() => {
  store.updateStatus()
  store.scan() // Auto scan on open
})

function handleScan() {
  store.scan()
}

function handleDisconnect() {
  store.disconnect()
}

function promptConnect(net: WifiNetwork) {
  if (net.in_use) return
  selectedSsid.value = net.ssid
  password.value = ''
  showPasswordModal.value = true
}

function closeModal() {
  showPasswordModal.value = false
  password.value = ''
  keyboardStore.close()
}

async function confirmConnect() {
  keyboardStore.close()
  try {
    await store.connect(selectedSsid.value, password.value)
    showPasswordModal.value = false
    password.value = ''
  } catch (e) {
    // Error is already in store.error
  }
}
</script>
