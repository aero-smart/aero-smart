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
          _buildSplCard(context, state),
          const SizedBox(height: 16),
          _buildFftCard(context, state),
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

  Widget _buildSplCard(BuildContext context, MonitorState state) {
    return _buildCard(
      context,
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
          Center(child: Text('dB', style: TextStyle(color: Theme.of(context).textTheme.bodySmall?.color))),
          const SizedBox(height: 24),
          // Progress Bar
          ClipRRect(
            borderRadius: BorderRadius.circular(4),
            child: LinearProgressIndicator(
              value: state.currentSpl / 120.0, // Assuming 120dB max
              minHeight: 12,
              backgroundColor: Theme.of(context).brightness == Brightness.dark ? Colors.grey[800] : Colors.grey[300],
              color: const Color(0xFF4CAF50),
            ),
          ),
          const SizedBox(height: 8),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text('0 dB', style: TextStyle(fontSize: 10, color: Theme.of(context).textTheme.bodySmall?.color)),
              Text('120 dB', style: TextStyle(fontSize: 10, color: Theme.of(context).textTheme.bodySmall?.color)),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildFftCard(BuildContext context, MonitorState state) {
    final gridColor = Theme.of(context).dividerColor.withOpacity(0.1);
    final axisColor = Theme.of(context).textTheme.bodySmall?.color ?? Colors.grey;

    return _buildCard(
      context,
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
                    color: gridColor,
                    strokeWidth: 1,
                    dashArray: [5, 5],
                  ),
                  getDrawingVerticalLine: (value) => FlLine(
                    color: gridColor,
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
                               child: Text('$freq', style: TextStyle(fontSize: 10, color: axisColor)),
                             ),
                           );
                        }
                        return const SizedBox.shrink();
                      },
                    ),
                  ),
                  leftTitles: AxisTitles(
                    axisNameWidget: Text('幅度', style: TextStyle(fontSize: 10, color: axisColor)),
                    sideTitles: SideTitles(
                      showTitles: true,
                      reservedSize: 30,
                      getTitlesWidget: (value, meta) {
                        return Text(value.toInt().toString(), style: TextStyle(fontSize: 10, color: axisColor));
                      },
                    ),
                  ),
                  topTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
                  rightTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
                ),
                borderData: FlBorderData(
                  show: true,
                  border: Border(
                    bottom: BorderSide(color: Theme.of(context).dividerColor),
                    left: BorderSide(color: Theme.of(context).dividerColor),
                  ),
                ),
                barGroups: state.fftData.asMap().entries.map((entry) {
                  return BarChartGroupData(
                    x: entry.key,
                    barRods: [
                      BarChartRodData(
                        toY: entry.value,
                        color: Theme.of(context).colorScheme.primary,
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
          Center(child: Text('频率 (Hz)', style: TextStyle(fontSize: 10, color: axisColor))),
          const SizedBox(height: 8),
          Text('16频段实时 FFT 频谱显示，频率范围: 0-2000 Hz', style: TextStyle(fontSize: 12, color: axisColor)),
        ],
      ),
    );
  }
}
