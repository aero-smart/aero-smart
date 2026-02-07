import { defineStore } from 'pinia'
import { ref } from 'vue'

// Helper to convert Quaternion to Euler angles (Roll, Pitch, Yaw)
function quaternionToEuler(w: number, x: number, y: number, z: number) {
  const t0 = +2.0 * (w * x + y * z)
  const t1 = +1.0 - 2.0 * (x * x + y * y)
  const roll = (Math.atan2(t0, t1) * 180) / Math.PI

  const t2 = +2.0 * (w * y - z * x)
  const t2_clamped = t2 > 1.0 ? 1.0 : t2 < -1.0 ? -1.0 : t2
  const pitch = (Math.asin(t2_clamped) * 180) / Math.PI

  const t3 = +2.0 * (w * z + x * y)
  const t4 = +1.0 - 2.0 * (y * y + z * z)
  const yaw = (Math.atan2(t3, t4) * 180) / Math.PI

  return { roll, pitch, yaw }
}

export const useDeviceStore = defineStore('device', () => {
  // Connection State
  const isConnected = ref(false)
  const lastHeartbeat = ref(0)
  const socket = ref<WebSocket | null>(null)
  let reconnectTimer: number | undefined = undefined

  // Sensor Data
  const airspeed = ref(0.0) // m/s

  const imu = ref({
    accel: { x: 0, y: 0, z: 0 },
    gyro: { x: 0, y: 0, z: 0 },
    attitude: { roll: 0, pitch: 0, yaw: 0 },
    vibration: { rms: 0, peak: 0 },
  })

  const battery = ref({
    voltage: 0.0,
    soc: 0.0,
  })

  const lidar = ref({
    distance: 0,
    signalStrength: 0,
  })

  const env = ref({
    pressure: 0.0,
    temperature: 0.0,
    humidity: 0.0,
  })

  const acoustic = ref({
    spl: 0.0,
    peakFreq: 0.0,
    spectrum: Array.from({ length: 16 }, () => 0.0),
  })

  // Control State (Outbound)
  const controls = ref({
    throttle: 0, // 0-255
    servo: 90, // 0-180 degrees mapped to u8
    imuHorizontal: false,
  })

  // System Status
  const status = ref('Idle') // Idle, Initializing, Running, Error

  // WebSocket Actions
  function connect() {
    if (
      socket.value &&
      (socket.value.readyState === WebSocket.OPEN ||
        socket.value.readyState === WebSocket.CONNECTING)
    ) {
      return
    }

    const wsUrl = `ws://${window.location.hostname}:3000/ws`
    console.log(`Connecting to ${wsUrl}...`)

    socket.value = new WebSocket(wsUrl)

    socket.value.onopen = () => {
      console.log('WebSocket Connected')
      isConnected.value = true
      // Clear reconnect timer if it exists
      if (reconnectTimer) {
        clearTimeout(reconnectTimer)
        reconnectTimer = undefined
      }
    }

    socket.value.onclose = () => {
      console.log('WebSocket Closed')
      isConnected.value = false
      scheduleReconnect()
    }

    socket.value.onerror = (error) => {
      console.error('WebSocket Error:', error)
      socket.value?.close()
    }

    socket.value.onmessage = (event) => {
      try {
        const payload = JSON.parse(event.data)
        handleMessage(payload)
      } catch (e) {
        console.error('Failed to parse WS message:', e)
      }
    }
  }

  function scheduleReconnect() {
    if (reconnectTimer) return
    reconnectTimer = window.setTimeout(() => {
      reconnectTimer = undefined
      connect()
    }, 3000)
  }

  function handleMessage(payload: any) {
    lastHeartbeat.value = Date.now()

    // Switch on SerialMessage Variants
    if (payload.ImuData) {
      const data = payload.ImuData
      imu.value.accel = { x: 0, y: 0, z: data.accel_z } // We only have Z in ImuData struct
      imu.value.gyro = { x: data.gyro_x, y: data.gyro_y, z: 0 } // We only have X/Y in ImuData struct

      // Update Attitude from Quaternion
      const euler = quaternionToEuler(data.quad_w, data.quad_i, data.quad_j, data.quad_k)
      imu.value.attitude = euler
    } else if (payload.PitotAirspeedData) {
      const data = payload.PitotAirspeedData
      // Calculate airspeed from differential pressure if needed,
      // but assuming the backend might eventually send calculated speed.
      // For now, let's just map static_port or use a placeholder if calculation is complex.
      // Or if the struct changes to include speed.
      // Wait, PitotAirspeedData has splitter_left, splitter_right, static_port.
      // It does NOT have calculated airspeed.
      // Simple approximation: sqrt(2 * (dynamic_pressure) / rho).
      // Let's assume splitter_left - static_port is roughly dynamic pressure for now.
      // Actually, firmware likely does this calculation.
      // Let's check shared/src/serial.rs again? No, it only has raw values.
      // For visualization, let's just use splitter_left as a proxy or 0 for now until we add calculation.
      // Or better: The `ThrottleConfig` uses `airspeed` field which suggests target speed.
      // Let's stick to raw values display or simple diff.
      // Update: The UI displays m/s. Let's use a dummy conversion: diff * constant.
      const diff = Math.abs(data.splitter_left - data.static_port)
      airspeed.value = Math.sqrt((2 * diff) / 1.225)
    } else if (payload.BatteryData) {
      battery.value = {
        voltage: payload.BatteryData.voltage_v,
        soc: payload.BatteryData.soc_percent,
      }
    } else if (payload.LidarData) {
      lidar.value = {
        distance: payload.LidarData.distance_cm,
        signalStrength: payload.LidarData.signal_strength,
      }
    } else if (payload.BarometerData) {
      env.value = {
        pressure: payload.BarometerData.pressure_pa,
        temperature: payload.BarometerData.temperature_c,
        humidity: payload.BarometerData.humidity_percent,
      }
    } else if (payload.AcousticData) {
      acoustic.value = {
        spl: payload.AcousticData.overall_spl,
        peakFreq: payload.AcousticData.peak_frequency,
        spectrum: payload.AcousticData.spectral_shape,
      }
    } else if (payload.AcknowledgementData) {
      // Just heartbeat
    }
    // Handle others...
  }

  // Uplink Actions
  function send(data: any) {
    if (socket.value && socket.value.readyState === WebSocket.OPEN) {
      socket.value.send(JSON.stringify(data))
    }
  }

  function setThrottle(val: number) {
    const value = Math.max(0, Math.min(255, Math.floor(val)))
    controls.value.throttle = value
    send({ ThrottleConfig: { airspeed: value } })
  }

  function setServo(val: number) {
    const value = Math.max(0, Math.min(180, Math.floor(val)))
    controls.value.servo = value
    send({ ServoConfig: { angle: value } })
  }

  function setCommand(cmd: 'Start' | 'Stop' | 'Calibrate') {
    if (cmd === 'Start') status.value = 'Running'
    else if (cmd === 'Stop') status.value = 'Idle'
    else if (cmd === 'Calibrate') status.value = 'Initializing'

    send({ Command: cmd })
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
    connect,
    setThrottle,
    setServo,
    setCommand,
  }
})
