# CLAUDE.md - AeroSmart AI Assistant Guide

## Project Overview

AeroSmart is a **desktop-grade intelligent ducted wind tunnel system** for aerodynamic testing. The system features dual electric ducted fans (EDFs), real-time sensor fusion, PID closed-loop control, and an AI-powered natural language interface.

**Target Users:** Aeromodeling enthusiasts, student engineering teams, and educational demonstrations.

**Current Phase:** Phase 2 (Perception Fusion) — integrating sensors with STM32 PID control and real-time dashboard.

## Repository Structure

```
aero-smart/
├── firmware/           # STM32H7 embedded Rust firmware (Embassy async runtime)
├── service/            # Backend service (Axum, WebSocket, serial bridge)
├── shared/             # Shared Rust library (no_std compatible, cross-platform types)
├── panel/              # Vue.js 3 + Tauri desktop dashboard
│   └── src-tauri/      # Tauri Rust backend (workspace member)
├── mobile/             # Flutter mobile companion app
├── docs/               # VitePress documentation site
├── misc/               # Python utility scripts (PID analysis, acoustic analysis)
├── schematics/         # KiCad electronic circuit schematics
├── model/              # CAD models (Fusion 360 .f3z/.f3d format)
├── Cargo.toml          # Rust workspace configuration
├── justfile            # Just task runner (CI, build, format commands)
├── pnpm-workspace.yaml # pnpm workspace (panel, docs)
└── .github/workflows/  # CI/CD (aarch64 Tauri build)
```

## Tech Stack

| Component    | Technology                                                           |
| ------------ | -------------------------------------------------------------------- |
| Firmware     | Rust + Embassy async runtime (STM32H7, Cortex-M7)                    |
| Backend      | Rust + Axum + WebSocket + tokio-serial                               |
| Frontend     | Vue 3 + TypeScript + Vite (beta) + Pinia + Tailwind CSS + ECharts    |
| Desktop      | Tauri v2 (wraps panel as native app for OrangePi)                    |
| Mobile       | Flutter (Android/iOS companion)                                      |
| Shared Types | rkyv (zero-copy, no_std) + Serde (JSON) + ts-rs (TypeScript codegen) |
| Hardware     | STM32H743VG @ 400MHz, probe-rs for flashing                          |
| Database     | PostgreSQL + SQLx + pgvector (schema defined, not yet wired)         |
| Docs         | VitePress                                                            |

## Development Commands

### Task Runner (justfile)

```bash
# Install tooling (taplo, cargo-shear)
just setup

# Generate TypeScript types from Rust shared types
just gen-ts-schema

# Format all code (Rust + frontend)
just fmt
```

### Firmware (STM32H7)

```bash
# Build firmware
cargo build -p aerosmart-firmware

# Build for target (CI/release)
cargo build -p aerosmart-firmware --target thumbv7em-none-eabi --release

# Flash to device (requires probe-rs and connected STM32)
cd firmware && cargo run --release

# CI checks
just ci-firmware
```

### Backend Service

```bash
# Build service
cargo build -p aerosmart-service

# Run service (connects to firmware via serial, serves WebSocket on port 3000)
cargo run -p aerosmart-service -- --port /dev/ttyUSB0 --baud 915200

# Run database migrations (requires DATABASE_URL env)
cd service && sqlx migrate run
```

### Frontend (panel/)

```bash
cd panel

# Install dependencies
pnpm install

# Development server (port 5173)
pnpm dev

# Build for production
pnpm build

# Type checking
pnpm type-check

# Run unit tests (Vitest)
pnpm test:unit

# Run E2E tests (Playwright - Chromium, Firefox, WebKit)
pnpm test:e2e

# Linting (Oxlint, type-aware)
pnpm lint

# Auto-fix linting issues
pnpm lint:fix

# Format code (Oxfmt with Prettier config)
pnpm format

# Tauri desktop app
pnpm tauri:dev             # Development
pnpm tauri:build           # Production build
pnpm tauri:build:aarch64   # Cross-compile for OrangePi Zero 3

# CI checks
just ci-panel
```

