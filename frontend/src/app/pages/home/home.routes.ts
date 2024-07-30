import { Routes } from '@angular/router';
import { HomeComponent } from './component/home.component';

export const homeRoutes: Routes = [
    {
        path: '',
        component: HomeComponent,
        data: {
            title: 'Groupes de sanitaires à proximité'
        }
    }
];