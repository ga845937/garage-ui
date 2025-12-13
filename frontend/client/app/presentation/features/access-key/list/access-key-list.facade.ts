import { Injectable, inject, signal } from "@angular/core";

import { DeleteAccessKeysCommand } from "../../../../application/commands";
import { ListAccessKeysQuery } from "../../../../application/queries";
import {
	AccessKeyDetailStore,
	AccessKeyListStore,
} from "../../../../application/stores";
import { NavigationService } from "../../../../infrastructure/navigation";
import { LayoutService } from "../../../layout/layout.service";
import { ConfirmDialogService } from "../../../shared/confirm-dialog";

@Injectable()
export class AccessKeyListFacade {
	private readonly store = inject(AccessKeyListStore);
	private readonly list_query = inject(ListAccessKeysQuery);
	private readonly delete_command = inject(DeleteAccessKeysCommand);
	private readonly navigation = inject(NavigationService);
	private readonly confirm_dialog = inject(ConfirmDialogService);
	private readonly layout = inject(LayoutService);

	// UI State
	public readonly selection_mode = signal(false);
	public readonly page_size = 20;

	public init(): void {
		this.load_data(1);
	}

	public destroy(): void {
		// Check if we need to clean up anything else
	}

	public async load_data(page: number, search?: string): Promise<void> {
		this.layout.set_loading(true);
		this.store.set_page(page);
		this.layout.set_error(null);

		try {
			const result = await this.list_query.execute({
				page,
				page_size: this.page_size,
				search,
			});

			this.store.set_items(result.rows);
			this.store.set_total(result.total);
		} catch (e) {
			const message =
				e instanceof Error ? e.message : "Failed to load access keys";
			this.layout.set_error(message);
		} finally {
			this.layout.set_loading(false);
		}
	}

	public async load_more(search?: string): Promise<void> {
		const current_page = this.store.page();
		const total_count = this.store.total();
		const current_count = this.store.items().length;

		if (!this.layout.loading() && current_count < total_count) {
			this.layout.set_loading(true);
			try {
				const result = await this.list_query.execute({
					page: current_page + 1,
					page_size: this.page_size,
					search,
				});
				this.store.append_items(result.rows);
				this.store.set_page(current_page + 1);
				this.store.set_total(result.total);
			} catch (e) {
				// Handle error silently or show toast
			} finally {
				this.layout.set_loading(false);
			}
		}
	}

	public open_detail(id: string): void {
		this.navigation.to_access_key_detail(id);
	}

	public create_key(): void {
		this.navigation.to_access_key_create();
	}

	public enter_selection_mode(): void {
		this.selection_mode.set(true);
	}

	public cancel_selection(): void {
		this.selection_mode.set(false);
		this.store.clear_selection();
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
			"access_key",
			ids.length,
		);
		if (confirmed) {
			await this.delete_command.execute(ids);
			this.cancel_selection();
			await this.load_data(this.store.page());
		}
	}
}
