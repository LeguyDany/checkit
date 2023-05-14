import 'package:flutter/material.dart';

void generalSnackbar(
    BuildContext context, Widget content, Function() onDismissed,
    [int? duration]) {
  ScaffoldMessenger.of(context)
      .showSnackBar(
        SnackBar(
          duration: Duration(seconds: duration ?? 2),
          content: content,
        ),
      )
      .closed
      .then((_) => onDismissed.call());
}
