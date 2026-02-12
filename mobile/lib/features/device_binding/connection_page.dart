import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_blue_plus/flutter_blue_plus.dart';
import 'package:go_router/go_router.dart';

// 页面状态枚举
enum BindingStep {
  scanning, // 扫描蓝牙设备
  connecting, // 连接蓝牙设备
  wifiInput, // 输入WiFi信息
  configuring, // 推送WiFi并等待回传
  success, // 绑定成功
  failure, // 绑定失败
}

class ConnectionPage extends ConsumerStatefulWidget {
  const ConnectionPage({super.key, this.qrData});
  final String? qrData; // 扫码得到的数据（例如设备MAC地址或序列号）

  @override
  ConsumerState<ConnectionPage> createState() => _ConnectionPageState();
}

class _ConnectionPageState extends ConsumerState<ConnectionPage> {
  BindingStep _currentStep = BindingStep.scanning;
  String _statusMessage = '正在搜索设备...';
  String _deviceIp = '';

  final TextEditingController _ssidController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();

  // 模拟进度
  double _progress = 0.0;

  @override
  void initState() {
    super.initState();
    _startBindingProcess();
  }

  @override
  void dispose() {
    _ssidController.dispose();
    _passwordController.dispose();
    super.dispose();
  }

  Future<void> _startBindingProcess() async {
    // Step 1: 扫描蓝牙
    // TODO: 实际接入时使用 FlutterBluePlus.startScan()
    // if (await FlutterBluePlus.isSupported == false) { ... }

    setState(() {
      _currentStep = BindingStep.scanning;
      _statusMessage = '正在搜索设备 ${widget.qrData ?? ""}...';
    });

    // 模拟搜索耗时
    await Future.delayed(const Duration(seconds: 2));

    // Step 2: 模拟找到设备并连接
    setState(() {
      _currentStep = BindingStep.connecting;
      _statusMessage = '正在连接设备...';
    });

    // 模拟连接耗时
    await Future.delayed(const Duration(seconds: 1));

    // 连接成功，跳转到 WiFi 输入
    setState(() {
      _currentStep = BindingStep.wifiInput;
      _statusMessage = '请配置网络';
    });
  }

  Future<void> _submitWifiConfig() async {
    final ssid = _ssidController.text;
    final password = _passwordController.text;

    if (ssid.isEmpty) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(const SnackBar(content: Text('请输入WiFi名称')));
      return;
    }

    setState(() {
      _currentStep = BindingStep.configuring;
      _statusMessage = '正在推送网络配置...';
      _progress = 0.1;
    });

    // Step 3: 模拟配网过程
    // TODO: 通过 BLE 特征值写入 SSID 和 Password
    // await characteristic.write(utf8.encode('WIFI:$ssid:$password'));

    // 模拟进度
    for (int i = 1; i <= 10; i++) {
      await Future.delayed(const Duration(milliseconds: 300));
      if (!mounted) return;
      setState(() {
        _progress = 0.1 + (i * 0.05);
        if (i == 5) _statusMessage = '设备正在连接 WiFi...';
        if (i == 8) _statusMessage = '等待设备回传 IP...';
      });
    }

    // Step 4: 模拟回传 IP
    // TODO: 监听 BLE 通知获取 IP
    final mockIp = '192.168.1.105';

