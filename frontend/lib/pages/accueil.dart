import 'dart:html';

import 'package:flutter/material.dart';

class ListToilette extends StatefulWidget {
  const ListToilette({super.key});

  @override
  // ignore: library_private_types_in_public_api
  _ListToiletteState createState() => _ListToiletteState();
}

class _ListToiletteState extends State<ListToilette> {
  @override
  Widget build(BuildContext context) {
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
