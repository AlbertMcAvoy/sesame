import 'package:flutter/material.dart';
import 'manage_door.dart';

class ToiletNotAvailable extends StatefulWidget {
  @override
  _ToiletNotAvailableState createState() => _ToiletNotAvailableState();
}

class _ToiletNotAvailableState extends State<ToiletNotAvailable> {
    @override
    Widget build(BuildContext context) {
        return ManageDoor(
            title: "Sanitaire pas disponible !",
            imageUrl: 'assets/images/toilet_opened.png',
            link: "Scanner d'autres toilettes à proximité ?",
        );
    }
}