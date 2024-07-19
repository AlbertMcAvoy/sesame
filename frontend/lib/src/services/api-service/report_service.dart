import 'dart:convert';
import 'package:http/http.dart' as http;
import '../../models/report.dart';

class ReportService {
  final String baseUrl = 'http://localhost:8080';

  Future<List<Report>> fetchReports() async {
    final response = await http.get(Uri.parse('$baseUrl/reports'));

    if (response.statusCode == 200) {
      Iterable list = json.decode(response.body);
      return list.map((model) => Report.fromJson(model)).toList();
    } else {
      throw Exception('Failed to load reports');
    }
  }

  Future<Report> createReport(Report report) async {
    final response = await http.post(
      Uri.parse('$baseUrl/reports'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(report.toJson()),
    );

    if (response.statusCode == 201) {
      return Report.fromJson(jsonDecode(response.body));
    } else {
      throw Exception('Failed to create report');
    }
  }
}
