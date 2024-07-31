import { TestBed } from '@angular/core/testing';

import { WaterClosetService } from './water-closet.service';

describe('WaterClosetService', () => {
  let service: WaterClosetService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(WaterClosetService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
