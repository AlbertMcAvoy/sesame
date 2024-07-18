import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:web_socket_channel/status.dart' as status;

class WebSocketService {
  final String url;
  late WebSocketChannel channel;

  WebSocketService(this.url) {
    channel = WebSocketChannel.connect(Uri.parse(url));
  }

  Stream get stream => channel.stream;

  void sendMessage(String message) {
    channel.sink.add(message); // "client-scan 15f4z54ezf"
  }

  void closeConnection() {
    channel.sink.close(status.normalClosure);
  }
}
