import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/main.dart';

void main() {
  testWidgets('App smoke test', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(const ProviderScope(child: AeroSmartApp()));

    // Verify that the app starts.
    // Since we start at /dashboard, we expect to find something related to it.
    // But since pages are placeholders, we might just check for no crash.
    await tester.pumpAndSettle();
  });
}
