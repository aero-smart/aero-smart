import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'settings_provider.dart';

class SettingsPage extends ConsumerWidget {
  const SettingsPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(settingsProvider);
    final notifier = ref.read(settingsProvider.notifier);

    return Scaffold(
      appBar: AppBar(
        title: const Text('设置', style: TextStyle(fontWeight: FontWeight.bold)),
        centerTitle: false,
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            _buildConnectionCard(context, state, notifier),
            const SizedBox(height: 16),
            _buildSensorConfigCard(state, notifier),
            const SizedBox(height: 16),
            _buildFirmwareUpdateCard(context, state, notifier),
            const SizedBox(height: 16),
            _buildAppConfigCard(state, notifier),
            const SizedBox(height: 16),
            _buildAboutCard(state),
          ],
        ),
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

  Widget _buildConnectionCard(
    BuildContext context,
    SettingsState state,
    SettingsNotifier notifier,
  ) {
    return _buildCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: const [
              Icon(Icons.wifi, size: 20),
              SizedBox(width: 8),
              Text(
                '连接设置',
                style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
              ),
            ],
          ),
          const SizedBox(height: 16),
          const Text(
            'IP地址',
            style: TextStyle(fontSize: 12, color: Colors.grey),
          ),
          const SizedBox(height: 8),
          TextField(
            decoration: InputDecoration(
              contentPadding: const EdgeInsets.symmetric(
                horizontal: 12,
                vertical: 12,
              ),
              border: OutlineInputBorder(
                borderRadius: BorderRadius.circular(4),
                borderSide: BorderSide(
                  color: Colors.black.withValues(alpha: 0.2),
                ),
              ),
              hintText: '192.168.1.100',
            ),
            controller: TextEditingController(text: state.ipAddress),
            onSubmitted: (value) => notifier.setIpAddress(value),
          ),
          const SizedBox(height: 16),
          SizedBox(
            width: double.infinity,
            child: ElevatedButton(
              onPressed: state.isConnected
                  ? notifier.disconnect
                  : notifier.connect,
              style: ElevatedButton.styleFrom(
                backgroundColor: state.isConnected
                    ? Colors.grey[300]
                    : Colors.black,
                foregroundColor: state.isConnected
                    ? Colors.grey[600]
                    : Colors.white,
                elevation: 0,
              ),
              child: Text(state.isConnected ? '已连接' : '连接'),
            ),
          ),
          const SizedBox(height: 16),
          const Divider(),
          const SizedBox(height: 16),
          Row(
            children: const [
              Icon(Icons.bluetooth, size: 20),
              SizedBox(width: 8),
              Text('蓝牙设备', style: TextStyle(fontSize: 14)),
            ],
          ),
          const SizedBox(height: 16),
          SizedBox(
            width: double.infinity,
            child: OutlinedButton(
              onPressed: () {
                context.go('/settings/binding/scan');
              },
              style: OutlinedButton.styleFrom(
                foregroundColor: Colors.black,
                side: BorderSide(color: Colors.black.withValues(alpha: 0.2)),
              ),
              child: const Text('扫描蓝牙设备'),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSensorConfigCard(
    SettingsState state,
    SettingsNotifier notifier,
  ) {
    return _buildCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: const [
              Icon(Icons.sensors, size: 20),
              SizedBox(width: 8),
              Text(
                '传感器配置',
                style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
              ),
            ],
          ),
          const SizedBox(height: 16),
          _buildDropdown<ImuOrientation>(
            label: 'IMU方向',
            value: state.imuOrientation,
            items: const [
              DropdownMenuItem(
                value: ImuOrientation.horizontal,
                child: Text('水平安装'),
              ),
              DropdownMenuItem(
                value: ImuOrientation.vertical,
                child: Text('垂直安装'),
              ),
              DropdownMenuItem(
                value: ImuOrientation.upsideDown,
                child: Text('倒置安装'),
              ),
            ],
            onChanged: (val) => notifier.setImuOrientation(val!),
          ),
          const SizedBox(height: 16),
          const Text(
            '空速管系数',
            style: TextStyle(fontSize: 12, color: Colors.grey),
          ),
          const SizedBox(height: 8),
          TextField(
            decoration: InputDecoration(
              contentPadding: const EdgeInsets.symmetric(
                horizontal: 12,
                vertical: 12,
              ),
              border: OutlineInputBorder(
                borderRadius: BorderRadius.circular(4),
                borderSide: BorderSide(
                  color: Colors.black.withValues(alpha: 0.2),
                ),
              ),
            ),
            controller: TextEditingController(
              text: state.pitotCoeff.toStringAsFixed(2),
            ),
            keyboardType: TextInputType.number,
            onSubmitted: (value) =>
                notifier.setPitotCoeff(double.tryParse(value) ?? 1.0),
          ),
          const SizedBox(height: 16),
          _buildDropdown<SamplingRate>(
            label: '采样率',
            value: state.samplingRate,
            items: const [
              DropdownMenuItem(value: SamplingRate.hz50, child: Text('50 Hz')),
              DropdownMenuItem(
                value: SamplingRate.hz100,
                child: Text('100 Hz'),
              ),
              DropdownMenuItem(
                value: SamplingRate.hz200,
                child: Text('200 Hz'),
              ),
              DropdownMenuItem(
                value: SamplingRate.hz400,
                child: Text('400 Hz'),
              ),
            ],
            onChanged: (val) => notifier.setSamplingRate(val!),
          ),
        ],
      ),
    );
  }

  Widget _buildDropdown<T>({
    required String label,
    required T value,
    required List<DropdownMenuItem<T>> items,
    required ValueChanged<T?> onChanged,
  }) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(label, style: const TextStyle(fontSize: 12, color: Colors.grey)),
        const SizedBox(height: 8),
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 12),
          decoration: BoxDecoration(
            border: Border.all(color: Colors.black.withValues(alpha: 0.2)),
            borderRadius: BorderRadius.circular(4),
          ),
          child: DropdownButtonHideUnderline(
            child: DropdownButton<T>(
              value: value,
              items: items,
              onChanged: onChanged,
              isExpanded: true,
              icon: const Icon(Icons.arrow_drop_down),
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildFirmwareUpdateCard(
    BuildContext context,
    SettingsState state,
    SettingsNotifier notifier,
  ) {
    return _buildCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: const [
              Icon(Icons.system_update, size: 20),
              SizedBox(width: 8),
              Text(
                '固件更新',
                style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
              ),
            ],
          ),
          const SizedBox(height: 16),
          const Text(
            '当前版本',
            style: TextStyle(fontSize: 12, color: Colors.grey),
          ),
          const SizedBox(height: 4),
          Text(
            state.firmwareVersion,
            style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 16),
          SizedBox(
            width: double.infinity,
            child: ElevatedButton(
              onPressed: () {
                context.go('/settings/firmware');
              },
              style: ElevatedButton.styleFrom(
                backgroundColor: Colors.black,
                foregroundColor: Colors.white,
              ),
              child: const Text('检查更新'),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildAppConfigCard(SettingsState state, SettingsNotifier notifier) {
    return _buildCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: const [
              Icon(Icons.settings_applications, size: 20),
              SizedBox(width: 8),
              Text(
                '应用设置',
                style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
              ),
            ],
          ),
          const SizedBox(height: 16),
          _buildDropdown<AppThemeMode>(
            label: '主题',
            value: state.themeMode,
            items: const [
              DropdownMenuItem(value: AppThemeMode.system, child: Text('跟随系统')),
              DropdownMenuItem(value: AppThemeMode.light, child: Text('浅色模式')),
              DropdownMenuItem(value: AppThemeMode.dark, child: Text('深色模式')),
            ],
            onChanged: (val) => notifier.setThemeMode(val!),
          ),
          const SizedBox(height: 16),
          _buildDropdown<UnitSystem>(
            label: '单位偏好',
            value: state.unitSystem,
            items: const [
              DropdownMenuItem(value: UnitSystem.metric, child: Text('公制')),
              DropdownMenuItem(value: UnitSystem.imperial, child: Text('英制')),
            ],
            onChanged: (val) => notifier.setUnitSystem(val!),
          ),
        ],
      ),
    );
  }

  Widget _buildAboutCard(SettingsState state) {
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      decoration: BoxDecoration(
        color: const Color(0xFFF9F9F9),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.black.withValues(alpha: 0.05)),
      ),
      child: Row(
        children: [
          const Icon(Icons.info, size: 20, color: Colors.grey),
          const SizedBox(width: 16),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  '关于 AeroSmart',
                  style: TextStyle(fontSize: 14, fontWeight: FontWeight.w500),
                ),
                Text(
                  state.appVersion,
                  style: const TextStyle(fontSize: 12, color: Colors.grey),
                ),
              ],
            ),
          ),
          const Icon(Icons.chevron_right, color: Colors.grey),
        ],
      ),
    );
  }
}
