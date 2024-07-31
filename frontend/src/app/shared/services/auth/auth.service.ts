import { HttpClient, HttpErrorResponse, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { catchError, throwError } from 'rxjs';
import { environment } from '../../../../environments/environment';
import { SsrCookieService } from 'ngx-cookie-service-ssr';
import { Credentials } from '../../../types/user';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  
  endpoint: string = environment.apiUrl;
  headers = new HttpHeaders().set('Content-Type', 'application/json');

  constructor(private http: HttpClient, private router: Router, private cookieService: SsrCookieService) {}

  // Sign-in
  signIn(credentials: Credentials) {
    return this.http
      .post<any>(`${this.endpoint}/auth`, {mail: credentials.mail})
      .pipe(catchError(this.handleError))
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
      this.router.navigate(['log-in']);
    }
  }

  // Error
  handleError(error: HttpErrorResponse) {
    let msg = '';
    if (error.error instanceof ErrorEvent) {
      // client-side error
      msg = error.error.message;
    } else {
      // server-side error
      msg = `Error Code: ${error.status}\nMessage: ${error.message}`;
    }
    return throwError(() => new Error(msg));
  }  
}