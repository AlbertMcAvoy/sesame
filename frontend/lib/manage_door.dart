import 'package:flutter/material.dart';

class ManageDoor extends StatelessWidget {
  final String title;
  final String imageUrl;
  final String link;

  ManageDoor({
    required this.title,
    required this.imageUrl,
    required this.link,
  });

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Container(
        width: MediaQuery.of(context).size.width * 0.9,
        height: MediaQuery.of(context).size.height * 0.7,
        decoration: BoxDecoration(
          color: Colors.white,
          borderRadius: BorderRadius.circular(10),
        ),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Padding(
              padding: const EdgeInsets.all(16.0),
              child: Text(
                title,
                style: TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.w600,
                  color: Color(0xff003366),
                ),
              ),
            ),
            Container(
              alignment: Alignment.center,
              child: Image.asset(
                imageUrl,
                fit: BoxFit.contain,
                width: 300,
              ),
            ),
            Padding(
              padding: const EdgeInsets.all(16.0),
              child: Text(
                link,
                style: TextStyle(
                  fontSize: 14,
                  color: Color(0xff003366),
                  decoration: TextDecoration.underline,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
