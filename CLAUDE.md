# CLAUDE.md - AeroSmart AI Assistant Guide

## Project Overview

AeroSmart is a **desktop-grade intelligent ducted wind tunnel system** for aerodynamic testing. The system features dual electric ducted fans (EDFs), real-time sensor fusion, PID closed-loop control, and an AI-powered natural language interface.

**Target Users:** Aeromodeling enthusiasts, student engineering teams, and educational demonstrations.

## Repository Structure

```
aero-smart/
├── firmware/           # STM32H7 embedded Rust firmware (Embassy async runtime)
├── service/            # Backend service (Axum web framework, GraphQL, PostgreSQL)
├── shared/             # Shared Rust library (no_std compatible, cross-platform types)
├── panel/              # Vue.js 3 frontend dashboard
├── schematics/         # Electronic circuit schematics
├── model/              # CAD models (STEP format)
├── Cargo.toml          # Rust workspace configuration
└── README.md           # Project documentation
```

## Tech Stack

| Component | Technology |
|-----------|------------|
| Firmware | Rust + Embassy async runtime (STM32H7, Cortex-M7) |
| Backend | Rust + Axum + Juniper GraphQL + SQLx + PostgreSQL |
| Frontend | Vue 3 + TypeScript + Vite + Pinia |
| Serialization | rkyv (zero-copy), Serde (JSON), Protobuf |
| Hardware | STM32H743VG @ 400MHz, probe-rs for flashing |

## Development Commands

### Firmware (STM32H7)

```bash
# Build firmware (from workspace root or firmware/)
cargo build -p aerosmart-firmware

# Build with release optimizations
cargo build -p aerosmart-firmware --release

# Flash to device (requires probe-rs and connected STM32)
cd firmware && cargo run --release

# Run with specific features
cargo build -p aerosmart-firmware --features "feature-name"
```

### Backend Service

```bash
# Build service
cargo build -p aerosmart-service

# Run service
cargo run -p aerosmart-service

# Run database migrations (requires DATABASE_URL env)
cd service && sqlx migrate run
```

### Frontend (panel/)

```bash
cd panel

# Install dependencies
pnpm install

# Development server
pnpm dev

# Build for production
pnpm build

# Type checking
pnpm type-check

# Run unit tests
pnpm test:unit

# Run E2E tests (Playwright)
pnpm test:e2e

# Linting
pnpm lint
```

### Workspace-Wide

```bash
# Build all workspace members
cargo build

# Check all code
cargo check

# Format code
cargo fmt

# Run clippy lints
cargo clippy
```

## Architecture Overview

### Firmware (`firmware/src/`)

The firmware uses the **Embassy async runtime** for concurrent task execution on STM32H7.

```
firmware/src/
├── main.rs                 # Entry point, hardware init, task spawning
├── consts.rs               # Physical constants (EDF specs, test section dimensions)
├── state.rs                # Global state (Mutex-protected GlobalState)
├── algorithms/             # Signal processing & control
│   ├── airspeed.rs         # PID airspeed control with feedforward
│   ├── madgwick.rs         # Madgwick AHRS (quaternion attitude estimation)
│   ├── motion_fft.rs       # FFT-based vibration analysis
│   └── hann_window.rs      # FFT windowing (generated at build time)
├── executors/              # Actuator control
│   ├── edf.rs              # DShot ESC protocol for dual EDFs
│   └── servo.rs            # Servo angle control (angle-of-attack)
├── sensors/                # Sensor interfaces
│   ├── imu_spi.rs          # ICM-42688-P IMU (SPI)
│   ├── pitot_i2c.rs        # MS4525DO Pitot tube (I2C)
│   ├── adc_i2c.rs          # ADS1115 ADC (battery, pressure)
│   ├── lidar_uart.rs       # TF-Mini S LIDAR (UART)
│   └── drivers/            # Low-level chip drivers
│       ├── icm_42688_p.rs  # IMU register definitions
│       ├── bme_280.rs      # Barometer calibration & conversion
│       ├── ads1115.rs      # ADC configuration
│       └── tfmini_s.rs     # LIDAR protocol
├── tasks/                  # Concurrent Embassy tasks
│   ├── airspeed.rs         # Airspeed monitoring & PID loop
│   ├── battery.rs          # Battery voltage & SOC monitoring
│   ├── edf.rs              # EDF thrust control task
│   ├── imu.rs              # IMU data collection
│   ├── imu_fft.rs          # FFT vibration metrics
│   ├── led.rs              # WS2812B status LED control
│   ├── lidar.rs            # Distance measurement
│   ├── servo.rs            # Angle-of-attack control
│   ├── watchdog.rs         # Safety watchdog timer
│   └── serial/             # UART communication
│       ├── uart_rx.rs      # Command reception
│       └── uart_tx.rs      # Telemetry transmission
└── utils/                  # Utility functions
    ├── mass_conservation.rs # Thrust/airspeed conversion
    ├── battery.rs          # Battery calculations
    ├── pitot.rs            # Pitot tube calculations
    └── magnus.rs           # Magnus effect calculations
```

