import 'package:flutter/material.dart';
import 'manage_door.dart';

class ToiletOpened extends StatefulWidget {
  @override
  _ToiletOpenedState createState() => _ToiletOpenedState();

}
class _ToiletOpenedState extends State<ToiletOpened> {
    @override
    Widget build(BuildContext context) {
        return ManageDoor(
            title: "Sanitaire ouvert !",
            imageUrl: 'assets/images/toilet_opened.png',
            link: "Un problème ?",
        );
    }
}