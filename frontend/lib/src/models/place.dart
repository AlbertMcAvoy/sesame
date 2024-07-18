class Place {
  final int id;
  final String name;
  final String coordinates;

  Place({
    required this.id,
    required this.name,
    required this.coordinates,
  });

  factory Place.fromJson(Map<String, dynamic> json) {
    return Place(
      id: json['id'],
      name: json['name'] ?? 'Unknown',
      coordinates: json['coordinates'] ?? 'No coordinates available',
    );
  }
}