### Workspace-Wide

```bash
# Build all workspace members
cargo build

# Check all code
cargo check

# Format Rust code
cargo fmt

# Run clippy lints
cargo clippy
```

## Architecture Overview

### Firmware (`firmware/src/`)

The firmware uses the **Embassy async runtime** for ~13 concurrent tasks on STM32H7.

```
firmware/src/
├── main.rs                 # Entry point, clock/peripheral init, task spawning
├── consts.rs               # Physical constants (EDF specs, PID gains, sample rates)
├── state.rs                # Global state (Mutex-protected GlobalState + Signals)
├── algorithms/
│   ├── airspeed.rs         # Feedforward + PID with adaptive gain scheduling
│   ├── airspeed_filter.rs  # Kalman filter for airspeed smoothing
│   ├── madgwick.rs         # Madgwick AHRS (quaternion attitude estimation)
│   ├── motion_fft.rs       # 1024-point FFT vibration analysis (Hann window)
│   ├── acoustic_fft.rs     # 4096-point acoustic FFT (Blackman-Harris window)
│   ├── hann_window.rs      # [generated] 1024-point Hann coefficients
│   └── blackman_harris_window.rs  # [generated] 1024-point Blackman-Harris coefficients
├── executors/
│   ├── edf_pwm.rs          # Dual EDF PWM control (TIM5, 50 Hz)
│   └── servo.rs            # Servo PWM control (TIM2, 50 Hz)
├── sensors/
│   ├── imu_spi.rs          # ICM-42688-P IMU (SPI, 1 MHz, 1 kHz poll)
│   ├── pitot_i2c.rs        # MS4525DO Pitot + BME280 barometer (I2C)
│   ├── adc_i2c.rs          # ADS1115 ADC for analog pressure sensors (I2C)
│   ├── audio_i2s.rs        # ICS-43434 microphone (I2S/SAI, 44.1 kHz)
│   ├── lidar_uart.rs       # TF-Mini S LIDAR (UART, 115200 baud)
│   ├── qei.rs              # Quadrature encoder interface
│   └── drivers/
│       ├── icm_42688_p.rs  # IMU register definitions & config
│       ├── ms4525do.rs     # Pitot linear transfer functions
│       ├── ads1115.rs      # ADC MUX/PGA configuration
│       ├── tfmini_s.rs     # LIDAR protocol (placeholder)
│       └── bme280/         # Complete BME280 driver
│           ├── mod.rs
│           ├── async.rs        # Async I2C wrapper
│           ├── calibration.rs  # Factory calibration
│           ├── configuration.rs
│           ├── sample.rs
│           └── constants.rs
├── tasks/
│   ├── imu.rs              # IMU polling (1 kHz), Madgwick AHRS update
│   ├── airspeed.rs         # Pitot polling (20 Hz) + barometer (1 Hz)
│   ├── edf.rs              # EDF PID control (event-driven via Signal)
│   ├── servo.rs            # Servo angle control (event-driven via Signal)
│   ├── imu_fft.rs          # Vibration FFT (event-driven on buffer full)
│   ├── led.rs              # WS2812B status LED (event-driven on status change)
│   ├── lidar.rs            # LIDAR polling (10 Hz), emergency stop on <30 cm
│   ├── battery.rs          # Battery voltage/SOC monitoring (1 Hz ADC)
│   ├── analog_pressure.rs  # Analog pressure sensors (5 Hz)
│   ├── acoustic.rs         # Microphone I2S sampling & FFT analysis
│   ├── watchdog.rs         # Watchdog heartbeat (1 Hz, 20 s timeout)
│   └── serial/
│       ├── mod.rs          # Handshake & RTC synchronization
│       ├── uart_rx.rs      # Command reception (rkyv deserialization)
│       └── uart_tx.rs      # Telemetry transmission
└── utils/
    ├── mod.rs              # rkyv message serialization helpers
    ├── battery.rs          # Voltage → SOC interpolation (4S LiPo)
    ├── magnus.rs           # Air density from P/T/RH (psychrometric)
    ├── mass_conservation.rs # Feedforward throttle with voltage compensation
    ├── pitot.rs            # Bernoulli: airspeed = √(2ΔP/ρ)
    ├── pressure_adc.rs     # Analog voltage → pressure conversion
    └── transfer.rs         # Linear transfer function macro
```

