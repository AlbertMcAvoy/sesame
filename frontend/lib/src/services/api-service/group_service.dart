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
      return data.map((groupData) => Group.fromJson(groupData)).toList();
    } else {
      throw Exception('Failed to load groups');
    }
  }

  Future<Place> fetchPlace(int groupId) async {
    final response = await http.get(Uri.parse('$baseUrl/places/$groupId'));

    if (response.statusCode == 200) {
      final data = json.decode(response.body);
      return Place.fromJson(data);
    } else {
      throw Exception('Failed to load place');
    }
  }
}
