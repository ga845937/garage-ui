import type { Bucket, BucketListItem } from "@shared/entity/bucket.entity";

import { CommonModule } from "@angular/common";
import { Component, EventEmitter, Input, inject, Output } from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { MatIconModule } from "@angular/material/icon";
import { MatTooltipModule } from "@angular/material/tooltip";
import { NgxTolgeeModule } from "@tolgee/ngx";

import { DateFormatService } from "../../../../../../infrastructure/services/date-format.service";
import { LayoutService } from "../../../../../layout/layout.service";
import { CopyButtonComponent } from "../../../../../shared/copy-button/copy-button.component";

@Component({
	imports: [
		CommonModule,
		MatButtonModule,
		MatCheckboxModule,
		MatIconModule,
		MatTooltipModule,
		NgxTolgeeModule,
		CopyButtonComponent,
	],
	selector: "app-bucket-card",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./bucket-card.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./bucket-card.component.html",
})
export class BucketCardComponent {
	@Input({ required: true }) public bucket!: Bucket | BucketListItem;
	@Input() public selection_mode = false;
	@Input() public selected = false;
	@Output() public card_click = new EventEmitter<void>();
	@Output() public files_click = new EventEmitter<void>();
	@Output() public delete_click = new EventEmitter<void>();
	@Output() public selection_change = new EventEmitter<boolean>();

	private readonly layout = inject(LayoutService);
	public readonly date_format = inject(DateFormatService);

	public get_primary_alias(): string {
		return this.bucket.global_aliases[0] ?? "Unnamed Bucket";
	}

	public get visible_aliases(): string[] {
		return this.bucket.global_aliases.slice(0, 2);
	}

	public get extra_aliases_count(): number {
		return Math.max(0, this.bucket.global_aliases.length - 2);
	}

	public get all_aliases_tooltip(): string {
		return this.bucket.global_aliases.join(", ");
	}

	public has_bucket_stats(): boolean {
		return "objects" in this.bucket && "bytes" in this.bucket;
	}

	public get_bucket_full(): Bucket {
		return this.bucket as Bucket;
	}

	public format_number(num: number): string {
		if (num >= 1000000) {
			return `${(num / 1000000).toFixed(1)}M`;
		}
		if (num >= 1000) {
			return `${(num / 1000).toFixed(1)}K`;
		}
		return num.toString();
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

	public copy_id(): void {
		this.layout.copy_to_clipboard(this.bucket.id);
	}

	public on_card_click(): void {
		if (this.selection_mode) {
			this.selection_change.emit(!this.selected);
		} else {
			this.card_click.emit();
		}
	}

	public on_selection_change(checked: boolean): void {
		this.selection_change.emit(checked);
	}

	public on_files_click(): void {
		this.files_click.emit();
	}

	public on_delete_click(): void {
		this.delete_click.emit();
	}
}
