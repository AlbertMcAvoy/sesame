import { Component } from '@angular/core';
import { HeaderComponent } from '../../components/header/header.component';
import { NavComponent } from '../../components/nav/nav.component';

@Component({
  selector: 'error-layout',
  standalone: true,
  imports: [HeaderComponent, NavComponent],
  templateUrl: './error-layout.component.html',
  styleUrl: './error-layout.component.scss'
})
export class ErrorLayoutComponent {

}
