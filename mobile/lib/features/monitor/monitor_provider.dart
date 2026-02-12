import 'dart:convert';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:fl_chart/fl_chart.dart';
import 'dart:math' as math;
import '../../core/services/websocket_service.dart';
import '../../core/protocol/messages.dart';

enum DataSource {
  airspeed,
  pressure,
  vibration,
  temperature,
}

class MonitorState {
  final DataSource selectedDataSource;
  final List<FlSpot> chartData;
  final double currentAirspeed;
  final double currentPressure;
  final double currentVibration;
  final double currentTemperature;
  final double currentSpl; // Sound Pressure Level (dB)
  final List<double> fftData; // 16 frequency bands
  
  // IMU Data
  final double pitch;
  final double roll;
  final double yaw;
  
  // Vibration Analysis
  final double vibrationRms;
  final String vibrationStatus;
  final double vibrationFreq;
  
  // Battery Data
  final double batteryVoltage;
  final double batterySoc;

  MonitorState({
    this.selectedDataSource = DataSource.airspeed,
    this.chartData = const [],
    this.currentAirspeed = 0.0,
    this.currentPressure = 0.0,
    this.currentVibration = 0.0,
    this.currentTemperature = 23.0,
    this.currentSpl = 40.0,
    this.fftData = const [
      0.2, 0.5, 0.8, 1.2, 1.8, 2.5, 3.2, 2.8,
      2.0, 1.5, 1.0, 0.8, 0.5, 0.3, 0.2, 0.1
    ],
    this.pitch = 0.0,
    this.roll = 0.0,
    this.yaw = 0.0,
    this.vibrationRms = 0.000,
    this.vibrationStatus = '正常',
    this.vibrationFreq = 0.0,
    this.batteryVoltage = 0.0,
    this.batterySoc = 0.0,
  });

  MonitorState copyWith({
    DataSource? selectedDataSource,
    List<FlSpot>? chartData,
    double? currentAirspeed,
    double? currentPressure,
    double? currentVibration,
    double? currentTemperature,
    double? currentSpl,
    List<double>? fftData,
    double? pitch,
    double? roll,
    double? yaw,
    double? vibrationRms,
    String? vibrationStatus,
    double? vibrationFreq,
    double? batteryVoltage,
    double? batterySoc,
  }) {
    return MonitorState(
      selectedDataSource: selectedDataSource ?? this.selectedDataSource,
      chartData: chartData ?? this.chartData,
      currentAirspeed: currentAirspeed ?? this.currentAirspeed,
      currentPressure: currentPressure ?? this.currentPressure,
      currentVibration: currentVibration ?? this.currentVibration,
      currentTemperature: currentTemperature ?? this.currentTemperature,
      currentSpl: currentSpl ?? this.currentSpl,
      fftData: fftData ?? this.fftData,
      pitch: pitch ?? this.pitch,
      roll: roll ?? this.roll,
      yaw: yaw ?? this.yaw,
      vibrationRms: vibrationRms ?? this.vibrationRms,
      vibrationStatus: vibrationStatus ?? this.vibrationStatus,
      vibrationFreq: vibrationFreq ?? this.vibrationFreq,
      batteryVoltage: batteryVoltage ?? this.batteryVoltage,
      batterySoc: batterySoc ?? this.batterySoc,
    );
  }
}

class MonitorNotifier extends StateNotifier<MonitorState> {
  final WebSocketService _wsService;
  
  MonitorNotifier(this._wsService) : super(MonitorState()) {
    // Initialize with empty chart data
    state = state.copyWith(
      chartData: List.generate(100, (index) => FlSpot(index.toDouble(), 0)),
    );
    _subscribeToData();
  }

  void _subscribeToData() {
    _wsService.stream.listen((message) {
      try {
        if (message is String) {
          final json = jsonDecode(message);
          _handleMessage(json);
        }
      } catch (e) {
        // print('Error parsing message: $e');
      }
    });
  }

