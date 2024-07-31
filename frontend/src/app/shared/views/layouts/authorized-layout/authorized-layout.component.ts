import { Component, ViewEncapsulation } from '@angular/core';
import { HeaderComponent } from '../../components/header/header.component';
import { NavComponent } from '../../components/nav/nav.component';

@Component({
  selector: 'authorized-layout',
  standalone: true,
  imports: [HeaderComponent, NavComponent],
  templateUrl: './authorized-layout.component.html',
  styleUrl: './authorized-layout.component.scss',
  encapsulation: ViewEncapsulation.None
})
export class AuthorizedLayoutComponent {
}
