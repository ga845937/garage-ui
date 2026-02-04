import type { AfterViewInit, OnDestroy } from "@angular/core";

import { CommonModule } from "@angular/common";
import { ChangeDetectorRef, Component, EventEmitter, Input, inject, Output } from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { MatIconModule } from "@angular/material/icon";
import { MatMenuModule } from "@angular/material/menu";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { MatTooltipModule } from "@angular/material/tooltip";
import { NgxTolgeeModule } from "@tolgee/ngx";

import { ThumbnailApiService } from "../../../../../../infrastructure/api/thumbnail-api.service";

export interface ObjectItem {
	key: string;
	name: string;
	size?: number;
	last_modified?: string;
	is_folder: boolean;
	etag?: string;
	object_count?: number;
	total_size?: number;
	is_truncated?: boolean; // 如果 true，表示檔案數量超過 1000
}

@Component({
	imports: [
		CommonModule,
		MatButtonModule,
		MatIconModule,
		MatMenuModule,
		MatTooltipModule,
		MatCheckboxModule,
		MatProgressSpinnerModule,
		NgxTolgeeModule,
	],
	selector: "app-object-card",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./object-card.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./object-card.component.html",
})
export class ObjectCardComponent implements OnDestroy, AfterViewInit {
	@Input({ required: true }) public item!: ObjectItem;
	@Input({ required: true }) public bucket_id!: string;
	@Input({ required: true }) public bucket_name!: string;
	@Input() public selection_mode = false;
	@Input() public selected = false;
	@Output() public navigate = new EventEmitter<string>();
	@Output() public download = new EventEmitter<string>();
	@Output() public delete = new EventEmitter<string>();
	@Output() public toggle_select = new EventEmitter<string>();

	private readonly thumbnail_api = inject(ThumbnailApiService);
	private readonly cdr = inject(ChangeDetectorRef);

	// Thumbnail state
	public thumbnail_url: string | null = null;
	public thumbnail_loading = false;
	public thumbnail_error = false;

	public get is_image(): boolean {
		return (
			!this.item.is_folder && this.thumbnail_api.is_image(this.item.name)
		);
	}

	// biome-ignore lint/style/useNamingConvention: Angular lifecycle
	public ngOnDestroy(): void {
		if (this.thumbnail_url) {
			this.thumbnail_api.revoke_url(this.thumbnail_url);
		}
	}

	// biome-ignore lint/style/useNamingConvention: Angular lifecycle
	public ngAfterViewInit(): void {
		if (this.is_image && this.item.etag) {
			setTimeout(() => {
				this.load_thumbnail();
			}, 0);
		}
	}

	public load_thumbnail(): void {
		if (
			!this.is_image ||
			this.thumbnail_loading ||
			this.thumbnail_url ||
			this.thumbnail_error ||
			!this.item.etag
		) {
			return;
		}

		this.thumbnail_loading = true;
		this.cdr.markForCheck();

		this.thumbnail_api
			.get_thumbnail(
				this.bucket_name,
				this.item.key,
				this.item.etag,
				"grid",
			)
			.subscribe({
				error: () => {
					this.thumbnail_error = true;
					this.thumbnail_loading = false;
					this.cdr.markForCheck();
				},
				next: (url: string) => {
					this.thumbnail_url = url;
					this.thumbnail_loading = false;
					this.cdr.markForCheck();
				},
			});
	}

	public on_click(): void {
		if (this.selection_mode) {
			this.toggle_select.emit(this.item.key);
		} else if (this.item.is_folder) {
			this.navigate.emit(this.item.key);
		}
	}

	public on_download(): void {
		this.download.emit(this.item.key);
	}

	public on_delete(): void {
		this.delete.emit(this.item.key);
	}

	public get_file_icon(): string {
		const ext = this.item.name.split(".").pop()?.toLowerCase() || "";
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

	public format_bytes(bytes: number): string {
		if (bytes === 0) {
			return "0 B";
		}
		const k = 1024;
		const sizes = ["B", "KB", "MB", "GB", "TB"];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return `${Number.parseFloat((bytes / k ** i).toFixed(1))} ${sizes[i]}`;
	}
}