### Backend Service (`service/src/`)

The service bridges firmware serial communication to WebSocket clients.

```
service/src/
├── main.rs          # Entry point: serial task, WebSocket server (port 3000), Axum router
├── wifi.rs          # WiFi management via nmcli (Linux only: scan, connect, status)
├── serial/mod.rs    # Serial port abstraction wrapper
├── context/mod.rs   # [placeholder] Request context
├── database/mod.rs  # [placeholder] PostgreSQL connectivity
├── graphql/mod.rs   # [placeholder] Juniper GraphQL schema
└── system/mod.rs    # [placeholder] System utilities
```

**Current implementation status:**

- **Implemented:** Serial communication (handshake + bidirectional rkyv/JSON bridge), WebSocket server, WiFi management
- **Placeholder:** GraphQL API, database layer, request context

### Shared Library (`shared/src/`)

Cross-platform types used by firmware, service, and frontend.

```
shared/src/
├── lib.rs           # no_std compatible library entry (#![cfg_attr(not(feature = "std"), no_std)])
├── serial.rs        # All message types (rkyv + optional serde/defmt/ts-rs/graphql)
├── sql.rs           # Database record types (requires "sql" feature)
└── bin/
    └── schema.rs    # TypeScript type generator (ts-rs → panel/src/types/)
```

**Feature flags:**
| Feature | Purpose |
|---------|---------|
| `std` (default) | Standard library support |
| `sql` | PostgreSQL types (sqlx, pgvector, uuid) |
| `serde` | JSON serialization |
| `proto` | Protocol buffers (prost) |
| `defmt` | Embedded debug formatting |
| `graphql` | Juniper GraphQL type derives |
| `ts-rs` | TypeScript type generation |

### Frontend Panel (`panel/src/`)

```
panel/src/
├── main.ts              # App initialization (Pinia + Router + i18n)
├── App.vue              # Root component, Tauri fullscreen setup
├── i18n.ts              # Internationalization (en, zh)
├── api/
│   └── wifi.ts          # WiFi HTTP API client
├── components/
│   ├── WifiManager.vue      # WiFi network management
│   ├── VirtualKeyboard.vue  # Touch keyboard input
│   ├── SettingsModal.vue    # Settings dialog
│   ├── layout/              # MainLayout, Sidebar, PageLayout, Container, Grid
│   └── ui/                  # Button, Card, Badge, Input
├── locales/
│   ├── en.ts            # English translations
│   └── zh.ts            # Chinese translations
├── router/index.ts      # Routes: /onboarding, /, /control, /analysis, /settings
├── stores/
│   ├── device.ts        # Core telemetry state, WebSocket connection, device control
│   ├── locale.ts        # Locale persistence (localStorage)
│   └── wifi.ts          # WiFi state management
├── types/               # [generated] TypeScript types from ts-rs
│   └── *.ts             # SerialMessage, ImuData, PitotAirspeedData, etc.
└── views/
    ├── DashboardView.vue    # Telemetry dashboard (ECharts gauges, waveforms, IMU cube)
    ├── ControlView.vue      # Sensor snapshots, AI suggestions, live metrics
    ├── PowerView.vue        # Time-series analysis, CSV export, zoom
    ├── SettingsView.vue     # Settings page, WiFi manager
    └── OnboardingView.vue   # Setup wizard (language, WiFi, calibration)
```

## Key Patterns & Conventions

### Firmware Conventions

