import { defineStore } from 'pinia'
import { ref } from 'vue'
import { type WifiNetwork, type WifiStatus, scanWifi, connectWifi, disconnectWifi, getWifiStatus } from '@/api/wifi'

export const useWifiStore = defineStore('wifi', () => {
  const networks = ref<WifiNetwork[]>([])
  const status = ref<WifiStatus>({ connected: false, ssid: null, ip: null })
  const scanning = ref(false)
  const connecting = ref(false)
  const error = ref<string | null>(null)

  async function scan() {
    scanning.value = true
    error.value = null
    try {
      networks.value = await scanWifi()
    } catch (e: any) {
      error.value = e.message
    } finally {
      scanning.value = false
    }
  }

  async function connect(ssid: string, password?: string) {
    connecting.value = true
    error.value = null
    try {
      await connectWifi(ssid, password)
      await updateStatus()
    } catch (e: any) {
      error.value = e.message
      throw e 
    } finally {
      connecting.value = false
    }
  }

  async function disconnect() {
    connecting.value = true
    error.value = null
    try {
      await disconnectWifi()
      await updateStatus()
    } catch (e: any) {
      error.value = e.message
    } finally {
      connecting.value = false
    }
  }

  async function updateStatus() {
    try {
      status.value = await getWifiStatus()
    } catch (e: any) {
      console.error('Failed to update wifi status', e)
    }
  }

  return {
    networks,
    status,
    scanning,
    connecting,
    error,
    scan,
    connect,
    disconnect,
    updateStatus
  }
})
