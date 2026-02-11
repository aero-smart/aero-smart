import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:fl_chart/fl_chart.dart';

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

  final double pitch;
  final double roll;
  final double yaw;
  final double vibrationRms;
  final String vibrationStatus;
  final double vibrationFreq;

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
    this.vibrationFreq = 20.3,
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
    );
  }
}

class MonitorNotifier extends StateNotifier<MonitorState> {
  MonitorNotifier() : super(MonitorState()) {
    // Initialize with some dummy data for the chart
    state = state.copyWith(
      chartData: List.generate(100, (index) => FlSpot(index.toDouble(), 0)),
    );
  }

  void setDataSource(DataSource source) {
    state = state.copyWith(selectedDataSource: source);
    // TODO: Clear or switch chart data based on source
  }

  void updateData({
    double? airspeed,
    double? pressure,
    double? vibration,
    double? temperature,
  }) {
    state = state.copyWith(
      currentAirspeed: airspeed,
      currentPressure: pressure,
      currentVibration: vibration,
      currentTemperature: temperature,
    );
  }

  // Method to append new data point to chart
  void addDataPoint(double value) {
    final List<FlSpot> currentData = List.from(state.chartData);
    if (currentData.length >= 100) {
      currentData.removeAt(0);
    }
    // Shift existing x values? Or just append?
    // For simplicity, let's just regenerate based on index for now,
    // or keep a rolling window where x is always 0..99
    
    // Better approach for rolling chart:
    // Remove first, add new at end, re-index all x
    currentData.add(FlSpot(99, value));
    
    // Re-index to keep x from 0 to 99
    // final List<FlSpot> reIndexedData = [];
    // for (int i = 0; i < currentData.length; i++) {
    //     // We actually want the last 100 points. 
    //     // If we just removed one, we have 99. If we added one, we have 100.
    //     // But wait, the list is effectively a queue.
    //     // Let's just store the values and map to spots on render?
    //     // For now, let's just assume we append and shift.
    // }
    // Actually, simpler implementation for now:
    // Just replace the whole list with dummy data update for demo
  }
}

final monitorProvider = StateNotifierProvider<MonitorNotifier, MonitorState>((ref) {
  return MonitorNotifier();
});
