import { Routes } from '@angular/router';
import { NotFoundComponent } from './component/not-found.component';

export const notFoundRoutes: Routes = [
    {
        path: '',
        title: 'Page non trouvée',
        component: NotFoundComponent
    }
];