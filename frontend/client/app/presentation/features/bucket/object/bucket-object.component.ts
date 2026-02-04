import type {
	OnDestroy,
	OnInit,
} from "@angular/core";
import type { ObjectItem } from "./components/object-card/object-card.component";

interface ThumbnailState {
	loading: boolean;
	error: boolean;
	url: string | null;
}

import { CommonModule } from "@angular/common";
import {
	ChangeDetectorRef,
	Component,
	computed,
	effect,
	HostListener,
	Input,
	inject,
} from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatButtonToggleModule } from "@angular/material/button-toggle";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { MatDialogModule } from "@angular/material/dialog";
import { MatIconModule } from "@angular/material/icon";
import { MatMenuModule } from "@angular/material/menu";
import { MatProgressBarModule } from "@angular/material/progress-bar";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { MatTableModule } from "@angular/material/table";
import { MatTooltipModule } from "@angular/material/tooltip";
import { NgxTolgeeModule } from "@tolgee/ngx";

import { BucketObjectStore } from "../../../../application/stores";
import { ThumbnailApiService } from "../../../../infrastructure/api/thumbnail-api.service";
import { I18nService } from "../../../../infrastructure/i18n/i18n.service";
import { BreadcrumbComponent } from "../../../layout/breadcrumb/breadcrumb.component";
import { LayoutService } from "../../../layout/layout.service";
import { BucketObjectFacade } from "./bucket-object.facade";
import { ObjectCardComponent } from "./components/object-card/object-card.component";

const TRAILING_SLASH_REGEX = /\/$/;

@Component({
	imports: [
		CommonModule,
		MatButtonModule,
		MatButtonToggleModule,
		MatCheckboxModule,
		MatDialogModule,
		MatIconModule,
		MatMenuModule,
		MatProgressBarModule,
		MatProgressSpinnerModule,
		MatTableModule,
		MatTooltipModule,
		NgxTolgeeModule,
		BreadcrumbComponent,
		ObjectCardComponent,
	],
	providers: [BucketObjectFacade],
	selector: "app-bucket-files",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./bucket-object.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./bucket-object.component.html",
})
export class BucketObjectComponent implements OnInit, OnDestroy {
	@Input() public bucket_id = "";

	public readonly store = inject(BucketObjectStore);
	private readonly facade = inject(BucketObjectFacade);
	public readonly thumbnail_api = inject(ThumbnailApiService);
	private readonly cdr = inject(ChangeDetectorRef);
	private readonly layout = inject(LayoutService);
	private readonly i18n = inject(I18nService);

	public readonly view_mode = this.facade.view_mode;
	public readonly is_dragging = this.facade.is_dragging;
	public readonly selection_mode = this.facade.selection_mode;
	public readonly selected_items = this.facade.selected_items;

	public bucket_name = computed(() => {
		const bucket = this.store.selected_bucket();
		if (bucket) {
			return bucket.global_aliases?.[0] || bucket.id;
		}
		return this.bucket_id;
	});

	// Thumbnail state for list view
	private thumbnail_states = new Map<string, ThumbnailState>();

	public displayed_columns = computed(() => {
		const base = ["icon", "name", "size", "modified", "actions"];
		if (this.selection_mode()) {
			return ["select", ...base];
		}
		return base;
	});

	public constructor() {
		effect(() => {
			const items = this.combined_items();
			const mode = this.view_mode();
			
			if (mode === "list") {
				setTimeout(() => {
					for (const item of items) {
						if (!item.is_folder && this.thumbnail_api.is_image(item.name) && item.etag) {
							this.load_thumbnail_for_item(item);
						}
					}
				}, 0);
			}
		});

		effect(() => {
			this.update_actions();
		});
	}

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnInit(): void {
		this.layout.set_title_key("nav.object");
		this.facade.init(this.bucket_id);
	}

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnDestroy(): void {
		this.layout.clear_actions();
		this.facade.destroy();
		for (const state of this.thumbnail_states.values()) {
			if (state.url) {
				this.thumbnail_api.revoke_url(state.url);
			}
		}
		this.thumbnail_states.clear();
	}

	@HostListener("document:visibilitychange")
	public on_visibility_change(): void {
		if (document.visibilityState === "visible") {
			this.facade.load_objects();
		}
	}

	public go_back(): void {
		this.facade.go_back();
	}

	public navigate_to(prefix: string): void {
		this.facade.navigate_to(prefix);
	}

	public go_back_in_history(): void {
		this.facade.go_back_in_history();
	}

	public go_forward_in_history(): void {
		this.facade.go_forward_in_history();
	}

	public set_view_mode(mode: "grid" | "list"): void {
		this.facade.set_view_mode(mode);
	}

	public open_upload_dialog(): void {
		this.facade.open_upload_dialog();
	}

	public download_file(key: string): void {
		this.facade.download_file(key);
	}

	public delete_file(key: string[]): void {
		this.facade.delete_file(key);
	}