- **Embassy async/await**: All I/O is non-blocking via Embassy's async executor
- **Signal-based coordination**: Tasks wake on events (`AIRSPEED_UPDATED_SIGNAL`, `IMU_BUFFER_FULL_SIGNAL`, `STATUS_UPDATED_SIGNAL`, `DESIRED_UPDATE_SIGNAL`)
- **Global state**: Single `GlobalState` struct protected by `Mutex<CriticalSectionRawMutex, _>`
- **defmt logging**: Use `defmt::info!`, `defmt::debug!`, `defmt::warn!`, `defmt::error!` for zero-cost embedded logging
- **Feature flags**: Sensors/algorithms conditionally compiled (default: `airspeed-control`, `fft`, `ahrs`, `microphone`, `imu`, `lidar`, `serial`, `pwm-edf`, `pwm-servo`)
- **Driver abstraction**: Hardware details encapsulated in `sensors/drivers/`
- **Build-time codegen**: `build.rs` generates FFT window coefficients (Hann 1024-pt, Blackman-Harris 1024-pt)

### Service Conventions

- **WebSocket bridge**: Serial telemetry is deserialized from rkyv, broadcast to WebSocket clients as JSON
- **Length-prefixed framing**: UART messages use 4-byte little-endian length prefix + rkyv payload
- **Handshake protocol**: Service waits for firmware `AcknowledgementData`, responds with `AcknowledgementConfig` (includes timestamp for RTC sync)

### Frontend Conventions

- **Vue 3 Composition API**: `<script setup>` with TypeScript throughout
- **State**: Pinia stores (no modules pattern)
- **Styling**: Tailwind CSS with CSS custom properties for theming
- **Charts**: ECharts for gauges, waveforms, and time-series
- **i18n**: English and Chinese via vue-i18n
- **Type safety**: Auto-generated types from Rust (ts-rs) in `src/types/`
- **Linting**: Oxlint (primary, Rust-based) + ESLint (fallback)
- **Formatting**: Oxfmt/Prettier — no semicolons, single quotes, 100-char width

### Naming Conventions

- **Rust**: `snake_case` for functions/variables, `PascalCase` for types, `SCREAMING_SNAKE_CASE` for constants
- **Files**: `snake_case.rs` for Rust, `PascalCase.vue` for Vue components
- **Feature flags**: `kebab-case`
- **TypeScript**: `PascalCase` for types/interfaces, `camelCase` for variables/functions

### Error Handling

- **Firmware**: Custom error enums with `defmt::Format` derive
- **Service**: `thiserror` for library errors, `anyhow::Result` for application errors
- **Shared**: `Result<T, E>` with specific error types

### Serialization Strategy

| Context            | Library  | Format                   | Purpose                      |
| ------------------ | -------- | ------------------------ | ---------------------------- |
| Firmware ↔ Service | rkyv     | Binary (length-prefixed) | Zero-copy, no_std, UART      |
| Service ↔ Frontend | Serde    | JSON (WebSocket)         | Web-friendly                 |
| Rust → TypeScript  | ts-rs    | Generated `.ts` files    | Type safety across stack     |
| Database           | SQLx     | SQL (PostgreSQL)         | Type-safe queries            |
| Vector embeddings  | pgvector | `vector(16)`             | Acoustic spectral similarity |

## Communication Protocol

### Firmware ↔ Service (UART, 915200 baud)

Wire format: `[4-byte LE length][rkyv payload]`

Messages defined in `shared/src/serial.rs`:

