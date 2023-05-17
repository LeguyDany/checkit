import 'package:checkit/config/globals.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../../../utils/input_numeric_minmax.dart';

class InputText extends StatefulWidget {
  const InputText(
      {Key? key,
      required this.placeholder,
      this.label,
      this.obscure = false,
      required this.targetVariable,
      this.widthReduction,
      this.isCentered,
      this.isNumeric = false,
      this.maxLength})
      : super(key: key);
  final bool? isCentered;
  final bool isNumeric;
  final num? widthReduction;
  final int? maxLength;
  final String placeholder;
  final TextEditingController targetVariable;
  final String? label;
  final bool obscure;

  @override
  State<InputText> createState() => _InputTextState();
}

class _InputTextState extends State<InputText> {
  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: MediaQuery.of(context).size.width - (widget.widthReduction ?? 0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          widget.label != null
              ? Container(
                  padding: const EdgeInsets.fromLTRB(0, 30, 0, 0),
                  child: Text(
                    widget.label!,
                    style: getTextStyle("h4", dark0),
                  ),
                )
              : const SizedBox.shrink(),
          TextField(
            keyboardType: widget.isNumeric == true ? TextInputType.number : null,
            textAlign: widget.isCentered != null && widget.isCentered == true
                ? TextAlign.center
                : TextAlign.start,
            controller: widget.targetVariable,
            obscureText: widget.obscure,
            style: getTextStyle('regular', dark1),
            inputFormatters: widget.isNumeric == true
                ? [
                    FilteringTextInputFormatter.digitsOnly,
                    RangeTextInputFormatter(min: 0, max: 12)
                  ]
                : [],
            decoration: InputDecoration(
              contentPadding: const EdgeInsets.fromLTRB(10, 15, 10, 10),
              enabledBorder: const UnderlineInputBorder(
                borderSide: BorderSide(color: red0, width: 2),
              ),
              focusedBorder: const UnderlineInputBorder(
                borderSide: BorderSide(color: red0, width: 3),
              ),
              counterText: "",
              hintText: widget.placeholder,
              hintStyle: getTextStyle('h4', medium2),
            ),
          ),
        ],
      ),
    );
  }
}
