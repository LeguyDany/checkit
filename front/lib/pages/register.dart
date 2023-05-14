import 'dart:async';
import 'package:checkit/controller/auth.dart';
import 'package:checkit/pages/components/general/CTA0.dart';
import 'package:checkit/pages/components/general/CTA1.dart';
import 'package:checkit/pages/components/general/input_text.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'components/general/CTA2.dart';
import 'components/general/header.dart';
import 'components/general/title.dart';
import 'package:lottie/lottie.dart';
import 'components/log_reg_snackbar.dart';

class Register extends StatefulWidget {
  const Register({Key? key}) : super(key: key);

  @override
  State<Register> createState() => _RegisterState();
}

class _RegisterState extends State<Register> {
  final TextEditingController username = TextEditingController();
  final TextEditingController password = TextEditingController();
  late bool isLoading = false;
  bool _buttonIsDisabled = false;

  void updateLoading(bool value) {
    setState(() {
      isLoading = value;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: SingleChildScrollView(
      child: Column(children: [
        Container(
          padding: const EdgeInsets.all(30),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Title1(title: 'Register'),
              InputText(
                  label: "Username",
                  placeholder: "Enter your username",
                  targetVariable: username),
              InputText(
                  label: "Password",
                  placeholder: "Enter your password",
                  targetVariable: password,
                  obscure: true),
              Row(
                children: [
                  CTA2(
                      content: "Back",
                      onPress: () => context.pop(),
                      isDisabled: false),
                  const SizedBox(
                    width: 20,
                  ),
                  CTA0(
                      isDisabled: _buttonIsDisabled,
                      content: "Register",
                      onPress: () {
                        setState(() {
                          _buttonIsDisabled = true;
                        });
                        notifySnackbar(
                          context,
                          auth.register(username.text, password.text),
                          'Registration succeeded',
                          'Registration failed',
                          () {
                            context.push("/settings");
                          },
                        );

                        Timer(const Duration(seconds: 2), () {
                          setState(() {
                            _buttonIsDisabled = false;
                          });
                        });
                      }),
                ],
              ),
              Transform.translate(
                offset: const Offset(0, -10),
                child: Center(
                  child: SizedBox(
                      height: 1,
                      child: Lottie.asset(
                        repeat: false,
                        'assets/images/REGISTER.json',
                        fit: BoxFit.cover,
                        width: MediaQuery.of(context).size.width - 100,
                      )),
                ),
              ),
            ],
          ),
        ),
      ]),
    ));
  }
}
