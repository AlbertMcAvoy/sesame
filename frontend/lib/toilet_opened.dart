import 'package:flutter/material.dart';
import 'manage_door.dart';

class ToiletOpened extends StatefulWidget {
  @override
  _ToiletOpenedState createState() => _ToiletOpenedState();
}
class _ToiletOpenedState extends State<ToiletOpened> {
    void initState() {
      super.initState();
    }

    @override
    Widget build(BuildContext context) {
      return Column(
        children: <Widget>[
          ManageDoor(
            title: "Sanitaire ouvert !",
            imageUrl: 'assets/images/toilet_opened.png',
            link: "Un problème ?",
          )
        ]
      );
    }
}