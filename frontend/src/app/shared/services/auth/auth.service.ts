import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { SsrCookieService } from 'ngx-cookie-service-ssr';
import { Credentials } from '../../../types/user';
import { ApiService } from '../api/api.service';

@Injectable({
  providedIn: 'root'
})
export class AuthService {

  constructor(
    private apiService: ApiService, 
    private router: Router, 
    private cookieService: SsrCookieService
  ) {}

  // Sign-in
  signIn(credentials: Credentials) {
    return this.apiService
    .post('auth', { mail: credentials.mail })
    .subscribe((res: any) => {
      this.cookieService.set('access_token', res, { expires: 90, path: '/' });
      this.router.navigate(["/"]);
    });
  }

  getToken() {
    return this.cookieService.get('access_token');
  }

  get isLoggedIn(): boolean {
    let authToken = this.getToken();
    return (authToken !== undefined && authToken !== null && authToken !== '') ? true : false;
  }

  doLogout() {
    let removeToken = this.cookieService.delete('access_token');
    if (removeToken == null) {
      this.router.navigate(["/login"]);
    }
  }
}