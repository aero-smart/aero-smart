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

/**
 * Helper function to handle API errors by parsing JSON or text response.
 * This ensures we don't try to read the stream twice.
 */
async function handleResponseError(res: Response, defaultMsg: string): Promise<never> {
  // First, read the text body once.
  const text = await res.text()

  // Log it for debugging
  console.error(`API Error [${res.url}]:`, text)

  let errorMessage = defaultMsg
  try {
    // Try to parse as JSON
    const data = JSON.parse(text)
    if (data && data.error) {
      errorMessage = data.error
    } else if (text) {
      // If valid JSON but no 'error' field, use the raw text if available
      errorMessage = text
    }
  } catch (e) {
    // Not JSON, fall back to raw text if available
    if (text) {
      errorMessage = text
    }
  }
  throw new Error(errorMessage)
}

export async function scanWifi(): Promise<WifiNetwork[]> {
  const res = await fetch(`${API_BASE}/scan`)
  if (!res.ok) {
    const text = await res.text()
    throw new Error(text || 'Failed to scan wifi')
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
    await handleResponseError(res, 'Failed to connect')
  }
}

export async function disconnectWifi(): Promise<void> {
  const res = await fetch(`${API_BASE}/disconnect`, { method: 'POST' })
  if (!res.ok) {
    const text = await res.text()
    throw new Error(text || 'Failed to disconnect')
  }
}

export async function getWifiStatus(): Promise<WifiStatus> {
  const res = await fetch(`${API_BASE}/status`)
  if (!res.ok) {
    const text = await res.text()
    throw new Error(text || 'Failed to get status')
  }
  return res.json()
}

export async function testWifiConnection(): Promise<void> {
  const res = await fetch(`${API_BASE}/test`)
  if (!res.ok) {
    await handleResponseError(res, 'Connectivity test failed')
  }
}
