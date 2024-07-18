import 'dart:convert';
import 'package:http/http.dart' as http;
import '../../models/history.dart';

class HistoryService {
  final String baseUrl = 'http://localhost:8080';

  Future<List<History>> fetchHistories() async {
    final response = await http.get(Uri.parse('$baseUrl/histories'));

    if (response.statusCode == 200) {
      Iterable list = json.decode(response.body);
      return list.map((model) => History.fromJson(model)).toList();
    } else {
      throw Exception('Failed to load histories');
    }
  }

  Future<History> createHistory(History history) async {
    final response = await http.post(
      Uri.parse('$baseUrl/histories'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(history.toJson()),
    );

    if (response.statusCode == 201) {
      return History.fromJson(jsonDecode(response.body));
    } else {
      throw Exception('Failed to create history');
    }
  }
}
