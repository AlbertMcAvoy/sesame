import { Routes } from '@angular/router';
import { NotFoundComponent } from './component/not-found.component';
import { PageTitle } from '../../types/page-utils';

export const notFoundRoutes: Routes = [
    {
        path: '',
        title: PageTitle.NOT_FOUND,
        component: NotFoundComponent
    }
];