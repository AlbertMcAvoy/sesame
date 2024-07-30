import { Component } from '@angular/core';
import { PageLayoutService } from '../../../services/layout/page-layout.service';
import { RouterLink } from '@angular/router';

@Component({
  selector: 'app-header',
  standalone: true,
  imports: [RouterLink],
  templateUrl: './header.component.html',
  styleUrl: './header.component.scss'
})
export class HeaderComponent {

  constructor(public pageLayoutService: PageLayoutService) {}
}
