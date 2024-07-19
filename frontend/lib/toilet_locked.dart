import 'package:flutter/material.dart';
import 'manage_door.dart';
import 'src/services/websocket_service.dart';
import 'toilet_dynamic.dart';

class ToiletLocked extends StatefulWidget {
  @override
  _ToiletLockedState createState() => _ToiletLockedState();
}

class _ToiletLockedState extends State<ToiletLocked> {
    late WebSocketService webSocketService;

    void initState() {
      super.initState();
      webSocketService = WebSocketService('ws://localhost:8080/ws');
    }

    @override
    Widget build(BuildContext context) {

        return Column(
          children: <Widget>[
            TextButton(
              onPressed: () {
                webSocketService.sendMessage("client-leave 1");
              },
              child: Text('Déverouiller !'),
            ),
            StreamBuilder(
              stream: webSocketService.stream,
              builder: (context, snapshot) {
                if (snapshot.connectionState == ConnectionState.waiting) {
                  return const Center(child: CircularProgressIndicator());
                } else if (snapshot.hasError) {
                  return Center(child: Text('Error: ${snapshot.error}'));
                } else if (snapshot.hasData) {
                  Navigator.pushReplacement(
                    context,
                    MaterialPageRoute(builder: (context) => ToiletDynamic(index: 4))
                  );
                  return Center(child: Text(snapshot.data));
                } else {
                  return const Center(child: Text('No data received'));
                }
              },
            ),
            ManageDoor(
              title: "Terminé ?",
              title2: "Maintenir le bouton pour ouvrir la porte",
              imageUrl: 'assets/images/toilet_locked.png',
              link: "Un problème ?",
            )
          ]
        );
        
    }
}