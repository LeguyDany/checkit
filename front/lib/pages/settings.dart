import 'package:checkit/config/globals.dart';
import 'package:checkit/controller/auth.dart';
import 'package:checkit/controller/user.dart';
import 'package:checkit/pages/components/general/CTA1.dart';
import 'package:checkit/pages/components/general/CTA2.dart';
import 'package:checkit/pages/components/general/input_text.dart';
import 'package:checkit/pages/components/general/title.dart';
import 'package:checkit/pages/components/modal/change_password.dart';
import 'package:checkit/pages/components/modal/change_username.dart';
import 'package:checkit/pages/login.dart';
import 'package:checkit/utils/loading.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../controller/response.dart';
import '../helper/api_requests.dart';

class Settings extends StatefulWidget {
  const Settings({Key? key}) : super(key: key);

  @override
  State<Settings> createState() => _SettingsState();
}

class _SettingsState extends State<Settings> {
  bool redirect = false;
  final TextEditingController notionPath = TextEditingController();

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

        return FutureBuilder(
          future: user.getCurrentUser(),
          builder: (context, AsyncSnapshot<Response> snapshot) {
            if (!snapshot.hasData) {
              return const RedLoader();
            }

            final userInfo = snapshot.data;

            return SingleChildScrollView(
              child: Container(
                color: light1,
                padding: const EdgeInsets.all(30),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Title1(title: 'Account Information'),
                    const SizedBox(
                      height: 10,
                    ),
                    Text.rich(TextSpan(
                        text: "ID: ",
                        style: getTextStyle("h4", dark1),
                        children: [
                          TextSpan(
                              text: userInfo?.data["userid"],
                              style: getTextStyle("regular", dark1))
                        ])),
                    Text.rich(TextSpan(
                        text: "Name: ",
                        style: getTextStyle("h4", dark1),
                        children: [
                          TextSpan(
                              text: userInfo?.data["username"],
                              style: getTextStyle("regular", dark1))
                        ])),
                    const SizedBox(
                      height: 20,
                    ),
                    const Title1(title: 'General'),
                    const SizedBox(
                      height: 10,
                    ),
                    CTA1(
                        content: "Change username",
                        onPress: () {
                          modalChangeUsername(context);
                        }),
                    CTA1(
                        content: "Reset password",
                        onPress: () {
                          modalResetPassword(context);
                        }),
                    const SizedBox(
                      height: 20,
                    ),
                    const Title1(title: 'Notion'),
                    const SizedBox(
                      height: 10,
                    ),
                    CTA1(
                        content: "Sync to my Notion",
                        onPress: () {
                          print("TODO");
                        }),
                    const SizedBox(
                      height: 10,
                    ),
                    Text(
                      "Setup a Notion path",
                      style: getTextStyle("h3", dark1),
                    ),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        InputText(
                            placeholder: "PageID", targetVariable: notionPath, widthReduction: 175,),
                        CTA2(
                            marginBottom: 10,
                            marginTop: 10,
                            content: "Set",
                            onPress: () {
                              print("TODO: SET NOTION PATH");
                            },
                            isDisabled: false)
                      ],
                    ),
                    const SizedBox(
                      height: 20,
                    ),
                    const Title1(title: "Exit account"),
                    const SizedBox(
                      height: 10,
                    ),
                    CTA1(
                      content: "Logout",
                      onPress: () async {
                        apiRequests.header['Authorization'] = '';
                        storage.write(key: "userToken", value: "");
                        context.push("/settings");
                      },
                    ),
                  ],
                ),
              ),
            );
          },
        );
      },
    );
  }
}