<template>
  <div class="bg-white rounded-2xl p-4 shadow-sm border border-white flex flex-col gap-4">
    <div class="flex justify-between items-center">
      <div class="text-xs font-bold text-gray-700">WiFi Management</div>
      <div v-if="status.connected" class="flex items-center gap-2">
        <span class="text-xs text-green-600 font-bold">Connected: {{ status.ssid }}</span>
        <button
          @click="handleDisconnect"
          :disabled="connecting"
          class="text-xs bg-red-50 text-red-600 px-2 py-1 rounded hover:bg-red-100"
        >
          Disconnect
        </button>
      </div>
      <div v-else class="text-xs text-gray-500">Not Connected</div>
    </div>

    <!-- Scan Button & Status -->
    <div class="flex items-center gap-2">
      <button
        @click="handleScan"
        :disabled="scanning"
        class="bg-gray-800 text-white text-xs px-3 py-1.5 rounded-lg hover:bg-gray-700 disabled:opacity-50 transition-colors"
      >
        {{ scanning ? 'Scanning...' : 'Scan Networks' }}
      </button>
      <span v-if="error" class="text-xs text-red-500">{{ error }}</span>
    </div>

    <!-- Network List -->
    <div v-if="networks.length > 0" class="flex flex-col gap-2 max-h-60 overflow-y-auto">
      <div
        v-for="net in networks"
        :key="net.ssid"
        class="flex justify-between items-center p-2 rounded-lg hover:bg-gray-50 border border-transparent hover:border-gray-200 transition-colors cursor-pointer"
        @click="promptConnect(net)"
      >
        <div class="flex flex-col">
          <span class="text-sm font-semibold text-gray-800">{{ net.ssid }}</span>
          <span class="text-[10px] text-gray-400"
            >Signal: {{ net.signal }}% | {{ net.security }}</span
          >
        </div>
        <div v-if="net.in_use" class="text-xs text-green-500 font-bold">Connected</div>
      </div>
    </div>
    <div v-else-if="!scanning" class="text-xs text-gray-400 text-center py-4">
      No networks found. Click scan to search.
    </div>

    <!-- Password Modal -->
    <div
      v-if="showPasswordModal"
      class="fixed inset-0 bg-black/20 flex items-center justify-center z-50"
    >
      <div class="bg-white p-4 rounded-2xl shadow-xl w-64 flex flex-col gap-3">
        <div class="text-sm font-bold text-gray-800">Connect to {{ selectedSsid }}</div>
        <input
          v-model="password"
          type="password"
          placeholder="Password"
          class="bg-gray-50 border border-gray-200 rounded px-2 py-1 text-sm outline-none focus:border-black"
          @keyup.enter="confirmConnect"
        />
        <div class="flex justify-end gap-2">
          <button @click="closeModal" class="text-xs text-gray-500 px-2 py-1">Cancel</button>
          <button
            @click="confirmConnect"
            :disabled="connecting"
            class="text-xs bg-black text-white px-3 py-1 rounded hover:bg-gray-800 disabled:opacity-50"
          >
            {{ connecting ? 'Connecting...' : 'Connect' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useWifiStore } from '@/stores/wifi'
import { storeToRefs } from 'pinia'
import type { WifiNetwork } from '@/api/wifi'

const store = useWifiStore()
const { networks, status, scanning, connecting, error } = storeToRefs(store)

const showPasswordModal = ref(false)
const selectedSsid = ref('')
const password = ref('')

onMounted(() => {
  store.updateStatus()
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
  password.value = '' // Always require password re-entry as requested
  showPasswordModal.value = true
}

function closeModal() {
  showPasswordModal.value = false
  password.value = ''
}

async function confirmConnect() {
  try {
    await store.connect(selectedSsid.value, password.value)
    closeModal()
  } catch (e) {
    // Error is already in store.error
  }
}
</script>
