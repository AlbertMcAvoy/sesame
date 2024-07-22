import 'dart:async';

import 'package:flutter/material.dart';
import 'package:google_sign_in/google_sign_in.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'src/sign_in_button.dart';
import 'layout.dart';
import 'pages/returnOk.dart';
import 'pages/report.dart';
import 'package:localstorage/localstorage.dart';

/// The scopes required by this application.
// #docregion Initialize
const List<String> scopes = <String>[
  'email',
  'https://www.googleapis.com/auth/contacts.readonly',
];

GoogleSignIn _googleSignIn = GoogleSignIn(
  // Optional clientId
  // clientId: 'your-client_id.apps.googleusercontent.com',
  scopes: scopes,
);
// #enddocregion Initialize

void main() {
  initLocalStorage();
  runApp(
    const MaterialApp(
      title: 'Sesame',
      debugShowCheckedModeBanner: false,
      home: SignInSesame() /** TODO: appeler cette classe dans le layout quand il sera pret */
    ),
  );
}

class SignInSesame extends StatefulWidget {
  ///
  const SignInSesame({super.key});

  @override
  State createState() => _SignInSesameState();
}

class _SignInSesameState extends State<SignInSesame> {
  GoogleSignInAccount? _currentUser;

  @override
  void initState() {
    super.initState();

    _googleSignIn.onCurrentUserChanged.listen((GoogleSignInAccount? account) async {
      setState(() {
        _currentUser = account;
        localStorage.setItem('email', account!.email);
      });
    });

    _googleSignIn.signInSilently();
  }

  Future<void> _handleSignIn() async {
    try {
      await _googleSignIn.signIn();
    } catch (error) {
      print(error);
    }
  }

  Future<void> _handleSignOut() async {
    _googleSignIn.disconnect();
    localStorage.setItem('auth', '');
  }

  authenticateUser(String email) async {
    final response = await http.post(
      Uri.parse('http://localhost:8080/auth'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(<String, String>{
        'mail': email,
      }),
    );
    if (response.statusCode == 200) {
    // If the server did return a 201 CREATED response,
    // then parse the JSON.
    localStorage.setItem('auth', jsonDecode(response.body));
    return;
  } else {
    // If the server did not return a 201 CREATED response,
    // then throw an exception.
    throw Exception('Failed to create user.');
  }
  }

  Widget _buildBody() {
    final GoogleSignInAccount? user = _currentUser;
    
    // The user is Authenticated
    if (user != null || localStorage.getItem('auth') != null && localStorage.getItem('auth') != '') {
      if (localStorage.getItem('email') != null && localStorage.getItem('email') != '') {
        authenticateUser(localStorage.getItem('email')!);
      }

      return MaterialApp(routes: {
        // '/': (context) => ReportToilette(),
        '/': (context) => const Layout(),
        '/report': (context) => const ReportToilette(),
        '/returnOk': (context) => const ReturnOk(),
      }, initialRoute: '/');
    } else {
      // The user is NOT Authenticated
      return Scaffold(
        appBar: AppBar(
          title: const Center(
            child: Text(
              'Sesame',
              style: TextStyle(
                color: Colors.white,
              ),
            ),
          ),
          backgroundColor: const Color(0xff003366),
        ),
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: <Widget>[
              buildSignInButton(
                onPressed: _handleSignIn,
              ),
            ],
          ),
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: ConstrainedBox(
      constraints: const BoxConstraints.expand(),
      child: _buildBody(),
    ));
  }
}
