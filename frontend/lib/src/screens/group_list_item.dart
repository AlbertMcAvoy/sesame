import 'package:flutter/material.dart';
import '../models/group.dart';
import '../models/place.dart';

class GroupListItem extends StatelessWidget {
  final Group group;
  final Place place;

  GroupListItem({required this.group, required this.place});

  @override
  Widget build(BuildContext context) {
    return ListTile(
      title: Text(group.name),
      subtitle: Text(place.coordinates),
    );
  }
}
