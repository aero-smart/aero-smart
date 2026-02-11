import 'package:flutter_riverpod/flutter_riverpod.dart';

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
    this.temperature = 22.1,
    this.humidity = 45.8,
    this.pressure = 1013,
    this.lidarDistance = 155.0,
    this.voltage = 12.4,
    this.current = 0.50,
    this.pitch = 0.0,
    this.roll = 0.0,
    this.yaw = 0.0,
    this.status = 'Idle',
  });

  DashboardState copyWith({
    double? airspeed,
    double? diffPressure,
    double? temperature,
    double? humidity,
    double? pressure,
    double? lidarDistance,
    double? voltage,
    double? current,
    double? pitch,
    double? roll,
    double? yaw,
    String? status,
  }) {
    return DashboardState(
      airspeed: airspeed ?? this.airspeed,
      diffPressure: diffPressure ?? this.diffPressure,
      temperature: temperature ?? this.temperature,
      humidity: humidity ?? this.humidity,
      pressure: pressure ?? this.pressure,
      lidarDistance: lidarDistance ?? this.lidarDistance,
      voltage: voltage ?? this.voltage,
      current: current ?? this.current,
      pitch: pitch ?? this.pitch,
      roll: roll ?? this.roll,
      yaw: yaw ?? this.yaw,
      status: status ?? this.status,
    );
  }
}

class DashboardNotifier extends StateNotifier<DashboardState> {
  DashboardNotifier() : super(DashboardState());

  void updateAirspeed(double value) {
    state = state.copyWith(airspeed: value);
  }

  // Add other update methods as needed
}

final dashboardProvider = StateNotifierProvider<DashboardNotifier, DashboardState>((ref) {
  return DashboardNotifier();
});
