import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../../../config/globals.dart';

class BottomNavBar extends StatefulWidget {
  const BottomNavBar({Key? key}) : super(key: key);

  @override
  State<BottomNavBar> createState() => _BottomNavBarState();
}

class _BottomNavBarState extends State<BottomNavBar> {
  int pageIndex = 0;

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        boxShadow: [
          shadow1
        ]
      ),
      child: BottomNavigationBar(
          currentIndex: pageIndex,
          items: [
            BottomNavigationBarItem(
              icon: Icon(Icons.home, color: pageIndex == 0 ? red0 : medium2),
              label: 'Home',
            ),
            BottomNavigationBarItem(
              icon: Icon(Icons.list, color: pageIndex == 1 ? red0 : medium2),
              label: 'Templates',
            ),
            BottomNavigationBarItem(
              icon: Icon(Icons.add, color: pageIndex == 2 ? red0 : medium2),
              label: 'Add',
            ),
            BottomNavigationBarItem(
              icon: Icon(Icons.settings, color: pageIndex == 3 ? red0 : medium2),
              label: 'Settings',
            ),
          ],
          unselectedItemColor: Colors.red,
          selectedItemColor: red0,
          onTap: (index) {
            setState(() {
              pageIndex = index;
            });
            GoRouter.of(context).go(pagesList[index]);
          }),
    );
  }
}
