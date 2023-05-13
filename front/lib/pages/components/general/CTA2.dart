import 'package:checkit/config/globals.dart';
import 'package:flutter/material.dart';

class CTA2 extends StatelessWidget {
  const CTA2({Key? key, required this.content, required this.onPress, required this.isDisabled})
      : super(key: key);
  final String content;
  final VoidCallback onPress;
  final bool isDisabled;

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(top: 30, bottom: 10),
      child: Stack(children: [
        OutlinedButton(
          onPressed: isDisabled ? null : onPress,
          style: OutlinedButton.styleFrom(
            side: const BorderSide(width: 2, color: red0),
            padding: const EdgeInsets.fromLTRB(25, 8, 25, 13),
            foregroundColor: isDisabled ? medium2 : red0,
            elevation: 0,
          ),
          child: Text(
            content,
            style: getTextStyle('h3', red0),
          ),
        ),
      ]),
    );
  }
}
