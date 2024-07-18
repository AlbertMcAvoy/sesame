import 'user_service.dart';
import 'group_service.dart';
import 'place_service.dart';
import 'report_service.dart';
import 'history_service.dart';
import 'water_closet_service.dart';

class ApiService {
  final UserService userService = UserService();
  final GroupService groupService = GroupService();
  final PlaceService placeService = PlaceService();
  final ReportService reportService = ReportService();
  final HistoryService historyService = HistoryService();
  final WaterClosetService waterClosetService = WaterClosetService();
}
