import 'package:checkit/controller/auth.dart';
import 'package:checkit/pages/components/general/header.dart';
import 'package:checkit/pages/login.dart';
import 'package:checkit/utils/loading.dart';
import 'package:flutter/material.dart';
import '../controller/response.dart';

class Settings extends StatefulWidget {
  const Settings({Key? key}) : super(key: key);

  @override
  State<Settings> createState() => _SettingsState();
}

class _SettingsState extends State<Settings> {
  bool redirect = false;

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
        future: auth.checkIsAuth(),
        builder: (context, AsyncSnapshot<Response> snapshot) {
          if (snapshot.hasError) {
            return const Login();
          }
          if (!snapshot.hasData) {
            return const RedLoader();
          }
          if (snapshot.data?.success == false) {
            return const Login();
          }

          return Scaffold(
            body: Column(
              children: const [
                Header(title: "Settings"),
              ],
            ),
          );
        });
  }
}
