enum CleanState { CLEANED, USED, DIRTY, OUT_OF_ORDER }

class WaterCloset {
  final int id;
  final int groupId;
  final bool isDisabled;
  final bool isDoorOpened;
  final bool isDoorLocked;
  final bool isAvailable;
  final CleanState cleanState;

  WaterCloset({required this.id, required this.groupId, required this.isDisabled, required this.isDoorOpened, required this.isDoorLocked, required this.cleanState, required this.isAvailable});

  factory WaterCloset.fromJson(Map<String, dynamic> json) {
    final cleanStateStr = json['clean_state'];
    print('Raw clean_state value from JSON: $cleanStateStr');

    final cleanState = CleanState.values.firstWhere(
          (e) => e.toString().split('.').last.toUpperCase() == cleanStateStr.toUpperCase(),
      orElse: () => CleanState.OUT_OF_ORDER, // Valeur par défaut en cas d'erreur
    );

    print('Parsed clean_state: $cleanState');

    return WaterCloset(
      id: json['id'],
      groupId: json['group_id'],
      isDisabled: json['is_disabled'],
      isDoorOpened: json['is_door_opened'],
      isDoorLocked: json['is_door_locked'],
      isAvailable: json['is_available'],
      cleanState: cleanState,
    );
  }


  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'group_id': groupId,
      'is_disabled': isDisabled,
      'is_door_opened': isDoorOpened,
      'is_door_locked': isDoorLocked,
      'is_available': isAvailable,
      'clean_state': cleanState.toString().split('.').last,
    };
  }
}
