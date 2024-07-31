import { Routes } from '@angular/router';
import { LoginComponent } from './component/login.component';
import { PageTitle } from '../../types/page-utils';

export const loginRoutes: Routes = [
    {
        path: '',
        title: PageTitle.LOGIN,
        component: LoginComponent
    }
];