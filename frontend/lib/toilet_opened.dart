import 'package:flutter/material.dart';
import 'manage_door.dart';
import 'src/services/websocket_service.dart';
import 'toilet_dynamic.dart';

class ToiletOpened extends StatefulWidget {

  ToiletOpened(); // Constructeur

  @override
  _ToiletOpenedState createState() => _ToiletOpenedState();
}
class _ToiletOpenedState extends State<ToiletOpened> {
    late WebSocketService webSocketService;
    void initState() {
      super.initState();
    }

    @override
    Widget build(BuildContext context) {
      return Column(
        children: <Widget>[
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
            title: "Sanitaire ouvert !",
            imageUrl: 'assets/images/toilet_opened.png',
            link: "Un problème ?",
          )
        ]
      );
    }
}