### Backend Service (`service/src/`)

```
service/src/
├── main.rs                 # Application entry point
├── context/                # Request context & dependency injection
├── database/               # PostgreSQL connectivity (SQLx)
├── graphql/                # Juniper GraphQL schema & resolvers
└── serial/                 # UART communication with firmware
```

### Shared Library (`shared/src/`)

```
shared/src/
├── lib.rs                  # no_std compatible library entry
├── serial.rs               # Message types (ThrottleConfig, ImuData, etc.)
└── sql.rs                  # Database record types
```

## Key Patterns & Conventions

### Firmware Conventions

- **Embassy async/await**: All I/O operations are non-blocking using Embassy's async executor
- **Signal-based communication**: Tasks communicate via `Signal<CriticalSectionRawMutex, T>` for wake-based notifications
- **Global state**: Single `GlobalState` struct protected by `Mutex<CriticalSectionRawMutex, _>`
- **defmt logging**: Use `defmt::info!`, `defmt::debug!`, etc. for zero-cost embedded logging
- **Feature flags**: Sensors and algorithms can be conditionally compiled
- **Driver abstraction**: Hardware details are encapsulated in `sensors/drivers/`

### Naming Conventions

- **Rust**: `snake_case` for functions/variables, `PascalCase` for types
- **Files**: `snake_case.rs` for Rust modules
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Feature flags**: `kebab-case`

### Error Handling

- **Firmware**: Custom error enums with `defmt::Format` derive
- **Service**: `anyhow::Result` for application errors, `thiserror` for library errors
- **Shared**: `Result<T, E>` with specific error types

### Serialization Strategy

| Context | Library | Purpose |
|---------|---------|---------|
| Firmware ↔ Service | rkyv | Zero-copy, no_std compatible |
| Service ↔ Frontend | Serde JSON | Web-friendly |
| Database | SQLx | Type-safe queries |
| Future ML features | pgvector | Vector embeddings |

## Hardware Configuration

### MCU: STM32H743VG
- **Core**: ARM Cortex-M7 @ 400MHz
- **Target**: `thumbv7em-none-eabi`
- **Toolchain**: Rust 1.90 with rust-src component
- **Debugger**: probe-rs with RTT logging

### Sensors
| Sensor | Interface | I2C Address | Purpose |
|--------|-----------|-------------|---------|
| MS4525DO | I2C | 0x28 | Pitot tube (airspeed) |
| BME280 | I2C | 0x76 | Barometer/humidity |
| ICM-42688-P | SPI | - | 6-DOF IMU |
| ADS1115 | I2C | 0x48 | 16-bit ADC |
| TF-Mini S | UART | - | LIDAR distance |
| ICS-43434 | I2S | - | Microphone (acoustic) |

### Actuators
| Actuator | Protocol | Purpose |
|----------|----------|---------|
| 70mm EDFs (x2) | DShot | Airflow generation |
| MG90S Servo | PWM | Angle-of-attack |
| WS2812B LEDs | Addressable | Status indication |

## Physical Constants

```rust
// Test section dimensions
const TEST_SECTION_WIDTH_MM: f32 = 125.0;   // 5"
const TEST_SECTION_HEIGHT_MM: f32 = 75.0;   // 3"
const TEST_SECTION_LENGTH_MM: f32 = 200.0;  // 8"

// Nozzle contraction ratio
const CONTRACTION_RATIO: f32 = 4.0;

// Air density at sea level
const AIR_DENSITY_KG_M3: f32 = 1.225;

// Max airspeed
const MAX_AIRSPEED_M_S: f32 = 24.0;
```

