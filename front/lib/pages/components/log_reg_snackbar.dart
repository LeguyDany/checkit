import 'package:flutter/material.dart';
import '../../controller/response.dart';
import '../../utils/loading.dart';
//
// class NotifySnackbar extends StatefulWidget {
//   const NotifySnackbar(
//       {Key? key,
//       required this.request,
//       required this.caseSuccess,
//       required this.caseFail,
//       required this.updateLoading})
//       : super(key: key);
//   final Future<Response> request;
//   final String caseSuccess;
//   final String caseFail;
//   final Function(bool) updateLoading;
//
//   @override
//   State<NotifySnackbar> createState() => _NotifySnackbarState();
// }
//
// class _NotifySnackbarState extends State<NotifySnackbar> {
//
//   @override
//   Widget build(BuildContext context) {
//     return SnackBar(
//       duration: const Duration(seconds: 3),
//       content: FutureBuilder<Response>(
//           future: widget.request,
//           builder: (context, snapshot) {
//             widget.updateLoading(true);
//
//             if (!snapshot.hasData) {
//               return const WhiteLoader();
//             }
//
//             final Response response = snapshot.data!;
//             if (response.success == false) {
//               return Text("$widget.caseFail: ${response.data}");
//             }
//
//             widget.updateLoading(false);
//             return Text(widget.caseSuccess);
//           }),
//     );
//   }
// }


Future<bool> notifySnackbar(BuildContext context,Future<Response> request, String caseSuccess, String caseFail) async {
  final Future<Response> res = request;
  ScaffoldMessenger.of(context).showSnackBar(
    SnackBar(
      duration: const Duration(seconds: 3),
      content: FutureBuilder<Response>(
          future: res,
          builder: (context, snapshot) {
            if (!snapshot.hasData) {
              return const WhiteLoader();
            }

            final Response response = snapshot.data!;
            if (response.success == false) {
              return Text("$caseFail: ${response.data}");
            }

            return Text(caseSuccess);
          }),
    ),
  );

  return true;
}
