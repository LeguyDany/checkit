import 'package:checkit/pages/components/general/buttom_nav_bar.dart';
import 'package:checkit/pages/components/general/header.dart';
import 'package:checkit/pages/home.dart';
import 'package:checkit/pages/login.dart';
import 'package:checkit/pages/register.dart';
import 'package:checkit/pages/settings.dart';
import 'package:checkit/utils/appbar_title.dart';
import 'package:checkit/utils/page_transition.dart';
import 'package:go_router/go_router.dart';
import 'package:flutter/material.dart';

final GoRouter router = GoRouter(
  initialLocation: '/tasks',
  routes: <RouteBase>[
    ShellRoute(
      builder: (context, state, child) {
        return Scaffold(
          appBar: PreferredSize(
            preferredSize: const Size.fromHeight(75.0),
            child: Header(
              title: getAppBarTitle(state.location),
            ),
          ),
          body: child,
          bottomNavigationBar: const BottomNavBar(),
        );
      },
      routes: [
        GoRoute(
          path: '/tasks',
          pageBuilder: (context, state) =>
              pageTransition1(const HomePage(), '/', state),
        ),
        GoRoute(
          path: '/settings',
          pageBuilder: (context, state) =>
              pageTransition1(const Settings(), '/settings', state),
        ),
        GoRoute(
            path: '/login',
            builder: (BuildContext context, GoRouterState state) {
              return const Login();
            },
            routes: <RouteBase>[
              GoRoute(
                path: 'register',
                builder: (BuildContext context, GoRouterState state) {
                  return const Register();
                },
              ),
            ]),
      ],
    ),
  ],
);
