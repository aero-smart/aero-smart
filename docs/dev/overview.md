## AeroSmart — System Overview

AeroSmart is a **desktop-scale intelligent ducted wind tunnel** for aerodynamic testing. It generates controlled airflow (up to 24 m/s) through a 125×75 mm test section using dual 70mm electric ducted fans (EDFs), measures aerodynamic forces via a suite of sensors, and presents real-time data through a web dashboard. The system targets aeromodeling enthusiasts, student engineering teams, and educational demonstrations.

The monorepo contains four major components plus hardware design files:

```
┌──────────────┐    UART/rkyv     ┌──────────────┐   GraphQL/JSON   ┌──────────────┐
│   FIRMWARE   │ ◄──────────────► │   SERVICE    │ ◄──────────────► │    PANEL     │
│  (STM32H7)   │   915200 baud    │  (Axum/PG)   │    HTTP/WS       │  (Vue 3/TS)  │
│ Embassy Rust │                  │  Rust Tokio  │                  │  Vite + Pinia│
└──────────────┘                  └──────────────┘                  └──────────────┘
       │                                │
       │ SPI/I2C/UART/PWM               │ SQLx
       ▼                                ▼
  ┌──────────┐                   ┌──────────────┐
  │ SENSORS  │                   │ PostgreSQL   │
  │ ACTUATORS│                   │  + pgvector  │
  └──────────┘                   └──────────────┘
```

The **shared** crate (`aerosmart-shared`) sits between firmware and service, providing `no_std`-compatible type definitions for all messages and database records.

------

## 1. Firmware (`firmware/`) — Embedded Real-Time Control

The firmware runs on an **STM32H743VG** (ARM Cortex-M7 @ 400 MHz) using the **Embassy async runtime** — a cooperative, interrupt-driven executor purpose-built for embedded Rust.

### 1.1 Initialization (`main.rs`)

On boot, the firmware:

1. **Configures clocks**: PLL1_P → 400 MHz core, 200 MHz AHB, 100 MHz APB
2. **Initializes peripherals**:
   - **SPI1** @ 1 MHz → ICM-42688-P IMU
   - **I2C1** @ 400 kHz → MS4525DO pitot tube + BME280 barometer
   - **I2C3** @ 400 kHz → ADS1115 ADC (analog pressure sensors)
   - **USART1** @ 915200 baud → backend serial link
   - **USART3** @ 115200 baud → TF-Mini S LIDAR
   - **TIM1** @ 600 kHz → DShot ESC protocol for dual EDFs
   - **TIM2** @ 50 Hz → MG90S servo PWM
   - **TIM3** → Quadrature encoder input (QEI)
   - **SPI2** → WS2812B addressable LEDs
   - **SAI1** → I2S stereo microphone @ 44.1 kHz
   - **IWDG** → Independent watchdog (20s timeout)
3. **Spawns concurrent tasks** based on compile-time feature flags (e.g., `airspeed-control`, `fft`, `ahrs`, `imu`, `lidar`, `serial`)

A `TestConfig` struct controls which subsystems are active, allowing selective hardware testing.

### 1.2 Global State (`state.rs`)

All tasks share a single `GlobalState` struct protected by `Mutex<CriticalSectionRawMutex, _>`. It holds:

- **Machine status**: `Idle | Initializing | Running | Error | EmergencyStop`
- **Control mode**: `Manual | Assisted | Autonomous`
- **Sensor readings**: current airspeed, IMU samples, barometer data, battery voltage/SOC, LIDAR distance, analog pressure channels, quaternion attitude, vibration metrics, acoustic FFT data
- **IMU circular buffers**: 1024-sample ring buffers for `accel_z`, `gyro_x`, `gyro_y` (used by FFT task)
- **Setpoints**: desired airspeed, desired servo angle
- **Environmental**: air density (updated from barometer)

Inter-task communication uses **Embassy Signals** (wake-based, single-value) and **Channels** (buffered MPSC queues):

