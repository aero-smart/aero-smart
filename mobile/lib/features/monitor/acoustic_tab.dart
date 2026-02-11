import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:fl_chart/fl_chart.dart';
import 'monitor_provider.dart';

class AcousticTab extends ConsumerWidget {
  const AcousticTab({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(monitorProvider);

    return SingleChildScrollView(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        children: [
          _buildSplCard(state),
          const SizedBox(height: 16),
          _buildFftCard(state),
        ],
      ),
    );
  }

  Widget _buildCard({required Widget child}) {
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFFF9F9F9),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.black.withValues(alpha: 0.05)),
      ),
      child: child,
    );
  }

  Widget _buildSplCard(MonitorState state) {
    return _buildCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: const [
              Icon(Icons.volume_up, size: 20),
              SizedBox(width: 8),
              Text('声压级 (SPL)', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
            ],
          ),
          const SizedBox(height: 24),
          Center(
            child: Text(
              state.currentSpl.toStringAsFixed(1),
              style: const TextStyle(
                fontSize: 64,
                fontWeight: FontWeight.bold,
                color: Color(0xFF4CAF50),
                fontFamily: 'Monospace',
              ),
            ),
          ),
          const Center(child: Text('dB', style: TextStyle(color: Colors.grey))),
          const SizedBox(height: 24),
          // Progress Bar
          ClipRRect(
            borderRadius: BorderRadius.circular(4),
            child: LinearProgressIndicator(
              value: state.currentSpl / 120.0, // Assuming 120dB max
              minHeight: 12,
              backgroundColor: Colors.grey[300],
              color: const Color(0xFF4CAF50),
            ),
          ),
          const SizedBox(height: 8),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: const [
              Text('0 dB', style: TextStyle(fontSize: 10, color: Colors.grey)),
              Text('120 dB', style: TextStyle(fontSize: 10, color: Colors.grey)),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildFftCard(MonitorState state) {
    return _buildCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('频谱分析 (FFT)', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
          const SizedBox(height: 16),
          SizedBox(
            height: 250,
            child: BarChart(
              BarChartData(
                gridData: FlGridData(
                  show: true,
                  drawVerticalLine: true,
                  getDrawingHorizontalLine: (value) => FlLine(
                    color: Colors.black.withValues(alpha: 0.1),
                    strokeWidth: 1,
                    dashArray: [5, 5],
                  ),
                  getDrawingVerticalLine: (value) => FlLine(
                    color: Colors.black.withValues(alpha: 0.1),
                    strokeWidth: 1,
                    dashArray: [5, 5],
                  ),
                ),
                titlesData: FlTitlesData(
                  show: true,
                  bottomTitles: AxisTitles(
                    sideTitles: SideTitles(
                      showTitles: true,
                      getTitlesWidget: (value, meta) {
                        // Display frequency labels: 0, 375, 750...
                        // We have 16 bands, mapped to 0-2000Hz approx
                        final int index = value.toInt();
                        if (index % 3 == 0 && index < state.fftData.length) {
                           final freq = (index / 16 * 2000).toInt();
                           return Padding(
                             padding: const EdgeInsets.only(top: 8.0),
                             child: Transform.rotate(
                               angle: -0.5,
                               child: Text('$freq', style: const TextStyle(fontSize: 10, color: Colors.grey)),
                             ),
                           );
                        }
                        return const SizedBox.shrink();
                      },
                    ),
                  ),
                  leftTitles: AxisTitles(
                    axisNameWidget: const Text('幅度', style: TextStyle(fontSize: 10, color: Colors.grey)),
                    sideTitles: SideTitles(
                      showTitles: true,
                      reservedSize: 30,
                      getTitlesWidget: (value, meta) {
                        return Text(value.toInt().toString(), style: const TextStyle(fontSize: 10, color: Colors.grey));
                      },
                    ),
                  ),
                  topTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
                  rightTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
                ),
                borderData: FlBorderData(
                  show: true,
                  border: const Border(
                    bottom: BorderSide(color: Colors.black12),
                    left: BorderSide(color: Colors.black12),
                  ),
                ),
                barGroups: state.fftData.asMap().entries.map((entry) {
                  return BarChartGroupData(
                    x: entry.key,
                    barRods: [
                      BarChartRodData(
                        toY: entry.value,
                        color: Colors.black87,
                        width: 12,
                        borderRadius: BorderRadius.circular(2),
                      ),
                    ],
                  );
                }).toList(),
                maxY: 4.0, // Adjust based on expected amplitude
              ),
            ),
          ),
          const SizedBox(height: 24),
          const Center(child: Text('频率 (Hz)', style: TextStyle(fontSize: 10, color: Colors.grey))),
          const SizedBox(height: 8),
          const Text('16频段实时 FFT 频谱显示，频率范围: 0-2000 Hz', style: TextStyle(fontSize: 12, color: Colors.grey)),
        ],
      ),
    );
  }
}
