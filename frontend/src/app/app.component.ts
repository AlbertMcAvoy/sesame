import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { AuthorizedLayoutComponent } from './shared/views/layouts/authorized-layout/authorized-layout.component';
import { UnauthorizedLayoutComponent } from './shared/views/layouts/unauthorized-layout/unauthorized-layout.component';
import { ErrorLayoutComponent } from './shared/views/layouts/error-layout/error-layout.component';
import { PageLayout } from './types/page-utils';
import { PageLayoutService } from './shared/services/layout/page-layout.service';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, AuthorizedLayoutComponent, UnauthorizedLayoutComponent, ErrorLayoutComponent],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  readonly PageLayout = PageLayout;

  constructor(public pageLayoutService: PageLayoutService) {}
}
