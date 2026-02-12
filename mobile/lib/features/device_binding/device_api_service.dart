import 'package:dio/dio.dart';

class WifiNetwork {
  final String ssid;
  final int signal;
  final String security;
  final bool inUse;

  WifiNetwork({
    required this.ssid,
    required this.signal,
    required this.security,
    required this.inUse,
  });

  factory WifiNetwork.fromJson(Map<String, dynamic> json) {
    return WifiNetwork(
      ssid: json['ssid'] ?? '',
      signal: json['signal'] ?? 0,
      security: json['security'] ?? '',
      inUse: json['in_use'] ?? false,
    );
  }
}

class WifiStatus {
  final bool connected;
  final String? ssid;
  final String? ip;

  WifiStatus({
    required this.connected,
    this.ssid,
    this.ip,
  });

  factory WifiStatus.fromJson(Map<String, dynamic> json) {
    return WifiStatus(
      connected: json['connected'] ?? false,
      ssid: json['ssid'],
      ip: json['ip'],
    );
  }
}

class DeviceApiService {
  final Dio _dio;
  final String baseUrl;

  DeviceApiService(String ip)
      : baseUrl = 'http://$ip:3000',
        _dio = Dio(BaseOptions(
          baseUrl: 'http://$ip:3000',
          connectTimeout: const Duration(seconds: 5),
          receiveTimeout: const Duration(seconds: 5),
        ));

  // Get current WiFi status
  Future<WifiStatus> getWifiStatus() async {
    try {
      final response = await _dio.get('/api/wifi/status');
      return WifiStatus.fromJson(response.data);
    } catch (e) {
      throw Exception('Failed to get WiFi status: $e');
    }
  }

  // Scan for available networks
  Future<List<WifiNetwork>> scanWifi() async {
    try {
      final response = await _dio.get('/api/wifi/scan');
      if (response.statusCode == 200) {
        final List<dynamic> data = response.data;
        return data.map((json) => WifiNetwork.fromJson(json)).toList();
      } else {
        throw Exception('Scan failed');
      }
    } catch (e) {
      throw Exception('Failed to scan WiFi: $e');
    }
  }

  // Connect to a WiFi network
  Future<void> connectWifi(String ssid, String? password) async {
    try {
      await _dio.post(
        '/api/wifi/connect',
        data: {
          'ssid': ssid,
          'password': password,
        },
      );
    } catch (e) {
      throw Exception('Failed to connect to WiFi: $e');
    }
  }

  // Disconnect current WiFi
  Future<void> disconnectWifi() async {
    try {
      await _dio.post('/api/wifi/disconnect');
    } catch (e) {
      throw Exception('Failed to disconnect WiFi: $e');
    }
  }

  // Test internet connectivity
  Future<bool> testInternet() async {
    try {
      await _dio.get('/api/wifi/test');
      return true;
    } catch (e) {
      return false;
    }
  }
}
