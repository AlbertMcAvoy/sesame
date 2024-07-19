import 'dart:convert';
import 'package:http/http.dart' as http;
import '../../models/group.dart';
import '../../models/place.dart';

class GroupService {
  static const String baseUrl = 'http://localhost:8080';

  Future<List<Group>> fetchGroups() async {
    final response = await http.get(Uri.parse('$baseUrl/groups'));

    if (response.statusCode == 200) {
      List<dynamic> data = json.decode(response.body);
      print('Response data 1: $data');
      return data.map((groupData) => Group.fromJson(groupData)).toList();
    } else {
      throw Exception('Failed to load groups');
    }
  }
}
