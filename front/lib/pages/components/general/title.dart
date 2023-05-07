import 'package:flutter/material.dart';
import 'package:checkit/config/globals.dart';

class Title1 extends StatelessWidget {
  const Title1({Key? key, required this.title}) : super(key: key);
  final String title;

  @override
  Widget build(BuildContext context) {
    return Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
      Text(
        title,
        style: getTextStyle("h2", dark0),
        textAlign: TextAlign.start,
      ),
      const SizedBox(
        height: 5,
      ),
      SizedBox(
        height: 2,
        width: MediaQuery.of(context).size.width,
        child: Container(color: medium2),
      ),
    ]);
  }
}
