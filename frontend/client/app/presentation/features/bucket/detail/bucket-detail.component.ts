import type { OnDestroy, OnInit } from "@angular/core";
import type { UpdateBucketData } from "../../../../infrastructure/repositories/bucket.repository";

import {
	Component,
	effect,
	HostListener,
	Input,
	inject,
	signal,
	ViewChild,
} from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatIconModule } from "@angular/material/icon";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { MatSnackBar, MatSnackBarModule } from "@angular/material/snack-bar";
import { MatTooltipModule } from "@angular/material/tooltip";
import { RouterLink } from "@angular/router";
import { BucketRoute } from "@shared/route-paths";

import { BucketDetailStore } from "../../../../application/stores";
import { I18nService } from "../../../../infrastructure/i18n/i18n.service";
import { LayoutService } from "../../../layout/layout.service";
import { BucketDetailFacade } from "./bucket-detail.facade";
import { BucketPermissionComponent } from "./components/permission-list/bucket-permission.component";
import { BucketPropertiesComponent } from "./components/properties/bucket-properties.component";

/**
 * Smart Component (Container) - Bucket Detail Page
 *
 * Responsibilities:
 * - Coordinate child Presentational/State components
 * - Inject Store, LayoutService, Router
 * - Handle loading/error states
 * - Manage layout actions
 */
@Component({
	imports: [
		BucketPermissionComponent,
		BucketPropertiesComponent,
		MatButtonModule,
		MatIconModule,
		MatProgressSpinnerModule,
		MatSnackBarModule,
		MatTooltipModule,
		RouterLink,
	],
	providers: [BucketDetailFacade],
	selector: "app-bucket-detail",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./bucket-detail.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./bucket-detail.component.html",
})
export class BucketDetailComponent implements OnInit, OnDestroy {
	@Input() public id = "";

	@ViewChild(BucketPropertiesComponent)
	private properties_component?: BucketPropertiesComponent;

	@ViewChild(BucketPermissionComponent)
	private permissions_component?: BucketPermissionComponent;

	public readonly store = inject(BucketDetailStore);
	private readonly facade = inject(BucketDetailFacade);
	protected readonly i18n = inject(I18nService);
	protected readonly layout = inject(LayoutService);
	private readonly snackbar = inject(MatSnackBar);

	public readonly is_editing = this.facade.is_editing;
	public readonly bucket_route = BucketRoute;

	// Permission State
	public readonly selection_count = signal(0);
	public readonly is_deleting = signal(false);

	public constructor() {
		effect(() => {
			this.update_actions();
		});
	}

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnInit(): void {
		this.facade.init(this.id);
	}

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnDestroy(): void {
		this.facade.destroy();
	}

	@HostListener("document:visibilitychange")
	public on_visibility_change(): void {
		if (document.visibilityState === "visible" && !this.is_editing()) {
			this.facade.load_bucket(this.id);
		}
	}

	// ========================================
	// Event Handlers from Child Components
	// ========================================

	public on_enter_edit_mode(): void {
		this.facade.enter_edit_mode();
	}

	public on_cancel_edit(): void {
		this.is_deleting.set(false);
		this.facade.cancel_edit(this.id);
		this.permissions_component?.reset_pending_changes();
	}

	public async on_save(data: UpdateBucketData): Promise<void> {
		const key_permissions =
			this.permissions_component?.get_pending_changes_array() ?? [];

		const success = await this.facade.save(this.id, {
			...data,
			key_permissions:
				key_permissions.length > 0 ? key_permissions : undefined,
		});

		if (success) {
			this.permissions_component?.reset_pending_changes();
			this.snackbar.open(
				this.i18n.t("bucket.updated"),
				this.i18n.t("common.confirm"),
				{ duration: 2000 },
			);
		}
	}

	public on_copy_to_clipboard(text: string): void {
		this.layout.copy_to_clipboard(text);
	}

	public on_trigger_save(): void {
		this.properties_component?.on_submit();
	}

	public on_add_permission(): void {
		this.permissions_component?.open_add_dialog();
	}

	public on_permission_selection_change(count: number): void {
		this.selection_count.set(count);
	}

	public on_exit_delete_mode(): void {
		this.is_deleting.set(false);
		if (this.permissions_component) {
			this.permissions_component.selection.set(new Set());
		}
		this.selection_count.set(0);
		this.update_actions();
	}

	public on_delete(): void {
		this.facade.delete(this.id);
	}

	public on_go_to_files(): void {
		this.facade.go_to_files(this.id);
	}

	// ========================================
	// Private Methods
	// ========================================

	public get_bucket_name(): string {
		const b = this.store.selected_item();
		return b?.display_name ?? "Bucket";
	}

	private update_actions(): void {
		if (this.is_deleting()) {
			this.layout.set_actions([
				{
					action: () => this.on_exit_delete_mode(),
					icon: "close",
					label_key: "common.cancel",
				},
			]);
		} else if (this.is_editing()) {
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
				{
					action: () => this.on_add_permission(),
					icon: "key",
					label: this.i18n.t("bucket.permission.actions.add"),
					variant: "primary",
				},
			]);
		} else {
			this.layout.set_actions([
				{
					action: () => this.on_go_to_files(),
					icon: "folder_open",
					label_key: "bucket.files",
					variant: "primary",
				},
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
