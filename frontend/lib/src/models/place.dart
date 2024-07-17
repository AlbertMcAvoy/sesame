class Place {
  final int id;
  final int groupId;
  final String coordinates;

  Place({required this.id, required this.groupId, required this.coordinates});

  factory Place.fromJson(Map<String, dynamic> json) {
    return Place(
      id: json['id'],
      groupId: json['group_id'],
      coordinates: json['coordinates'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'group_id': groupId,
      'coordinates': coordinates,
    };
  }
}
