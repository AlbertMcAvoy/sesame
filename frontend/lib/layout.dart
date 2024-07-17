import 'package:flutter/material.dart';
import 'pages/accueil.dart';
import 'pages/report.dart';
import 'pages/returnOk.dart';
import 'toilet_locked.dart';
import 'toilet_not_available.dart';
import 'toilet_opened.dart';
import 'toilet_see_you_soon.dart';
import 'toilet_dynamic.dart';
import 'manage_door.dart';

class Layout extends StatefulWidget {
  @override
  _LayoutState createState() => _LayoutState();
}

class _LayoutState extends State<Layout> {
  int _selectedIndex = 0;
  static const TextStyle optionStyle =
      TextStyle(fontSize: 30, fontWeight: FontWeight.bold);
  static List<Widget> _widgetOptions = <Widget>[
    ListToilette(),
    // ReportToilette(),
    ToiletDynamic(),
    Text(
      'Index 2: School',
      style: optionStyle,
    ),
    ReturnOk(),
  ];

  void _onItemTapped(int index) {
    setState(() {
      _selectedIndex = index;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Color(0xffe2e2e2),
      appBar: AppBar(
        title: Center(
          child: Text(
            'Sanitaires à proximité',
            style: TextStyle(
              color: Colors.white,
            ),
          ),
        ),
        backgroundColor: Color(0xff003366),
      ),
      body: Center(
        child: _widgetOptions.elementAt(_selectedIndex),
      ),
      bottomNavigationBar: BottomNavigationBar(
        items: const <BottomNavigationBarItem>[
          BottomNavigationBarItem(
            icon: Icon(Icons.home),
            label: 'Accueil',
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.qr_code_outlined),
            label: 'Scanner',
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.account_circle),
            label: 'Profile',
          ),
        ],
        currentIndex: _selectedIndex,
        unselectedItemColor: Colors.white,
        selectedItemColor: Color(0xff98FF98),
        backgroundColor: Color(0xff003366),
        onTap: _onItemTapped,
      ),
    );
  }
}