	public toggle_select_item(key: string): void {
		this.facade.toggle_select_item(key);
	}

	public select_all(): void {
		this.facade.select_all();
	}

	public clear_uploads(): void {
		this.facade.clear_uploads();
	}

	public upload_files(files: File[]): void {
		this.facade.upload_files(files);
	}

	public set_dragging(dragging: boolean): void {
		this.facade.set_dragging(dragging);
	}

	public combined_items = computed<ObjectItem[]>(() => {
		const stats = this.store.folder_stats();

		const folders = this.store.prefixes().map((p) => ({
			is_folder: true,
			is_truncated: stats[p]?.is_truncated || false,
			key: p,
			name: p
				.replace(this.store.current_prefix(), "")
				.replace(TRAILING_SLASH_REGEX, ""),
			object_count: stats[p]?.count || 0,
			total_size: stats[p]?.size || 0,
		}));

		const files = this.store.objects().map((o) => ({
			etag: o.etag,
			is_folder: false,
			key: o.key,
			last_modified: o.last_modified,
			name: o.key.replace(this.store.current_prefix(), ""),
			size: o.size,
		}));

		return [...folders, ...files];
	});



	public on_files_selected(event: Event): void {
		const input = event.target as HTMLInputElement;
		if (input.files) {
			this.upload_files(Array.from(input.files));
		}
	}

	public on_drag_over(event: DragEvent): void {
		event.preventDefault();
		this.set_dragging(true);
	}

	public on_drag_leave(event: DragEvent): void {
		event.preventDefault();
		this.set_dragging(false);
	}

	public on_drop(event: DragEvent): void {
		event.preventDefault();
		this.set_dragging(false);

		if (event.dataTransfer?.files) {
			this.upload_files(Array.from(event.dataTransfer.files));
		}
	}

	public format_bytes(bytes: number): string {
		if (bytes === 0) {
			return "0 B";
		}
		const k = 1024;
		const sizes = ["B", "KB", "MB", "GB", "TB"];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return `${Number.parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`;
	}

	// Thumbnail methods for list view
	public get_thumbnail_state(key: string): ThumbnailState {
		const existing = this.thumbnail_states.get(key);
		if (existing) {
			return existing;
		}

		const new_state: ThumbnailState = {
			error: false,
			loading: false,
			url: null,
		};
		this.thumbnail_states.set(key, new_state);
		return new_state;
	}

	public load_thumbnail_for_item(item: ObjectItem): void {
		if (
			item.is_folder ||
			!this.thumbnail_api.is_image(item.name) ||
			!item.etag
		) {
			return;
		}

		const state = this.get_thumbnail_state(item.key);
		if (state.loading || state.url || state.error) {
			return;
		}

		state.loading = true;
		this.cdr.markForCheck();

		this.thumbnail_api
			.get_thumbnail(this.bucket_name(), item.key, item.etag, "list")
			.subscribe({
				error: () => {
					state.error = true;
					state.loading = false;
					this.cdr.markForCheck();
				},
				next: (url: string) => {
					state.url = url;
					state.loading = false;
					this.cdr.markForCheck();
				},
			});
	}

	public on_thumbnail_error(key: string): void {
		const state = this.get_thumbnail_state(key);
		state.error = true;
		this.cdr.markForCheck();
	}

	public get_file_icon(filename: string): string {
		const ext = filename.split(".").pop()?.toLowerCase() || "";
		const icon_map: Record<string, string> = {
			"7z": "folder_zip",
			css: "code",
			doc: "description",
			docx: "description",
			gif: "image",
			html: "code",
			jpeg: "image",
			jpg: "image",
			js: "code",
			json: "data_object",
			mp3: "audio_file",
			mp4: "movie",
			pdf: "picture_as_pdf",
			png: "image",
			rar: "folder_zip",
			svg: "image",
			ts: "code",
			txt: "article",
			wav: "audio_file",
			webp: "image",
			zip: "folder_zip",
		};
		return icon_map[ext] || "insert_drive_file";
	}

	private update_actions(): void {
		if (this.selection_mode()) {
			const count = this.selected_items().size;
			this.layout.set_actions([
				{
					action: () => this.facade.exit_selection_mode(),
					icon: "close",
					label_key: "common.cancel",
					variant: "default",
				},
				{
					action: () => this.facade.delete_selected(),
					disabled: count === 0,
					icon: "delete",
					label: `${this.i18n.t("common.delete")} (${count})`,
					variant: "danger",
				},
			]);
		} else {
			this.layout.set_actions([
				{
					action: () => this.facade.create_folder(),
					icon: "create_new_folder",
					label_key: "file.create_folder",
					variant: "default",
				},
				{
					action: () => this.facade.enter_selection_mode(),
					icon: "delete",
					label_key: "common.delete",
					variant: "danger",
				},
				{
					action: () => this.open_upload_dialog(),
					icon: "upload",
					label_key: "bucket.upload",
					variant: "primary",
				},
			]);
		}
}
	}
