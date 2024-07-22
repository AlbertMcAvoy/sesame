import 'package:flutter/material.dart';
import '../src/models/group.dart';
import '../src/screens/group_list_item.dart';
import '../src/services/api-service/group_service.dart';

class ListToilette extends StatefulWidget {
  const ListToilette({super.key});

  @override
  State createState() => _ListToiletteState();
}

class _ListToiletteState extends State<ListToilette> {
  late Future<List<Group>> futureGroups;
  GroupService groupService = GroupService();

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
            return const Center(child: CircularProgressIndicator());
          } else if (snapshot.hasError) {
            return Center(child: Text('Error: ${snapshot.error}'));
          } else if (!snapshot.hasData || snapshot.data!.isEmpty) {
            return const Center(child: Text('No groups found'));
          } else {
            List<Group> groups = snapshot.data!;
            return ListView.builder(
              itemCount: groups.length,
              itemBuilder: (context, index) {
                return GroupListItem(group: groups[index]);
              },
            );
          }
        },
      ),
    );
  }
}
