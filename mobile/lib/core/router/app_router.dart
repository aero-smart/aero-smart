import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../../features/dashboard/dashboard_page.dart';
import '../../features/monitor/monitor_page.dart';
import '../../features/controls/controls_page.dart';
import '../../features/settings/settings_page.dart';
import '../../features/firmware/firmware_page.dart';
import '../../features/device_binding/qr_scanner_page.dart';
import '../../features/device_binding/connection_page.dart';
import '../../shared/widgets/shell_layout.dart';

final GlobalKey<NavigatorState> _rootNavigatorKey = GlobalKey<NavigatorState>();
final GlobalKey<NavigatorState> _shellNavigatorKey =
    GlobalKey<NavigatorState>();

final GoRouter appRouter = GoRouter(
  navigatorKey: _rootNavigatorKey,
  initialLocation: '/dashboard',
  routes: [
    ShellRoute(
      navigatorKey: _shellNavigatorKey,
      builder: (context, state, child) {
        return ShellLayout(child: child);
      },
      routes: [
        GoRoute(
          path: '/dashboard',
          pageBuilder: (context, state) =>
              const NoTransitionPage(child: DashboardPage()),
        ),
        GoRoute(
          path: '/monitor',
          pageBuilder: (context, state) =>
              const NoTransitionPage(child: MonitorPage()),
        ),
        GoRoute(
          path: '/controls',
          pageBuilder: (context, state) =>
              const NoTransitionPage(child: ControlsPage()),
        ),
        GoRoute(
          path: '/settings',
          pageBuilder: (context, state) =>
              const NoTransitionPage(child: SettingsPage()),
          routes: [
            GoRoute(
              path: 'firmware',
              parentNavigatorKey:
                  _rootNavigatorKey, // Firmware page covers the bottom nav
              builder: (context, state) => const FirmwarePage(),
            ),
            GoRoute(
              path: 'binding/scan',
              parentNavigatorKey: _rootNavigatorKey,
              builder: (context, state) => const QRScannerPage(),
            ),
            GoRoute(
              path: 'binding/connection',
              parentNavigatorKey: _rootNavigatorKey,
              builder: (context, state) {
                final extra = state.extra as Map<String, dynamic>?;
                final ip = extra?['ip'] as String?;
                final ssid = extra?['ssid'] as String?;
                return ConnectionPage(qrData: ip, targetSsid: ssid);
              },
            ),
          ],
        ),
      ],
    ),
  ],
);