```rust
// Envelope type
pub enum SerialMessage {
    AcknowledgementConfig(AcknowledgementConfig),
    ThrottleConfig(ThrottleConfig),
    ServoConfig(ServoConfig),
    SensorConfig(SensorConfig),
    Command(Command),
    AcknowledgementData(AcknowledgementData),
    PitotAirspeedData(PitotAirspeedData),
    ImuData(ImuData),
    AcousticData(AcousticData),
    LidarData(LidarData),
    BarometerData(BarometerData),
    ImuVibrationMetrics { accel_z, gyro_x, gyro_y },
    BatteryData(BatteryData),
    AnalogPressureSensorData(AnalogPressureSensorData),
    QeiData(QeiData),
}

// Commands (Service → Firmware)
pub struct ThrottleConfig { pub airspeed: u8 }
pub struct ServoConfig { pub angle: u8 }
pub struct SensorConfig { pub imu_horizontal: bool }
pub enum Command { Start, Stop, Calibrate }

// Telemetry (Firmware → Service)
pub struct ImuData { pub accel_z: f32, pub gyro_x: f32, pub gyro_y: f32,
                     pub quad_w: f32, pub quad_i: f32, pub quad_j: f32, pub quad_k: f32,
                     pub time_elapsed_ms: u64 }
pub struct PitotAirspeedData { pub splitter_left: f32, pub splitter_right: f32,
                               pub static_port: f32, pub time_elapsed_ms: u64 }
pub struct BarometerData { pub pressure_pa: f32, pub temperature_c: f32,
                           pub humidity_percent: f32, pub time_elapsed_ms: u64 }
pub struct LidarData { pub distance_cm: u16, pub signal_strength: u16, pub time_elapsed_ms: u64 }
pub struct BatteryData { pub voltage_v: f32, pub soc_percent: f32, pub time_elapsed_ms: u64 }
pub struct AcousticData { pub overall_spl: f32, pub peak_frequency: f32, pub peak_magnitude: f32,
                          pub spectral_shape: [f32; 16], pub turbulence_index: f32,
                          pub time_elapsed_ms: u64 }
pub struct AnalogPressureSensorData { pub pressures_pa: [f32; 4], pub valid_bitmask: u8,
                                     pub time_elapsed_ms: u64 }
```

### Service ↔ Frontend (WebSocket, port 3000)

- Endpoint: `ws://{host}:3000/ws`
- Telemetry: Firmware messages broadcast as JSON to all connected clients
- Commands: Frontend sends JSON `SerialMessage` variants, service serializes to rkyv for firmware

### WiFi API (HTTP, port 3000)

- `GET /api/wifi/scan` — Scan available networks
- `POST /api/wifi/connect` — Connect (body: `{ssid, password}`)
- `POST /api/wifi/disconnect` — Disconnect
- `GET /api/wifi/status` — Connection status
- `GET /api/wifi/test` — Test internet connectivity

## Hardware Configuration

### MCU: STM32H743VG

- **Core**: ARM Cortex-M7 @ 400 MHz (HSE → PLL1 50×/4 = 400 MHz)
- **AHB**: 200 MHz, **APB**: 100 MHz
- **Target**: `thumbv7em-none-eabi`
- **Toolchain**: Rust 1.90 with `rust-src` component
- **Debugger**: probe-rs with RTT logging (`Embed.toml`)

### Peripheral Mapping

```
SPI1:      ICM-42688-P IMU (PB3/PB5/PB4, 1 MHz)
I2C1:      MS4525DO Pitot + BME280 (PB8/PB9, 400 kHz)
I2C3:      ADS1115 ADC (PA8/PC9, 400 kHz)
USART1:    Service serial link (PA9/PA10, 915.2k baud)
USART3:    TF-Mini S LIDAR (PC10/PC11, 115.2k baud)
TIM5:      Dual EDF PWM (PA0/PA1, 50 Hz)
TIM2:      Servo PWM (PA2, 50 Hz)
SPI2:      WS2812B LED (PD3)
SAI1:      ICS-43434 Microphone I2S (PE2-PE6)
ADC1:      Battery voltage (PA3, VREFINT calibration)
IWDG:      Independent watchdog (20 s timeout)
```

### Sensors

| Sensor      | Interface   | Address | Sample Rate | Purpose               |
| ----------- | ----------- | ------- | ----------- | --------------------- |
| ICM-42688-P | SPI (1 MHz) | —       | 1 kHz       | 6-DOF IMU             |
| MS4525DO    | I2C         | 0x28    | 20 Hz       | Pitot tube (airspeed) |
| BME280      | I2C         | 0x76    | 1 Hz        | Barometer/humidity    |
| ADS1115     | I2C         | 0x48    | 5 Hz        | 16-bit ADC (pressure) |
| TF-Mini S   | UART        | —       | 10 Hz       | LIDAR distance        |
| ICS-43434   | I2S/SAI     | —       | 44.1 kHz    | Microphone (acoustic) |

