import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/services/websocket_service.dart';

enum ImuOrientation { horizontal, vertical, upsideDown }
enum SamplingRate { hz50, hz100, hz200, hz400 }
enum AppThemeMode { system, light, dark }
enum UnitSystem { metric, imperial }

class SettingsState {
  final String ipAddress;
  final bool isConnected;
  final ImuOrientation imuOrientation;
  final double pitotCoeff;
  final SamplingRate samplingRate;
  final String firmwareVersion;
  final AppThemeMode themeMode;
  final UnitSystem unitSystem;
  final String appVersion;

  SettingsState({
    this.ipAddress = '192.168.1.100',
    this.isConnected = false,
    this.imuOrientation = ImuOrientation.horizontal,
    this.pitotCoeff = 1.00,
    this.samplingRate = SamplingRate.hz100,
    this.firmwareVersion = 'v2.3.1',
    this.themeMode = AppThemeMode.system,
    this.unitSystem = UnitSystem.metric,
    this.appVersion = 'v1.0.0',
  });

  SettingsState copyWith({
    String? ipAddress,
    bool? isConnected,
    ImuOrientation? imuOrientation,
    double? pitotCoeff,
    SamplingRate? samplingRate,
    String? firmwareVersion,
    AppThemeMode? themeMode,
    UnitSystem? unitSystem,
    String? appVersion,
  }) {
    return SettingsState(
      ipAddress: ipAddress ?? this.ipAddress,
      isConnected: isConnected ?? this.isConnected,
      imuOrientation: imuOrientation ?? this.imuOrientation,
      pitotCoeff: pitotCoeff ?? this.pitotCoeff,
      samplingRate: samplingRate ?? this.samplingRate,
      firmwareVersion: firmwareVersion ?? this.firmwareVersion,
      themeMode: themeMode ?? this.themeMode,
      unitSystem: unitSystem ?? this.unitSystem,
      appVersion: appVersion ?? this.appVersion,
    );
  }
}

class SettingsNotifier extends StateNotifier<SettingsState> {
  final ConnectionService _connectionService;

  SettingsNotifier(this._connectionService) : super(SettingsState()) {
    _init();
  }

  void _init() {
    // 监听连接服务的状态变化
    // 注意：这里无法直接监听 ConnectionService 的状态流，只能通过方法同步
    // 或者让 SettingsNotifier 依赖 ref 并 watch connectionServiceProvider
    // 为了简单起见，我们在 setIpAddress 中同步调用 ConnectionService
    
    // 初始化时同步当前 IP
    if (_connectionService.currentIp != null) {
      state = state.copyWith(ipAddress: _connectionService.currentIp);
    }
  }
  
  // 更新连接状态（由 ConnectionService 的状态变化驱动会更好，但这里先提供手动方法）
  void updateConnectionStatus(bool isConnected) {
    state = state.copyWith(isConnected: isConnected);
  }

  void setIpAddress(String ip) {
    state = state.copyWith(ipAddress: ip);
    _connectionService.connect(ip);
  }

  void connect() {
    _connectionService.connect(state.ipAddress);
  }

  void disconnect() {
    _connectionService.disconnect();
  }

  void setImuOrientation(ImuOrientation orientation) {
    state = state.copyWith(imuOrientation: orientation);
  }

  void setPitotCoeff(double coeff) {
    state = state.copyWith(pitotCoeff: coeff);
  }

  void setSamplingRate(SamplingRate rate) {
    state = state.copyWith(samplingRate: rate);
  }

  void setThemeMode(AppThemeMode mode) {
    state = state.copyWith(themeMode: mode);
  }

  void setUnitSystem(UnitSystem system) {
    state = state.copyWith(unitSystem: system);
  }

  void checkFirmwareUpdate() {
    // Simulate update check
  }
}

final settingsProvider = StateNotifierProvider<SettingsNotifier, SettingsState>((ref) {
  final connectionService = ref.watch(connectionServiceProvider.notifier);
  final connectionStatus = ref.watch(connectionServiceProvider);
  
  final notifier = SettingsNotifier(connectionService);
  
  // 同步连接状态到 UI
  // 这里用 scheduleMicrotask 避免构建期间 setState
  // 但更优雅的方式是在 StateNotifier 中监听，或者直接在 build 中组合状态
  // 这里我们仅同步 IP，状态由 UI 直接读取 connectionServiceProvider 即可
  
  return notifier;
});
