# AeroSmart 上下位机通信协议与前端页面规划文档

本文档详细描述了 AeroSmart 系统中上位机（Panel/Service）与下位机（Firmware）之间的通信协议，WebSocket 接口细节，以及前端各页面的功能期待。

## 1. 通信架构概览

系统采用如下通信链路：
`Firmware (STM32)` <==[Serial (UART)]==> `Service (Tauri Background Task)` <==[WebSocket (JSON)]==> `Frontend (Vue 3)`

*   **下位机 <-> Service**: 使用基于 `rkyv` 的零拷贝二进制协议，为了保证传输稳定，采用了 **Length-Prefixed (4字节长度前缀)** 的分包机制。
*   **Service <-> 前端**: 使用标准 WebSocket 协议，数据格式为 JSON。Service 负责将二进制数据转译为 JSON 广播给前端，并将前端的 JSON 指令转译为二进制发送给下位机。

## 2. 握手协议 (Handshake)

在正式通信前，下位机与 Service 必须完成一次握手以同步状态和时间。

1.  **Service 启动**: 打开串口，进入等待状态。
2.  **下位机发送 Ping**: 发送 `AcknowledgementData` 消息。
3.  **Service 回复 Pong**: 收到合法 Ping 后，Service 获取当前 Unix 时间戳，封装在 `AcknowledgementConfig` 消息中回复。
4.  **连接建立**: 双方进入主循环，开始全双工数据传输。

## 3. WebSocket 接口文档

*   **Endpoint**: `ws://localhost:3000/ws`
*   **数据格式**: JSON
*   **序列化规则**: 遵循 Rust `serde` 的 Enum 外部标签格式，即 `{ "VariantName": { ...fields... } }`。

### 3.1 下行链路 (Downlink): 遥测数据
Service 会将收到的串口数据广播给所有连接的 WebSocket 客户端。

#### 1. IMU 姿态数据 (`ImuData`)
频率: ~20Hz
```json
{
  "ImuData": {
    "accel_z": 9.81,
    "gyro_x": 0.01,
    "gyro_y": -0.02,
    "quad_w": 1.0,
    "quad_i": 0.0,
    "quad_j": 0.0,
    "quad_k": 0.0,
    "time_elapsed_ms": 123456
  }
}
```
*   前端用途: 更新 3D 姿态球或飞机模型。

#### 2. 空速数据 (`PitotAirspeedData`)
频率: ~10Hz
```json
{
  "PitotAirspeedData": {
    "splitter_left": 12.5,
    "splitter_right": 13.0,
    "static_port": 101325.0,
    "time_elapsed_ms": 123456
  }
}
```
*   前端用途: 显示实时空速 (m/s 或 km/h)。

#### 3. 电池状态 (`BatteryData`)
频率: ~1Hz
```json
{
  "BatteryData": {
    "voltage_v": 12.4,
    "soc_percent": 85.0,
    "time_elapsed_ms": 123456
  }
}
```
*   前端用途: 顶部状态栏显示电量和电压图标。

#### 4. 激光雷达测距 (`LidarData`)
频率: ~5Hz
```json
{
  "LidarData": {
    "distance_cm": 150,
    "signal_strength": 100,
    "time_elapsed_ms": 123456
  }
}
```
*   前端用途: 显示对地高度或避障距离。

#### 5. 环境数据 (`BarometerData`)
频率: ~1Hz
```json
{
  "BarometerData": {
    "pressure_pa": 101300.0,
    "temperature_c": 25.5,
    "humidity_percent": 45.0,
    "time_elapsed_ms": 123456
  }
}
```
*   前端用途: 显示环境仪表盘。

#### 6. 声学特征 (`AcousticData`)
频率: ~20Hz
```json
{
  "AcousticData": {
    "overall_spl": 80.5,
    "peak_frequency": 1200.0,
    "peak_magnitude": 0.8,
    "spectral_shape": [0.1, 0.2, ...], // 16 bins
    "turbulence_index": 0.45,
    "time_elapsed_ms": 123456
  }
}
```
*   前端用途: 绘制实时频谱图 (Spectrum Analyzer) 和噪声等级仪表。

