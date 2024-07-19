import 'package:flutter/material.dart';
import 'package:qr_code_scanner/qr_code_scanner.dart';
import 'src/services/websocket_service.dart';
import 'toilet_dynamic.dart';

class ToiletQrCode extends StatefulWidget {
  @override
  _ToiletQrCodeState createState() => _ToiletQrCodeState();

}
class _ToiletQrCodeState extends State<ToiletQrCode> {
  final GlobalKey _qrKey = GlobalKey(debugLabel: 'QR');
  QRViewController? _controller;
  late WebSocketService webSocketService;

  void initState() {
    super.initState();
    webSocketService = WebSocketService('ws://localhost:8080/ws');
  }

  @override
  void dispose() {
    _controller?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('QR Code Scanner'),
      ),
      body: Column(
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
                  MaterialPageRoute(builder: (context) => ToiletDynamic(index: snapshot.data == "UNAVAILABLE" ? 1 : 2))
                );
                return Center(child: Text(snapshot.data));
              } else {
                return const Center(child: Text('No data received'));
              }
            },
          ),
          TextButton(
            onPressed: () {
              webSocketService.sendMessage("client-scan 1 QR_CODE");
            },
            child: Text('Scan QR CODE'),
          ),
          Expanded(
            child: QRView(
              key: _qrKey,
              onQRViewCreated: _onQRViewCreated,
            ),
          ),
        ],
      ),
    );
  }

  void _onQRViewCreated(QRViewController controller) {
    setState(() {
      _controller = controller;
    });
    _controller!.scannedDataStream.listen((scanData) {
      print('QR Code scanned: ${scanData.code}');
      // Ici vous pouvez traiter le code QR scanné comme vous le souhaitez
    });
  }
}