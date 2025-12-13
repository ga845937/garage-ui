import type { OnDestroy } from "@angular/core";

import {
	Component,
	computed,
	EventEmitter,
	Input,
	inject,
	Output,
	signal,
} from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatCardModule } from "@angular/material/card";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { MatChipsModule } from "@angular/material/chips";
import { MatDialog, MatDialogModule } from "@angular/material/dialog";
import { MatFormFieldModule } from "@angular/material/form-field";
import { MatIconModule } from "@angular/material/icon";
import { MatInputModule } from "@angular/material/input";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { MatSelectModule } from "@angular/material/select";
import { MatTooltipModule } from "@angular/material/tooltip";
import { Router } from "@angular/router";
import { KeyRoute } from "@shared/route-paths";

import { BucketDetailStore } from "../../../../../../application/stores";
import { I18nService } from "../../../../../../infrastructure/i18n/i18n.service";
import { BucketPermissionAddDialogComponent } from "../permission-add-dialog/permission-add-dialog.component";
import { BucketPermissionFacade } from "./bucket-permission.facade";

// ==========================================================================
interface KeyPermission {
	access_key_id: string;
	name?: string;
	bucket_id: string;
	permissions: {
		read: boolean;
		write: boolean;
		owner: boolean;
	};
}

@Component({
	imports: [
		MatButtonModule,
		MatCardModule,
		MatCheckboxModule,
		MatChipsModule,
		MatDialogModule,
		MatFormFieldModule,
		MatIconModule,
		MatInputModule,
		MatProgressSpinnerModule,
		MatSelectModule,
		MatTooltipModule,
	],
	providers: [BucketPermissionFacade],
	selector: "app-bucket-permission",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./bucket-permission.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./bucket-permission.component.html",
})
export class BucketPermissionComponent implements OnDestroy {
	@Input() public id = "";
	@Input() public is_editing = false;
	@Input() public is_deleting = false;
	@Output() public selection_change = new EventEmitter<number>();

	public readonly store = inject(BucketDetailStore);
	public readonly facade = inject(BucketPermissionFacade);
	public readonly dialog = inject(MatDialog);
	protected readonly i18n = inject(I18nService);
	private readonly router = inject(Router);

	// Route Paths
	public readonly key_route = KeyRoute;

	// UI State
	public readonly selection = signal<Set<string>>(new Set());

	public readonly pending_changes = signal<
		Map<string, { read: boolean; write: boolean; owner: boolean }>
	>(new Map());
	public readonly new_keys_info = signal<Map<string, { name: string }>>(
		new Map(),
	);

	public readonly permissions = computed<KeyPermission[]>(() => {
		const bucket = this.store.selected_item();
		const changes = this.pending_changes();
		const new_info = this.new_keys_info();

		let list: KeyPermission[] = [];

		if (bucket?.keys) {
			list = bucket.keys.map((key) => {
				const pending = changes.get(key.access_key_id);
				return {
					access_key_id: key.access_key_id,
					bucket_id: bucket.id,
					name: key.name,
					permissions: pending ?? {
						owner: key.permissions?.owner ?? false,
						read: key.permissions?.read ?? false,
						write: key.permissions?.write ?? false,
					},
				};
			});
		}

		const existing_ids = new Set(list.map((k) => k.access_key_id));
		for (const [id, perms] of changes.entries()) {
			if (!existing_ids.has(id)) {
				const info = new_info.get(id);
				list.push({
					access_key_id: id,
					bucket_id: bucket?.id ?? "",
					name: info?.name ?? id,
					permissions: perms,
				});
			}
		}

		return list.sort((a, b) => {
			const is_new_a = !existing_ids.has(a.access_key_id);
			const is_new_b = !existing_ids.has(b.access_key_id);

			if (is_new_a && !is_new_b) {
				return -1;
			}
			if (!is_new_a && is_new_b) {
				return 1;
			}

			return (a.name || "").localeCompare(b.name || "");
		});
	});

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnDestroy(): void {
		this.facade.destroy();
	}

	public open_add_dialog(): void {
		const existing_ids = this.permissions().map((p) => p.access_key_id);
		const dialog_ref = this.dialog.open(
			BucketPermissionAddDialogComponent,
			{
				data: { existing_ids },
				width: "500px",
			},
		);

		dialog_ref.afterClosed().subscribe((result: KeyPermission[]) => {
			if (result && result.length > 0) {
				this.pending_changes.update((map) => {
					const new_map = new Map(map);
					for (const item of result) {
						new_map.set(item.access_key_id, item.permissions);
					}
					return new_map;
				});

				this.new_keys_info.update((map) => {
					const new_map = new Map(map);
					for (const item of result) {
						if (item.name) {
							new_map.set(item.access_key_id, {
								name: item.name,
							});
						}
					}
					return new_map;
				});
			}
		});
	}

