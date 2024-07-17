import 'package:flutter/material.dart';

class ManageDoor extends StatelessWidget {
  final String imageUrl;

  ManageDoor({required this.imageUrl});

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 200,
      height: 200,
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(10),
        image: DecorationImage(
          image: AssetImage(imageUrl),
          fit: BoxFit.cover,
        ),
      ),
    );
  }
}