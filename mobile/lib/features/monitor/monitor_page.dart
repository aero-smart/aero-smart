import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:fl_chart/fl_chart.dart';
import 'monitor_provider.dart';
import 'acoustic_tab.dart';
import 'imu_tab.dart';

class MonitorPage extends ConsumerStatefulWidget {
  const MonitorPage({super.key});

  @override
  ConsumerState<MonitorPage> createState() => _MonitorPageState();
}

class _MonitorPageState extends ConsumerState<MonitorPage> with SingleTickerProviderStateMixin {
  late TabController _tabController;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 3, vsync: this);
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('数据分析', style: TextStyle(fontWeight: FontWeight.bold)),
        centerTitle: false,
        bottom: TabBar(
          controller: _tabController,
          tabs: const [
            Tab(text: '图表'),
            Tab(text: '声学'),
            Tab(text: 'IMU'),
          ],
        ),
      ),
      body: TabBarView(
        controller: _tabController,
        children: [
          const ChartsTab(),
          const AcousticTab(),
          const ImuTab(),
        ],
      ),
    );
  }
}

class ChartsTab extends ConsumerWidget {
  const ChartsTab({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(monitorProvider);

    return SingleChildScrollView(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        children: [
          _buildChartCard(context, state),
          const SizedBox(height: 16),
          _buildRealTimeDataCard(context, state),
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

  Widget _buildChartCard(BuildContext context, MonitorState state) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final gridColor = Theme.of(context).dividerColor.withOpacity(0.1);
    final axisColor = Theme.of(context).textTheme.bodySmall?.color ?? Colors.grey;
    
    return _buildCard(
      context,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('数据源', style: TextStyle(color: axisColor, fontSize: 12)),
          const SizedBox(height: 8),
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
            decoration: BoxDecoration(
              color: isDark ? const Color(0xFF2C2C2C) : const Color(0xFFF0F0F0),
              borderRadius: BorderRadius.circular(4),
              border: Border.all(color: Theme.of(context).dividerColor),
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Text('风速', style: TextStyle(fontSize: 14)),
                Icon(Icons.arrow_drop_down, color: Theme.of(context).iconTheme.color),
              ],
            ),
          ),
          const SizedBox(height: 24),
          SizedBox(
            height: 200,
            child: LineChart(
              LineChartData(
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
                      reservedSize: 30,
                      getTitlesWidget: (value, meta) {
                        if (value % 10 == 0) {
                           return Text(value.toInt().toString(), style: TextStyle(color: axisColor, fontSize: 10));
                        }
                        return const SizedBox.shrink();
                      },
                    ),
                  ),
                  leftTitles: AxisTitles(
                    axisNameWidget: Text('风速 (m/s)', style: TextStyle(fontSize: 10, color: axisColor)),
                    sideTitles: SideTitles(
                      showTitles: true,
                      getTitlesWidget: (value, meta) {
                         return Text(value.toInt().toString(), style: TextStyle(color: axisColor, fontSize: 10));
                      },
                      reservedSize: 30,
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
                minX: 0,
                maxX: 100,
                minY: 0,
                maxY: 5, // Example max
                lineBarsData: [
                  LineChartBarData(
                    spots: [
                      // Dummy data for visual matching
                      const FlSpot(0, 0), const FlSpot(10, 0.5), const FlSpot(20, 1.2),
                      const FlSpot(30, 2.5), const FlSpot(40, 2.0), const FlSpot(50, 1.8),
                      const FlSpot(60, 3.0), const FlSpot(70, 2.5), const FlSpot(80, 2.8),
                      const FlSpot(90, 3.5), const FlSpot(100, 3.2),
                    ],
                    isCurved: true,
                    color: Theme.of(context).colorScheme.primary,
                    barWidth: 2,
                    isStrokeCapRound: true,
                    dotData: const FlDotData(show: false),
                    belowBarData: BarAreaData(show: false),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 8),
          Center(child: Text('-o- 风速 (m/s)', style: TextStyle(fontSize: 12, color: axisColor))),
        ],
      ),
    );
  }

  Widget _buildRealTimeDataCard(BuildContext context, MonitorState state) {
    return _buildCard(
      context,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('实时数据', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(child: _buildDataField(context, '风速', '${state.currentAirspeed.toStringAsFixed(2)} m/s')),
              Expanded(child: _buildDataField(context, '压力', '${state.currentPressure.toStringAsFixed(2)} Pa')),
            ],
          ),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(child: _buildDataField(context, '震动', '${state.currentVibration.toStringAsFixed(3)} RMS')),
              Expanded(child: _buildDataField(context, '温度', '${state.currentTemperature.toStringAsFixed(1)} °C')),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildDataField(BuildContext context, String label, String value) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(label, style: TextStyle(fontSize: 12, color: Theme.of(context).textTheme.bodySmall?.color)),
        const SizedBox(height: 4),
        Text(value, style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Monospace')),
      ],
    );
  }
}