	public async delete_selected(): Promise<void> {
		const ids = Array.from(this.selection());
		if (ids.length === 0) {
			return;
		}

		const confirmed = window.confirm(
			this.i18n.t("bucket.permission.confirm_delete_multiple", {
				count: ids.length,
			}),
		);
		if (!confirmed) {
			return;
		}

		await this.facade.refresh(this.id);
		this.selection.set(new Set());
		this.selection_change.emit(0);
	}

	public toggle_selection(key_id: string): void {
		this.selection.update((set) => {
			const new_set = new Set(set);
			if (new_set.has(key_id)) {
				new_set.delete(key_id);
			} else {
				new_set.add(key_id);
			}
			return new_set;
		});
		this.selection_change.emit(this.selection().size);
	}

	public toggle_all_selection(checked: boolean): void {
		if (checked) {
			const all_ids = this.permissions().map((p) => p.access_key_id);
			this.selection.set(new Set(all_ids));
		} else {
			this.selection.set(new Set());
		}
		this.selection_change.emit(this.selection().size);
	}

	public is_new(access_key_id: string): boolean {
		const bucket = this.store.selected_item();
		if (!bucket?.keys) {
			return true;
		}
		return !bucket.keys.find((k) => k.access_key_id === access_key_id);
	}

	public remove_key(event: Event, access_key_id: string): void {
		event.stopPropagation();

		if (this.is_new(access_key_id)) {
			// Case 1: New Key (Pending) -> Remove completely from Map
			this.pending_changes.update((map) => {
				const new_map = new Map(map);
				new_map.delete(access_key_id);
				return new_map;
			});
			this.new_keys_info.update((map) => {
				const new_map = new Map(map);
				new_map.delete(access_key_id);
				return new_map;
			});
		} else {
			// Case 2: Existing Key -> Set all permissions to false (Delete action)
			const empty_perms = { owner: false, read: false, write: false };
			this.pending_changes.update((map) => {
				const new_map = new Map(map);
				new_map.set(access_key_id, empty_perms);
				return new_map;
			});
		}
	}

	public toggle_permission(
		perm: KeyPermission,
		type: "read" | "write" | "owner",
	): void {
		if (!this.is_editing) {
			return;
		}

		const current = perm.permissions;
		const new_perms = {
			owner: current.owner,
			read: current.read,
			write: current.write,
		};

		if (type === "read") {
			new_perms.read = !new_perms.read;
		}
		if (type === "write") {
			new_perms.write = !new_perms.write;
		}
		if (type === "owner") {
			new_perms.owner = !new_perms.owner;
		}

		this.pending_changes.update((map) => {
			const new_map = new Map(map);
			new_map.set(perm.access_key_id, new_perms);
			return new_map;
		});
	}

	public async save_pending_changes(): Promise<void> {
		const changes = this.pending_changes();
		if (changes.size === 0) {
			return;
		}

		await this.facade.refresh(this.id);
		this.pending_changes.set(new Map());
	}

	public reset_pending_changes(): void {
		this.pending_changes.set(new Map());
	}

	public has_pending_changes(): boolean {
		return this.pending_changes().size > 0;
	}

	public get_pending_changes_array(): Array<{
		access_key_id: string;
		permissions: { read: boolean; write: boolean; owner: boolean };
	}> {
		const changes = this.pending_changes();
		const result: Array<{
			access_key_id: string;
			permissions: { read: boolean; write: boolean; owner: boolean };
		}> = [];
		for (const [key_id, perms] of changes.entries()) {
			result.push({
				access_key_id: key_id,
				permissions: perms,
			});
		}
		return result;
	}

	public on_row_click(perm: KeyPermission): void {
		if (!this.is_editing && !this.is_deleting) {
			this.router.navigate(["/", KeyRoute.LIST, perm.access_key_id]);
		}
	}

	public on_badge_click(
		event: Event,
		perm: KeyPermission,
		type: "read" | "write" | "owner",
	): void {
		if (this.is_editing && !this.is_deleting) {
			event.stopPropagation();
			this.toggle_permission(perm, type);
		}
	}
}
