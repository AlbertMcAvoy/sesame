import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { PageLayout } from '../../../types/page-utils';

@Injectable({
  providedIn: 'root'
})

export class PageLayoutService {
  private layoutSubject = new BehaviorSubject<PageLayout>(PageLayout.UnAuthorized);

  private titleSubject = new BehaviorSubject<string>('');

  public layout$ = this.layoutSubject.asObservable();

  public title$ = this.titleSubject.asObservable();

  setLayout(value: PageLayout) {
    this.layoutSubject.next(value);
    return this;
  }

  setTitle(value: string) {
    this.titleSubject.next(value);
    return this;
  }

  get title() {
    return this.titleSubject.value;
  }
}