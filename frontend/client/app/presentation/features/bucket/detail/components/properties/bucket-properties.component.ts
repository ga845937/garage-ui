import type { OnChanges, SimpleChanges } from "@angular/core";
import type { BucketAggregate } from "../../../../../../domain/aggregates/bucket.aggregate";
import type { UpdateBucketData } from "../../../../../../infrastructure/repositories/bucket.repository";

import {
	Component,
	EventEmitter,
	Input,
	inject,
	Output,
	signal,
} from "@angular/core";
import { FormBuilder, FormControl, ReactiveFormsModule } from "@angular/forms";
import { MatButtonModule } from "@angular/material/button";
import { MatCardModule } from "@angular/material/card";
import { MatChipsModule } from "@angular/material/chips";
import { MatFormFieldModule } from "@angular/material/form-field";
import { MatIconModule } from "@angular/material/icon";
import { MatInputModule } from "@angular/material/input";
import { MatSlideToggleModule } from "@angular/material/slide-toggle";
import { MatTooltipModule } from "@angular/material/tooltip";

import { I18nService } from "../../../../../../infrastructure/i18n/i18n.service";

@Component({
	imports: [
		MatButtonModule,
		MatCardModule,
		MatChipsModule,
		MatFormFieldModule,
		MatIconModule,
		MatInputModule,
		MatSlideToggleModule,
		MatTooltipModule,
		ReactiveFormsModule,
	],
	selector: "app-bucket-properties",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./bucket-properties.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./bucket-properties.component.html",
})
export class BucketPropertiesComponent implements OnChanges {
	private readonly form_builder = inject(FormBuilder);
	protected readonly i18n = inject(I18nService);

	@Input({ required: true })
	public bucket!: BucketAggregate;

	@Input()
	public editable = false;

	@Output()
	public readonly save = new EventEmitter<UpdateBucketData>();

	@Output()
	public readonly cancel = new EventEmitter<void>();

	@Output()
	public readonly copy_to_clipboard = new EventEmitter<string>();

	public readonly form = this.form_builder.group({
		quotas: this.form_builder.group({
			max_objects: new FormControl<number | null>(null),
			max_size: new FormControl<number | null>(null),
		}),
		quotas_enabled: [false],
		website_access: [false],
	});

	public editing_aliases = signal<string[]>([]);
	public new_alias_input = new FormControl<string>("");

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnChanges(changes: SimpleChanges): void {
		// biome-ignore lint/complexity/useLiteralKeys: Angular
		if (changes["bucket"] && this.bucket) {
			this.sync_form_from_aggregate();
		}

		// biome-ignore lint/complexity/useLiteralKeys: Angular
		if (changes["editable"]) {
			if (this.editable) {
				this.form.enable();
				this.sync_form_from_aggregate();
			} else {
				this.form.disable();
			}
		}
	}

	public on_submit(): void {
		if (this.form.invalid) {
			this.form.markAllAsTouched();
			return;
		}

		const form_value = this.form.getRawValue();
		const quotas: {
			max_size?: number | null;
			max_objects?: number | null;
		} = {};

		if (form_value.quotas_enabled) {
			if (form_value.quotas.max_size !== null) {
				quotas.max_size = Number(form_value.quotas.max_size);
			}
			if (form_value.quotas.max_objects !== null) {
				quotas.max_objects = Number(form_value.quotas.max_objects);
			}
		} else {
			quotas.max_size = null;
			quotas.max_objects = null;
		}

		this.save.emit({
			global_aliases: this.editing_aliases(),
			quotas,
			website_access: form_value.website_access ?? false,
		});
	}

	public on_cancel(): void {
		this.sync_form_from_aggregate();
		this.cancel.emit();
	}

	public add_alias(): void {
		const alias = this.new_alias_input.value?.trim();
		if (!alias) {
			return;
		}

		if (this.editing_aliases().includes(alias)) {
			return;
		}

		this.editing_aliases.update((aliases) => [...aliases, alias]);
		this.new_alias_input.reset();
	}

	public remove_alias(alias: string): void {
		this.editing_aliases.update((aliases) =>
			aliases.filter((a) => a !== alias),
		);
	}

	public on_copy(text: string): void {
		this.copy_to_clipboard.emit(text);
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

	public has_quotas(): boolean {
		return this.bucket?.has_quotas ?? false;
	}

	private sync_form_from_aggregate(): void {
		if (!this.bucket) {
			return;
		}

		this.form.patchValue({
			quotas: {
				max_objects: this.bucket.quotas?.max_objects ?? null,
				max_size: this.bucket.quotas?.max_size ?? null,
			},
			quotas_enabled: this.bucket.has_quotas,
			website_access: this.bucket.website_access,
		});

		this.editing_aliases.set([...this.bucket.global_aliases]);
		this.new_alias_input.reset();
		this.form.markAsPristine();
	}
}