  void _handleMessage(Map<String, dynamic> json) {
    // 假设消息格式为 { "type": "ImuData", "payload": { ... } }
    // 根据下位机通信文档，这里需要适配实际的 JSON 结构
    // 假设共享层将 SerialMessage 序列化为带 type 字段的结构
    
    final type = json['type'];
    final payload = json['payload'];

    if (payload == null) return;

    switch (type) {
      case 'ImuData':
        final data = ImuData.fromJson(payload);
        _updateImu(data);
        break;
      case 'PitotAirspeedData':
        final data = PitotAirspeedData.fromJson(payload);
        _updateAirspeed(data);
        break;
      case 'BarometerData':
        final data = BarometerData.fromJson(payload);
        _updateEnvironment(data);
        break;
      case 'BatteryData':
        final data = BatteryData.fromJson(payload);
        _updateBattery(data);
        break;
      case 'ImuVibrationMetrics':
        // Assuming payload structure matches what we need
        // double rms = payload['accel_z_rms'] ?? 0.0;
        // _updateVibration(rms);
        break;
    }
  }

  void _updateImu(ImuData data) {
    // Convert quaternion to Euler angles (simplified)
    // q0=w, q1=i, q2=j, q3=k
    final q0 = data.quaternion[0];
    final q1 = data.quaternion[1];
    final q2 = data.quaternion[2];
    final q3 = data.quaternion[3];

    final roll = math.atan2(2 * (q0 * q1 + q2 * q3), 1 - 2 * (q1 * q1 + q2 * q2));
    final pitch = math.asin(2 * (q0 * q2 - q3 * q1));
    final yaw = math.atan2(2 * (q0 * q3 + q1 * q2), 1 - 2 * (q2 * q2 + q3 * q3));

    state = state.copyWith(
      roll: roll * 180 / math.pi,
      pitch: pitch * 180 / math.pi,
      yaw: yaw * 180 / math.pi,
      currentVibration: data.accelZ.abs(), // 简单示例，实际振动需 RMS
    );
    
    if (state.selectedDataSource == DataSource.vibration) {
      _addChartPoint(data.accelZ);
    }
  }

  void _updateAirspeed(PitotAirspeedData data) {
    // 假设差压换算为空速的简单逻辑，实际需要更复杂的公式
    // V = sqrt(2 * (P_total - P_static) / rho)
    // 这里直接使用 diff 作为演示
    final diff = ((data.splitterLeft + data.splitterRight) / 2) - data.staticPort;
    final velocity = diff > 0 ? math.sqrt(2 * diff / 1.225) : 0.0;
    
    state = state.copyWith(currentAirspeed: velocity);
    
    if (state.selectedDataSource == DataSource.airspeed) {
      _addChartPoint(velocity);
    }
  }

  void _updateEnvironment(BarometerData data) {
    state = state.copyWith(
      currentPressure: data.pressurePa / 100.0, // Pa to hPa
      currentTemperature: data.temperatureC,
    );
    
    if (state.selectedDataSource == DataSource.pressure) {
      _addChartPoint(data.pressurePa / 100.0);
    } else if (state.selectedDataSource == DataSource.temperature) {
      _addChartPoint(data.temperatureC);
    }
  }
  
  void _updateBattery(BatteryData data) {
    state = state.copyWith(
      batteryVoltage: data.voltageV,
      batterySoc: data.socPercent,
    );
  }

  void setDataSource(DataSource source) {
    state = state.copyWith(selectedDataSource: source);
    // 重置图表数据
    state = state.copyWith(
      chartData: List.generate(100, (index) => FlSpot(index.toDouble(), 0)),
    );
  }

  void _addChartPoint(double value) {
    final List<FlSpot> currentData = List.from(state.chartData);
    
    // 移除第一个点
    if (currentData.isNotEmpty) {
      currentData.removeAt(0);
    }
    
    // 所有点的 x 坐标减 1
    for (int i = 0; i < currentData.length; i++) {
      currentData[i] = FlSpot(currentData[i].x - 1, currentData[i].y);
    }
    
    // 添加新点到末尾，x 坐标为 99
    currentData.add(FlSpot(99, value));
    
    // 重新修正 x 坐标为 0..99 (如果上面循环不够高效，可以直接重构)
    final List<FlSpot> normalizedData = [];
    for (int i = 0; i < currentData.length; i++) {
      normalizedData.add(FlSpot(i.toDouble(), currentData[i].y));
    }

    state = state.copyWith(chartData: normalizedData);
  }
}

final monitorProvider = StateNotifierProvider<MonitorNotifier, MonitorState>((ref) {
  final connectionService = ref.watch(connectionServiceProvider.notifier);
  return MonitorNotifier(connectionService.ws);
});
