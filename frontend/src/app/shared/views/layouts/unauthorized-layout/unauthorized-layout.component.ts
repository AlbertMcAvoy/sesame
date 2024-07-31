import { Component } from '@angular/core';
import { HeaderComponent } from '../../components/header/header.component';

@Component({
  selector: 'unauthorized-layout',
  standalone: true,
  imports: [HeaderComponent],
  templateUrl: './unauthorized-layout.component.html',
  styleUrl: './unauthorized-layout.component.scss'
})
export class UnauthorizedLayoutComponent {

}
