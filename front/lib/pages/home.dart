import 'package:checkit/config/globals.dart';
import 'package:checkit/controller/auth.dart';
import 'package:checkit/helper/api_requests.dart';
import 'package:checkit/pages/components/general/CTA1.dart';
import 'package:checkit/pages/components/general/CTA_Add.dart';
import 'package:checkit/pages/components/general/buttom_nav_bar.dart';
import 'package:checkit/pages/components/general/header.dart';
import 'package:checkit/pages/components/modal/add_task.dart';
import 'package:flutter/material.dart';

import 'components/general/CTA2.dart';

class HomePage extends StatefulWidget {
  const HomePage({Key? key}) : super(key: key);

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        color: light1,
        child: Stack(children: [
          Positioned(
            bottom: 20,
            right: 20,
            child: CTAAdd(
                onPress: () {
                  modalAddTask(context);
                },
                isDisabled: false),
          ),
          Container(
            padding: const EdgeInsets.all(30),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                SizedBox(
                  width: MediaQuery.of(context).size.width,
                ),
                CTA2(
                  content: 'Try2',
                  onPress: () async {
                    print(apiRequests.header);
                  },
                  isDisabled: false,
                ),
                CTA2(
                  content: 'Try3',
                  onPress: () async {
                    apiRequests.header['Authorization'] = '';
                    storage.write(key: "userToken", value: "");
                  },
                  isDisabled: false,
                ),
              ],
            ),
          ),
        ]),
      ),
    );
  }
}
