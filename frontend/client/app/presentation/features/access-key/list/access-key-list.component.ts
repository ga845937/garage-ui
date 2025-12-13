import type {
	AfterViewInit,
	ElementRef,
	OnDestroy,
	OnInit,
} from "@angular/core";
import type { AccessKeyListItem } from "@shared/entity/access-key.entity";

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
import { MatIconModule } from "@angular/material/icon";
import { MatMenuModule } from "@angular/material/menu";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { MatTooltipModule } from "@angular/material/tooltip";
import { NgxTolgeeModule } from "@tolgee/ngx";

import { AccessKeyListStore } from "../../../../application/stores";
import { LayoutService } from "../../../layout/layout.service";
import { AccessKeyListFacade } from "./access-key-list.facade";
import { AccessKeyCardComponent } from "./components/card/access-key-card.component";

@Component({
	imports: [
		CommonModule,
		FormsModule,
		MatButtonModule,
		MatCheckboxModule,
		MatIconModule,
		MatProgressSpinnerModule,
		MatMenuModule,
		MatTooltipModule,
		NgxTolgeeModule,
		AccessKeyCardComponent,
	],
	providers: [AccessKeyListFacade],
	selector: "app-access-key-list",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./access-key-list.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./access-key-list.component.html",
})
export class AccessKeyListComponent
	implements OnInit, OnDestroy, AfterViewInit
{
	private readonly facade = inject(AccessKeyListFacade);

	public readonly store = inject(AccessKeyListStore);
	public readonly layout = inject(LayoutService);

	public readonly selection_mode = this.facade.selection_mode;

	@ViewChild("loading_sentinel") protected loading_sentinel!: ElementRef;
	public observer!: IntersectionObserver;

	public search_query = "";

	public constructor() {
		effect(() => {
			this.update_actions();
		});

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
		this.layout.set_title_key("nav.access_key");
		this.facade.init();
	}

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngAfterViewInit(): void {
		this.setup_observer();
	}

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnDestroy(): void {
		this.facade.destroy();
		this.layout.clear_actions();
		if (this.observer) {
			this.observer.disconnect();
		}
	}

	@HostListener("document:visibilitychange")
	public on_visibility_change(): void {
		if (document.visibilityState === "visible") {
			this.facade.load_data(this.store.page());
		}
	}

	private setup_observer(): void {
		if (typeof IntersectionObserver === "undefined") {
			return;
		}

		this.observer = new IntersectionObserver(
			(entries) => {
				if (entries[0].isIntersecting) {
					this.facade.load_more(this.search_query || undefined);
				}
			},
			{ root: null, threshold: 0.1 },
		);
	}

	public open_detail(key: AccessKeyListItem): void {
		this.facade.open_detail(key.id);
	}

	public toggle_item_selection(key: AccessKeyListItem): void {
		this.facade.toggle_item_selection(key.id);
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

	public create_key(): void {
		this.facade.create_key();
	}

	public enter_selection_mode(): void {
		this.facade.enter_selection_mode();
	}

	private update_actions(): void {
		if (this.selection_mode()) {
			this.layout.set_actions([]);
		} else {
			this.layout.set_actions([
				{
					action: () => this.facade.create_key(),
					icon: "add",
					label_key: "common.create",
					variant: "primary",
				},
				{
					action: () => this.facade.enter_selection_mode(),
					icon: "delete",
					label_key: "common.delete",
					variant: "danger",
				},
			]);
		}
	}
}
