import 'package:frontend/src/models/place.dart';

class Group {
  final int id;
  final String name;
  final int placeId;
  final Place? place;

  Group({
    required this.id,
    required this.name,
    required this.placeId,
    required this.place,
  });

  factory Group.fromJson(Map<String, dynamic> json) {
    return Group(
      id: json['id'] ?? 0,
      placeId: json['user_id'] ?? 0,
      name: json['name'] ?? 'Unknown',
      place: json['place'] != null ? Place.fromJson(json['place']) : null,
    );
  }
}
