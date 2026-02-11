import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'controls_provider.dart';

class ControlsPage extends ConsumerWidget {
  const ControlsPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(controlsProvider);
    final notifier = ref.read(controlsProvider.notifier);

    return Scaffold(
      appBar: AppBar(
        title: const Text('控制面板', style: TextStyle(fontWeight: FontWeight.bold)),
        centerTitle: false,
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            _buildThrottleControl(state, notifier),
            const SizedBox(height: 16),
            _buildAngleControl(state, notifier),
            const SizedBox(height: 16),
            _buildCommandPanel(state, notifier),
            const SizedBox(height: 16),
            _buildSensorSwitches(state, notifier),
          ],
        ),
      ),
    );
  }

  Widget _buildCard({required Widget child}) {
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        color: const Color(0xFFF9F9F9),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.black.withValues(alpha: 0.05)),
      ),
      child: child,
    );
  }

  Widget _buildThrottleControl(ControlsState state, ControlsNotifier notifier) {
    return _buildCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text('油门控制', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                decoration: BoxDecoration(
                  color: Colors.grey[600],
                  borderRadius: BorderRadius.circular(4),
                ),
                child: Text('${state.throttle.toInt()}%', style: const TextStyle(color: Colors.white, fontSize: 12)),
              ),
            ],
          ),
          const SizedBox(height: 24),
          Row(
            children: [
              IconButton(onPressed: () => notifier.setThrottle((state.throttle - 1).clamp(0, 100)), icon: const Icon(Icons.remove)),
              Expanded(
                child: SliderTheme(
                  data: SliderThemeData(
                    activeTrackColor: Colors.black54,
                    inactiveTrackColor: Colors.grey[300],
                    thumbColor: Colors.black,
                    overlayColor: Colors.black.withValues(alpha: 0.1),
                    trackHeight: 4.0,
                    thumbShape: const RoundSliderThumbShape(enabledThumbRadius: 10.0),
                  ),
                  child: Slider(
                    value: state.throttle,
                    min: 0,
                    max: 100,
                    onChanged: (value) => notifier.setThrottle(value),
                  ),
                ),
              ),
              IconButton(onPressed: () => notifier.setThrottle((state.throttle + 1).clamp(0, 100)), icon: const Icon(Icons.add)),
            ],
          ),
          const SizedBox(height: 16),
          Center(
            child: Column(
              children: [
                Text('${state.throttle.toInt()}%', style: const TextStyle(fontSize: 32, fontWeight: FontWeight.bold)),
                const Text('当前风速: 0.00 m/s', style: TextStyle(color: Colors.grey)),
              ],
            ),
          ),
          const SizedBox(height: 24),
          const Text('预设档位', style: TextStyle(fontSize: 12, color: Colors.grey)),
          const SizedBox(height: 8),
          Wrap(
            spacing: 8,
            children: [0, 25, 50, 75, 100].map((val) {
              final isSelected = state.throttle.toInt() == val;
              return SizedBox(
                width: 60,
                child: OutlinedButton(
                  onPressed: () => notifier.setThrottle(val.toDouble()),
                  style: OutlinedButton.styleFrom(
                    backgroundColor: isSelected ? Colors.black : Colors.transparent,
                    foregroundColor: isSelected ? Colors.white : Colors.black,
                    padding: EdgeInsets.zero,
                    side: BorderSide(color: Colors.black.withValues(alpha: 0.2)),
                  ),
                  child: Text('$val%'),
                ),
              );
            }).toList(),
          ),
        ],
      ),
    );
  }

  Widget _buildAngleControl(ControlsState state, ControlsNotifier notifier) {
    return _buildCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text('舵机/攻角控制', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                decoration: BoxDecoration(
                  color: Colors.grey[600],
                  borderRadius: BorderRadius.circular(4),
                ),
                child: Text('${state.angle.toInt()}°', style: const TextStyle(color: Colors.white, fontSize: 12)),
              ),
            ],
          ),
          const SizedBox(height: 24),
          Row(
            children: [
              IconButton(onPressed: () => notifier.setAngle((state.angle - 1).clamp(-45, 45)), icon: const Icon(Icons.remove)),
              Expanded(
                child: SliderTheme(
                  data: SliderThemeData(
                    activeTrackColor: Colors.black54,
                    inactiveTrackColor: Colors.grey[300],
                    thumbColor: Colors.black,
                    overlayColor: Colors.black.withValues(alpha: 0.1),
                    trackHeight: 4.0,
                    thumbShape: const RoundSliderThumbShape(enabledThumbRadius: 10.0),
                  ),
                  child: Slider(
                    value: state.angle,
                    min: -45,
                    max: 45,
                    onChanged: (value) => notifier.setAngle(value),
                  ),
                ),
              ),
              IconButton(onPressed: () => notifier.setAngle((state.angle + 1).clamp(-45, 45)), icon: const Icon(Icons.add)),
            ],
          ),
          const SizedBox(height: 16),
          Center(
            child: Column(
              children: [
                Text('${state.angle.toStringAsFixed(1)}°', style: const TextStyle(fontSize: 32, fontWeight: FontWeight.bold)),
                const Text('攻角', style: TextStyle(color: Colors.grey)),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildCommandPanel(ControlsState state, ControlsNotifier notifier) {
    return _buildCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('指令面板', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(
                child: ElevatedButton.icon(
                  onPressed: state.isRunning ? null : notifier.startSystem,
                  icon: const Icon(Icons.play_arrow),
                  label: const Text('启动'),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFF1B5E20), // Green
                    foregroundColor: Colors.white,
                  ),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: ElevatedButton.icon(
                  onPressed: !state.isRunning ? null : notifier.stopSystem,
                  icon: const Icon(Icons.stop),
                  label: const Text('停止'),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Colors.grey[300],
                    foregroundColor: Colors.grey[600],
                    elevation: 0,
                  ),
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(
                child: OutlinedButton.icon(
                  onPressed: notifier.calibrateSensors,
                  icon: const Icon(Icons.tune),
                  label: const Text('校准'),
                  style: OutlinedButton.styleFrom(
                    foregroundColor: Colors.black,
                    side: BorderSide(color: Colors.black.withValues(alpha: 0.2)),
                  ),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: OutlinedButton(
                  onPressed: notifier.resetDefaults,
                  style: OutlinedButton.styleFrom(
                    foregroundColor: Colors.black,
                    side: BorderSide(color: Colors.black.withValues(alpha: 0.2)),
                  ),
                  child: const Text('清零'),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildSensorSwitches(ControlsState state, ControlsNotifier notifier) {
    return _buildCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('传感器开关', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
          const SizedBox(height: 16),
          _buildSwitch('IMU (惯性测量单元)', state.isImuEnabled, notifier.toggleImu),
          _buildSwitch('Acoustic (声学传感器)', state.isAcousticEnabled, notifier.toggleAcoustic),
          _buildSwitch('Lidar (激光雷达)', state.isLidarEnabled, notifier.toggleLidar),
          _buildSwitch('Pressure (压力传感器)', state.isPressureEnabled, notifier.togglePressure),
        ],
      ),
    );
  }

  Widget _buildSwitch(String label, bool value, Function(bool) onChanged) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: Row(
        children: [
          Switch(
            value: value,
            onChanged: onChanged,
            activeThumbColor: Colors.black,
            activeTrackColor: Colors.black,
            inactiveThumbColor: Colors.grey,
            inactiveTrackColor: Colors.grey[300],
          ),
          const SizedBox(width: 12),
          Text(label, style: const TextStyle(fontSize: 14)),
        ],
      ),
    );
  }
}
