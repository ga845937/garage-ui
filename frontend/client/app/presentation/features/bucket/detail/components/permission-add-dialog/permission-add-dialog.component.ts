import { ScrollingModule } from "@angular/cdk/scrolling";
import { Component, inject, signal } from "@angular/core";
import {
	ReactiveFormsModule,
	FormControl as typed_form_control,
} from "@angular/forms";
import { MatButtonModule } from "@angular/material/button";
import { MatCheckboxModule } from "@angular/material/checkbox";
import {
	MAT_DIALOG_DATA,
	MatDialogModule,
	MatDialogRef,
} from "@angular/material/dialog";
import { MatFormFieldModule } from "@angular/material/form-field";
import { MatIconModule } from "@angular/material/icon";
import { MatInputModule } from "@angular/material/input";
import { MatListModule } from "@angular/material/list";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { debounceTime, distinctUntilChanged } from "rxjs";

import { I18nService } from "../../../../../../infrastructure/i18n/i18n.service";
import { AccessKeyRepository } from "../../../../../../infrastructure/repositories/access-key.repository";

@Component({
	imports: [
		MatDialogModule,
		MatButtonModule,
		MatFormFieldModule,
		MatInputModule,
		MatCheckboxModule,
		MatListModule,
		MatIconModule,
		ReactiveFormsModule,
		MatProgressSpinnerModule,
		ScrollingModule,
	],
	selector: "app-bucket-permission-add-dialog",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./permission-add-dialog.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./permission-add-dialog.component.html",
})
export class BucketPermissionAddDialogComponent {
	protected readonly i18n = inject(I18nService);
	private readonly repository = inject(AccessKeyRepository);
	private readonly dialog_ref = inject(
		MatDialogRef<BucketPermissionAddDialogComponent>,
	);
	public readonly dialog_data = inject<{ existing_ids: string[] }>(
		MAT_DIALOG_DATA,
	);

	// biome-ignore lint/style/useNamingConvention: Angular FormControl options
	public filter_control = new typed_form_control("", { nonNullable: true });

	public keys = signal<Array<{ id: string; name: string }>>([]);
	public selected_ids = signal<Set<string>>(new Set());
	public loading = signal(false);

	private page = 1;
	private readonly page_size = 20;
	private has_more = true;
	private search_term = "";

	public constructor() {
		// Initial load
		this.load_keys();

		// Setup search debounce
		this.filter_control.valueChanges
			.pipe(
				debounceTime(500),
				distinctUntilChanged(),
			)
			.subscribe((val) => {
				this.search_term = val.trim();
				this.reset_and_load();
			});
	}

	public on_scroll(index: number): void {
		if (this.loading() || !this.has_more) {
			return;
		}

		const total = this.keys().length;
		// Load more when scrolled to bottom (approx last 5 items)
		if (index + 10 >= total) {
			this.load_keys();
		}
	}

	private async load_keys(): Promise<void> {
		if (this.loading()) {
			return;
		}
		this.loading.set(true);

		try {
			const result = await this.repository.find_all({
				page: this.page,
				page_size: this.page_size,
				search: this.search_term || undefined,
			});

			const existing = this.dialog_data.existing_ids || [];
			const new_keys = result.rows.filter(
				(k) => !existing.includes(k.id),
			);

			if (this.page === 1) {
				this.keys.set(new_keys);
			} else {
				this.keys.update((current) => [...current, ...new_keys]);
			}

			this.has_more = result.rows.length === this.page_size;
			if (this.has_more) {
				this.page++;
			}
		} catch (error) {
			console.error("Failed to load keys", error);
		} finally {
			this.loading.set(false);
		}
	}

	private reset_and_load(): void {
		this.page = 1;
		this.has_more = true;
		this.keys.set([]); // Clear list to show loading state correctly or reset
		this.load_keys();
	}

	public is_selected(id: string): boolean {
		return this.selected_ids().has(id);
	}

	public toggle_selection(id: string): void {
		this.selected_ids.update((set) => {
			const new_set = new Set(set);
			if (new_set.has(id)) {
				new_set.delete(id);
			} else {
				new_set.add(id);
			}
			return new_set;
		});
	}

	public on_save(): void {
		if (this.selected_ids().size === 0) {
			return;
		}

		const selected_items = this.keys()
			.filter((k) => this.selected_ids().has(k.id))
			.map((k) => ({
				access_key_id: k.id,
				name: k.name,
				// Default permissions: Read only
				permissions: {
					owner: false,
					read: true,
					write: false,
				},
			}));

		this.dialog_ref.close(selected_items);
	}
}
