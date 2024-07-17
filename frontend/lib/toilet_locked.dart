import 'package:flutter/material.dart';
import 'manage_door.dart';

class ToiletLocked extends StatefulWidget {
  @override
  _ToiletLockedState createState() => _ToiletLockedState();
}

class _ToiletLockedState extends State<ToiletLocked> {
    @override
    Widget build(BuildContext context) {
        return ManageDoor(
            title: "Terminé ?",
            imageUrl: 'assets/images/toilet_locked.png',
            link: "Un problème ?",
        );
    }
}