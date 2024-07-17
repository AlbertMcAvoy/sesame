enum Action { DOOR_OPENING, DOOR_CLOSING, LOCK_OPENING, LOCK_CLOSING, QR_CODE_SCAN, NFC_SCAN }

class History {
  final int id;
  final int waterClosetId;
  final String datetime;
  final Action action;

  History({required this.id, required this.waterClosetId, required this.datetime, required this.action});

  factory History.fromJson(Map<String, dynamic> json) {
    return History(
      id: json['id'],
      waterClosetId: json['water_closet_id'],
      datetime: json['datetime'],
      action: Action.values.firstWhere((e) => e.toString() == 'Action.' + json['action']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'water_closet_id': waterClosetId,
      'datetime': datetime,
      'action': action.toString().split('.').last,
    };
  }
}
