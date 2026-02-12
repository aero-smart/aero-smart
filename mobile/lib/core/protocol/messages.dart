import '../services/websocket_service.dart';

// --- 控制指令 (Service -> Firmware) ---

// 命令枚举
enum CommandType {
  start,
  stop,
  calibrate,
}

class CommandMessage implements AeroMessage {
  final CommandType type;

  CommandMessage(this.type);

  @override
  Map<String, dynamic> toJson() {
    return {
      'type': 'Command',
      'payload': {
        'action': type.name.replaceFirst(type.name[0], type.name[0].toUpperCase()), // Start, Stop, Calibrate
      },
    };
  }
}

class ThrottleConfig implements AeroMessage {
  final int airspeed; // 0-255

  ThrottleConfig(this.airspeed);

  @override
  Map<String, dynamic> toJson() {
    return {
      'type': 'ThrottleConfig',
      'payload': {'airspeed': airspeed},
    };
  }
}

class ServoConfig implements AeroMessage {
  final int angle; // 0-255

  ServoConfig(this.angle);

  @override
  Map<String, dynamic> toJson() {
    return {
      'type': 'ServoConfig',
      'payload': {'angle': angle},
    };
  }
}

class SensorConfig implements AeroMessage {
  final bool imuHorizontal;

  SensorConfig({required this.imuHorizontal});

  @override
  Map<String, dynamic> toJson() {
    return {
      'type': 'SensorConfig',
      'payload': {'imu_horizontal': imuHorizontal},
    };
  }
}

// --- 遥测数据 (Firmware -> Service) ---

class ImuData {
  final double accelZ;
  final double gyroX;
  final double gyroY;
  final List<double> quaternion; // w, i, j, k

  ImuData({
    required this.accelZ,
    required this.gyroX,
    required this.gyroY,
    required this.quaternion,
  });

  factory ImuData.fromJson(Map<String, dynamic> json) {
    return ImuData(
      accelZ: (json['accel_z'] as num).toDouble(),
      gyroX: (json['gyro_x'] as num).toDouble(),
      gyroY: (json['gyro_y'] as num).toDouble(),
      quaternion: [
        (json['quad_w'] as num).toDouble(),
        (json['quad_i'] as num).toDouble(),
        (json['quad_j'] as num).toDouble(),
        (json['quad_k'] as num).toDouble(),
      ],
    );
  }
}

class PitotAirspeedData {
  final double splitterLeft;
  final double splitterRight;
  final double staticPort;

  PitotAirspeedData({
    required this.splitterLeft,
    required this.splitterRight,
    required this.staticPort,
  });

  factory PitotAirspeedData.fromJson(Map<String, dynamic> json) {
    return PitotAirspeedData(
      splitterLeft: (json['splitter_left'] as num).toDouble(),
      splitterRight: (json['splitter_right'] as num).toDouble(),
      staticPort: (json['static_port'] as num).toDouble(),
    );
  }
}

class BatteryData {
  final double voltageV;
  final double socPercent;

  BatteryData({required this.voltageV, required this.socPercent});

  factory BatteryData.fromJson(Map<String, dynamic> json) {
    return BatteryData(
      voltageV: (json['voltage_v'] as num).toDouble(),
      socPercent: (json['soc_percent'] as num).toDouble(),
    );
  }
}

class BarometerData {
  final double pressurePa;
  final double temperatureC;
  final double humidityPercent;

  BarometerData({
    required this.pressurePa,
    required this.temperatureC,
    required this.humidityPercent,
  });

  factory BarometerData.fromJson(Map<String, dynamic> json) {
    return BarometerData(
      pressurePa: (json['pressure_pa'] as num).toDouble(),
      temperatureC: (json['temperature_c'] as num).toDouble(),
      humidityPercent: (json['humidity_percent'] as num).toDouble(),
    );
  }
}