### Actuators

| Actuator       | Protocol    | Timer        | Purpose            |
| -------------- | ----------- | ------------ | ------------------ |
| 70mm EDFs (×2) | PWM 50 Hz   | TIM5 CH1/CH2 | Airflow generation |
| MG90S Servo    | PWM 50 Hz   | TIM2 CH3     | Angle-of-attack    |
| WS2812B LEDs   | Addressable | SPI2         | Status indication  |

## Database Schema (PostgreSQL)

Located in `service/migrations/20260111031124_initial.sql`. Extensions: `pgvector`, `uuid-ossp`.

| Table                    | Purpose                    | Key Columns                                                                                         |
| ------------------------ | -------------------------- | --------------------------------------------------------------------------------------------------- |
| `experiments`            | Experiment sessions        | id (UUID), name, description, created_at, updated_at                                                |
| `imu_records`            | IMU telemetry              | session (FK), timestamp, accel_z, gyro_x/y, quad_w/i/j/k                                            |
| `pitot_airspeed_records` | Airspeed measurements      | session (FK), timestamp, splitter_left/right, static_port                                           |
| `acoustic_records`       | Acoustic data + embeddings | session (FK), timestamp, overall_spl, peak_frequency, spectral_shape (vector(16)), turbulence_index |

Indexes: composite (session, timestamp DESC) for range queries; HNSW on `spectral_shape` for vector similarity.

## Build System

### Workspace (`Cargo.toml`)

```toml
[workspace]
resolver = "2"
members = ["firmware", "service", "shared", "panel/src-tauri"]

[workspace.package]
edition = "2024"
license = "MIT"
version = "0.1.0"
```

- **LTO**: Enabled for both dev and release profiles
- **Firmware**: `opt-level = "z"` (size optimized) in both profiles
- **Release**: `codegen-units = 1`, `panic = "abort"`, `strip = true`, `opt-level = 3`

### Firmware Build Script (`firmware/build.rs`)

Generates at compile time:

- **Hann window**: 1024-point coefficients → `src/algorithms/hann_window.rs`
- **Blackman-Harris window**: 1024-point coefficients → `src/algorithms/blackman_harris_window.rs`

### Shared Build Script (`shared/build.rs`)

Compiles Protocol Buffer schema (`proto/schema.proto`) via prost-build.

### TypeScript Type Generation

```bash
just gen-ts-schema
# Runs: cargo run -p aerosmart-shared --bin gen-ts-schemas -F ts-rs,sql
# Output: panel/src/types/*.ts (15 type files)
```

### CI/CD

**GitHub Actions** (`.github/workflows/build-aarch64.yml`):

- Triggers on push/PR to `main` (panel or shared changes)
- Builds Tauri app for `aarch64-unknown-linux-gnu` (OrangePi Zero 3)
- Produces `.deb` package and AppImage artifacts (30-day retention)

**Just CI commands:**

- `just ci-shared` — check + clippy + build shared
- `just ci-firmware` — check + clippy + build firmware (thumbv7em-none-eabi)
- `just ci-panel` — type-check + lint + build panel

### Cross-Compilation (OrangePi Zero 3)

```bash
# Docker-based (build_aarch64.sh)
./build_aarch64.sh

# Or via Tauri CLI
cd panel && pnpm tauri:build:aarch64
```

Target: `aarch64-unknown-linux-gnu`, display: 1024×600 fullscreen (no decorations).

## Testing

### Firmware

- Real-time debugging via RTT (Real-Time Transfer) with defmt
- Use `cargo run` in `firmware/` with probe-rs for on-device debugging
- Check `firmware/Embed.toml` for probe-rs configuration

### Shared

```bash
cargo test -p aerosmart-shared
# Tests rkyv serialization roundtrips for all message types
```