| Signal                    | Producer      | Consumer   | Purpose                     |
| :------------------------ | :------------ | :--------- | :-------------------------- |
| `AIRSPEED_UPDATED_SIGNAL` | Airspeed task | EDF task   | Triggers PID recalculation  |
| `IMU_UPDATED_SIGNAL`      | IMU task      | Serial TX  | New attitude data available |
| `IMU_BUFFER_FULL_SIGNAL`  | IMU task      | FFT task   | 1024 samples collected      |
| `STATUS_UPDATED_SIGNAL`   | Various       | LED task   | Machine state changed       |
| `DESIRED_UPDATE_SIGNAL`   | Serial RX     | Servo task | New angle setpoint          |

### 1.3 Sensor Drivers

Each sensor has a low-level driver (register definitions) and a high-level interface:

**ICM-42688-P IMU** (`sensors/imu_spi.rs`, `drivers/icm_42688_p.rs`):

- 6-DOF: ±16g accelerometer + ±2000 dps gyroscope
- SPI register access with configurable full-scale range and output data rate
- Event-driven at ~1 kHz via GPIO interrupt (data-ready pin)

**MS4525DO Pitot Tube** (`sensors/pitot_i2c.rs`):

- Differential pressure sensor at I2C address 0x28
- 14-bit raw pressure → ±1 PSI (±6894 Pa)
- 11-bit temperature: `(raw × 200/2047) − 50`
- Polled at 100 Hz
- Kalman filter implemented

**BME280 Barometer** (`drivers/bme_280.rs`):

- Pressure, temperature, humidity at I2C address 0x76
- 4× oversampling, normal mode
- Polled at 10 Hz (decimated from the 100 Hz pitot loop)
- Used to compute real-time air density via the Magnus formula

**ADS1115 ADC** (`sensors/adc_i2c.rs`, `drivers/ads1115.rs`):

- 16-bit, 4-channel ADC at I2C address 0x48
- Reads analog pressure transducers (XGZP6847A) and battery voltage
- Single-shot conversion triggered with data-ready interrupt

**TF-Mini S LIDAR** (`sensors/lidar_uart.rs`):

- 9-byte UART frames (header `0x59 0x59`, distance_cm, signal_strength)
- Polled every 3 seconds
- **Safety**: triggers `EmergencyStop` if distance < 30 cm

**ICS-43434 Microphone** (`audio_i2s.rs`):

- I2S stereo, 24-bit, 44.1 kHz via SAI1
- 4096-sample buffer for acoustic FFT

### 1.4 Algorithms

**PID Airspeed Controller** (`algorithms/airspeed.rs`):

The controller uses a **feedforward + feedback** architecture with three operating states:

| State           | Condition        | Strategy                                      |
| :-------------- | :--------------- | :-------------------------------------------- |
| **Reaching**    | error > 20%      | Pure feedforward from mass conservation model |
| **Approaching** | 5% < error ≤ 20% | Sigmoid-weighted blend of feedforward + PID   |
| **Cruising**    | error ≤ 5%       | Full PID feedback control                     |

The feedforward model derives throttle from physics:
$$
v_{\mathrm{edf}} = \sqrt{\dfrac{F_\mathrm{thrust}}{\rho \cdot A_{\mathrm{edf}}}}\\
\mathrm{throttle} = \dfrac{v_{\mathrm{desired}}}{v_\max}
$$
Voltage compensation scales for battery sag. PID gains: $K_p=1.0, K_i=0.1, K_d=0.05$, output clamped to ±50.

**Madgwick AHRS** (`algorithms/madgwick.rs`):

- Fuses accelerometer + gyroscope into a quaternion ($w, i, j, k$)
- Sample rate: 1000 Hz, beta: 0.033
- Outputs Euler angles (roll, pitch, yaw) for telemetry

**Kalman Airspeed Filter** (`algorithms/airspeed_filter.rs`):

- Single-state Kalman filter for smoothing pitot readings
- Process variance $Q=0.001$, measurement variance R=0.1

