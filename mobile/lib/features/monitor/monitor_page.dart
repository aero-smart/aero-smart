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
          labelColor: Colors.black,
          unselectedLabelColor: Colors.grey,
          indicatorColor: Colors.black,
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
          _buildChartCard(state),
          const SizedBox(height: 16),
          _buildRealTimeDataCard(state),
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

  Widget _buildChartCard(MonitorState state) {
    return _buildCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('数据源', style: TextStyle(color: Colors.grey, fontSize: 12)),
          const SizedBox(height: 8),
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
            decoration: BoxDecoration(
              color: const Color(0xFFF0F0F0),
              borderRadius: BorderRadius.circular(4),
              border: Border.all(color: Colors.black12),
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: const [
                Text('风速', style: TextStyle(fontSize: 14)),
                Icon(Icons.arrow_drop_down),
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
                      reservedSize: 30,
                      getTitlesWidget: (value, meta) {
                        if (value % 10 == 0) {
                           return Text(value.toInt().toString(), style: const TextStyle(color: Colors.grey, fontSize: 10));
                        }
                        return const SizedBox.shrink();
                      },
                    ),
                  ),
                  leftTitles: AxisTitles(
                    axisNameWidget: const Text('风速 (m/s)', style: TextStyle(fontSize: 10, color: Colors.grey)),
                    sideTitles: SideTitles(
                      showTitles: true,
                      getTitlesWidget: (value, meta) {
                         return Text(value.toInt().toString(), style: const TextStyle(color: Colors.grey, fontSize: 10));
                      },
                      reservedSize: 30,
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
                    color: Colors.black,
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
          const Center(child: Text('-o- 风速 (m/s)', style: TextStyle(fontSize: 12))),
        ],
      ),
    );
  }

  Widget _buildRealTimeDataCard(MonitorState state) {
    return _buildCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('实时数据', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(child: _buildDataField('风速', '${state.currentAirspeed.toStringAsFixed(2)} m/s')),
              Expanded(child: _buildDataField('压力', '${state.currentPressure.toStringAsFixed(2)} Pa')),
            ],
          ),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(child: _buildDataField('震动', '${state.currentVibration.toStringAsFixed(3)} RMS')),
              Expanded(child: _buildDataField('温度', '${state.currentTemperature.toStringAsFixed(1)} °C')),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildDataField(String label, String value) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(label, style: const TextStyle(fontSize: 12, color: Colors.grey)),
        const SizedBox(height: 4),
        Text(value, style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Monospace')),
      ],
    );
  }
}
