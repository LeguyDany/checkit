import 'package:checkit/controller/user.dart';
import 'package:checkit/pages/components/general/title.dart';
import 'package:checkit/pages/components/snackbars/general_snackbar.dart';
import 'package:checkit/pages/components/snackbars/log_reg_snackbar.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../../../config/globals.dart';
import '../general/CTA0.dart';
import '../general/CTA2.dart';
import '../general/input_text.dart';

void modalChangeUsername(
    BuildContext context) async {
  final TextEditingController newUsername = TextEditingController();
  final TextEditingController password = TextEditingController();

  showModalBottomSheet<void>(
    isScrollControlled: true,
    backgroundColor: white,
    shape: const RoundedRectangleBorder(
      borderRadius: BorderRadius.only(
          topRight: Radius.circular(20), topLeft: Radius.circular(20)),
    ),
    context: context,
    builder: (BuildContext context) {
      return ListView(shrinkWrap: true, children: <Widget>[
        Container(
          padding:
              const EdgeInsets.only(top: 30, left: 30, right: 30, bottom: 20),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: <Widget>[
              const Title1(title: "Change username"),
              InputText(
                  label: "New username",
                  placeholder: "Enter your new username",
                  targetVariable: newUsername),
              InputText(
                  label: "Password",
                  placeholder: "Enter your password",
                  obscure: true,
                  targetVariable: password),
              const SizedBox(
                height: 10,
              ),
              Row(
                children: [
                  CTA2(
                    content: 'Back',
                    onPress: () {
                      context.pop();
                    },
                    isDisabled: false,
                    marginTop: 20,
                  ),
                  const SizedBox(
                    width: 20,
                  ),
                  CTA0(
                    content: 'Set',
                    onPress: () async {
                      if (password.text == '' || newUsername.text == '') {
                        generalSnackbar(context, const Text("The username or password is missing."), (){});
                        return ;
                      }

                      await notifySnackbar(
                          context,
                          user.changeUsername(newUsername.text, password.text),
                          "Username updated.",
                          "Error", () {
                        context.push('/settings');
                      }, 4);

                    },
                    isDisabled: false,
                    marginTop: 20,
                  ),
                ],
              )
            ],
          ),
        ),
      ]);
    },
  );
}