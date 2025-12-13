import type { UpdateBucketData } from "../../../../infrastructure/repositories/bucket.repository";

import { Injectable, inject, signal } from "@angular/core";

import {
	DeleteBucketsCommand,
	UpdateBucketCommand,
} from "../../../../application/commands";
import { GetBucketQuery } from "../../../../application/queries";
import { BucketDetailStore } from "../../../../application/stores";
import { NavigationService } from "../../../../infrastructure/navigation";
import { LayoutService } from "../../../layout/layout.service";
import { ConfirmDialogService } from "../../../shared/confirm-dialog";

@Injectable()
export class BucketDetailFacade {
	private readonly get_query = inject(GetBucketQuery);
	private readonly update_command = inject(UpdateBucketCommand);
	private readonly delete_command = inject(DeleteBucketsCommand);
	private readonly navigation = inject(NavigationService);
	private readonly layout = inject(LayoutService);
	private readonly confirm_dialog = inject(ConfirmDialogService);
	private readonly store = inject(BucketDetailStore);

	// UI State
	public readonly is_editing = signal(false);

	public async init(id: string): Promise<void> {
		this.layout.set_title_key("bucket.detail_title");
		if (id) {
			await this.load_bucket(id);
		}
	}

	public async load_bucket(id: string): Promise<void> {
		this.layout.set_loading(true);
		try {
			const bucket = await this.get_query.execute(id);
			this.store.set_item(bucket);
		} catch (e) {
			const message = e instanceof Error ? e.message : "Failed to load bucket";
			this.layout.set_error(message);
		} finally {
			this.layout.set_loading(false);
		}
	}

	public destroy(): void {
		this.layout.clear_actions();
		this.store.clear();
	}

	public get_bucket_name(): string {
		const bucket = this.store.selected_item();
		return bucket?.display_name ?? "Bucket";
	}

	public enter_edit_mode(): void {
		this.is_editing.set(true);
	}

	public cancel_edit(id: string): void {
		this.is_editing.set(false);
		this.load_bucket(id);
	}

	public async save(id: string, data: UpdateBucketData): Promise<boolean> {
		this.layout.set_loading(true);
		try {
			const bucket = await this.update_command.execute(id, data);
			this.store.set_item(bucket);
			this.is_editing.set(false);
			return true;
		} catch (e) {
			const message =
				e instanceof Error ? e.message : "Failed to update bucket";
			this.layout.set_error(message);
			return false;
		} finally {
			this.layout.set_loading(false);
		}
	}

	public async delete(id: string): Promise<void> {
		const confirmed = await this.confirm_dialog.confirm_delete("bucket");
		if (confirmed) {
			this.layout.set_loading(true);
			try {
				await this.delete_command.execute([id]);
				this.navigation.to_bucket_list();
			} catch (e) {
				const message =
					e instanceof Error ? e.message : "Failed to delete bucket";
				this.layout.set_error(message);
			} finally {
				this.layout.set_loading(false);
			}
		}
	}

	public go_to_files(id: string): void {
		this.navigation.to_bucket_objects(id);
	}
}
