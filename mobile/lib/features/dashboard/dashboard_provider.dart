import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../monitor/monitor_provider.dart';
import '../controls/controls_provider.dart';

// 直接复用 MonitorState 和 ControlsState 的数据来聚合 DashboardState
// Dashboard 不需要自己维护独立的状态源，它只是 Monitor 和 Controls 的一个视图聚合

final dashboardProvider = Provider<DashboardState>((ref) {
  final monitorState = ref.watch(monitorProvider);
  final controlsState = ref.watch(controlsProvider);

  return DashboardState(
    airspeed: monitorState.currentAirspeed,
    diffPressure: 0.0, // 暂无直接字段，可从 Airspeed 反推或加字段
    temperature: monitorState.currentTemperature,
    humidity: 0.0, // BarometerData 含 humidity 但 MonitorState 目前未暴露
    pressure: monitorState.currentPressure,
    lidarDistance: 0.0, // 需从 LidarData 获取
    voltage: monitorState.batteryVoltage,
    current: 0.0,
    pitch: monitorState.pitch,
    roll: monitorState.roll,
    yaw: monitorState.yaw,
    status: controlsState.isRunning ? 'Running' : 'Idle',
  );
});

class DashboardState {
  final double airspeed;
  final double diffPressure;
  final double temperature;
  final double humidity;
  final double pressure;
  final double lidarDistance;
  final double voltage;
  final double current;
  final double pitch;
  final double roll;
  final double yaw;
  final String status;

  DashboardState({
    this.airspeed = 0.00,
    this.diffPressure = 0.00,
    this.temperature = 0.0,
    this.humidity = 0.0,
    this.pressure = 0.0,
    this.lidarDistance = 0.0,
    this.voltage = 0.0,
    this.current = 0.00,
    this.pitch = 0.0,
    this.roll = 0.0,
    this.yaw = 0.0,
    this.status = 'Idle',
  });
}
