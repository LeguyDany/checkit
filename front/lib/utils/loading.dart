import 'package:flutter/material.dart';
import 'package:lottie/lottie.dart';

class RedLoader extends StatelessWidget {
  const RedLoader({super.key});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Lottie.asset('assets/images/RED_LOADER.json', width: 100, height: 100),
    );
  }
}

class WhiteLoader extends StatelessWidget {
  const WhiteLoader({super.key});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Lottie.asset('assets/images/WHITE_LOADER.json', width: 100, height: 100),
    );
  }
}
