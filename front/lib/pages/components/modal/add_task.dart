import 'package:checkit/controller/user.dart';
import 'package:checkit/pages/components/general/input_select.dart';
import 'package:checkit/pages/components/general/title.dart';
import 'package:checkit/pages/components/snackbars/general_snackbar.dart';
import 'package:checkit/pages/components/snackbars/log_reg_snackbar.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../../../config/globals.dart';
import '../general/CTA0.dart';
import '../general/CTA2.dart';
import '../general/input_text.dart';

void modalAddTask(BuildContext context) async {
  final TextEditingController taskContent = TextEditingController();
  final TextEditingController dueTime = TextEditingController();

  showModalBottomSheet<void>(
    barrierColor: Colors.black.withOpacity(0.3),
    isScrollControlled: true,
    backgroundColor: white,
    shape: const RoundedRectangleBorder(
      borderRadius: BorderRadius.only(
          topRight: Radius.circular(20), topLeft: Radius.circular(20)),
    ),
    context: context,
    builder: (BuildContext context) {
      const String selectValue = "";
      const String meridiem = "";

      return ListView(shrinkWrap: true, children: <Widget>[
        Container(
          padding:
              const EdgeInsets.only(top: 30, left: 30, right: 30, bottom: 20),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: <Widget>[
              const Title1(title: "Add new task"),
              InputSelect(
                selectOptions: const [
                  'None',
                ],
                updatedValue: selectValue,
                label: "Template",
              ),
              const SizedBox(
                height: 20,
              ),
              Text(
                'Due time',
                style: getTextStyle("h4", dark0),
              ),
              Row(
                crossAxisAlignment: CrossAxisAlignment.end,
                children: [
                  InputText(
                    isNumeric: true,
                    isCentered: true,
                    maxLength: 2,
                    placeholder: '06',
                    targetVariable: dueTime,
                    widthReduction: 310,
                  ),
                  const SizedBox(
                    width: 20,
                  ),
                  InputSelect(
                    selectOptions: const ['AM', 'PM'],
                    updatedValue: meridiem,
                    length: 98,
                  ),
                ],
              ),
              InputText(
                placeholder: "Enter the content of the task",
                targetVariable: taskContent,
                label: "Task content",
              ),
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
                    content: 'Add',
                    onPress: () {},
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
