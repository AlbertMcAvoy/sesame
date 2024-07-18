import 'dart:convert';
import 'package:http/http.dart' as http;
import '../../models/place.dart';

class PlaceService {
  final String baseUrl = 'http://your-api-url.com';

  Future<List<Place>> fetchPlaces() async {
    final response = await http.get(Uri.parse('$baseUrl/places'));

    if (response.statusCode == 200) {
      Iterable list = json.decode(response.body);
      return list.map((model) => Place.fromJson(model)).toList();
    } else {
      throw Exception('Failed to load places');
    }
  }

  Future<Place> createPlace(Place place) async {
    final response = await http.post(
      Uri.parse('$baseUrl/places'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(place.toJson()),
    );

    if (response.statusCode == 201) {
      return Place.fromJson(jsonDecode(response.body));
    } else {
      throw Exception('Failed to create place');
    }
  }
}
