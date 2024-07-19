import 'dart:html' as html;  // Importer dart:html pour Flutter Web

import 'package:flutter/material.dart';
import '../models/group.dart';
import '../models/place.dart';

class GroupListItem extends StatelessWidget {
  final Group group;
  final Place place;

  GroupListItem({required this.group, required this.place});

  // Fonction pour ouvrir Google Maps
  void _launchMapsUrl() {
    final coordinates = place.coordinates.trim();
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
      onTap: _launchMapsUrl,  // Appel de la fonction lors du clic
      child: Card(
        margin: EdgeInsets.all(8.0),
        child: Padding(
          padding: const EdgeInsets.all(8.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                children: [
                  Chip(
                    label: Text('Accès handicapé', style: TextStyle(color: Colors.black, fontSize: 12)),
                    backgroundColor: Color(0xFFB0FFDF),
                    padding: EdgeInsets.symmetric(vertical: 0, horizontal: 6),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(8.0),
                      side: BorderSide(color: Colors.transparent),
                    ),
                  ),
                  SizedBox(width: 5),
                ],
              ),
              SizedBox(height: 8.0),
              Text(
                group.name,
                style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
              ),

              Text(
                place.coordinates,
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
