import type { OnDestroy, OnInit } from "@angular/core";
import type { UpdateAccessKeyContract } from "@shared/contracts";

import {
	Component,
	HostListener,
	Input,
	inject,
	signal,
	ViewChild,
} from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatCardModule } from "@angular/material/card";
import { MatIconModule } from "@angular/material/icon";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { MatSnackBar, MatSnackBarModule } from "@angular/material/snack-bar";
import { MatTooltipModule } from "@angular/material/tooltip";
import { RouterLink } from "@angular/router";
import { KeyRoute } from "@shared/route-paths";
import { NgxTolgeeModule } from "@tolgee/ngx";

import { AccessKeyDetailStore } from "../../../../application/stores";
import { I18nService } from "../../../../infrastructure/i18n/i18n.service";
import { LayoutService } from "../../../layout/layout.service";
import { AccessKeyDetailFacade } from "./access-key-detail.facade";
import { AccessKeyPropertiesComponent } from "./components/properties/access-key-properties.component";

/**
 * Smart Component (Container) - AccessKey Detail Page
 *
 * Responsibilities:
 * - Coordinate child Presentational/State components
 * - Inject Store, LayoutService, Router, ConfirmDialogService
 * - Handle loading/error states
 * - Manage layout actions
 */
@Component({
	imports: [
		AccessKeyPropertiesComponent,
		MatButtonModule,
		MatCardModule,
		MatIconModule,
		MatProgressSpinnerModule,
		MatSnackBarModule,
		MatTooltipModule,
		RouterLink,
		NgxTolgeeModule,
	],
	providers: [AccessKeyDetailFacade],
	selector: "app-access-key-detail",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./access-key-detail.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./access-key-detail.component.html",
})
export class AccessKeyDetailComponent implements OnInit, OnDestroy {
	@Input() public id = "";
	@ViewChild(AccessKeyPropertiesComponent)
	private properties_component?: AccessKeyPropertiesComponent;

	private readonly facade = inject(AccessKeyDetailFacade);
	public readonly store = inject(AccessKeyDetailStore);
	public readonly layout = inject(LayoutService);
	private readonly snackbar = inject(MatSnackBar);
	private readonly i18n = inject(I18nService);

	public readonly is_editing = this.facade.is_editing;

	public readonly key_route = KeyRoute;

	public readonly show_secret = signal(false);

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnInit(): void {
		this.layout.set_title_key("access_key.detail_title");
		this.facade.init(this.id);
		this.update_actions();
	}

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnDestroy(): void {
		this.layout.clear_actions();
	}

	@HostListener("document:visibilitychange")
	public on_visibility_change(): void {
		if (document.visibilityState === "visible" && !this.is_editing()) {
			this.facade.load_key(this.id);
		}
	}

	// ========================================
	// Event Handlers from Child Components
	// ========================================

	public on_toggle_secret(): void {
		this.show_secret.update((v) => !v);
	}

	public on_copy_to_clipboard(text: string): void {
		this.layout.copy_to_clipboard(text);
	}

	public on_enter_edit_mode(): void {
		this.facade.enter_edit_mode();
		this.update_actions();
	}

	public on_cancel_edit(): void {
		this.facade.cancel_edit(this.id);
		this.update_actions();
	}

	public async on_save(contract: UpdateAccessKeyContract): Promise<void> {
		const success = await this.facade.update(contract);
		if (success) {
			this.update_actions();
			this.snackbar.open(
				this.i18n.t("access_key.updated"),
				this.i18n.t("common.confirm"),
				{ duration: 2000 },
			);
		}
	}

	public async on_delete(): Promise<void> {
		await this.facade.delete();
	}

	public on_trigger_save(): void {
		if (this.properties_component) {
			this.properties_component.on_submit();
		}
	}

	// ========================================
	// Private Methods
	// ========================================

	private update_actions(): void {
		if (this.is_editing()) {
			this.layout.set_actions([
				{
					action: () => this.on_cancel_edit(),
					icon: "close",
					label_key: "common.cancel",
				},
				{
					action: () => this.on_trigger_save(),
					icon: "save",
					label_key: "common.save",
					variant: "primary",
				},
			]);
		} else {
			this.layout.set_actions([
				{
					action: () => this.on_enter_edit_mode(),
					icon: "edit",
					label_key: "common.edit",
					variant: "primary",
				},
				{
					action: () => this.on_delete(),
					icon: "delete",
					label_key: "common.delete",
					variant: "danger",
				},
			]);
		}
	}
}
