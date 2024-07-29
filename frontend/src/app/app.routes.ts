import { Routes } from '@angular/router';
import { loginRoutes } from './pages/login/login.routes';
import { notFoundRoutes } from './pages/not-found/not-found.routes';
import { homeRoutes } from './pages/home/home.routes';
import { authGuard, authLoginGuard } from './shared/guards/auth.guard';

export const routes: Routes = [
    { path: '', children: homeRoutes, canActivate: [authGuard] },
    { path: 'login', title: 'login', children: loginRoutes, canActivate: [authLoginGuard]},
    { path: '**', children: notFoundRoutes }
];
