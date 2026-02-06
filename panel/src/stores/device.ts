import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useDeviceStore = defineStore('device', () => {
  // Connection State
  const isConnected = ref(false)
  const lastHeartbeat = ref(0)

  // Sensor Data
  const airspeed = ref(0.0) // m/s
  
  const imu = ref({
    accel: { x: 0, y: 0, z: 0 },
    gyro: { x: 0, y: 0, z: 0 },
    attitude: { roll: 0, pitch: 0, yaw: 0 },
    vibration: { rms: 0, peak: 0 }
  })

  const battery = ref({
    voltage: 0.0,
    soc: 0.0
  })

  const lidar = ref({
    distance: 0,
    signalStrength: 0
  })

  const env = ref({
    pressure: 0.0,
    temperature: 0.0,
    humidity: 0.0
  })

  const acoustic = ref({
    spl: 0.0,
    peakFreq: 0.0,
    spectrum: new Array(16).fill(0)
  })

  // Control State (Outbound)
  const controls = ref({
    throttle: 0, // 0-255
    servo: 90,   // 0-180 degrees mapped to u8
    imuHorizontal: false
  })

  // System Status
  const status = ref('Idle') // Idle, Initializing, Running, Error

  // Actions
  function setConnected(state: boolean) {
    isConnected.value = state
    if (state) lastHeartbeat.value = Date.now()
  }

  function updateTelemetry(data: any) {
    // Mock implementation for data updates
    // In real app, this would parse the incoming JSON/Protobuf/Rkyv data
    if (data.airspeed !== undefined) airspeed.value = data.airspeed
    if (data.battery) battery.value = data.battery
    if (data.imu) imu.value = { ...imu.value, ...data.imu }
    // ... handle others
  }

  function setThrottle(val: number) {
    controls.value.throttle = Math.max(0, Math.min(255, val))
    // Trigger API call to backend here
  }

  function setServo(val: number) {
    controls.value.servo = Math.max(0, Math.min(180, val))
    // Trigger API call to backend here
  }

  return {
    isConnected,
    lastHeartbeat,
    airspeed,
    imu,
    battery,
    lidar,
    env,
    acoustic,
    controls,
    status,
    setConnected,
    updateTelemetry,
    setThrottle,
    setServo
  }
})
