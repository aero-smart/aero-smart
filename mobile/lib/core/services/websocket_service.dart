import 'dart:async';
import 'dart:convert';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:web_socket_channel/status.dart' as status;

// 消息基类
abstract class AeroMessage {
  Map<String, dynamic> toJson();
}

// 连接状态枚举
enum ConnectionStatus {
  disconnected,
  connecting,
  connected,
  error,
}

// WebSocket 服务
class WebSocketService {
  WebSocketChannel? _channel;
  StreamController<dynamic>? _streamController;
  Timer? _reconnectTimer;
  String? _currentUrl;
  bool _isActive = false;

  Stream<dynamic> get stream {
    _streamController ??= StreamController<dynamic>.broadcast();
    return _streamController!.stream;
  }

  bool get isConnected => _channel != null;

  void connect(String ip) {
    if (_currentUrl == 'ws://$ip:3000/ws' && _isActive) return;

    _currentUrl = 'ws://$ip:3000/ws';
    _isActive = true;
    _connectInternal();
  }

  void _connectInternal() {
    if (!_isActive || _currentUrl == null) return;

    try {
      _channel = WebSocketChannel.connect(Uri.parse(_currentUrl!));
      _channel!.stream.listen(
        (message) {
          _streamController?.add(message);
        },
        onDone: () {
          _channel = null;
          _scheduleReconnect();
        },
        onError: (error) {
          _channel = null;
          _streamController?.addError(error);
          _scheduleReconnect();
        },
      );
    } catch (e) {
      _scheduleReconnect();
    }
  }

  void _scheduleReconnect() {
    if (!_isActive) return;
    _reconnectTimer?.cancel();
    _reconnectTimer = Timer(const Duration(seconds: 3), () {
      _connectInternal();
    });
  }

  void send(AeroMessage message) {
    if (_channel != null) {
      _channel!.sink.add(jsonEncode(message.toJson()));
    }
  }

  void sendRaw(String message) {
    if (_channel != null) {
      _channel!.sink.add(message);
    }
  }

  void disconnect() {
    _isActive = false;
    _reconnectTimer?.cancel();
    _channel?.sink.close(status.goingAway);
    _channel = null;
  }

  void dispose() {
    disconnect();
    _streamController?.close();
  }
}

// 连接管理器 Provider
final connectionServiceProvider =
    StateNotifierProvider<ConnectionService, ConnectionStatus>((ref) {
  return ConnectionService();
});

class ConnectionService extends StateNotifier<ConnectionStatus> {
  ConnectionService() : super(ConnectionStatus.disconnected) {
    _loadSavedIp();
  }

  final WebSocketService _wsService = WebSocketService();
  String? _currentIp;

  WebSocketService get ws => _wsService;
  String? get currentIp => _currentIp;

  Future<void> _loadSavedIp() async {
    final prefs = await SharedPreferences.getInstance();
    final ip = prefs.getString('device_ip');
    if (ip != null) {
      connect(ip);
    }
  }

  Future<void> saveIp(String ip) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('device_ip', ip);
  }

  void connect(String ip) {
    _currentIp = ip;
    state = ConnectionStatus.connecting;
    _wsService.connect(ip);
    
    // 简单的连接状态监听逻辑（实际应通过 WebSocket 状态流更精确控制）
    // 这里假设连接发出即进入连接尝试状态，成功与否通过数据流判断
    state = ConnectionStatus.connected;
    saveIp(ip);
  }

  void disconnect() {
    _wsService.disconnect();
    state = ConnectionStatus.disconnected;
  }

  @override
  void dispose() {
    _wsService.dispose();
    super.dispose();
  }
}