    setState(() {
      _progress = 1.0;
      _currentStep = BindingStep.success;
      _deviceIp = mockIp;
      _statusMessage = '设备绑定成功！';
    });
  }

  void _retry() {
    _startBindingProcess();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('设备绑定')),
      body: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          children: [
            _buildStepIndicator(),
            const SizedBox(height: 40),
            Expanded(child: _buildCurrentContent()),
          ],
        ),
      ),
    );
  }

  Widget _buildStepIndicator() {
    // 简易步骤条
    return Row(
      children: [
        _buildStepIcon(1, _currentStep.index >= BindingStep.scanning.index),
        _buildStepLine(_currentStep.index >= BindingStep.wifiInput.index),
        _buildStepIcon(2, _currentStep.index >= BindingStep.wifiInput.index),
        _buildStepLine(_currentStep.index >= BindingStep.success.index),
        _buildStepIcon(3, _currentStep.index == BindingStep.success.index),
      ],
    );
  }

  Widget _buildStepIcon(int step, bool isActive) {
    return Container(
      width: 32,
      height: 32,
      decoration: BoxDecoration(
        shape: BoxShape.circle,
        color: isActive ? Colors.black : Colors.grey[300],
      ),
      child: Center(
        child: Text(
          '$step',
          style: const TextStyle(
            color: Colors.white,
            fontWeight: FontWeight.bold,
          ),
        ),
      ),
    );
  }

  Widget _buildStepLine(bool isActive) {
    return Expanded(
      child: Container(
        height: 2,
        color: isActive ? Colors.black : Colors.grey[300],
      ),
    );
  }

  Widget _buildCurrentContent() {
    switch (_currentStep) {
      case BindingStep.scanning:
      case BindingStep.connecting:
        return Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const CircularProgressIndicator(color: Colors.black),
            const SizedBox(height: 24),
            Text(_statusMessage, style: const TextStyle(fontSize: 16)),
          ],
        );

      case BindingStep.wifiInput:
        return SingleChildScrollView(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text(
                '请输入无线网络信息',
                style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
              ),
              const SizedBox(height: 8),
              const Text(
                '设备需要连接到同一个局域网才能进行通讯。',
                style: TextStyle(color: Colors.grey),
              ),
              const SizedBox(height: 32),
              TextField(
                controller: _ssidController,
                decoration: const InputDecoration(
                  labelText: 'WiFi 名称 (SSID)',
                  border: OutlineInputBorder(),
                  prefixIcon: Icon(Icons.wifi),
                ),
              ),
              const SizedBox(height: 16),
              TextField(
                controller: _passwordController,
                obscureText: true,
                decoration: const InputDecoration(
                  labelText: 'WiFi 密码',
                  border: OutlineInputBorder(),
                  prefixIcon: Icon(Icons.lock_outline),
                ),
              ),
              const SizedBox(height: 32),
              SizedBox(
                width: double.infinity,
                height: 48,
                child: ElevatedButton(
                  onPressed: _submitWifiConfig,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Colors.black,
                    foregroundColor: Colors.white,
                  ),
                  child: const Text('连接设备'),
                ),
              ),
            ],
          ),
        );

      case BindingStep.configuring:
        return Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            CircularProgressIndicator(value: _progress, color: Colors.black),
            const SizedBox(height: 24),
            Text(_statusMessage, style: const TextStyle(fontSize: 16)),
            const SizedBox(height: 8),
            Text(
              '${(_progress * 100).toInt()}%',
              style: const TextStyle(color: Colors.grey),
            ),
          ],
        );

      case BindingStep.success:
        return Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.check_circle, color: Colors.green, size: 80),
            const SizedBox(height: 24),
            const Text(
              '绑定成功',
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 16),
            Text('设备 IP 地址: $_deviceIp', style: const TextStyle(fontSize: 16)),
            const SizedBox(height: 40),
            SizedBox(
              width: double.infinity,
              height: 48,
              child: ElevatedButton(
                onPressed: () {
                  // 返回设置页或首页
                  context.pop();
                },
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.black,
                  foregroundColor: Colors.white,
                ),
                child: const Text('完成'),
              ),
            ),
          ],
        );

      case BindingStep.failure:
        return Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.error, color: Colors.red, size: 80),
            const SizedBox(height: 24),
            const Text(
              '绑定失败',
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 16),
            Text(_statusMessage, style: const TextStyle(fontSize: 16)),
            const SizedBox(height: 40),
            SizedBox(
              width: double.infinity,
              height: 48,
              child: ElevatedButton(
                onPressed: _retry,
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.black,
                  foregroundColor: Colors.white,
                ),
                child: const Text('重试'),
              ),
            ),
          ],
        );
    }
  }
}
