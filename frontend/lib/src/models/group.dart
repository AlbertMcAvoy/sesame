class Group {
  final int id;
  final String name;
  final int userId;

  Group({required this.id, required this.name, required this.userId});

  factory Group.fromJson(Map<String, dynamic> json) {
    return Group(
      id: json['id'],
      name: json['name'],
      userId: json['user_id'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name': name,
      'user_id': userId,
    };
  }
}
