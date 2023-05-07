import 'package:checkit/pages/components/general/CTA0.dart';
import 'package:checkit/pages/components/general/CTA1.dart';
import 'package:checkit/pages/components/general/input_text.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'components/general/header.dart';
import 'components/general/title.dart';
import 'package:lottie/lottie.dart';

class Register extends StatefulWidget {
  const Register({Key? key}) : super(key: key);

  @override
  State<Register> createState() => _RegisterState();
}

class _RegisterState extends State<Register> {
  final TextEditingController username = TextEditingController();
  final TextEditingController password = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: SingleChildScrollView(
          child: Column(children: [
            const Header(
                title: "Check'it", message: "A very simplistic to-do list."),
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
                  CTA0(
                    content: "Register",
                    onPress: () {
                      print("Username: ${username.text}");
                      print("Password: ${password.text}");
                    },
                  ),
                  CTA1(content: "Log in here.", onPress: () => {context.go("/")},),
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
