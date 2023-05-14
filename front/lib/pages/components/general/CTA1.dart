import 'package:checkit/config/globals.dart';
import 'package:flutter/material.dart';

class CTA1 extends StatelessWidget {
  const CTA1({Key? key, required this.content, required this.onPress})
      : super(key: key);
  final String content;
  final VoidCallback onPress;

  @override
  Widget build(BuildContext context) {
    return TextButton(
      onPressed: onPress,
      style: ButtonStyle(
        tapTargetSize: MaterialTapTargetSize.shrinkWrap,
        padding: MaterialStateProperty.all<EdgeInsets>(
          const EdgeInsets.all(0),
        ),
      ),
      child: Text(
        content,
        style: getTextStyle('h4', red0),
      ),
    );
  }
}
