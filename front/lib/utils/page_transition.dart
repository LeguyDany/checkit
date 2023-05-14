import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import '../config/globals.dart';

CustomTransitionPage pageTransition1(Widget page, String currentPage, GoRouterState state) {
  return CustomTransitionPage(
      key: state.pageKey,
      child: page,
      transitionsBuilder: (context, animation, secondaryAnimation, child) {
        const begin = Offset(1.0, 0.0);
        // final targetPage = GoRouter.of(context).location;
        // final begin = getPageTransitionDirection(currentPage, targetPage);
        const end = Offset.zero;
        var curve = Curves.easeInOut;
        var curveTween = CurveTween(curve: curve);
        final tween = Tween(begin: begin, end: end).chain(curveTween);
        final offsetAnimation = animation.drive(tween);

        return SlideTransition(
          position: offsetAnimation,
          child: child,
        );
      });
}

Offset getPageTransitionDirection(String currentPage, String targetPage) {
  if (pagesList.indexOf(currentPage) < pagesList.indexOf(targetPage)) {
    return const Offset(0.0, 1.0);
  }
  return const Offset(1.0, 0.0);
}
