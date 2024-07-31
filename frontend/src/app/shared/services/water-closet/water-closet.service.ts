import { Injectable } from '@angular/core';
import { ApiService } from '../api/api.service';
import { WaterCloset } from '../../../types/water-closet';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class WaterClosetService {

  endpoint: string = 'waterclosets';

  constructor(
    private apiService: ApiService
  ) {}

  getAll(): Observable<WaterCloset[]> {
    return this.apiService.get(this.endpoint);
  }

  get(id: number): Observable<WaterCloset> {
    return this.apiService.get(`${this.endpoint}/${id}`);
  }
}