#### 7. 振动监测 (`ImuVibrationMetrics`)
频率: ~1Hz
```json
{
  "ImuVibrationMetrics": {
    "accel_z": { "rms_vibration": 0.5, "dominant_frequency_hz": 50.0, "peak_magnitude": 1.2, "time_elapsed_ms": ... },
    "gyro_x": { ... },
    "gyro_y": { ... }
  }
}
```
*   前端用途: 显示机身震动健康度。

### 3.2 上行链路 (Uplink): 控制指令
前端通过 WebSocket 发送 JSON 字符串控制设备。

#### 1. 设置油门/空速 (`ThrottleConfig`)
```json
{
  "ThrottleConfig": {
    "airspeed": 128 // 0-255 映射到 PWM 或 目标速度
  }
}
```

#### 2. 设置舵机角度 (`ServoConfig`)
```json
{
  "ServoConfig": {
    "angle": 90 // 0-180 度
  }
}
```

#### 3. 系统命令 (`Command`)
*   **启动**:
    ```json
    { "Command": "Start" }
    ```
*   **停止**:
    ```json
    { "Command": "Stop" }
    ```
*   **校准**:
    ```json
    { "Command": "Calibrate" }
    ```

#### 4. 传感器配置 (`SensorConfig`)
```json
{
  "SensorConfig": {
    "imu_horizontal": true
  }
}
```

---

## 4. 前端页面功能规划与期待

基于上述数据流，前端应用 (`panel`) 的各页面应包含以下核心功能模块：

### 4.1 仪表盘 (Dashboard)
**定位**: 全局状态监视，核心飞行数据的实时展示。

*   **飞行姿态仪 (PFD)**:
    *   利用 `ImuData` (Quaternions) 渲染 3D 姿态球 (Artificial Horizon)。
    *   显示俯仰角 (Pitch) 和滚转角 (Roll)。
*   **核心指标栏**:
    *   **空速**: 仪表盘样式展示 `PitotAirspeedData`。
    *   **高度/距离**: 数字展示 `LidarData`。
    *   **环境**: 小卡片展示 `BarometerData` (温湿度、气压)。
*   **声学频谱分析**:
    *   使用 ECharts 或 Canvas 绘制 `AcousticData.spectral_shape` 的实时柱状图或折线图。
    *   显示当前 SPL (分贝) 和湍流指数。

### 4.2 控制台 (Control)
**定位**: 主动控制飞机的执行机构和运行状态。

*   **系统状态控制**:
    *   大尺寸按钮组: [启动 (Start)] [停止 (Stop)] [传感器校准 (Calibrate)]。
    *   状态指示灯: 对应下位机反馈的状态（需下位机配合回传状态，目前可通过心跳或特定回传判断）。
*   **油门推杆**:
    *   垂直滑块 (Slider) 控制 `ThrottleConfig`。
    *   实时显示当前设定值 (0-100%)。
*   **舵机测试**:
    *   旋钮或水平滑块控制 `ServoConfig` (0-180°)。
    *   提供“归中 (90°)”快捷按钮。
*   **旋钮编码器反馈**:
    *   显示 `QeiData` 的计数和按键状态，可用于验证物理旋钮输入。

### 4.3 设置 (Settings)
**定位**: 系统级配置与调试信息。

*   **连接配置**:
    *   显示当前连接的串口号、波特率（从 Service 状态获取，或仅做静态展示）。
    *   WebSocket 连接状态指示 (Connected/Disconnected)。
*   **传感器配置**:
    *   开关组件 (Switch) 控制 `SensorConfig.imu_horizontal` (水平/垂直安装校正)。
*   **原始数据监控 (Debug Console)**:
    *   滚动文本框显示收到的原始 JSON 消息流，方便开发调试。
    *   提供“暂停滚动”和“清屏”功能。
*   **阈值报警设置** (前端逻辑):
    *   设置电压低报警阈值。
    *   设置过载 (G-force) 报警阈值。

### 4.4 全局布局 (Layout)
*   **顶部栏 (Status Bar)**:
    *   始终显示: WebSocket 连接状态图标。
    *   始终显示: `BatteryData` (电压/电量)。
    *   始终显示: 系统时间。
*   **侧边栏 (Sidebar)**:
    *   导航菜单: Dashboard / Control / Settings。
