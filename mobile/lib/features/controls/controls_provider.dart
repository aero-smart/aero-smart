import 'package:flutter_riverpod/flutter_riverpod.dart';

class ControlsState {
  final double throttle;
  final double angle;
  final bool isImuEnabled;
  final bool isAcousticEnabled;
  final bool isLidarEnabled;
  final bool isPressureEnabled;
  final bool isRunning;

  ControlsState({
    this.throttle = 0.0,
    this.angle = 0.0,
    this.isImuEnabled = true,
    this.isAcousticEnabled = false,
    this.isLidarEnabled = true,
    this.isPressureEnabled = true,
    this.isRunning = false,
  });

  ControlsState copyWith({
    double? throttle,
    double? angle,
    bool? isImuEnabled,
    bool? isAcousticEnabled,
    bool? isLidarEnabled,
    bool? isPressureEnabled,
    bool? isRunning,
  }) {
    return ControlsState(
      throttle: throttle ?? this.throttle,
      angle: angle ?? this.angle,
      isImuEnabled: isImuEnabled ?? this.isImuEnabled,
      isAcousticEnabled: isAcousticEnabled ?? this.isAcousticEnabled,
      isLidarEnabled: isLidarEnabled ?? this.isLidarEnabled,
      isPressureEnabled: isPressureEnabled ?? this.isPressureEnabled,
      isRunning: isRunning ?? this.isRunning,
    );
  }
}

class ControlsNotifier extends StateNotifier<ControlsState> {
  ControlsNotifier() : super(ControlsState());

  void setThrottle(double value) {
    state = state.copyWith(throttle: value);
  }

  void setAngle(double value) {
    state = state.copyWith(angle: value);
  }

  void toggleImu(bool value) {
    state = state.copyWith(isImuEnabled: value);
  }

  void toggleAcoustic(bool value) {
    state = state.copyWith(isAcousticEnabled: value);
  }

  void toggleLidar(bool value) {
    state = state.copyWith(isLidarEnabled: value);
  }

  void togglePressure(bool value) {
    state = state.copyWith(isPressureEnabled: value);
  }

  void startSystem() {
    state = state.copyWith(isRunning: true);
  }

  void stopSystem() {
    state = state.copyWith(isRunning: false, throttle: 0.0);
  }

  void calibrateSensors() {
    // Implement calibration logic
  }

  void resetDefaults() {
    state = ControlsState();
  }
}

final controlsProvider = StateNotifierProvider<ControlsNotifier, ControlsState>((ref) {
  return ControlsNotifier();
});
