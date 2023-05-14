import 'package:flutter/material.dart';
import 'package:checkit/config/globals.dart';
import 'package:flutter_svg/flutter_svg.dart';

class Header extends StatelessWidget {
  const Header({Key? key, this.message, required this.title}) : super(key: key);
  final String title;
  final String? message;

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: red0,
        boxShadow: [
          shadow0,
        ],
      ),
      width: MediaQuery.of(context).size.width,
      height: message != null ? 165 : 122,
      child: Stack(
        children: [
          Positioned.fill(
            child: SvgPicture.asset(
              "assets/images/dotted_pattern.svg",
              fit: BoxFit.cover,
            ),
          ),
          Center(
            child: Container(
              padding: const EdgeInsets.fromLTRB(30, 50, 30, 20),
              child: Column(
                children: [
                  Text(
                    title,
                    style: getTextStyle("h1", white),
                    textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: 10),
                  message != null
                      ? Column(
                          children: [
                            Text(
                              "A very simplistic to-do list.",
                              style: getTextStyle("h3", white),
                              textAlign: TextAlign.center,
                            ),
                            const SizedBox(height: 3),
                            SizedBox(
                              height: 1.5,
                              width: 225,
                              child: Container(color: white),
                            )
                          ],
                        )
                      : const SizedBox.shrink(),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
