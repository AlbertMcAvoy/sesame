import 'dart:html' as html;
import 'package:flutter/material.dart';
import '../src/models/group.dart';
import '../src/models/water_closet.dart';
import '../src/services/api-service/water_closet_service.dart';

class ToiletListPage extends StatelessWidget {
  final int groupId;
  final Group group;

  const ToiletListPage({required this.groupId, required this.group});

  Future<List<WaterCloset>> fetchToilets() async {
    final waterClosetService = WaterClosetService();
    return waterClosetService.fetchWaterClosets(groupId);
  }

  void _launchMapsUrl() {
    print('coordinates: ${group.place?.coordinates}');
    final coordinates = group.place?.coordinates.trim();
    final url = 'https://www.google.com/maps/search/?api=1&query=$coordinates';

    // Ouvrir l'URL dans un nouvel onglet
    try {
      html.window.open(url, '_blank');
    } catch (e) {
      print('Error launching URL: $e');
      // Affiche une alerte en cas d'erreur pour Flutter Web
      html.window.alert('Failed to open maps');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Toilettes'),
        actions: <Widget>[
          IconButton(
            icon: const Icon(Icons.map),
            onPressed: _launchMapsUrl,
          ),
        ],
      ),
      body: FutureBuilder<List<WaterCloset>>(
        future: fetchToilets(),
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            print('test 1');
            return const Center(child: CircularProgressIndicator());
          } else if (snapshot.hasError) {
            print('test 2');
            return Center(child: Text('Error: ${snapshot.error}'));
          } else if (!snapshot.hasData || snapshot.data!.isEmpty) {
            return const Center(child: Text('No toilets available'));
          }

          print('n"importe quoi');

          return ListView.builder(
            itemCount: snapshot.data!.length,
            itemBuilder: (context, index) {
              final toilet = snapshot.data![index];
              print('toilet : $toilet.cleanState');
              return Card(
                margin: EdgeInsets.all(8.0),
                child: ListTile(
                  leading: Icon(
                    toilet.isAvailable ? Icons.check : Icons.close,
                    color: toilet.isAvailable ? Colors.green : Colors.red,
                  ),
                  title: Text('Toilette ${toilet.id}'),
                  subtitle: Text('État de propreté: ${toilet.cleanState}'),
                  trailing: Column(
                    children: [
                      Icon(toilet.isDisabled ? Icons.accessible : Icons.not_accessible),
                      Icon(toilet.isDoorLocked ? Icons.lock : Icons.lock_open),
                    ],
                  ),
                ),
              );
            },
          );
        },
      ),
    );
  }
}
