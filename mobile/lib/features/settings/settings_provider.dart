import 'package:flutter_riverpod/flutter_riverpod.dart';

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
  SettingsNotifier() : super(SettingsState());

  void setIpAddress(String ip) {
    state = state.copyWith(ipAddress: ip);
  }

  void connect() {
    // Simulate connection
    state = state.copyWith(isConnected: true);
  }

  void disconnect() {
    state = state.copyWith(isConnected: false);
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
  return SettingsNotifier();
});
