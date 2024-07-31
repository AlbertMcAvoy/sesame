import { Component, OnInit, ViewEncapsulation } from '@angular/core';
import { WaterCloset } from '../../../types/water-closet';
import { WaterClosetService } from '../../../shared/services/water-closet/water-closet.service';
import { VirtualScrollerLazyLoadEvent, VirtualScrollerModule } from 'primeng/virtualscroller';

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [VirtualScrollerModule],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss',
  encapsulation: ViewEncapsulation.None
})
export class HomeComponent implements OnInit {

  waterClosetList: WaterCloset[] = [];
  virtualWaterClosetList: WaterCloset[] = [];

  constructor(
    private waterClosetService: WaterClosetService
  ) {}

  ngOnInit() {
    this.waterClosetService
      .getAll()
      .subscribe(list => this.waterClosetList = list);
  }

  loadWaterClosetLazy(event: VirtualScrollerLazyLoadEvent) {
    //load data of required page
    let loadedWaterCloset = this.waterClosetList.slice(event.first, (event.first! + event.rows!));

    //populate page of virtual cars
    Array.prototype.splice.apply(this.virtualWaterClosetList, [event.first!, event.rows!, ...loadedWaterCloset]);

    //trigger change detection
    this.virtualWaterClosetList = [...this.virtualWaterClosetList];
}
}
