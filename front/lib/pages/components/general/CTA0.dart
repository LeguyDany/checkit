import 'package:checkit/config/globals.dart';
import 'package:flutter/material.dart';

class CTA0 extends StatelessWidget {
  const CTA0({Key? key, required this.content, required this.onPress})
      : super(key: key);
  final String content;
  final VoidCallback onPress;

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(top: 30, bottom: 10),
      child: Stack(children: [
        ElevatedButton(
          onPressed: onPress,
          style: ElevatedButton.styleFrom(
            padding: const EdgeInsets.fromLTRB(25, 8, 25, 13),
            backgroundColor: red0,
            elevation: 0,
          ),
          child: Text(
            content,
            style: getTextStyle('h3', white),
          ),
        ),
      ]),
    );
  }
}
