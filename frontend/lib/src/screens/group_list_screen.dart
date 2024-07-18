import 'package:flutter/material.dart';
import 'package:frontend/src/services/api-service/place_service.dart';
import '../models/group.dart';
import '../models/place.dart';
import '../services/api-service/group_service.dart';
import 'group_list_item.dart';

class GroupListScreen extends StatefulWidget {
  @override
  _GroupListScreenState createState() => _GroupListScreenState();
}

class _GroupListScreenState extends State<GroupListScreen> {
  late Future<List<Group>> futureGroups;
  late Future<List<Place>> futurePlaces;
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
      appBar: AppBar(
        title: Text('Groups'),
      ),
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
                      return ListTile(
                        title: Text('Error loading place for ${groups[index].name}'),
                        subtitle: Text(placeSnapshot.error.toString()),
                      );
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
