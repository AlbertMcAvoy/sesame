import 'package:flutter/material.dart';
import 'pages/accueil.dart';
import 'pages/report.dart';
import 'pages/returnOk.dart';
import 'toilet_dynamic.dart';

class Layout extends StatefulWidget {
  const Layout({super.key});

  @override
  State createState() => _LayoutState();
}

class _LayoutState extends State<Layout> {
  int _selectedIndex = 0;
  static final List<Widget> _widgetOptions = <Widget>[
    const MaterialApp(
      home: ListToilette(),
    ),
    ToiletDynamic(
      index: 0,
    ),
    const ReportToilette(),
    const ReturnOk(),
  ];

  void _onItemTapped(int index) {
    setState(() {
      _selectedIndex = index;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xffe2e2e2),
      appBar: AppBar(
        title: const Center(
          child: Text(
            'Sanitaires à proximité',
            style: TextStyle(
              color: Colors.white,
            ),
          ),
        ),
        backgroundColor: const Color(0xff003366),
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
        selectedItemColor: const Color(0xff98FF98),
        backgroundColor: const Color(0xff003366),
        onTap: _onItemTapped,
      ),
    );
  }
}
