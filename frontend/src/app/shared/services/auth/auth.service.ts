import { HttpClient, HttpErrorResponse, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { catchError, throwError } from 'rxjs';
import { User } from '../../../types/user';
import { environment } from '../../../../environments/environment';
import { SsrCookieService } from 'ngx-cookie-service-ssr';
 
@Injectable({
  providedIn: 'root'
})
export class AuthService {
  
  endpoint: string = environment.apiUrl;
  headers = new HttpHeaders().set('Content-Type', 'application/json');

  constructor(private http: HttpClient, private router: Router, private cookieService: SsrCookieService) {}

  // Sign-in
  signIn(user: User) {
    return this.http
      .post<any>(`${this.endpoint}/login`, user)
      .pipe(catchError(this.handleError))
      .subscribe((res: any) => {
        this.cookieService.set('access_token', res.token, { expires: 90, path: '/' });
        this.router.navigate(["/"]);
      });
  }

  getToken() {
    return this.cookieService.get('access_token');
  }

  get isLoggedIn(): boolean {
    let authToken = this.getToken();
    return (authToken !== null && authToken !== '') ? true : false;
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