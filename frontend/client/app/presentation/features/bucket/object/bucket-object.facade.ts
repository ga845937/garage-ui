import type { UploadDialogResult } from "./components/upload-dialog/upload-dialog.component";

import { Injectable, inject, signal } from "@angular/core";
import { MatDialog } from "@angular/material/dialog";

import {
	DeleteObjectCommand,
	UploadObjectCommand,
} from "../../../../application/commands";
import {
	GetBucketQuery,
	ListObjectsQuery,
} from "../../../../application/queries";
import { BucketObjectStore } from "../../../../application/stores";
import { BucketService } from "../../../../infrastructure/api/bucket.service";
import { I18nService } from "../../../../infrastructure/i18n/i18n.service";
import { NavigationService } from "../../../../infrastructure/navigation";
import { LayoutService } from "../../../layout/layout.service";
import { ConfirmDialogService } from "../../../shared/confirm-dialog";
import { UploadDialogComponent } from "./components/upload-dialog/upload-dialog.component";

@Injectable()
export class BucketObjectFacade {
	private readonly store = inject(BucketObjectStore);
	private readonly get_bucket_query = inject(GetBucketQuery);
	private readonly list_objects_query = inject(ListObjectsQuery);
	private readonly upload_command = inject(UploadObjectCommand);
	private readonly delete_command = inject(DeleteObjectCommand);
	private readonly bucket_service = inject(BucketService);
	private readonly navigation = inject(NavigationService);
	private readonly layout = inject(LayoutService);
	private readonly i18n = inject(I18nService);
	private readonly confirm_dialog = inject(ConfirmDialogService);
	private readonly dialog = inject(MatDialog);

	// UI State
	public readonly view_mode = signal<"grid" | "list">("grid");
	public readonly is_dragging = signal(false);
	public readonly selection_mode = signal(false);
	public readonly selected_items = signal<Set<string>>(new Set());

	public get bucket_id(): string {
		return this.store.selected_bucket()?.id ?? "";
	}

	public get bucket_name(): string {
		const bucket = this.store.selected_bucket();
		if (bucket) {
			return bucket.global_aliases[0] ?? bucket.id;
		}
		return "";
	}

	public async init(bucket_id: string): Promise<void> {
		this.store.clear_state();

		this.layout.set_loading(true);
		try {
			const bucket = await this.get_bucket_query.execute(bucket_id);
			this.store.set_selected_bucket(bucket);
		} catch (e) {
			const message =
				e instanceof Error ? e.message : "Failed to load bucket details";
			this.layout.set_error(message);
			this.layout.set_loading(false);
			return;
		}

		await this.load_objects();
		this.layout.set_loading(false);
	}

	public destroy(): void {
		this.exit_selection_mode();
	}

	public async load_objects(prefix?: string): Promise<void> {
		const p = prefix ?? this.store.current_prefix();
		
		this.store.set_loading_objects(true);
		this.store.set_current_prefix(p);
		
		try {
			const result = await this.list_objects_query.execute(this.bucket_name, p);
			this.store.set_objects(result.objects);
			this.store.set_prefixes(result.prefixes);
			this.store.set_folder_stats(result.folder_stats);
		} catch (e) {
			// Handle error
		} finally {
			this.store.set_loading_objects(false);
		}
	}

	public async navigate_to(prefix: string): Promise<void> {
		if (this.selection_mode()) {
			return;
		}
		this.store.push_to_history(prefix);
		await this.load_objects(prefix);
	}

	public async go_back_in_history(): Promise<void> {
		const prefix = this.store.navigate_back();
		if (prefix !== null) {
			await this.load_objects(prefix);
		}
	}

	public async go_forward_in_history(): Promise<void> {
		const prefix = this.store.navigate_forward();
		if (prefix !== null) {
			await this.load_objects(prefix);
		}
	}

