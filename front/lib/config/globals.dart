import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

// Color styles
const black = Color(0xff000000);
const dark0 = Color(0xff232c43);
const dark1 = Color(0xff3a4c78);
const dark2 = Color(0xff47609e);
const dark3 = Color(0xff6e86c4);
const medium0 = Color(0xff8ca2d9);
const medium1 = Color(0xffadbfeb);
const medium2 = Color(0xffd4def7);
const light0 = Color(0xffd4def7);
const light1 = Color(0xfffafcfe);
const white = Color(0xffffffff);
const red0 = Color(0xffd7263d);
const red1 = Color(0xffe54d60);
const red2 = Color(0xfff07585);
const red3 = Color(0xfff7a1ac);

// Font styles
TextStyle getTextStyle(String type, Color color, [double? lineHeight]) {
  final height = lineHeight ?? 1;
  switch (type) {
    case "h1":
      return TextStyle(fontSize: 32, fontWeight: FontWeight.w800, color: color, height: height);
    case "h2":
      return TextStyle(fontSize: 28, fontWeight: FontWeight.w600, color: color, height: height);
    case "h3":
      return TextStyle(fontSize: 24, fontWeight: FontWeight.w500, color: color, height: height);
    case "h4":
      return TextStyle(fontSize: 20, fontWeight: FontWeight.w500, color: color, height: height);
  }

  return TextStyle(fontSize: 20, fontWeight: FontWeight.w400, color: color, height: height);
}

// Shadows styles
final shadow0 = BoxShadow(
  color: Colors.black.withOpacity(0.23),
  blurRadius: 10,
  offset: const Offset(0, 5), // changes position of shadow
);
final shadow1 = BoxShadow(
  color: Colors.black.withOpacity(0.04),
  blurRadius: 10,
  offset: const Offset(0, -5), // changes position of shadow
);
final shadow2 = BoxShadow(
  color: Colors.black.withOpacity(0.15),
  blurRadius: 10,
);

// Global page index
List<String> pagesList = [
  '/tasks',
  '/templates',
  '/settings',
  '/login',
  '/register'
];

// Local storage
const storage = FlutterSecureStorage();