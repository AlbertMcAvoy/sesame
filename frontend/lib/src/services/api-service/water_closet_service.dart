import 'dart:convert';
import 'package:http/http.dart' as http;
import '../../models/water_closet.dart';

class WaterClosetService {
  final String baseUrl = 'http://your-api-url.com';

  Future<List<WaterCloset>> fetchWaterClosets() async {
    final response = await http.get(Uri.parse('$baseUrl/water_closets'));

    if (response.statusCode == 200) {
      Iterable list = json.decode(response.body);
      return list.map((model) => WaterCloset.fromJson(model)).toList();
    } else {
      throw Exception('Failed to load water closets');
    }
  }

  Future<WaterCloset> createWaterCloset(WaterCloset waterCloset) async {
    final response = await http.post(
      Uri.parse('$baseUrl/water_closets'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(waterCloset.toJson()),
    );

    if (response.statusCode == 201) {
      return WaterCloset.fromJson(jsonDecode(response.body));
    } else {
      throw Exception('Failed to create water closet');
    }
  }
}