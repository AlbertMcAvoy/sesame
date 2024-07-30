import { Routes } from '@angular/router';
import { LoginComponent } from './component/login.component';

export const loginRoutes: Routes = [
    {
        path: '',
        title: 'Connexion',
        component: LoginComponent
    }
];