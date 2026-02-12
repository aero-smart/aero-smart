import { defineStore } from 'pinia'
import { ref } from 'vue'
import {
  type WifiNetwork,
  type WifiStatus,
  scanWifi,
  connectWifi,
  disconnectWifi,
  getWifiStatus,
  testWifiConnection,
} from '@/api/wifi'

export const useWifiStore = defineStore('wifi', () => {
  const networks = ref<WifiNetwork[]>([])
  const status = ref<WifiStatus>({ connected: false, ssid: null, ip: null })
  const scanning = ref(false)
  const connecting = ref(false)
  const testing = ref(false)
  const error = ref<string | null>(null)
  const testResult = ref<boolean | null>(null)

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
    testResult.value = null
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

  async function testConnection() {
    testing.value = true
    testResult.value = null
    try {
      await testWifiConnection()
      testResult.value = true
    } catch (e: any) {
      testResult.value = false
      error.value = e.message
    } finally {
      testing.value = false
    }
  }

  async function disconnect() {
    connecting.value = true
    error.value = null
    testResult.value = null
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
    testing,
    testResult,
    error,
    scan,
    connect,
    testConnection,
    disconnect,
    updateStatus,
  }
})
