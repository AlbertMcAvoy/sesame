import { Routes } from '@angular/router';
import { loginRoutes } from './pages/login/login.routes';
import { notFoundRoutes } from './pages/not-found/not-found.routes';
import { homeRoutes } from './pages/home/home.routes';
import { authGuard, authLoginGuard } from './shared/guards/auth.guard';
import { setLayout } from './shared/resolvers/layout.resolver';
import { PageLayout, PageTitle } from './types/page-utils';

export const routes: Routes = [
    {
        path: '',
        loadChildren: () => import('./pages/home/home.routes').then(mod => mod.homeRoutes),
        canActivate: [authGuard],
        resolve: {
            layout: setLayout(PageLayout.Authorized, PageTitle.HOME)
        }
    },
    {
        path: 'login',
        loadChildren: () => import('./pages/login/login.routes').then(mod => mod.loginRoutes),
        canActivate: [authLoginGuard],
        resolve: {
            layout: setLayout(PageLayout.UnAuthorized, PageTitle.LOGIN)
        }
    },
    {
        path: '**',
        loadChildren: () => import('./pages/not-found/not-found.routes').then(mod => mod.notFoundRoutes),
        resolve: {
            layout: setLayout(PageLayout.Error, PageTitle.NOT_FOUND)
        }
    }
];
