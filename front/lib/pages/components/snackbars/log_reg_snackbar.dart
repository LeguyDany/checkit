import 'package:flutter/material.dart';
import '../../../models/response.dart';
import '../../../utils/loading.dart';

Future<bool> notifySnackbar(BuildContext context, Future<Response> request,
    String caseSuccess, String caseFail, Function() onDismissed, [int? duration]) async {
  final Future<Response> res = request;
  bool isSuccess = false;

  ScaffoldMessenger.of(context)
      .showSnackBar(
        SnackBar(
          duration: Duration(seconds: duration ?? 2),
          content: FutureBuilder<Response>(
              future: res,
              builder: (context, snapshot) {
                if (!snapshot.hasData) {
                  return const WhiteLoader();
                }

                final Response response = snapshot.data!;
                if (response.success == false) {
                  isSuccess = false;
                  return Text("$caseFail: ${response.data}");
                }

                isSuccess = true;
                return Text(caseSuccess);
              }),
        ),
      )
      .closed
      .then((_) => isSuccess ? onDismissed.call() : () {});

  return true;
}
