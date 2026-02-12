import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export type SerialStatus =
  | 'Idle'
  | 'Connecting'
  | 'Handshaking'
  | 'WaitingForFirstMessage'
  | 'Active'
  | { Failed: string }

export const useSerialStore = defineStore('serial', () => {
  const status = ref<SerialStatus>('Idle')
  const error = ref<string | null>(null)
  let pollInterval: number | null = null

  const statusString = computed(() => {
    if (typeof status.value === 'object' && 'Failed' in status.value) {
      return 'Failed'
    }
    return status.value as string
  })

  const isConnected = computed(() => statusString.value === 'Active')

  async function fetchStatus() {
    try {
      const res = await fetch('http://localhost:3000/api/activation/status')
      if (!res.ok) throw new Error('Failed to fetch status')
      const data = await res.json()
      status.value = data
      
      if (typeof data === 'object' && 'Failed' in data) {
        error.value = data.Failed
      } else {
        error.value = null
      }
    } catch (e) {
      // If fetch fails (e.g. service down), treat as connecting/failed
      // We'll just keep the last status or set to specific error state if needed
      // For now, logging it
      console.error('Serial status poll failed', e)
    }
  }

  async function restart() {
    try {
      await fetch('http://localhost:3000/api/activation/restart', { method: 'POST' })
    } catch (e) {
      console.error('Failed to restart serial', e)
    }
  }

  function startPolling() {
    fetchStatus()
    if (!pollInterval) {
      pollInterval = window.setInterval(fetchStatus, 1000)
    }
  }

  function stopPolling() {
    if (pollInterval) {
      clearInterval(pollInterval)
      pollInterval = null
    }
  }

  return {
    status,
    statusString,
    error,
    isConnected,
    fetchStatus,
    startPolling,
    stopPolling,
    restart
  }
})
