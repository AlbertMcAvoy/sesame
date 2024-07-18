import 'dart:async';
import 'dart:convert';
import 'package:http/http.dart' as http;
import '../../models/place.dart';

class PlaceService {
  static const String baseUrl = 'http://localhost:8080';

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
