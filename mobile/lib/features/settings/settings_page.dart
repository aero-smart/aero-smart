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
            _buildSensorConfigCard(context, state, notifier),
            const SizedBox(height: 16),
            _buildFirmwareUpdateCard(context, state, notifier),
            const SizedBox(height: 16),
            _buildAppConfigCard(context, state, notifier),
            const SizedBox(height: 16),
            _buildAboutCard(context, state),
          ],
        ),
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
        border: Border.all(
          color: Theme.of(context).dividerColor.withOpacity(0.1),
        ),
      ),
      child: child,
    );
  }

  Widget _buildConnectionCard(
    BuildContext context,
    SettingsState state,
    SettingsNotifier notifier,
  ) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    return _buildCard(
      context,
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
          Text(
            'IP地址',
            style: TextStyle(
              fontSize: 12,
              color: Theme.of(context).textTheme.bodySmall?.color,
            ),
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
                borderSide: BorderSide(color: Theme.of(context).dividerColor),
              ),
              hintText: '192.168.1.100',
              hintStyle: TextStyle(color: Theme.of(context).hintColor),
            ),
            style: TextStyle(
              color: Theme.of(context).textTheme.bodyMedium?.color,
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
                    ? (isDark ? Colors.grey[800] : Colors.grey[300])
                    : Theme.of(context).colorScheme.primary,
                foregroundColor: state.isConnected
                    ? Colors.grey[600]
                    : Theme.of(context).colorScheme.onPrimary,
                elevation: 0,
              ),
              child: Text(state.isConnected ? '已连接' : '连接'),
            ),
          ),
          const SizedBox(height: 16),
          const Divider(),
          const SizedBox(height: 16),
          SizedBox(
            width: double.infinity,
            child: OutlinedButton.icon(
              onPressed: () {
                context.go('/settings/binding/scan');
              },
              style: OutlinedButton.styleFrom(
                foregroundColor: Theme.of(context).colorScheme.onSurface,
                side: BorderSide(color: Theme.of(context).dividerColor),
              ),
              icon: const Icon(Icons.qr_code_scanner, size: 18),
              label: const Text('连接设备 (扫码)'),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSensorConfigCard(
    BuildContext context,
    SettingsState state,
    SettingsNotifier notifier,
  ) {
    return _buildCard(
      context,
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
            context: context,
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
          Text(
            '空速管系数',
            style: TextStyle(
              fontSize: 12,
              color: Theme.of(context).textTheme.bodySmall?.color,
            ),
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
                borderSide: BorderSide(color: Theme.of(context).dividerColor),
              ),
            ),
            style: TextStyle(
              color: Theme.of(context).textTheme.bodyMedium?.color,
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
            context: context,
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
    required BuildContext context,
    required String label,
    required T value,
    required List<DropdownMenuItem<T>> items,
    required ValueChanged<T?> onChanged,
  }) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          label,
          style: TextStyle(
            fontSize: 12,
            color: Theme.of(context).textTheme.bodySmall?.color,
          ),
        ),
        const SizedBox(height: 8),
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 12),
          decoration: BoxDecoration(
            border: Border.all(color: Theme.of(context).dividerColor),
            borderRadius: BorderRadius.circular(4),
          ),
          child: DropdownButtonHideUnderline(
            child: DropdownButton<T>(
              value: value,
              items: items,
              onChanged: onChanged,
              isExpanded: true,
              icon: Icon(
                Icons.arrow_drop_down,
                color: Theme.of(context).iconTheme.color,
              ),
              dropdownColor: Theme.of(context).cardColor,
              style: TextStyle(
                color: Theme.of(context).textTheme.bodyMedium?.color,
              ),
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
      context,
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
          Text(
            '当前版本',
            style: TextStyle(
              fontSize: 12,
              color: Theme.of(context).textTheme.bodySmall?.color,
            ),
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
                backgroundColor: Theme.of(context).colorScheme.primary,
                foregroundColor: Theme.of(context).colorScheme.onPrimary,
              ),
              child: const Text('检查更新'),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildAppConfigCard(
    BuildContext context,
    SettingsState state,
    SettingsNotifier notifier,
  ) {
    return _buildCard(
      context,
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
            context: context,
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
            context: context,
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

  Widget _buildAboutCard(BuildContext context, SettingsState state) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      decoration: BoxDecoration(
        color: isDark ? const Color(0xFF1E1E1E) : const Color(0xFFF9F9F9),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(
          color: Theme.of(context).dividerColor.withOpacity(0.1),
        ),
      ),
      child: Row(
        children: [
          Icon(
            Icons.info,
            size: 20,
            color:
                Theme.of(context).iconTheme.color?.withOpacity(0.5) ??
                Colors.grey,
          ),
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
                  style: TextStyle(
                    fontSize: 12,
                    color: Theme.of(context).textTheme.bodySmall?.color,
                  ),
                ),
              ],
            ),
          ),
          Icon(
            Icons.chevron_right,
            color:
                Theme.of(context).iconTheme.color?.withOpacity(0.5) ??
                Colors.grey,
          ),
        ],
      ),
    );
  }
}
