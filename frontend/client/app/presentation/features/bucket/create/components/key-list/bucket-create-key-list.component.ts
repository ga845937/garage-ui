import type { OnInit } from "@angular/core";
import type { BucketKeyPermissionContract } from "@shared/contracts/bucket.contract";
import type { AccessKeyListItemAggregate } from "../../../../../../domain/aggregates/access-key.aggregate";

import {
	Component,
	computed,
	EventEmitter,
	inject,
	Output,
	signal,
} from "@angular/core";
import { takeUntilDestroyed } from "@angular/core/rxjs-interop";
import { FormControl, ReactiveFormsModule } from "@angular/forms";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { MatFormFieldModule } from "@angular/material/form-field";
import { MatIconModule } from "@angular/material/icon";
import { MatInputModule } from "@angular/material/input";
import { MatTooltipModule } from "@angular/material/tooltip";
import { debounceTime, distinctUntilChanged, tap } from "rxjs/operators";

import { ListAccessKeysQuery } from "../../../../../../application/queries/access-key/list-access-keys.query";
import { I18nService } from "../../../../../../infrastructure/i18n/i18n.service";

@Component({
	imports: [
		ReactiveFormsModule,
		MatCheckboxModule,
		MatFormFieldModule,
		MatIconModule,
		MatInputModule,
		MatTooltipModule,
	],
	selector: "app-bucket-create-key-list",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./bucket-create-key-list.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./bucket-create-key-list.component.html",
})
export class BucketCreateKeyListComponent implements OnInit {
	@Output() public selection_change = new EventEmitter<
		BucketKeyPermissionContract[]
	>();

	protected readonly i18n = inject(I18nService);
	private readonly list_keys_query = inject(ListAccessKeysQuery);

	// Data State
	private readonly keys = signal<AccessKeyListItemAggregate[]>([]);
	public readonly loading = signal(false);
	public readonly has_more = signal(true);
	private page = 1;
	private readonly page_size = 20;

	// Search
	public readonly search_control = new FormControl("");

	// Permissions State
	private readonly permissions = signal<
		Map<string, { read: boolean; write: boolean; owner: boolean }>
	>(new Map());

	public constructor() {
		this.search_control.valueChanges
			.pipe(
				debounceTime(300),
				distinctUntilChanged(),
				tap(() => {
					this.page = 1;
					this.keys.set([]);
					this.has_more.set(true);
					void this.load_keys();
				}),
				takeUntilDestroyed(),
			)
			.subscribe();
	}

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnInit(): void {
		void this.load_keys();
	}

	private async load_keys(): Promise<void> {
		if (this.loading() || !this.has_more()) {
			return;
		}

		this.loading.set(true);
		try {
			const response = await this.list_keys_query.execute({
				page: this.page,
				page_size: this.page_size,
				search: this.search_control.value || undefined,
			});

			if (response.rows.length < this.page_size) {
				this.has_more.set(false);
			}

			this.keys.update((current) => [...current, ...response.rows]);
			this.page++;
		} catch (error) {
			console.error("Failed to load access keys", error);
		} finally {
			this.loading.set(false);
		}
	}

	public on_scroll(event: Event): void {
		const target = event.target as HTMLElement;
		if (
			target.scrollHeight - target.scrollTop <=
				target.clientHeight + 50 &&
			this.has_more() &&
			!this.loading()
		) {
			void this.load_keys();
		}
	}

	// Computed view model combining keys and permissions
	public readonly key_list = computed(() => {
		const perms = this.permissions();
		return this.keys().map((key) => {
			const current = perms.get(key.id) || {
				owner: false,
				read: false,
				write: false,
			};
			return {
				display_name: key.display_name,
				id: key.id,
				permissions: current,
				selected: current.read || current.write || current.owner,
			};
		});
	});

	public toggle_permission(
		key_id: string,
		type: "read" | "write" | "owner",
	): void {
		this.permissions.update((map) => {
			const new_map = new Map(map);
			const current = new_map.get(key_id) || {
				owner: false,
				read: false,
				write: false,
			};

			const new_perms = { ...current };
			if (type === "read") {
				new_perms.read = !new_perms.read;
			}
			if (type === "write") {
				new_perms.write = !new_perms.write;
			}
			if (type === "owner") {
				new_perms.owner = !new_perms.owner;
			}

			// If no permissions left, remove from map (deselect)
			if (!new_perms.read && !new_perms.write && !new_perms.owner) {
				new_map.delete(key_id);
			} else {
				new_map.set(key_id, new_perms);
			}
			return new_map;
		});

		this.emit_changes();
	}

	private emit_changes(): void {
		const result: BucketKeyPermissionContract[] = [];
		for (const [key_id, perms] of this.permissions().entries()) {
			result.push({
				access_key_id: key_id,
				permissions: perms,
			});
		}
		this.selection_change.emit(result);
	}
}