## Communication Protocol

### Firmware ↔ Service (UART)

Messages defined in `shared/src/serial.rs`:

```rust
// Commands (Service → Firmware)
pub struct ThrottleConfig { pub throttle_percent: f32 }
pub struct ServoConfig { pub angle_degrees: f32 }
pub enum Command { Start, Stop, Calibrate, EmergencyStop }

// Telemetry (Firmware → Service)
pub struct ImuData { pub accel: [f32; 3], pub gyro: [f32; 3], pub quaternion: [f32; 4] }
pub struct PitotAirspeedData { pub differential_pressure_pa: f32, pub airspeed_m_s: f32 }
pub struct BarometerData { pub pressure_pa: f32, pub temperature_c: f32, pub humidity_percent: f32 }
pub struct LidarData { pub distance_mm: u16 }
pub struct BatteryData { pub voltage_v: f32, pub soc_percent: f32 }
```

## Database Schema (PostgreSQL)

Located in `service/migrations/`:

```sql
-- experiments: Experiment sessions
-- imu_records: IMU telemetry (accel, gyro, attitude)
-- pitot_airspeed_records: Airspeed measurements
-- acoustic_records: FFT spectra with pgvector embeddings
```

## Build System Notes

### Firmware Build Script (`firmware/build.rs`)
- Generates Hann window coefficients (1024-point) at compile time for FFT processing
- Output: `OUT_DIR/hann_window.rs`

### Workspace Optimization
- LTO enabled for both dev and release profiles
- Firmware optimized for size (`opt-level = "z"`)

### Frontend Tooling
- **Package Manager**: pnpm
- **Build**: Vite (beta)
- **Type Check**: vue-tsc
- **Test**: Vitest (unit) + Playwright (E2E)
- **Lint**: ESLint + Oxlint
- **Format**: Prettier

## Testing

### Firmware
- Real-time debugging via RTT (Real-Time Transfer) with defmt
- Use `cargo run` with probe-rs for on-device debugging

### Service
```bash
cargo test -p aerosmart-service
```

### Frontend
```bash
cd panel
pnpm test:unit      # Vitest unit tests
pnpm test:e2e       # Playwright E2E tests
```

## Common Tasks

### Adding a New Sensor Driver
1. Create driver in `firmware/src/sensors/drivers/`
2. Add sensor interface in `firmware/src/sensors/`
3. Create task in `firmware/src/tasks/`
4. Add to global state in `firmware/src/state.rs`
5. Define message type in `shared/src/serial.rs`

### Adding a New Algorithm
1. Create module in `firmware/src/algorithms/`
2. Export in `firmware/src/algorithms/mod.rs`
3. Call from appropriate task

### Adding a GraphQL Query/Mutation
1. Add resolver in `service/src/graphql/`
2. Update schema types
3. Add database query if needed

### Adding a Frontend Feature
1. Create Vue component in `panel/src/components/`
2. Add route in `panel/src/router/`
3. Create Pinia store if needed in `panel/src/stores/`

## Debugging Tips

### Firmware
- Use `defmt::info!()`, `defmt::debug!()`, `defmt::warn!()`, `defmt::error!()`
- RTT output visible in probe-rs console
- Check `firmware/Embed.toml` for probe-rs configuration

### Service
- Use `tracing` crate for structured logging
- Check serial connection: `ls /dev/ttyUSB*` or `ls /dev/ttyACM*`

### Frontend
- Vue DevTools browser extension
- Vite HMR for fast iteration

## Safety Considerations

- **Watchdog timer**: Firmware has watchdog task for safety
- **Emergency stop**: `Command::EmergencyStop` immediately cuts EDF power
- **PID limits**: Airspeed controller has output clamping
- **Battery monitoring**: Low voltage triggers warnings

## Project Status

Current development phase: **Phase 2 (Perception Fusion)** - Integrating sensors with STM32 PID control and real-time dashboard.

Recent work:
- Battery monitoring (ADC-based voltage & SOC)
- ICM-42688 IMU and BME280 barometer drivers
- Task reorganization for modularity
- Serial protocol standardization
