import 'dart:async';
import 'dart:convert' show json;

import 'package:flutter/foundation.dart';
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
  bool _isAuthorized = false; // has granted permissions?
  String _contactText = '';

  @override
  void initState() {
    super.initState();

    _googleSignIn.onCurrentUserChanged
        .listen((GoogleSignInAccount? account) async {
// #docregion CanAccessScopes
      // In mobile, being authenticated means being authorized...
      bool isAuthorized = account != null;
      // However, on web...
      if (kIsWeb && account != null) {
        isAuthorized = await _googleSignIn.canAccessScopes(scopes);
      }
// #enddocregion CanAccessScopes

      setState(() {
        _currentUser = account;
        _isAuthorized = isAuthorized;
      });

      if (isAuthorized) {
        unawaited(_handleGetContact(account!));
      }
    });

    _googleSignIn.signInSilently();
  }

  // Calls the People API REST endpoint for the signed-in user to retrieve information.
  Future<void> _handleGetContact(GoogleSignInAccount user) async {
    setState(() {
      _contactText = 'Loading contact info...';
    });
    final http.Response response = await http.get(
      Uri.parse('https://people.googleapis.com/v1/people/me/connections'
          '?requestMask.includeField=person.names'),
      headers: await user.authHeaders,
    );
    if (response.statusCode != 200) {
      setState(() {
        _contactText = 'People API gave a ${response.statusCode} '
            'response. Check logs for details.';
      });
      print('People API ${response.statusCode} response: ${response.body}');
      return;
    }
    final Map<String, dynamic> data =
        json.decode(response.body) as Map<String, dynamic>;
    final String? namedContact = _pickFirstNamedContact(data);
    setState(() {
      if (namedContact != null) {
        _contactText = 'I see you know $namedContact!';
      } else {
        _contactText = 'No contacts to display.';
      }
    });
  }

  String? _pickFirstNamedContact(Map<String, dynamic> data) {
    final List<dynamic>? connections = data['connections'] as List<dynamic>?;
    final Map<String, dynamic>? contact = connections?.firstWhere(
      (dynamic contact) => (contact as Map<Object?, dynamic>)['names'] != null,
      orElse: () => null,
    ) as Map<String, dynamic>?;
    if (contact != null) {
      final List<dynamic> names = contact['names'] as List<dynamic>;
      final Map<String, dynamic>? name = names.firstWhere(
        (dynamic name) =>
            (name as Map<Object?, dynamic>)['displayName'] != null,
        orElse: () => null,
      ) as Map<String, dynamic>?;
      if (name != null) {
        return name['displayName'] as String?;
      }
    }
    return null;
  }

  Future<void> _handleSignIn() async {
    try {
      await _googleSignIn.signIn();
    } catch (error) {
      print(error);
    }
  }

  Future<void> _handleAuthorizeScopes() async {
    final bool isAuthorized = await _googleSignIn.requestScopes(scopes);
    // #enddocregion RequestScopes
    setState(() {
      _isAuthorized = isAuthorized;
    });
    // #docregion RequestScopes
    if (isAuthorized) {
      unawaited(_handleGetContact(_currentUser!));
    }
    // #enddocregion RequestScopes
  }

  Future<void> _handleSignOut() => _googleSignIn.disconnect();

  createUser(String email) async {
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
    throw Exception('Failed to create album.');
  }
  }

  Widget _buildBody() {
    final GoogleSignInAccount? user = _currentUser;
    if (user != null ) {
      if (user != null) {
        createUser(user.email);
      }

      return MaterialApp(routes: {
        // '/': (context) => ReportToilette(),
        '/': (context) => Layout(),
        '/report': (context) => ReportToilette(),
        '/returnOk': (context) => ReturnOk(),
      }, initialRoute: '/');
      // The user is Authenticated
      // return Column(
      //   mainAxisAlignment: MainAxisAlignment.spaceAround,
      //   children: <Widget>[
      //     ListTile(
      //       leading: GoogleUserCircleAvatar(
      //         identity: user,
      //       ),
      //       title: Text(user.displayName ?? ''),
      //       subtitle: Text(user.id),
      //     ),
      //     const Text('Signed in successfully.'),
      //     if (_isAuthorized) ...<Widget>[
      //       // The user has Authorized all required scopes
      //       Text(_contactText),
      //       ElevatedButton(
      //         child: const Text('REFRESH'),
      //         onPressed: () => _handleGetContact(user),
      //       ),
      //     ],
      //     if (!_isAuthorized) ...<Widget>[
      //       // The user has NOT Authorized all required scopes.
      //       // (Mobile users may never see this button!)
      //       const Text('Additional permissions needed to read your contacts.'),
      //       ElevatedButton(
      //         onPressed: _handleAuthorizeScopes,
      //         child: const Text('REQUEST PERMISSIONS'),
      //       ),
      //     ],
      //     ElevatedButton(
      //       onPressed: _handleSignOut,
      //       child: const Text('SIGN OUT'),
      //     ),
      //   ],
      // );
    } else {
      // The user is NOT Authenticated
      return Scaffold(
        appBar: AppBar(
          title: Center(
            child: Text(
              'Sesame',
              style: TextStyle(
                color: Colors.white,
              ),
            ),
          ),
          backgroundColor: Color(0xff003366),
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
      // return Column(
      //   mainAxisAlignment: MainAxisAlignment.spaceAround,
      //   children: <Widget>[
      //     const Text('You are not currently signed in.'),
      //     // This method is used to separate mobile from web code with conditional exports.
      //     // See: src/sign_in_button.dart
      //     buildSignInButton(
      //       onPressed: _handleSignIn,
      //     ),
      //   ],
      // );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        // appBar: AppBar(
        //   title: const Text('Google Sign In'),
        // ),
      body: ConstrainedBox(
      constraints: const BoxConstraints.expand(),
      child: _buildBody(),
    ));
  }
}