**Motion FFT** (`algorithms/motion_fft.rs`):

- 1024-point real FFT on IMU circular buffer using `microfft`
- Hann windowing (generated at compile time by `build.rs`)
- Extracts: RMS vibration, dominant frequency, peak magnitude

**Acoustic FFT** (`algorithms/acoustic_fft.rs`):

- 4096-point real FFT on microphone data
- Blackman-Harris windowing (better sidelobe suppression)
- Extracts: overall SPL, peak frequency, 16-band spectral shape, turbulence index (spectral flux)

### 1.5 Concurrent Tasks

All tasks run as Embassy async functions on the Cortex-M7:

| Task                  | Rate         | Key Logic                                                    |
| :-------------------- | :----------- | :----------------------------------------------------------- |
| **Airspeed**          | 100 Hz       | Reads pitot, computes airspeed from Bernoulli `v=√(2ΔP/ρ)`, updates state |
| **IMU**               | 1 kHz        | Reads IMU via SPI, runs Madgwick AHRS, fills FFT buffer      |
| **EDF**               | event-driven | Waits for `AIRSPEED_UPDATED_SIGNAL`, runs PID, sends DShot to both EDFs |
| **Servo**             | event-driven | Waits for `DESIRED_UPDATE_SIGNAL`, sets PWM duty cycle       |
| **IMU FFT**           | event-driven | Waits for `IMU_BUFFER_FULL_SIGNAL`, computes vibration metrics |
| **LIDAR**             | 0.33 Hz      | Measures distance, triggers emergency stop on proximity      |
| **Battery**           | 1 Hz         | Reads ADC, computes voltage and SOC from lookup table        |
| **Analog Pressure**   | 20 Hz        | Polls ADS1115 across 4 channels                              |
| **LED**               | event-driven | Maps machine status to WS2812B color                         |
| **Watchdog**          | 1 Hz         | Pets the independent watchdog                                |
| **Acoustic Sample**   | continuous   | Streams I2S audio, buffers 4096 samples                      |
| **Acoustic Analysis** | event-driven | FFT on audio, extracts spectral features                     |
| **Serial RX**         | continuous   | Deserializes rkyv commands from service                      |
| **Serial TX**         | 100 Hz base  | Transmits telemetry at tiered rates (IMU@20Hz, airspeed@10Hz, LIDAR@5Hz, battery@1Hz) |
| **QEI**               | 20 Hz        | Reads encoder position and button state                      |

### 1.6 Actuator Control

**Dual EDFs** (`executors/edf.rs`): Controlled via DShot digital protocol over TIM1 PWM at 600 kHz. Throttle range 0–2000. Both fans receive symmetric commands.

**Servo** (`executors/servo.rs`): Standard 50 Hz PWM on TIM2. Pulse width maps linearly: 1.0 ms (0°) to 2.0 ms (180°).

### 1.7 Utility Modules

- **`utils/pitot.rs`**: Bernoulli equation $v=\sqrt{\dfrac{2\cdot\Delta P}{\rho}}$
- **`utils/mass_conservation.rs`**: Feedforward throttle from thrust equation and continuity
- **`utils/magnus.rs`**: Moist air density from Magnus saturation vapor pressure formula
- **`utils/battery.rs`**: VREFINT-calibrated ADC conversion, 21-point SOC lookup table for 4S LiPo
- **`utils/pressure_adc.rs`**: Analog pressure transducer voltage-to-Pa conversion

------

## 2. Shared Library (`shared/`) — Cross-Platform Types

The `aerosmart-shared` crate is `#![no_std]`-compatible with feature gates:

| Feature         | What it enables                        |
| :-------------- | :------------------------------------- |
| `std` (default) | Standard library, rkyv std, chrono std |
| `sql`           | SQLx `FromRow`, pgvector, UUID types   |
| `serde`         | JSON serialization for web API         |
| `defmt`         | Embedded logging format                |
| `graphql`       | Juniper GraphQL derives                |
| `proto`         | Protobuf via prost                     |

