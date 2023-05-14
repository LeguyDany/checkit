import 'dart:async';
import 'package:checkit/controller/auth.dart';
import 'package:checkit/pages/components/general/CTA0.dart';
import 'package:checkit/pages/components/general/CTA1.dart';
import 'package:checkit/pages/components/general/buttom_nav_bar.dart';
import 'package:checkit/pages/components/general/input_text.dart';
import 'package:checkit/pages/components/snackbars/log_reg_snackbar.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'components/general/header.dart';
import 'components/general/title.dart';
import 'package:lottie/lottie.dart';

class Login extends StatefulWidget {
  const Login({Key? key}) : super(key: key);

  @override
  State<Login> createState() => _LoginState();
}

class _LoginState extends State<Login> {
  final TextEditingController username = TextEditingController();
  final TextEditingController password = TextEditingController();
  late bool _buttonIsDisabled = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SingleChildScrollView(
        child: Container(
          padding: const EdgeInsets.all(30),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Title1(title: 'Login'),
              InputText(
                  label: "Username",
                  placeholder: "Enter your username",
                  targetVariable: username),
              InputText(
                  label: "Password",
                  placeholder: "Enter your password",
                  targetVariable: password,
                  obscure: true),
              CTA0(
                isDisabled: _buttonIsDisabled,
                content: "Login",
                onPress: () async {
                  setState(() {
                    _buttonIsDisabled = true;
                  });

                  await notifySnackbar(
                    context,
                    auth.login(username.text, password.text),
                    "Login success",
                    "Login failed",
                    () {
                      context.push("/settings");
                    },
                  );

                  Timer(const Duration(seconds: 2), () {
                    setState(() {
                      _buttonIsDisabled = false;
                    });
                  });
                },
              ),
              CTA1(
                content: "No account? Register here.",
                onPress: () => {context.push('/login/register')},
              ),
            ],
          ),
        ),
      ),
    );
  }
}
