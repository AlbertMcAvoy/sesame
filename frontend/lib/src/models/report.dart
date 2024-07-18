enum State { TODO, IN_PROGRESS, DONE }
enum Topic { DOOR, TOILET, SUPPLY, CLEANLINESS, OTHER }

class Report {
  final int id;
  final int userId;
  final int waterClosetId;
  final String datetime;
  final State state;
  final Topic topic;
  final String comment;

  Report({required this.id, required this.userId, required this.waterClosetId, required this.datetime, required this.state, required this.topic, required this.comment});

  factory Report.fromJson(Map<String, dynamic> json) {
    return Report(
      id: json['id'],
      userId: json['user_id'],
      waterClosetId: json['water_closet_id'],
      datetime: json['datetime'],
      state: State.values.firstWhere((e) => e.toString() == 'State.' + json['state']),
      topic: Topic.values.firstWhere((e) => e.toString() == 'Topic.' + json['topic']),
      comment: json['comment'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'user_id': userId,
      'water_closet_id': waterClosetId,
      'datetime': datetime,
      'state': state.toString().split('.').last,
      'topic': topic.toString().split('.').last,
      'comment': comment,
    };
  }
}