### 2.1 Serial Protocol (`serial.rs`)

Defines all messages exchanged over UART:

**Service → Firmware (Commands)**:

- `ThrottleConfig { airspeed: u8 }` — target airspeed percentage
- `ServoConfig { angle: u8 }` — angle of attack
- `SensorConfig { imu_horizontal: bool }` — IMU orientation
- `Command { Start | Stop | Calibrate }` — state machine control
- `AcknowledgementConfig { ack: bool, unix_timestamp_ms: u64 }`

**Firmware → Service (Telemetry)**:

- `ImuData` — accel_z, gyro_x/y, quaternion (w/i/j/k), timestamp
- `PitotAirspeedData` — splitter_left/right, static_port, timestamp
- `AcousticData` — SPL, peak_frequency, peak_magnitude, 16-band spectral_shape, turbulence_index
- `ImuVibrationMetrics` — RMS, dominant_frequency, peak_magnitude (×3 axes)
- `BarometerData` — pressure, temperature, humidity
- `BatteryData` — voltage, SOC percentage
- `LidarData` — distance_cm, signal_strength
- `AnalogPressureSensorData` — 4 channels + validity bitmask
- `QeiData` — encoder position, direction, button state

All types are wrapped in a `SerialMessage` enum. Serialized with **rkyv** for zero-copy decoding — the firmware can interpret messages in-place without allocation.

### 2.2 Database Records (`sql.rs`)

Conversion methods (`to_record(session_uuid)`) transform telemetry structs into SQL-ready records:

- `ImuRecord` — includes full quaternion, references experiment session UUID
- `PitotAirspeedRecord` — three pressure channels per session
- `AcousticRecord` — SPL + spectral_shape stored as `pgvector::Vector` for ML similarity queries
- `Experiment` — session metadata (name, description, UUID)

------

## 3. Backend Service (`service/`) — Web API & Persistence

The service is built on **Axum** (web framework), **Juniper** (GraphQL), **SQLx** (PostgreSQL), and **tokio-serial** (UART).

**Current status**: The service is in **skeleton phase** — dependencies are configured, the database migration is written, and the serial wrapper exists, but the Axum server, GraphQL schema, and database query layer are not yet implemented.

### 3.1 Database Schema (`migrations/20260111031124_initial.sql`)

```sql
-- Extensions
CREATE EXTENSION IF NOT EXISTS vector;       -- pgvector for embeddings
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";  -- UUID generation

-- Tables
experiments         (id UUID PK, name, description, created_at, updated_at)
imu_records         (id SERIAL PK, session FK, timestamp, accel/gyro/quaternion fields)
pitot_airspeed_records (id SERIAL PK, session FK, timestamp, 3 pressure channels)
acoustic_records    (id SERIAL PK, session FK, timestamp, SPL, peak_freq,
                     spectral_shape vector(32), turbulence_index)
```

Indexes cover session+timestamp lookups (descending) and an HNSW vector cosine similarity index on `acoustic_records.spectral_shape` for nearest-neighbor queries.

### 3.2 Serial Communication (`serial/mod.rs`)

A thin wrapper around `tokio_serial::SerialPort` with raw `read()`/`write()` methods. The intended flow:

1. Service opens UART port at 915200 baud
2. Incoming bytes are deserialized via rkyv into `SerialMessage` variants
3. Telemetry data is persisted to PostgreSQL and broadcast to frontend
4. Commands from frontend are serialized and sent to firmware

### 3.3 Planned Architecture

Based on dependencies and the shared library:

- **Axum routes**: REST + GraphQL endpoint (`/graphql`)
- **Juniper resolvers**: queries for experiments, sensor data; mutations for experiment management and control commands
- **SQLx pool**: connection pooling with type-safe queries
- **Context/DI**: `AppState` holding DB pool, serial handle, and schema

------

## 4. Frontend Panel (`panel/`) — Vue 3 Dashboard

A **Vue 3 + TypeScript + Vite** application using Pinia for state management.

