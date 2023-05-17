import 'package:checkit/config/globals.dart';
import 'package:flutter/material.dart';

class CTAAdd extends StatelessWidget {
  const CTAAdd({
    Key? key,
    required this.onPress,
    required this.isDisabled,
  }) : super(key: key);
  final VoidCallback onPress;
  final bool isDisabled;

  @override
  Widget build(BuildContext context) {
    return Container(
      // decoration: BoxDecoration(
      //   boxShadow: [shadow2],
      // ),
      child: Stack(children: [
        OutlinedButton(
            onPressed: isDisabled ? null : onPress,
            style: OutlinedButton.styleFrom(
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(10),
              ),
              side: BorderSide(width: 2, color: isDisabled ? medium2 : red0),
              padding: const EdgeInsets.all(12),
              backgroundColor: white,
              foregroundColor: red0,
              elevation: 0,
            ),
            child: Icon(
              Icons.add,
              color: isDisabled ? medium2 :  red0,
              size: 36,
            )),
      ]),
    );
  }
}
