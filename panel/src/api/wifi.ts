const API_PORT = 3000
const API_BASE = `http://${window.location.hostname}:${API_PORT}/api/wifi`

export interface WifiNetwork {
  ssid: string
  signal: number
  security: string
  in_use: boolean
}

export interface WifiStatus {
  connected: boolean
  ssid: string | null
  ip: string | null
}

export async function scanWifi(): Promise<WifiNetwork[]> {
  const res = await fetch(`${API_BASE}/scan`)
  if (!res.ok) {
    let errorMessage = 'Failed to scan wifi'
    const text = await res.text()
    try {
      const data = JSON.parse(text)
      if (data && data.error) errorMessage = data.error
    } catch (e) {
      if (text) errorMessage = text
    }
    throw new Error(errorMessage)
  }
  return res.json()
}

export async function connectWifi(ssid: string, password?: string): Promise<void> {
  const res = await fetch(`${API_BASE}/connect`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ ssid, password }),
  })
  if (!res.ok) {
    let errorMessage = 'Failed to connect'
    const text = await res.text()
    try {
      const data = JSON.parse(text)
      if (data && data.error) errorMessage = data.error
    } catch (e) {
      if (text) errorMessage = text
    }
    throw new Error(errorMessage)
  }
}

export async function disconnectWifi(): Promise<void> {
  const res = await fetch(`${API_BASE}/disconnect`, { method: 'POST' })
  if (!res.ok) {
    let errorMessage = 'Failed to disconnect'
    const text = await res.text()
    try {
      const data = JSON.parse(text)
      if (data && data.error) errorMessage = data.error
    } catch (e) {
      if (text) errorMessage = text
    }
    throw new Error(errorMessage)
  }
}

export async function getWifiStatus(): Promise<WifiStatus> {
  const res = await fetch(`${API_BASE}/status`)
  if (!res.ok) {
    let errorMessage = 'Failed to get status'
    const text = await res.text()
    try {
      const data = JSON.parse(text)
      if (data && data.error) errorMessage = data.error
    } catch (e) {
      if (text) errorMessage = text
    }
    throw new Error(errorMessage)
  }
  return res.json()
}

export async function testWifiConnection(): Promise<void> {
  const res = await fetch(`${API_BASE}/test`)
  if (!res.ok) {
    let errorMessage = 'Connectivity test failed'
    const text = await res.text()
    try {
      const data = JSON.parse(text)
      if (data && data.error) errorMessage = data.error
    } catch (e) {
      if (text) errorMessage = text
    }
    throw new Error(errorMessage)
  }
}
