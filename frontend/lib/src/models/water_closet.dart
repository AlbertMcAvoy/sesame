enum CleanState { CLEANED, USED, DIRTY, OUT_OF_ORDER }

class WaterCloset {
  final int id;
  final int groupId;
  final bool isDisabled;
  final bool isDoorOpened;
  final bool isDoorLocked;
  final CleanState cleanState;

  WaterCloset({required this.id, required this.groupId, required this.isDisabled, required this.isDoorOpened, required this.isDoorLocked, required this.cleanState});

  factory WaterCloset.fromJson(Map<String, dynamic> json) {
    return WaterCloset(
      id: json['id'],
      groupId: json['group_id'],
      isDisabled: json['is_disabled'],
      isDoorOpened: json['is_door_opened'],
      isDoorLocked: json['is_door_locked'],
      cleanState: CleanState.values.firstWhere((e) => e.toString() == 'CleanState.' + json['clean_state']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'group_id': groupId,
      'is_disabled': isDisabled,
      'is_door_opened': isDoorOpened,
      'is_door_locked': isDoorLocked,
      'clean_state': cleanState.toString().split('.').last,
    };
  }
}
