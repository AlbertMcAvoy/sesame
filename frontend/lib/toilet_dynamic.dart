import 'package:flutter/material.dart';
import 'toilet_locked.dart';
import 'toilet_not_available.dart';
import 'toilet_opened.dart';
import 'toilet_see_you_soon.dart';
import 'toilet_qr_code.dart';
import 'package:go_router/go_router.dart';

class ToiletDynamic extends StatelessWidget {
    final GoRouter _router = GoRouter(
        routes: <RouteBase>[
            GoRoute(
                path: '/',
                builder: (BuildContext context, GoRouterState state) {
                    return ToiletQrCode();
                },
                routes: <RouteBase>[
                    GoRoute(
                        path: 'locked',
                        builder: (BuildContext context, GoRouterState state) {
                            return ToiletLocked();
                        },
                    ),
                    GoRoute(
                        path: 'opened',
                        builder: (BuildContext context, GoRouterState state) {
                            return ToiletOpened();
                        },
                    ),
                    GoRoute(
                        path: 'not-available',
                        builder: (BuildContext context, GoRouterState state) {
                            return ToiletNotAvailable();
                        },
                    ),
                    GoRoute(
                        path: 'see-you-soon',
                        builder: (BuildContext context, GoRouterState state) {
                            return ToiletSeeYouSoon();
                        },
                    ),
                ],
            ),
        ],
    );

    ToiletDynamic();

    @override
    Widget build(BuildContext context) {
        return MaterialApp.router(
            routerConfig: _router,
        );
    }
}