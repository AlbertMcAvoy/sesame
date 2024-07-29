import { GoogleSigninButtonModule, SocialAuthService, SocialUser } from '@abacritt/angularx-social-login';
import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { AuthService } from '../../../shared/services/auth/auth.service';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [
    CommonModule,
    GoogleSigninButtonModule
  ],
  templateUrl: './login.component.html',
  styleUrl: './login.component.scss'
})
export class LoginComponent {
  constructor(
    private googleAuthService:SocialAuthService,
    private authService: AuthService
  ) {}

  ngOnInit(): void {
    this.googleAuthService.authState.subscribe((user: SocialUser) => {
      console.log(user)
      this.authService.signIn({mail: user.email});
    });
  }
}
