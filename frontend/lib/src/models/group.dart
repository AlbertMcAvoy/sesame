class Group {
  final int id;
  final String name;
  final int placeId;

  Group({
    required this.id,
    required this.name,
    required this.placeId,
  });

  factory Group.fromJson(Map<String, dynamic> json) {
    return Group(
      id: json['id'] ?? 0,
      name: json['name'] ?? 'Unknown',
      placeId: json['placeId'] ?? 0,
    );
  }
}
