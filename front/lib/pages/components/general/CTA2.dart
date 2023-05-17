import 'package:checkit/config/globals.dart';
import 'package:flutter/material.dart';

class CTA2 extends StatelessWidget {
  const CTA2({Key? key, required this.content, required this.onPress, required this.isDisabled, this.marginTop, this.marginBottom})
      : super(key: key);
  final double? marginTop;
  final double? marginBottom;
  final String content;
  final VoidCallback onPress;
  final bool isDisabled;

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: EdgeInsets.only(top: marginTop ?? 30, bottom: marginBottom ?? 10),
      child: Stack(children: [
        OutlinedButton(
          onPressed: isDisabled ? null : onPress,
          style: OutlinedButton.styleFrom(
            side: const BorderSide(width: 2, color: red0),
            padding: const EdgeInsets.fromLTRB(25, 15, 25, 15),
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
