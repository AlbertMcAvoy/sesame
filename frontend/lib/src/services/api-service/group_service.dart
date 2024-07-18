import 'dart:convert';
import 'package:http/http.dart' as http;
import '../../models/group.dart';

class GroupService {
  final String baseUrl = 'http://your-api-url.com';

  Future<List<Group>> fetchGroups() async {
    final response = await http.get(Uri.parse('$baseUrl/groups'));

    if (response.statusCode == 200) {
      Iterable list = json.decode(response.body);
      return list.map((model) => Group.fromJson(model)).toList();
    } else {
      throw Exception('Failed to load groups');
    }
  }

  Future<Group> createGroup(Group group) async {
    final response = await http.post(
      Uri.parse('$baseUrl/groups'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(group.toJson()),
    );

    if (response.statusCode == 201) {
      return Group.fromJson(jsonDecode(response.body));
    } else {
      throw Exception('Failed to create group');
    }
  }
}
