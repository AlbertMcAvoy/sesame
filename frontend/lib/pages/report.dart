import 'package:flutter/material.dart';

const List<String> list = <String>['One', 'Two', 'Three', 'Four'];

void main() => runApp(const ReportToilette());

class ReportToilette extends StatelessWidget {
  const ReportToilette({super.key});

  @override
  Widget build(BuildContext context) {
    return ReportToiletteForm();
  }
}

class ReportToiletteForm extends StatefulWidget {
  const ReportToiletteForm({super.key});

  @override
  State<ReportToiletteForm> createState() => _ReportToiletteFormState();
}

class _ReportToiletteFormState extends State<ReportToiletteForm> {
  String dropdownValue = list.first;

  @override
  Widget build(BuildContext context) {
    return Form(
      child: Column(
        children: <Widget>[
          Padding(
            padding: EdgeInsets.all(30.0),
            child: Text('Choisissez un theme'),
          ),
          DropdownMenu<String>(
            initialSelection: list.first,
            onSelected: (String? value) {
              // This is called when the user selects an item.
              setState(() {
                dropdownValue = value!;
              });
            },
            dropdownMenuEntries:
                list.map<DropdownMenuEntry<String>>((String value) {
              return DropdownMenuEntry<String>(value: value, label: value);
            }).toList(),
          ),
          Padding(
            padding: EdgeInsets.fromLTRB(130, 16, 130, 25),
            child: TextFormField(
              decoration: const InputDecoration(
                hintText: 'Dites nous en plus !',
              ),
              validator: (String? value) {
                if (value == null || value.isEmpty) {
                  return 'Please enter some text';
                }
                return null;
              },
            ),
          ),
          TextButton(
            style: ButtonStyle(
              foregroundColor: MaterialStateProperty.all<Color>(Colors.white),
              backgroundColor:
                  MaterialStateProperty.all<Color>(Color(0xff003366)),
              padding: MaterialStateProperty.all(
                  EdgeInsets.fromLTRB(100, 20, 100, 20)),
            ),
            onPressed: () {
               Navigator.pushNamed(context, '/returnOk');
            },
            child: Text('Envoyer'),
          )
        ],
      ),
    );
  }
}
