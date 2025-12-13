import { Injectable, inject, signal } from "@angular/core";

import { DeleteBucketsCommand } from "../../../../application/commands";
import { ListBucketsQuery } from "../../../../application/queries";
import { BucketListStore } from "../../../../application/stores";
import { NavigationService } from "../../../../infrastructure/navigation";
import { LayoutService } from "../../../layout/layout.service";
import { ConfirmDialogService } from "../../../shared/confirm-dialog";

@Injectable()
export class BucketListFacade {
	private readonly store = inject(BucketListStore);
	private readonly list_query = inject(ListBucketsQuery);
	private readonly delete_command = inject(DeleteBucketsCommand);
	private readonly navigation = inject(NavigationService);
	private readonly layout = inject(LayoutService);
	private readonly confirm_dialog = inject(ConfirmDialogService);

	// UI State
	public readonly selection_mode = signal(false);
	public readonly page_size = 20;

	public init(): void {
		this.layout.set_title_key("nav.bucket");
		this.set_default_actions();
		this.load_buckets(1);
	}

	public destroy(): void {
		this.layout.clear_actions();
	}

	public async load_buckets(page: number): Promise<void> {
		this.layout.set_loading(true);
		this.store.set_page(page);
		this.layout.set_error(null);

		try {
			const result = await this.list_query.execute({
				page,
				page_size: this.page_size,
			});
			this.store.set_items(result.rows);
			this.store.set_total(result.total);
		} catch (e) {
			const message =
				e instanceof Error ? e.message : "Failed to load buckets";
			this.layout.set_error(message);
		} finally {
			this.layout.set_loading(false);
		}
	}

	public async load_more(): Promise<void> {
		const current_page = this.store.page();
		const total_count = this.store.total();
		const current_count = this.store.items().length;

		if (!this.layout.loading() && current_count < total_count) {
			this.layout.set_loading(true);
			try {
				const result = await this.list_query.execute({
					page: current_page + 1,
					page_size: this.page_size,
				});
				this.store.append_items(result.rows);
				this.store.set_page(current_page + 1);
				this.store.set_total(result.total);
			} catch {
				// Silent error or toast
			} finally {
				this.layout.set_loading(false);
			}
		}
	}

	public open_detail(id: string): void {
		this.navigation.to_bucket_detail(id);
	}

	public open_detail_files(bucket_id: string): void {
		this.navigation.to_bucket_objects(bucket_id);
	}

	public create_bucket(): void {
		this.navigation.to_bucket_create();
	}

	public enter_selection_mode(): void {
		this.selection_mode.set(true);
		this.update_selection_actions();
	}

	public cancel_selection(): void {
		this.selection_mode.set(false);
		this.store.clear_selection();
		this.set_default_actions();
	}

	public toggle_all(): void {
		if (this.store.all_selected()) {
			this.store.clear_selection();
		} else {
			this.store.select_all();
		}
	}

	public toggle_item_selection(id: string): void {
		this.store.toggle_select(id);
	}

	public async confirm_delete(): Promise<void> {
		const ids = this.store.selected_ids();
		if (ids.length === 0) {
			return;
		}

		const confirmed = await this.confirm_dialog.confirm_delete(
			"bucket",
			ids.length,
		);
		if (confirmed) {
			this.layout.set_loading(true);
			try {
				await this.delete_command.execute(ids);
				this.store.remove_items(ids);
				this.store.clear_selected_ids(ids);
				this.cancel_selection();
			} catch (e) {
				const message =
					e instanceof Error ? e.message : "Failed to delete buckets";
				this.layout.set_error(message);
			} finally {
				this.layout.set_loading(false);
			}
		}
	}

	public async delete_single(id: string): Promise<void> {
		const confirmed = await this.confirm_dialog.confirm_delete("bucket");
		if (confirmed) {
			this.layout.set_loading(true);
			try {
				await this.delete_command.execute([id]);
				this.store.remove_items([id]);
			} catch (e) {
				const message =
					e instanceof Error ? e.message : "Failed to delete bucket";
				this.layout.set_error(message);
			} finally {
				this.layout.set_loading(false);
			}
		}
	}

	private set_default_actions(): void {
		this.layout.set_actions([
			{
				action: () => this.create_bucket(),
				icon: "add",
				label_key: "common.create",
				variant: "primary",
			},
			{
				action: () => this.enter_selection_mode(),
				icon: "delete",
				label_key: "common.delete",
				variant: "danger",
			},
		]);
	}

	private update_selection_actions(): void {
		this.layout.set_actions([]);
	}
}