	public async create_folder(): Promise<void> {
		const folder_name = window.prompt(
			this.i18n.t("file.create_folder_prompt"),
		);
		if (folder_name?.trim()) {
			const key = `${this.store.current_prefix()}${folder_name.trim()}/`;
			const empty_blob = new Blob([], {
				type: "application/x-directory",
			});
			const empty_file = new File([empty_blob], ".folder", {
				type: "application/x-directory",
			});
			await this.upload_command.execute(
				this.bucket_id,
				this.bucket_name,
				key,
				empty_file,
			);
			await this.load_objects();
		}
	}

	public open_upload_dialog(): void {
		const dialog_ref = this.dialog.open(UploadDialogComponent, {
			// biome-ignore lint/style/useNamingConvention: Angular Material
			backdropClass: "upload-dialog-backdrop",
			// biome-ignore lint/style/useNamingConvention: Angular Material
			panelClass: "upload-dialog-panel",
		});

		dialog_ref
			.afterClosed()
			.subscribe((result: UploadDialogResult | undefined) => {
				if (result?.files && result.files.length > 0) {
					this.upload_files_with_paths(result.files);
				}
			});
	}

	private pending_uploads = 0;

	private upload_files_with_paths(
		// biome-ignore lint/style/useNamingConvention: WebAPI interface
		files: { file: File; relativePath: string }[],
	): void {
		this.pending_uploads += files.length;
		for (const { file, relativePath: relative_path } of files) {
			const key = this.store.current_prefix() + relative_path;
			this.upload_command
				.execute(this.bucket_id, this.bucket_name, key, file)
				.finally(() => {
					this.pending_uploads -= 1;
					if (this.pending_uploads === 0) {
						this.load_objects();
					}
				});
		}
	}

	public upload_files(files: File[]): void {
		this.pending_uploads += files.length;
		for (const file of files) {
			const key = this.store.current_prefix() + file.name;
			this.upload_command
				.execute(this.bucket_id, this.bucket_name, key, file)
				.finally(() => {
					this.pending_uploads -= 1;
					if (this.pending_uploads === 0) {
						this.load_objects();
					}
				});
		}
	}

	public download_file(key: string): void {
		const url = this.bucket_service.get_download_url(this.bucket_name, key);
		
		const link = document.createElement('a');
		link.href = url;
		link.download = key.split('/').pop() || key;
		document.body.appendChild(link);
		link.click();
		document.body.removeChild(link);
	}

	public async delete_file(key: string[]): Promise<void> {
		const confirmed = await this.confirm_dialog.confirm({
			message: this.i18n.t("file.delete_confirm"),
			title: this.i18n.t("file.delete_confirm_title"),
			type: "danger",
		});
		if (confirmed) {
			await this.delete_command.execute(this.bucket_name, key);
			await this.load_objects();
			if (this.selection_mode()) {
				this.exit_selection_mode();
			}
		}
	}

	public enter_selection_mode(): void {
		this.selection_mode.set(true);
		this.selected_items.set(new Set());
	}

	public exit_selection_mode(): void {
		this.selection_mode.set(false);
		this.selected_items.set(new Set());
	}

	public toggle_select_item(key: string): void {
		this.selected_items.update((current) => {
			const next = new Set(current);
			if (next.has(key)) {
				next.delete(key);
			} else {
				next.add(key);
			}
			return next;
		});
	}

	public select_all(): void {
		const all_keys = this.store.objects().map(o => o.key).concat(this.store.prefixes());
		this.selected_items.set(new Set(all_keys));
	}

	public async delete_selected(): Promise<void> {
		const keys = Array.from(this.selected_items());
		if (keys.length === 0) {
			return;
		}
		await this.delete_file(keys);
	}

	public clear_uploads(): void {
		this.store.clear_upload_progress();
	}

	public go_back(): void {
		this.navigation.to_bucket_list();
	}

	public set_dragging(dragging: boolean): void {
		this.is_dragging.set(dragging);
	}

	public set_view_mode(mode: "grid" | "list"): void {
		this.view_mode.set(mode);
	}
}
