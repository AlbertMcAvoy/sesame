import 'package:flutter/material.dart';
import 'toilet_locked.dart';
import 'toilet_not_available.dart';
import 'toilet_opened.dart';
import 'toilet_see_you_soon.dart';
import 'toilet_qr_code.dart';
import 'src/services/websocket_service.dart';

class ToiletDynamic extends StatelessWidget {
    final int index;

    ToiletDynamic({required this.index});

    @override
    Widget build(BuildContext context) {
        switch (index) {
            case 0:
                return ToiletQrCode();
            case 1:
                return ToiletNotAvailable();
            case 2:
                return ToiletOpened();
            case 3:
                return ToiletLocked();
            case 4:
                return ToiletSeeYouSoon();
            default:
                return Text("Aucun widget de toilettes !");
        }
    }
}