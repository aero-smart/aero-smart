import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../../features/dashboard/dashboard_page.dart';
import '../../features/monitor/monitor_page.dart';
import '../../features/controls/controls_page.dart';
import '../../features/settings/settings_page.dart';
import '../../features/firmware/firmware_page.dart';
import '../../shared/widgets/shell_layout.dart';

final GlobalKey<NavigatorState> _rootNavigatorKey = GlobalKey<NavigatorState>();
final GlobalKey<NavigatorState> _shellNavigatorKey = GlobalKey<NavigatorState>();

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
          builder: (context, state) => const DashboardPage(),
        ),
        GoRoute(
          path: '/monitor',
          builder: (context, state) => const MonitorPage(),
        ),
        GoRoute(
          path: '/controls',
          builder: (context, state) => const ControlsPage(),
        ),
        GoRoute(
          path: '/settings',
          builder: (context, state) => const SettingsPage(),
          routes: [
            GoRoute(
              path: 'firmware',
              parentNavigatorKey: _rootNavigatorKey, // Firmware page covers the bottom nav
              builder: (context, state) => const FirmwarePage(),
            ),
          ],
        ),
      ],
    ),
  ],
);
