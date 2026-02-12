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
  if (!res.ok) throw new Error('Failed to scan wifi')
  return res.json()
}

export async function connectWifi(ssid: string, password?: string): Promise<void> {
  const res = await fetch(`${API_BASE}/connect`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ ssid, password }),
  })
  if (!res.ok) {
    const text = await res.text()
    throw new Error(text || 'Failed to connect')
  }
}

export async function disconnectWifi(): Promise<void> {
  const res = await fetch(`${API_BASE}/disconnect`, { method: 'POST' })
  if (!res.ok) throw new Error('Failed to disconnect')
}

export async fn getWifiStatus(): Promise<WifiStatus> {
  const res = await fetch(`${API_BASE}/status`);
  if (!res.ok) throw new Error('Failed to get status');
  return res.json();
}

export async fn testWifiConnection(): Promise<void> {
  const res = await fetch(`${API_BASE}/test`);
  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || 'Connectivity test failed');
  }
}
