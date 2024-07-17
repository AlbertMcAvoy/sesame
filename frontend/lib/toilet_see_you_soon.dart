import 'package:flutter/material.dart';
import 'manage_door.dart';

class ToiletSeeYouSoon extends StatefulWidget {
  @override
  _ToiletSeeYouSoonState createState() => _ToiletSeeYouSoonState();
}

class _ToiletSeeYouSoonState extends State<ToiletSeeYouSoon> {
    @override
    Widget build(BuildContext context) {
        return ManageDoor(
            title: "Au plaisir de vous revoir !",
            imageUrl: 'assets/images/toilet_see_you_soon.png',
            link: "Un problème ?",
        );
    }
}