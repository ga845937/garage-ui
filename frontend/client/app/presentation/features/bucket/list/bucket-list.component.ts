import type {
	AfterViewInit,
	ElementRef,
	OnDestroy,
	OnInit,
} from "@angular/core";
import type { BucketListItem } from "@shared/entity/bucket.entity";

import { CommonModule } from "@angular/common";
import {
	Component,
	effect,
	HostListener,
	inject,
	ViewChild,
} from "@angular/core";
import { FormsModule } from "@angular/forms";
import { MatButtonModule } from "@angular/material/button";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { MatFormFieldModule } from "@angular/material/form-field";
import { MatIconModule } from "@angular/material/icon";
import { MatInputModule } from "@angular/material/input";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { MatTooltipModule } from "@angular/material/tooltip";
import { NgxTolgeeModule } from "@tolgee/ngx";

import { BucketListStore } from "../../../../application/stores";
import { LayoutService } from "../../../layout/layout.service";
import { BucketListFacade } from "./bucket-list.facade";
import { BucketCardComponent } from "./components/bucket-card/bucket-card.component";

@Component({
	imports: [
		CommonModule,
		FormsModule,
		MatButtonModule,
		MatCheckboxModule,
		MatFormFieldModule,
		MatIconModule,
		MatInputModule,
		MatProgressSpinnerModule,
		MatTooltipModule,
		NgxTolgeeModule,
		BucketCardComponent,
	],
	providers: [BucketListFacade],
	selector: "app-bucket-list",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./bucket-list.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./bucket-list.component.html",
})
export class BucketListComponent implements OnInit, OnDestroy, AfterViewInit {
	public readonly store = inject(BucketListStore);
	private readonly facade = inject(BucketListFacade);
	public readonly layout = inject(LayoutService);

	public readonly selection_mode = this.facade.selection_mode;

	@ViewChild("loading_sentinel") protected loading_sentinel!: ElementRef;
	public observer!: IntersectionObserver;

	public constructor() {
		effect(() => {
			if (
				this.store.items().length > 0 &&
				this.loading_sentinel &&
				this.observer
			) {
				this.observer.observe(this.loading_sentinel.nativeElement);
			}
		});
	}

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnInit(): void {
		this.facade.init();
	}

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngAfterViewInit(): void {
		this.setup_observer();
	}

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnDestroy(): void {
		this.facade.destroy();
		if (this.observer) {
			this.observer.disconnect();
		}
	}

	@HostListener("document:visibilitychange")
	public on_visibility_change(): void {
		if (document.visibilityState === "visible") {
			this.facade.load_buckets(this.store.page());
		}
	}

	public cancel_selection(): void {
		this.facade.cancel_selection();
	}

	public toggle_all(): void {
		this.facade.toggle_all();
	}

	public confirm_delete(): void {
		this.facade.confirm_delete();
	}

	public open_detail(id: string): void {
		this.facade.open_detail(id);
	}
	public open_detail_files(bucket: BucketListItem): void {
		this.facade.open_detail_files(bucket.id);
	}

	public delete_single(id: string): void {
		this.facade.delete_single(id);
	}

	public toggle_item_selection(id: string): void {
		this.facade.toggle_item_selection(id);
	}

	public create_bucket(): void {
		this.facade.create_bucket();
	}

	private setup_observer(): void {
		if (typeof IntersectionObserver === "undefined") {
			return;
		}

		this.observer = new IntersectionObserver(
			(entries) => {
				if (entries[0].isIntersecting) {
					this.facade.load_more();
				}
			},
			{ root: null, threshold: 0.1 },
		);
	}
}
