import 'package:checkit/config/globals.dart';
import 'package:flutter/material.dart';

class InputText extends StatefulWidget {
  const InputText({Key? key, required this.placeholder, this.label, this.obscure = false, required this.targetVariable, this.widthReduction})
      : super(key: key);
  final num? widthReduction;
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
                  padding: const EdgeInsets.fromLTRB(0, 20, 0, 0),
                  child:
                    Text(widget.label!, style: getTextStyle("h4", dark0)),
                )
              : const SizedBox.shrink(),
          TextField(
            controller: widget.targetVariable,
            obscureText: widget.obscure,
            style: getTextStyle('regular', dark2),
            decoration: InputDecoration(
              contentPadding: const EdgeInsets.fromLTRB(10, 0, 10, 0),
              enabledBorder: const UnderlineInputBorder(
                borderSide: BorderSide(color: red2, width: 2),
              ),
              focusedBorder: const UnderlineInputBorder(
                borderSide: BorderSide(color: red2, width: 3),
              ),
              hintText: widget.placeholder,
              hintStyle: getTextStyle('h4', medium2),
            ),
          ),
        ],
      ),
    );
  }
}
