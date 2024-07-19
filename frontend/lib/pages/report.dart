import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'package:localstorage/localstorage.dart';

const List<String> list = <String>[
  'Door',
  'Toilet',
  'Supply',
  'Cleanliness',
  'Other'
];
void main() => runApp(const ReportToilette());

class ReportToilette extends StatelessWidget {
  const ReportToilette({super.key});

  @override
  Widget build(BuildContext context) {
    return const ReportToiletteForm();
  }
}

class ReportToiletteForm extends StatefulWidget {
  const ReportToiletteForm({super.key});

  @override
  State<ReportToiletteForm> createState() => _ReportToiletteFormState();
}

class _ReportToiletteFormState extends State<ReportToiletteForm> {
  String dropdownValue = list.first;
  String commentaire = "";
  @override
  Widget build(BuildContext context) {
    return Form(
      child: Column(
        children: <Widget>[
          const Padding(
            padding: EdgeInsets.fromLTRB(0, 100, 0, 5),
            child: DefaultTextStyle(
              style: TextStyle(
                color: Colors.black,
                fontSize: 26,
              ),
              child: Text('Type de probléme'),
            ),
          ),
          const Padding(
            padding: EdgeInsets.fromLTRB(0, 25, 0, 5),
            child: DefaultTextStyle(
              style: TextStyle(
                color: Colors.black,
                fontSize: 18,
              ),
              child: Text('Choisissez un theme'),
            ),
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
          const Padding(
            padding: EdgeInsets.fromLTRB(0, 25, 0, 5),
            child: DefaultTextStyle(
              style: TextStyle(color: Colors.black, fontSize: 18),
              child: Text('Description'),
            ),
          ),
          Padding(
            padding: const EdgeInsets.fromLTRB(130, 0, 130, 25),
            child: TextFormField(
              onChanged: (value) {
                commentaire = value;
              },
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
                  const EdgeInsets.fromLTRB(100, 20, 100, 20)),
            ),
            onPressed: () {
              reportToilette(context, 1, dropdownValue, "Todo", commentaire);
            },
            child: const Text('Envoyer'),
          )
        ],
      ),
    );
  }
}

Future<void> reportToilette(
    context, water_closet_id, topic, state, comment) async {
  var date = DateTime.now().toIso8601String();
  print('http://localhost:8080/reports/${localStorage.getItem('auth')}');
  final response = await http.post(
    Uri.parse('http://localhost:8080/reports/${localStorage.getItem('auth')}'),
    headers: <String, String>{
      'Content-Type': 'application/json; charset=UTF-8',
    },
    body: jsonEncode(<String, Object>{
      'water_closet_id': water_closet_id,
      'datetime': date,
      'state': state,
      'topic': topic,
      'comment': comment
    }),
  );
  if (response.statusCode == 201) {
    // If the server did return a 201 CREATED response,
    // then parse the JSON.
    print(jsonDecode(response.body));
    Navigator.pushNamed(context, '/returnOk');

    return;
  } else {
    // If the server did not return a 201 CREATED response,
    // then throw an exception.
    print("erreur");
    throw Exception('Failed to create report.');
  }
}
