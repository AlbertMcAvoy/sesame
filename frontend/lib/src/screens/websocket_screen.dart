import 'package:flutter/material.dart';
import '../services/websocket_service.dart';

/** TODO: appeler cette classe dans le layout quand il sera pret */

class WebSocketScreen extends StatefulWidget {
  @override
  _WebSocketScreenState createState() => _WebSocketScreenState();
}

class _WebSocketScreenState extends State<WebSocketScreen> {
  late WebSocketService webSocketService;

  @override
  void initState() {
    super.initState();
    webSocketService = WebSocketService('ws://localhost:8080/ws');
  }

  @override
  void dispose() {
    webSocketService.closeConnection();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('WebSocket Example'),
      ),
      body: StreamBuilder(
        stream: webSocketService.stream,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return const Center(child: CircularProgressIndicator());
          } else if (snapshot.hasError) {
            return Center(child: Text('Error: ${snapshot.error}'));
          } else if (snapshot.hasData) {
            return Center(child: Text(snapshot.data));
          } else {
            return const Center(child: Text('No data received'));
          }
        },
      ),
        floatingActionButton: FloatingActionButton(
        onPressed: () {
          print("message en cours d'envoie");
          webSocketService.sendMessage("Hello from Flutter app");
        },
          child: Icon(Icons.send),
      ),
    );
  }
}
