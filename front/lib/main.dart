import 'package:checkit/config/globals.dart';
import 'package:checkit/helper/api_requests.dart';
import 'package:checkit/router.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:flutter/material.dart';

Future<void> main() async {
  // WidgetsFlutterBinding.ensureInitialized();
  await dotenv.load(fileName: ".env");

  String? userToken = await storage.read(key: "userToken");
  apiRequests.header = {"Authorization": userToken ?? ''};

  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      routerConfig: router,
      title: 'Checkit',
      theme: ThemeData(
        dropdownMenuTheme: const DropdownMenuThemeData(
          inputDecorationTheme: InputDecorationTheme(
            isDense: true,
           contentPadding: EdgeInsets.symmetric(horizontal: 0, vertical: 0)
          )
        ),
        fontFamily: 'IBMPlexSans',
        inputDecorationTheme: const InputDecorationTheme(
          isDense: true,
          contentPadding: EdgeInsets.symmetric(horizontal: 0, vertical: 0),
        ),
      ),
      debugShowCheckedModeBanner: false,
    );
  }
}
