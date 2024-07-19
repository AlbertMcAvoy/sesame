import 'dart:html' as html;
import 'package:flutter/material.dart';
import '../models/group.dart';
import '../../pages/toiletListPage.dart'; // Importez la page ToiletListPage

class GroupListItem extends StatelessWidget {
  final Group group;

  GroupListItem({required this.group});

  // Fonction pour ouvrir Google Maps
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
    return GestureDetector(
      onTap: () {
        Navigator.push(
          context,
          MaterialPageRoute(
            builder: (context) => ToiletListPage(groupId: group.id, group: group,),
          ),
        );
      },  // Appel de la fonction lors du clic
      child: Card(
        margin: EdgeInsets.all(8.0),
        child: Padding(
          padding: const EdgeInsets.all(8.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  IconButton(
                    icon: Icon(Icons.map),
                    onPressed: _launchMapsUrl,
                  ),
                ],
              ),
              SizedBox(height: 8.0),
              Text(
                group.name,
                style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
              ),
              Text(
                group.place?.coordinates ?? "Coordonnée indisponible",
                style: TextStyle(color: Colors.grey),
              ),
              SizedBox(height: 8.0),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    '700 mètres',
                    style: TextStyle(color: Colors.blue),
                  ),
                  Row(
                    children: [
                      Icon(Icons.star, color: Colors.amber),
                      SizedBox(width: 4.0),
                      Text('4 (17 notes)'),
                    ],
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
