import 'package:flutter/material.dart';
import '../src/models/group.dart';
import '../src/models/place.dart';
import '../src/screens/group_list_item.dart';
import '../src/services/api-service/group_service.dart';
import '../src/services/api-service/place_service.dart';

class ListToilette extends StatefulWidget {
  const ListToilette({super.key});

  @override
  _ListToiletteState createState() => _ListToiletteState();
}

class _ListToiletteState extends State<ListToilette> {
  late Future<List<Group>> futureGroups;
  GroupService groupService = GroupService();
  PlaceService placeService = PlaceService();

  @override
  void initState() {
    super.initState();
    futureGroups = groupService.fetchGroups();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: FutureBuilder<List<Group>>(
        future: futureGroups,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return Center(child: CircularProgressIndicator());
          } else if (snapshot.hasError) {
            return Center(child: Text('Error: ${snapshot.error}'));
          } else if (!snapshot.hasData || snapshot.data!.isEmpty) {
            return Center(child: Text('No groups found'));
          } else {
            List<Group> groups = snapshot.data!;
            return ListView.builder(
              itemCount: groups.length,
              itemBuilder: (context, index) {
                return FutureBuilder<Place>(
                  future: placeService.fetchPlace(groups[index].id),
                  builder: (context, placeSnapshot) {
                    if (placeSnapshot.connectionState == ConnectionState.waiting) {
                      return ListTile(
                        title: Text(groups[index].name),
                        subtitle: Text('Loading place...'),
                      );
                    } else if (placeSnapshot.hasError) {
                      Place place = placeSnapshot.data!;
                      return GroupListItem(group: groups[index], place: place);
                    } else if (!placeSnapshot.hasData) {
                      return ListTile(
                        title: Text(groups[index].name),
                        subtitle: Text('No place found'),
                      );
                    } else {
                      Place place = placeSnapshot.data!;
                      return GroupListItem(group: groups[index], place: place);
                    }
                  },
                );
              },
            );
          }
        },
      ),
    );
  }
}
