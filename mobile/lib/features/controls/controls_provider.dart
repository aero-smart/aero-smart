import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/services/websocket_service.dart';
import '../../core/protocol/messages.dart';

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
  final WebSocketService _wsService;

  ControlsNotifier(this._wsService) : super(ControlsState());

  void setThrottle(double value) {
    state = state.copyWith(throttle: value);
    _wsService.send(ThrottleConfig((value * 255).toInt()));
  }

  void setAngle(double value) {
    state = state.copyWith(angle: value);
    _wsService.send(ServoConfig((value * 255).toInt()));
  }

  void toggleImu(bool value) {
    state = state.copyWith(isImuEnabled: value);
    // 假设下位机有对应开关指令，暂无协议定义，先保留本地状态
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
    _wsService.send(CommandMessage(CommandType.start));
  }

  void stopSystem() {
    state = state.copyWith(isRunning: false, throttle: 0.0);
    _wsService.send(CommandMessage(CommandType.stop));
    _wsService.send(ThrottleConfig(0));
  }

  void calibrateSensors() {
    _wsService.send(CommandMessage(CommandType.calibrate));
  }

  void resetDefaults() {
    state = ControlsState();
    // 恢复默认设置时也发送归零指令
    _wsService.send(ThrottleConfig(0));
    _wsService.send(ServoConfig(0));
  }
}

final controlsProvider = StateNotifierProvider<ControlsNotifier, ControlsState>((ref) {
  final connectionService = ref.watch(connectionServiceProvider.notifier);
  return ControlsNotifier(connectionService.ws);
});
