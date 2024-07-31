import { HttpHeaders, HttpClient, HttpErrorResponse } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { environment } from '../../../../environments/environment';
import { catchError, Observable, throwError } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class ApiService {

  endpoint: string = environment.apiUrl;
  headers = new HttpHeaders().set('Content-Type', 'application/json');

  constructor(
    private http: HttpClient
  ) {}

  get(endpoint: string): Observable<any> {
    return this.http
      .get<any>(`${this.endpoint}/${endpoint}`)
      .pipe(catchError(this.handleError))
  }

  post(endpoint: string, object: Object): Observable<any> {
    return this.http
      .post<any>(`${this.endpoint}/${endpoint}`, object)
      .pipe(catchError(this.handleError))
  }

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