**Current status**: Clean boilerplate — the toolchain is fully configured but no application features are implemented yet.

### 4.1 Toolchain

| Tool            | Version         | Purpose                                   |
| :-------------- | :-------------- | :---------------------------------------- |
| Vite            | 8.0.0-beta.7    | Build tool + HMR dev server               |
| Vue             | 3.5.26          | UI framework (Composition API)            |
| Pinia           | 3.0.4           | State management                          |
| Vue Router      | 4.6.4           | Client-side routing                       |
| TypeScript      | 5.9.3           | Type safety                               |
| vue-tsc         | 3.2.2           | Vue template type checking                |
| Vitest          | 4.0.16          | Unit testing                              |
| Playwright      | 1.57.0          | E2E testing (Chromium, Firefox, WebKit)   |
| ESLint + Oxlint | 9.39.2 / 1.38.0 | Linting                                   |
| Prettier        | 3.7.4           | Formatting (no semicolons, single quotes) |

### 4.2 Application Bootstrap

```
index.html → src/main.ts → createApp(App)
  → app.use(createPinia())    // State management
  → app.use(router)           // Routing
  → app.mount('#app')         // DOM mount
```

The router has no routes defined yet. A single example Pinia store (`counter.ts`) demonstrates the Composition API pattern.

------

## 5. End-to-End Data Flow

Here's how a complete airspeed control loop works:

```
1. USER sets target airspeed in Vue dashboard
2. PANEL sends GraphQL mutation to SERVICE
3. SERVICE serializes ThrottleConfig via rkyv, writes to UART
4. FIRMWARE Serial RX task deserializes command, updates desired_airspeed in GlobalState
5. FIRMWARE Airspeed task reads MS4525DO pitot tube at 100 Hz
   → Bernoulli: v = √(2ΔP/ρ)
   → ρ updated from BME280 every 10 cycles
   → Kalman filter smooths measurement
   → Fires AIRSPEED_UPDATED_SIGNAL
6. FIRMWARE EDF task wakes, reads current vs desired airspeed
   → PID controller computes throttle (feedforward + feedback)
   → Sends DShot command to both EDFs via TIM1
7. FIRMWARE Serial TX sends PitotAirspeedData @ 10 Hz over UART
8. SERVICE deserializes telemetry, stores in PostgreSQL, pushes to frontend
9. PANEL updates dashboard in real-time
```

Safety runs in parallel:

- Watchdog task pets IWDG every 1s (20s timeout → hardware reset on hang)
- LIDAR checks proximity every 3s → `EmergencyStop` if < 30 cm
- Battery task monitors voltage → warnings on low SOC

------

## 6. Build System

**Firmware**: Rust 1.90, target `thumbv7em-none-eabi`, flashed via `probe-rs`. The `build.rs` generates FFT window coefficients (Hann 1024-point, Blackman-Harris 1024-point) at compile time. Both dev and release profiles use LTO and size optimization (`opt-level = "z"`).

**Service**: Standard Rust with Tokio async runtime. Database migrations via SQLx.

**Frontend**: pnpm + Vite. `pnpm build` runs `vue-tsc --build` (type check) then Vite production build in parallel.

**Workspace**: Cargo workspace with resolver v2, three members (`firmware`, `service`, `shared`). Panel is outside the Cargo workspace (Node.js project).

------

## 7. Project Status

The project is in **Phase 2 (Perception Fusion)**:

| Component       | Status                                                       |
| :-------------- | :----------------------------------------------------------- |
| Firmware        | **Substantially complete** — all sensor drivers, algorithms, tasks, and actuator control are implemented |
| Shared library  | **Complete** — all message types, database records, and serialization are defined with tests |
| Backend service | **Skeleton** — dependencies configured, migration written, serial wrapper exists, but no server logic |
| Frontend        | **Boilerplate** — toolchain configured, no application features |
| Hardware design | **In progress** — KiCAD schematics and CAD models for EDF/ESC present |

