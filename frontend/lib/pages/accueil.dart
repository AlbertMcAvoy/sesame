import 'dart:html';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'package:flutter/material.dart';

class ListToilette extends StatefulWidget {
  const ListToilette({super.key});

  @override
  // ignore: library_private_types_in_public_api
  _ListToiletteState createState() => _ListToiletteState();
}

// Future<Toilette> getAllToilettes() async {
//   final response = await http.post(
//     Uri.parse('http://localhost'),
//     headers: <String, String>{
//       'Content-Type': 'application/json; charset=UTF-8',
//     },
//   );

//   if (response.statusCode == 201) {
//     // If the server did return a 201 CREATED response,
//     // then parse the JSON.
//     return Toilette.fromJson(jsonDecode(response.body) as Map<String, dynamic>);
//   } else {
//     // If the server did not return a 201 CREATED response,
//     // then throw an exception.
//     throw Exception('Failed to create album.');
//   }
// }

class Toilette {
  final String nom;
  final int distance;

  const Toilette({required this.nom, required this.distance});

  factory Toilette.fromJson(Map<String, dynamic> json) {
    return switch (json) {
      {
        'nom': String nom,
        'distance': int distance,
      } =>
        Toilette(
          nom: nom,
          distance: distance,
        ),
      _ => throw const FormatException('Failed to load album.'),
    };
  }
}

class _ListToiletteState extends State<ListToilette> {
  @override
  Widget build(BuildContext context) {
    // print(getAllToilettes());
    return ListView(
      children: const <Widget>[
        Card(
          child: ListTile(
            // leading: Icon(Icons.map),
            title: Text('24, rue Gabillot, 69000 Lyon'),
            subtitle: Text('300 metres'),
            tileColor: Colors.white,
          ),
        ),
        Card(
          child: ListTile(
            title: Text('24, rue Gabillot, 69000 Lyon'),
            subtitle: Text('300 metres'),
            tileColor: Colors.white,
          ),
        ),
        Card(
          child: ListTile(
            title: Text('24, rue Gabillot, 69000 Lyon'),
            subtitle: Text('300 metres'),
            tileColor: Colors.white,
          ),
        ),
      ],
    );
  }
}
