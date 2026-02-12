import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'device_api_service.dart';
import '../settings/settings_provider.dart';

class ConnectionPage extends ConsumerStatefulWidget {
  const ConnectionPage({super.key, this.qrData});
  final String? qrData; // This is the IP address from the QR code

  @override
  ConsumerState<ConnectionPage> createState() => _ConnectionPageState();
}

class _ConnectionPageState extends ConsumerState<ConnectionPage> {
  late DeviceApiService _apiService;
  WifiStatus? _currentStatus;
  List<WifiNetwork> _availableNetworks = [];
  bool _isLoading = false;
  String _errorMessage = '';

  @override
  void initState() {
    super.initState();
    if (widget.qrData != null) {
      _apiService = DeviceApiService(widget.qrData!);
      _loadInitialData();

      // 同步 IP 到全局设置并触发 WebSocket 连接
      WidgetsBinding.instance.addPostFrameCallback((_) {
        ref.read(settingsProvider.notifier).setIpAddress(widget.qrData!);
      });
    }
  }

  Future<void> _loadInitialData() async {
    setState(() {
      _isLoading = true;
      _errorMessage = '';
    });
    try {
      final status = await _apiService.getWifiStatus();
      setState(() {
        _currentStatus = status;
      });
    } catch (e) {
      setState(() {
        _errorMessage = '连接设备失败: $e';
      });
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  Future<void> _scanNetworks() async {
    setState(() {
      _isLoading = true;
    });
    try {
      final networks = await _apiService.scanWifi();
      setState(() {
        _availableNetworks = networks;
      });
    } catch (e) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('扫描失败: $e')));
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  Future<void> _connectToNetwork(String ssid, String? security) async {
    String? password;
    if (security != null &&
        security != '' &&
        security.toUpperCase() != 'NONE') {
      password = await _showPasswordDialog(ssid);
      if (password == null) return; // User cancelled
    }

    setState(() {
      _isLoading = true;
    });
    try {
      await _apiService.connectWifi(ssid, password);
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(const SnackBar(content: Text('已发送连接请求，请等待...')));
      // Poll status after a delay
      await Future.delayed(const Duration(seconds: 5));
      await _loadInitialData();
    } catch (e) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('连接失败: $e')));
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  Future<String?> _showPasswordDialog(String ssid) async {
    final controller = TextEditingController();
    return showDialog<String>(
      context: context,
      builder: (context) {
        return AlertDialog(
          title: Text('连接到 $ssid'),
          content: TextField(
            controller: controller,
            obscureText: true,
            decoration: const InputDecoration(labelText: '密码'),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('取消'),
            ),
            TextButton(
              onPressed: () => Navigator.pop(context, controller.text),
              child: const Text('连接'),
            ),
          ],
        );
      },
    );
  }

  Future<void> _disconnect() async {
    setState(() {
      _isLoading = true;
    });
    try {
      await _apiService.disconnectWifi();
      await Future.delayed(const Duration(seconds: 2));
      await _loadInitialData();
    } catch (e) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('断开连接失败: $e')));
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  Future<void> _testInternet() async {
    setState(() {
      _isLoading = true;
    });
    try {
      final success = await _apiService.testInternet();
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(success ? '互联网连接正常' : '无法连接到互联网'),
          backgroundColor: success ? Colors.green : Colors.red,
        ),
      );
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    if (widget.qrData == null) {
      return const Scaffold(body: Center(child: Text('无效的设备IP')));
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('设备网络管理'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: _loadInitialData,
          ),
        ],
      ),
      body: _isLoading && _currentStatus == null
          ? const Center(child: CircularProgressIndicator())
          : _errorMessage.isNotEmpty
          ? Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Text(
                    _errorMessage,
                    style: const TextStyle(color: Colors.red),
                  ),
                  const SizedBox(height: 16),
                  ElevatedButton(
                    onPressed: _loadInitialData,
                    child: const Text('重试'),
                  ),
                ],
              ),
            )
          : SingleChildScrollView(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  _buildStatusCard(),
                  const SizedBox(height: 24),
                  _buildActionsCard(),
                  const SizedBox(height: 24),
                  _buildNetworksList(),
                ],
              ),
            ),
    );
  }

  Widget _buildStatusCard() {
    return Card(
      elevation: 0,
      color: Colors.grey[100],
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              '当前状态',
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 16),
            Row(
              children: [
                Icon(
                  _currentStatus?.connected == true
                      ? Icons.wifi
                      : Icons.wifi_off,
                  color: _currentStatus?.connected == true
                      ? Colors.green
                      : Colors.grey,
                  size: 32,
                ),
                const SizedBox(width: 16),
                Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      _currentStatus?.connected == true ? '已连接' : '未连接',
                      style: const TextStyle(fontWeight: FontWeight.bold),
                    ),
                    if (_currentStatus?.ssid != null)
                      Text('SSID: ${_currentStatus!.ssid}'),
                    if (_currentStatus?.ip != null)
                      Text('IP: ${_currentStatus!.ip}'),
                  ],
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildActionsCard() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          '操作',
          style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
        ),
        const SizedBox(height: 16),
        Row(
          children: [
            Expanded(
              child: OutlinedButton.icon(
                onPressed: _isLoading ? null : _testInternet,
                icon: const Icon(Icons.language),
                label: const Text('网络测试'),
              ),
            ),
            const SizedBox(width: 16),
            Expanded(
              child: OutlinedButton.icon(
                onPressed: _isLoading ? null : _disconnect,
                icon: const Icon(Icons.link_off),
                label: const Text('断开连接'),
                style: OutlinedButton.styleFrom(foregroundColor: Colors.red),
              ),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildNetworksList() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            const Text(
              '可用网络',
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
            TextButton.icon(
              onPressed: _isLoading ? null : _scanNetworks,
              icon: const Icon(Icons.refresh),
              label: const Text('扫描'),
            ),
          ],
        ),
        const SizedBox(height: 8),
        if (_availableNetworks.isEmpty)
          const Padding(
            padding: EdgeInsets.symmetric(vertical: 24),
            child: Center(child: Text('暂无网络列表，请点击扫描')),
          )
        else
          ListView.builder(
            shrinkWrap: true,
            physics: const NeverScrollableScrollPhysics(),
            itemCount: _availableNetworks.length,
            itemBuilder: (context, index) {
              final network = _availableNetworks[index];
              return ListTile(
                leading: Icon(
                  Icons.wifi,
                  color: network.inUse ? Colors.green : null,
                ),
                title: Text(network.ssid),
                subtitle: Text(
                  '信号: ${network.signal}%  安全: ${network.security}',
                ),
                trailing: network.inUse
                    ? const Icon(Icons.check, color: Colors.green)
                    : const Icon(Icons.chevron_right),
                onTap: network.inUse
                    ? null
                    : () => _connectToNetwork(network.ssid, network.security),
              );
            },
          ),
      ],
    );
  }
}
