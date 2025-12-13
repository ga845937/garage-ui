import type { UpdateAccessKeyContract } from "@shared/contracts";

import { Injectable, inject, signal } from "@angular/core";

import {
	DeleteAccessKeysCommand,
	UpdateAccessKeyCommand,
} from "../../../../application/commands";
import { GetAccessKeyQuery } from "../../../../application/queries";
import { AccessKeyDetailStore } from "../../../../application/stores";
import { NavigationService } from "../../../../infrastructure/navigation/navigation.service";
import { LayoutService } from "../../../layout/layout.service";
import { ConfirmDialogService } from "../../../shared/confirm-dialog/confirm-dialog.service";

@Injectable()
export class AccessKeyDetailFacade {
	private readonly store = inject(AccessKeyDetailStore);
	private readonly navigation = inject(NavigationService);
	private readonly confirm_dialog = inject(ConfirmDialogService);
	private readonly delete_command = inject(DeleteAccessKeysCommand);
	private readonly get_query = inject(GetAccessKeyQuery);
	private readonly update_command = inject(UpdateAccessKeyCommand);
	private readonly layout = inject(LayoutService);

	// UI State
	public readonly is_editing = signal(false);

	public async init(id: string): Promise<void> {
		this.layout.set_loading(true);
		this.layout.set_error(null);
		try {
			const item = await this.get_query.execute(id);
			this.store.set_item(item);
		} catch (e) {
			const message =
				e instanceof Error ? e.message : "Failed to load access key detail";
			this.layout.set_error(message);
		} finally {
			this.layout.set_loading(false);
		}
	}

	public async load_key(id: string): Promise<void> {
		try {
			const item = await this.get_query.execute(id);
			this.store.set_item(item);
		} catch (e) {
			const message =
				e instanceof Error ? e.message : "Failed to load access key detail";
			this.layout.set_error(message);
		} finally {
			this.layout.set_loading(false);
		}
	}

	public destroy(): void {
		this.store.clear();
	}

	public enter_edit_mode(): void {
		this.is_editing.set(true);
	}

	public cancel_edit(id: string): void {
		this.is_editing.set(false);
		this.load_key(id);
	}

	public async update(contract: UpdateAccessKeyContract): Promise<boolean> {
		this.layout.set_loading(true);
		this.layout.set_error(null);
		try {
			const updated_item = await this.update_command.execute(contract);
			this.store.set_item(updated_item);
			this.is_editing.set(false);
			return true;
		} catch (e) {
			const message =
				e instanceof Error ? e.message : "Failed to update access key";
			this.layout.set_error(message);
			return false;
		} finally {
			this.layout.set_loading(false);
		}
	}

	public async delete(): Promise<void> {
		const confirmed = await this.confirm_dialog.confirm_delete("access_key");
		if (confirmed) {
			const access_key = this.store.selected_item();
			if (access_key) {
				await this.delete_command.execute([access_key.id]);
				this.navigation.to_access_key_list();
			}
		}
	}
}
