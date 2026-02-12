import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'monitor_provider.dart';

class ImuTab extends ConsumerWidget {
  const ImuTab({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(monitorProvider);

    return SingleChildScrollView(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        children: [
          _buildAttitudeIndicator(context, state),
          const SizedBox(height: 16),
          _buildEulerAnglesCard(context, state),
          const SizedBox(height: 16),
          _buildVibrationCard(context, state),
        ],
      ),
    );
  }

  Widget _buildCard(BuildContext context, {required Widget child}) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: isDark ? const Color(0xFF1E1E1E) : const Color(0xFFF9F9F9),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Theme.of(context).dividerColor.withOpacity(0.1)),
      ),
      child: child,
    );
  }

  Widget _buildAttitudeIndicator(BuildContext context, MonitorState state) {
    return _buildCard(
      context,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('姿态指示器', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
          const SizedBox(height: 16),
          Container(
            height: 250,
            width: double.infinity,
            alignment: Alignment.center,
            decoration: BoxDecoration(
              color: Theme.of(context).colorScheme.surface,
              borderRadius: BorderRadius.circular(8),
              border: Border.all(color: Theme.of(context).dividerColor),
            ),
            child: Stack(
              alignment: Alignment.center,
              children: [
                // Simplified 3D representation using basic shapes and transforms
                // In a real app, this would use flutter_3d_controller or custom painter
                
                // Crosshair
                Container(width: 200, height: 1, color: Theme.of(context).dividerColor),
                Container(width: 1, height: 200, color: Theme.of(context).dividerColor),

                // Moving Object (Simplified Plane/Arrow)
                Transform(
                  alignment: Alignment.center,
                  transform: Matrix4.identity()
                    ..setEntry(3, 2, 0.001)
                    ..rotateX(state.pitch * 3.14159 / 180)
                    ..rotateY(state.yaw * 3.14159 / 180) // Yaw typically rotates around Z in 2D top-down, but here Y for 3D effect
                    ..rotateZ(state.roll * 3.14159 / 180),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Container(
                        width: 20,
                        height: 20,
                        decoration: const BoxDecoration(
                          color: Colors.red,
                          shape: BoxShape.circle,
                        ),
                      ),
                      Container(
                        width: 60,
                        height: 100,
                        decoration: BoxDecoration(
                          color: Theme.of(context).colorScheme.onSurface.withOpacity(0.87),
                          borderRadius: const BorderRadius.vertical(bottom: Radius.circular(4)),
                        ),
                        child: CustomPaint(
                          painter: TrianglePainter(),
                        ),
                      ),
                    ],
                  ),
                ),
                
                // Compass / Orientation Hint
                Positioned(
                  top: 16,
                  right: 16,
                  child: Container(
                    width: 40,
                    height: 40,
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      border: Border.all(color: Theme.of(context).colorScheme.onSurface),
                    ),
                    child: const Icon(Icons.navigation, size: 24),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildEulerAnglesCard(BuildContext context, MonitorState state) {
    return _buildCard(
      context,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('欧拉角', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
          const SizedBox(height: 16),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              _buildAngleItem(context, 'Pitch (俯仰)', state.pitch),
              _buildAngleItem(context, 'Roll (横滚)', state.roll),
              _buildAngleItem(context, 'Yaw (偏航)', state.yaw),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildAngleItem(BuildContext context, String label, double value) {
    return Column(
      children: [
        Text(label, style: TextStyle(fontSize: 12, color: Theme.of(context).textTheme.bodySmall?.color)),
        const SizedBox(height: 4),
        Text(
          '${value.toStringAsFixed(1)}°',
          style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Monospace'),
        ),
      ],
    );
  }

  Widget _buildVibrationCard(BuildContext context, MonitorState state) {
    return _buildCard(
      context,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: const [
              Icon(Icons.vibration, size: 20),
              SizedBox(width: 8),
              Text('震动分析', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
            ],
          ),
          const SizedBox(height: 24),
          Center(
            child: Text(
              state.vibrationRms.toStringAsFixed(3),
              style: const TextStyle(
                fontSize: 48,
                fontWeight: FontWeight.bold,
                fontFamily: 'Monospace',
              ),
            ),
          ),
          Center(child: Text('RMS 震动值', style: TextStyle(color: Theme.of(context).textTheme.bodySmall?.color))),
          const SizedBox(height: 24),
          // Status Bar
          ClipRRect(
            borderRadius: BorderRadius.circular(4),
            child: LinearProgressIndicator(
              value: 0.1, // Dummy safe value
              minHeight: 12,
              backgroundColor: Theme.of(context).brightness == Brightness.dark ? Colors.grey[800] : Colors.grey[300],
              color: const Color(0xFF4CAF50),
            ),
          ),
          const SizedBox(height: 16),
          Row(
            children: [
              Text.rich(
                TextSpan(
                  text: '状态: ',
                  style: const TextStyle(fontWeight: FontWeight.bold),
                  children: [
                    TextSpan(text: state.vibrationStatus, style: const TextStyle(fontWeight: FontWeight.normal)),
                  ],
                ),
              ),
            ],
          ),
          const SizedBox(height: 8),
          Row(
            children: [
              Text.rich(
                TextSpan(
                  text: '主频: ',
                  style: const TextStyle(fontWeight: FontWeight.bold),
                  children: [
                    TextSpan(text: '${state.vibrationFreq} Hz', style: const TextStyle(fontWeight: FontWeight.normal)),
                  ],
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

class TrianglePainter extends CustomPainter {
  @override
  void paint(Canvas canvas, Size size) {
    // Just a placeholder to make the black box look more like a wing/fuselage if needed
    // Currently the container is already black, so this might be redundant or for detail
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => false;
}
