import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dashboard_provider.dart';

class DashboardPage extends ConsumerWidget {
  const DashboardPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(dashboardProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('AeroSmart', style: TextStyle(fontWeight: FontWeight.bold)),
        centerTitle: false,
        actions: [
          const Icon(Icons.wifi, color: Colors.green),
          const SizedBox(width: 16),
          Row(
            children: [
              const Icon(Icons.battery_std, size: 20),
              const SizedBox(width: 4),
              const Text('85%', style: TextStyle(fontWeight: FontWeight.bold)),
            ],
          ),
          const SizedBox(width: 16),
        ],
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _buildSystemStatusCard(context, state),
            const SizedBox(height: 24),
            const Text('关键指标', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
            const SizedBox(height: 12),
            _buildMetricsGrid(context, state),
            const SizedBox(height: 24),
            const Text('姿态预览', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
            const SizedBox(height: 12),
            _buildAttitudePreview(context, state),
            const SizedBox(height: 24),
            _buildEmergencyStopButton(context),
          ],
        ),
      ),
    );
  }

  Widget _buildSystemStatusCard(BuildContext context, DashboardState state) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        color: isDark ? const Color(0xFF1E1E1E) : const Color(0xFFF5F5F5), // Light gray background
        borderRadius: BorderRadius.circular(16),
      ),
      child: Column(
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text('系统状态', style: TextStyle(fontSize: 16, fontWeight: FontWeight.w500)),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                decoration: BoxDecoration(
                  color: Theme.of(context).dividerColor,
                  borderRadius: BorderRadius.circular(20),
                ),
                child: Text(state.status, style: const TextStyle(fontSize: 12, fontWeight: FontWeight.bold)),
              ),
            ],
          ),
          const SizedBox(height: 32),
          Text(
            state.airspeed.toStringAsFixed(2),
            style: const TextStyle(fontSize: 48, fontWeight: FontWeight.bold, fontFamily: 'Monospace'),
          ),
          Text('m/s', style: TextStyle(fontSize: 16, color: Theme.of(context).textTheme.bodySmall?.color)),
          const SizedBox(height: 16),
          Text('差压: ${state.diffPressure.toStringAsFixed(2)} Pa', style: TextStyle(color: Theme.of(context).textTheme.bodySmall?.color)),
          const SizedBox(height: 16),
        ],
      ),
    );
  }

  Widget _buildMetricsGrid(BuildContext context, DashboardState state) {
    return LayoutBuilder(
      builder: (context, constraints) {
        final double itemWidth = (constraints.maxWidth - 24) / 3; // 24 = 12 gap * 2
        return Wrap(
          spacing: 12,
          runSpacing: 12,
          children: [
            _buildMetricItem(context, '温度', '${state.temperature}°C', Icons.thermostat, width: itemWidth),
            _buildMetricItem(context, '湿度', '${state.humidity}%', Icons.water_drop, width: itemWidth),
            _buildMetricItem(context, '大气压', '${state.pressure.toInt()} hPa', Icons.speed, width: itemWidth),
            _buildMetricItem(context, 'Lidar距离', '${state.lidarDistance} cm', Icons.straighten, width: itemWidth),
            _buildMetricItem(context, '电压', '${state.voltage} V', Icons.flash_on, width: itemWidth),
            _buildMetricItem(context, '电流', '${state.current.toStringAsFixed(2)} A', Icons.electric_bolt, width: itemWidth),
          ],
        );
      },
    );
  }

  Widget _buildMetricItem(BuildContext context, String label, String value, IconData icon, {required double width}) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    return Container(
      width: width,
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: isDark ? const Color(0xFF1E1E1E) : const Color(0xFFF9F9F9),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Theme.of(context).dividerColor.withOpacity(0.1)),
      ),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(icon, size: 24, color: Theme.of(context).colorScheme.onSurface),
          const SizedBox(height: 12),
          Text(
            value,
            style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 4),
          Text(
            label,
            style: TextStyle(fontSize: 12, color: Theme.of(context).textTheme.bodySmall?.color),
            textAlign: TextAlign.center,
            overflow: TextOverflow.ellipsis,
          ),
        ],
      ),
    );
  }

  Widget _buildAttitudePreview(BuildContext context, DashboardState state) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: isDark ? const Color(0xFF1E1E1E) : const Color(0xFFF9F9F9),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: Theme.of(context).dividerColor.withOpacity(0.1)),
      ),
      child: Column(
        children: [
          // Placeholder for 3D View
          Container(
            height: 180,
            width: double.infinity,
            alignment: Alignment.center,
            decoration: BoxDecoration(
              color: Theme.of(context).colorScheme.surface,
              borderRadius: BorderRadius.circular(8),
            ),
            child: Stack(
              alignment: Alignment.center,
              children: [
                 // Simple 2D representation for now
                 Transform(
                   transform: Matrix4.identity()
                     ..setEntry(3, 2, 0.001)
                     ..rotateX(state.pitch * 3.14159 / 180)
                     ..rotateY(state.yaw * 3.14159 / 180)
                     ..rotateZ(state.roll * 3.14159 / 180),
                   alignment: Alignment.center,
                   child: Container(
                     width: 80,
                     height: 10,
                     color: Theme.of(context).colorScheme.onSurface,
                   ),
                 ),
                 Transform(
                   transform: Matrix4.identity()
                     ..setEntry(3, 2, 0.001)
                     ..rotateX(state.pitch * 3.14159 / 180)
                     ..rotateY(state.yaw * 3.14159 / 180)
                     ..rotateZ(state.roll * 3.14159 / 180),
                   alignment: Alignment.center,
                   child: Container(
                     width: 10,
                     height: 80,
                     color: Theme.of(context).colorScheme.onSurface,
                   ),
                 ),
                 Positioned(
                   right: 8,
                   top: 50, // Roughly centered vertically
                   child: Icon(Icons.refresh, size: 16, color: Theme.of(context).textTheme.bodySmall?.color),
                 ),
              ],
            ),
          ),
          const SizedBox(height: 16),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              _buildAttitudeValue(context, 'Pitch', state.pitch),
              _buildAttitudeValue(context, 'Roll', state.roll),
              _buildAttitudeValue(context, 'Yaw', state.yaw),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildAttitudeValue(BuildContext context, String label, double value) {
    return Column(
      children: [
        Text(label, style: TextStyle(fontSize: 12, color: Theme.of(context).textTheme.bodySmall?.color)),
        const SizedBox(height: 4),
        Text('${value.toStringAsFixed(1)}°', style: const TextStyle(fontWeight: FontWeight.bold)),
      ],
    );
  }

  Widget _buildEmergencyStopButton(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      height: 56,
      child: ElevatedButton(
        onPressed: () {
          // TODO: Implement emergency stop logic
        },
        style: ElevatedButton.styleFrom(
          backgroundColor: Theme.of(context).colorScheme.error,
          foregroundColor: Theme.of(context).colorScheme.onError,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(8),
          ),
        ),
        child: const Text('紧急停止', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
      ),
    );
  }
}
