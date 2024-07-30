import { Component } from '@angular/core';
import { HeaderComponent } from '../../components/header/header.component';

@Component({
  selector: 'error-layout',
  standalone: true,
  imports: [HeaderComponent],
  templateUrl: './error-layout.component.html',
  styleUrl: './error-layout.component.scss'
})
export class ErrorLayoutComponent {

}
