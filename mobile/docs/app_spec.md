# AeroSmart Mobile App Specification

## 1. Overview
This document outlines the specifications for the AeroSmart mobile application, built with Flutter. The app serves as a control and monitoring interface for the AeroSmart wind tunnel system.

**Design Philosophy:**
- **Style:** Minimalist Material Design (Black & White).
- **Focus:** Clarity, performance, and professional engineering aesthetics.

## 2. Core Features

### 2.1 Configuration Management
- Modify all system configuration parameters.
- Real-time adjustment of control parameters (e.g., PID coefficients, if exposed).
- Control actuators:
    - **Throttle:** Dual EDF thrust control (0-100%).
    - **Servo:** Angle of attack adjustment.
- System Commands: Start, Stop, Calibrate, Emergency Stop.

### 2.2 Firmware Update
- Over-the-air (OTA) or wired firmware updates for the STM32H7 microcontroller.
- Progress tracking and verification.

### 2.3 Data Monitoring
- Real-time telemetry visualization.
- **Sensors:**
    - **Airspeed:** Differential pressure & calculated velocity.
    - **IMU:** Acceleration, Gyroscope, Quaternion (Attitude).
    - **Environmental:** Pressure, Temperature, Humidity.
    - **Lidar:** Distance measurements.
    - **Battery:** Voltage and State of Charge (SoC).
    - **Acoustic:** FFT spectrum data (future).

### 2.4 3D Attitude Indicator
- Real-time 3D visualization of the wind tunnel model's orientation based on IMU quaternion data.
- Interactive 3D model viewer.

## 3. Page Structure

### 3.1 Dashboard (Home)
The main landing page providing an immediate system status overview.
- **Header:** Connection status (Bluetooth/WiFi), Battery level.
- **Primary Metrics:** Current Airspeed, Throttle %, Active Mode.
- **3D Widget:** A small, live 3D representation of the model's attitude.
- **Quick Actions:** Emergency Stop, Start/Stop Toggle.

### 3.2 Data Monitor
Detailed view of all sensor streams.
- **Tabbed/Scrollable View:**
    - **Flight Dynamics:** IMU data graphs (Accel/Gyro), Euler angles.
    - **Aerodynamics:** Airspeed, Pitot pressure graphs.
    - **Environment:** Barometer, Lidar distance.
- **Features:** Data logging toggle (future), graph time-scale adjustment.

### 3.3 Controls & Settings
Comprehensive control panel for system parameters.
- **Actuator Control:** Sliders/Inputs for Throttle and Servo Angle.
- **System Config:**
    - PID Tuning parameters (P, I, D for Airspeed/Attitude).
    - Sensor Calibration triggers.
    - Safety limits configuration (Max Throttle, Max Angle).

### 3.4 Firmware
Dedicated page for system maintenance.
- **Current Version:** Display running firmware version.
- **Update:** File picker or cloud fetch for new firmware binaries.
- **Process:** Flash button with progress bar and status log.

## 4. Technical Architecture

### 4.1 Tech Stack
- **Framework:** Flutter (Dart)
- **UI Library:** Material Design 3
- **State Management:** Riverpod
- **Routing:** GoRouter
- **3D Rendering:** `flutter_3d_controller` or `model_viewer_plus`

### 4.2 Project Structure
```
lib/
├── main.dart           # Entry point
├── core/               # Core utilities, theme, constants
│   ├── theme/          # AppTheme (Black & White)
│   └── router/         # GoRouter configuration
├── features/
│   ├── dashboard/      # Dashboard page & widgets
│   ├── monitor/        # Data monitoring page
│   ├── controls/       # Control panel
│   ├── firmware/       # Firmware update logic
│   └── settings/       # App settings
├── shared/             # Shared widgets & models
│   ├── models/         # Data models (ImuData, Config, etc.)
│   └── widgets/        # Reusable UI components
└── services/           # Communication services (Bluetooth/Http - Placeholders for now)
```
