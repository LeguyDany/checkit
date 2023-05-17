import 'package:checkit/config/globals.dart';
import 'package:flutter/material.dart';

class InputSelect extends StatefulWidget {
  InputSelect(
      {Key? key,
      required this.selectOptions,
      required this.updatedValue,
      this.length,
      this.label})
      : super(key: key);
  final List<String> selectOptions;
  late String updatedValue;
  final double? length;
  final String? label;

  @override
  State<InputSelect> createState() => _InputSelectState();
}

class _InputSelectState extends State<InputSelect> {
  late String dropDownValue;

  @override
  void initState() {
    super.initState();
    dropDownValue = widget.selectOptions.first;
  }

  @override
  Widget build(BuildContext context) {
    return Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
      widget.label != null
          ? Container(
              padding: const EdgeInsets.fromLTRB(0, 20, 0, 0),
              child: Text(widget.label!, style: getTextStyle("h4", dark0)),
            )
          : const SizedBox.shrink(),
      Container(
        width: widget.length ?? MediaQuery.of(context).size.width,
        decoration: const BoxDecoration(
          border: Border(bottom: BorderSide(color: red0, width: 2)),
        ),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: <Widget>[
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 10),
              child: Text(dropDownValue, style: getTextStyle('h4', dark1)),
            ),
            PopupMenuButton<String>(
              child: const SizedBox(
                height: 40,
                width: 40,
                child: Icon(Icons.keyboard_arrow_down),
              ),
              onSelected: (String? value) {
                setState(() {
                  dropDownValue = value!;
                  widget.updatedValue = value!;
                });
              },
              itemBuilder: (BuildContext context) {
                return widget.selectOptions.map((String value) {
                  String displayValue = value;
                  if (value.length > 10) {
                    displayValue = '${value.substring(0, 10)}...';
                  }
                  return PopupMenuItem<String>(
                    value: value,
                    child: Text(
                      displayValue,
                      style: getTextStyle('h4', dark1),
                    ),
                  );
                }).toList();
              },
            ),
          ],
        ),
      ),
    ]);
  }
}
