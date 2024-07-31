import { CommonModule } from '@angular/common';
import { Component, ViewEncapsulation } from '@angular/core';
import { RouterLink } from '@angular/router';
import { MenuItem, PrimeIcons } from 'primeng/api';
import { MenubarModule } from 'primeng/menubar';
import { DockModule} from 'primeng/dock';

@Component({
  selector: 'app-nav',
  standalone: true,
  imports: [MenubarModule, RouterLink, CommonModule, DockModule],
  templateUrl: './nav.component.html',
  styleUrl: './nav.component.scss',
  encapsulation: ViewEncapsulation.None
})
export class NavComponent {

  items: MenuItem[] = [];

  ngOnInit() {
    this.items = [
        {
            icon: PrimeIcons.HOME,
            route: ['/'],
        },
        {
          icon: PrimeIcons.QRCODE,
          route: ['/scanning'] 
        },
        {
          icon: PrimeIcons.USER,
          route: ['/profile']
        }
    ];
}
}
