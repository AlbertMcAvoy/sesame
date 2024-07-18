import 'dart:html';

import 'package:flutter/material.dart';
import '../layout.dart';

class ReturnOk extends StatefulWidget {
  const ReturnOk({super.key});

  @override
  // ignore: library_private_types_in_public_api
  _ReturnOkState createState() => _ReturnOkState();
}

class _ReturnOkState extends State<ReturnOk> {
  @override
  Widget build(BuildContext context) {
    returnHomeTimeout();
    return Text("Votre retour a bien été envoyé ! ");
  }
  Future<void> returnHomeTimeout() async {  
    // Simulate a delay of 3 seconds  
    await Future.delayed(Duration(seconds: 5));  
      
    Navigator.pushNamed(context, '/');
  }  
}