### Service

```bash
cargo test -p aerosmart-service
```

### Frontend

```bash
cd panel
pnpm test:unit           # Vitest (jsdom environment)
pnpm test:e2e            # Playwright (Chromium, Firefox, WebKit)
pnpm type-check          # vue-tsc type verification
pnpm lint                # Oxlint type-aware linting
```

## Common Tasks

### Adding a New Sensor Driver

1. Create driver in `firmware/src/sensors/drivers/` (register defs, transfer functions)
2. Add sensor interface in `firmware/src/sensors/` (async I2C/SPI/UART reads)
3. Create Embassy task in `firmware/src/tasks/` (polling loop or event-driven)
4. Add data fields to `GlobalState` in `firmware/src/state.rs`
5. Define message type in `shared/src/serial.rs` (with rkyv + optional serde/defmt derives)
6. Add variant to `SerialMessage` enum
7. Run `just gen-ts-schema` to regenerate TypeScript types
8. Update `panel/src/stores/device.ts` to handle the new message type

### Adding a New Algorithm

1. Create module in `firmware/src/algorithms/`
2. Export in `firmware/src/algorithms/mod.rs`
3. Call from the appropriate task
4. If build-time codegen is needed, update `firmware/build.rs`

### Adding a Frontend Feature

1. Create Vue component in `panel/src/components/` (use `<script setup lang="ts">`)
2. Add route in `panel/src/router/index.ts` if it's a new page
3. Create Pinia store in `panel/src/stores/` if state management is needed
4. Add i18n keys to `panel/src/locales/en.ts` and `panel/src/locales/zh.ts`
5. Use existing UI components (`Button`, `Card`, `Badge`, `Input`) and layout components

### Adding a Database Table

1. Create migration in `service/migrations/` (SQL with UUID primary keys)
2. Add record struct in `shared/src/sql.rs` (with `sqlx::FromRow` + `serde`)
3. Add `to_record()` method on the corresponding serial type in `shared/src/serial.rs`
4. Implement query functions in `service/src/database/` (when wired up)

## Debugging Tips

### Firmware

- Use `defmt::info!()`, `defmt::debug!()`, `defmt::warn!()`, `defmt::error!()`
- RTT output visible in probe-rs console
- Check `firmware/Embed.toml` for probe-rs configuration
- `TestConfig` in `main.rs` controls which subsystems are enabled at startup

### Service

- Use `tracing` crate for structured logging
- Check serial connection: `ls /dev/ttyUSB*` or `ls /dev/ttyACM*`
- Service CLI: `--port` and `--baud` flags for serial config
- WebSocket endpoint: `ws://localhost:3000/ws`

### Frontend

- Vue DevTools browser extension (Vite plugin included)
- Vite HMR for fast iteration (port 5173)
- Device store auto-reconnects WebSocket every 3 seconds on disconnect
- ECharts: watch for memory leaks with `ResizeObserver`

## Safety Considerations

- **Watchdog timer**: Independent watchdog with 20 s timeout, heartbeat task at 1 Hz
- **LIDAR proximity stop**: Emergency stop triggered when object detected <30 cm
- **PID limits**: Airspeed controller clamps output to 0–2000 DShot/PWM units
- **Battery monitoring**: Voltage → SOC interpolation for 4S LiPo, low voltage warnings
- **Adaptive PID**: Three-state gain scheduling (Reaching → Approaching → Cruising) prevents overshoot

## Physical Constants

```rust
const TEST_SECTION_WIDTH_MM: f32 = 125.0;   // 5"
const TEST_SECTION_HEIGHT_MM: f32 = 75.0;   // 3"
const TEST_SECTION_LENGTH_MM: f32 = 200.0;  // 8"
const CONTRACTION_RATIO: f32 = 4.0;         // Nozzle contraction
const AIR_DENSITY_KG_M3: f32 = 1.225;       // Sea level
const MAX_AIRSPEED_M_S: f32 = 24.0;         // ~86 km/h
```
