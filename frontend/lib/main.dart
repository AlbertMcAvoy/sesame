import 'package:flutter/material.dart';
import 'layout.dart'; // Importer le fichier du widget Layout

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Layout(),
    );
  }
}
