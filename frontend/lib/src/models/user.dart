enum Role { USER, CLIENT, WORKER, ADMIN }

class User {
  final int id;
  final String mail;
  final String phone;
  final Role role;

  User({required this.id, required this.mail, required this.phone, required this.role});

  factory User.fromJson(Map<String, dynamic> json) {
    return User(
      id: json['id'],
      mail: json['mail'],
      phone: json['phone'],
      role: Role.values.firstWhere((e) => e.toString() == 'Role.' + json['role']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'mail': mail,
      'phone': phone,
      'role': role.toString().split('.').last,
    };
  }
}
