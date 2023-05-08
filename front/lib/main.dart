import 'package:checkit/router.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:flutter/material.dart';

main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await dotenv.load(fileName: ".env");
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
      return MaterialApp.router(
        routerConfig: router,
        title: 'Checkit',
        theme: ThemeData(fontFamily: 'IBMPlexSans'),
        debugShowCheckedModeBanner: false,
    );
  }
}
