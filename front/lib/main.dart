import 'package:checkit/pages/login.dart';
import 'package:checkit/pages/register.dart';
import 'package:checkit/router.dart';
import 'package:go_router/go_router.dart';
import 'package:flutter/material.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
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
