import { Routes } from '@angular/router';
import { HomeComponent } from './component/home.component';
import { PageTitle } from '../../types/page-utils';

export const homeRoutes: Routes = [
    {
        path: '',
        title: PageTitle.HOME,
        component: HomeComponent
    }
];