import type { OnChanges, SimpleChanges } from "@angular/core";
import type { UpdateAccessKeyContract } from "@shared/contracts";
import type { ValidationResult } from "../../../../../../domain/aggregates/access-key.aggregate";

import { CommonModule, DatePipe } from "@angular/common";
import { Component, EventEmitter, Input, inject, Output } from "@angular/core";
import { FormBuilder, ReactiveFormsModule, Validators } from "@angular/forms";
import { MatButtonModule } from "@angular/material/button";
import { MatFormFieldModule } from "@angular/material/form-field";
import { MatIconModule } from "@angular/material/icon";
import { MatInputModule } from "@angular/material/input";
import { MatSlideToggleModule } from "@angular/material/slide-toggle";
import { MatTooltipModule } from "@angular/material/tooltip";
import { NgxTolgeeModule } from "@tolgee/ngx";

import { AccessKeyAggregate } from "../../../../../../domain/aggregates/access-key.aggregate";
import { DatetimePickerComponent } from "../../../../../shared/datetime-picker/datetime-picker.component";

@Component({
	imports: [
		CommonModule,
		DatePipe,
		DatetimePickerComponent,
		ReactiveFormsModule,
		MatButtonModule,
		MatFormFieldModule,
		MatIconModule,
		MatInputModule,
		MatSlideToggleModule,
		MatTooltipModule,
		NgxTolgeeModule,
	],
	selector: "app-access-key-properties",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./access-key-properties.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./access-key-properties.component.html",
})
export class AccessKeyPropertiesComponent implements OnChanges {
	private readonly form_builder = inject(FormBuilder);

	@Input({ required: true })
	public access_key!: AccessKeyAggregate;

	@Input()
	public show_secret = false;

	@Input()
	public editable = false;

	@Output()
	public readonly save = new EventEmitter<UpdateAccessKeyContract>();

	@Output()
	public readonly cancel = new EventEmitter<void>();

	@Output()
	public readonly toggle_secret = new EventEmitter<void>();

	@Output()
	public readonly copy_to_clipboard = new EventEmitter<string>();

	public readonly form = this.form_builder.group({
		allow_create_bucket: [false],
		expiration: [""],
		name: ["", [Validators.required, Validators.maxLength(100)]],
	});

	public name_error: string | null = null;
	public expiration_error: string | null = null;

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnChanges(changes: SimpleChanges): void {
		// biome-ignore lint/complexity/useLiteralKeys: Angular
		if (changes["access_key"] && this.access_key) {
			this.sync_form_from_aggregate();
		}
		
		// biome-ignore lint/complexity/useLiteralKeys: Angular
		if (changes["editable"]) {
			if (this.editable) {
				this.form.enable();
			} else {
				this.form.disable();
			}
		}
	}

	public on_submit(): void {
		if (this.validate_form()) {
			const expiration_value =
				this.form.value.expiration === ""
					? undefined
					: this.form.value.expiration || undefined;

			const contract = this.access_key.to_update_contract({
				allow_create_bucket:
					this.form.value.allow_create_bucket ?? undefined,
				expiration: expiration_value,
				name: this.form.value.name ?? undefined,
			});
			this.save.emit(contract);
		} else {
			console.log("Form is invalid");
		}
	}

	public on_cancel(): void {
		this.sync_form_from_aggregate();
		this.cancel.emit();
	}

	public on_expiration_change(value: string | null): void {
		this.form.patchValue({ expiration: value ?? "" });
		this.form.markAsDirty();
	}

	public on_toggle_secret(): void {
		this.toggle_secret.emit();
	}

	public on_copy(text: string): void {
		this.copy_to_clipboard.emit(text);
	}

	public get is_valid(): boolean {
		return this.form.valid && this.form.dirty;
	}

	private sync_form_from_aggregate(): void {
		this.form.patchValue({
			allow_create_bucket: this.access_key.allow_create_bucket,
			expiration: this.access_key.expiration_formatted,
			name: this.access_key.name,
		});
		this.form.markAsPristine();
		this.name_error = null;
		this.expiration_error = null;
		
		if (!this.editable) {
			this.form.disable();
		}
	}

	private validate_form(): boolean {
		const name_result: ValidationResult = AccessKeyAggregate.validate_name(
			this.form.value.name ?? "",
		);

		const expiration_control = this.form.get("expiration");
		let expiration_result: ValidationResult = { is_valid: true };

		if (expiration_control?.dirty) {
			expiration_result = AccessKeyAggregate.validate_expiration(
				this.form.value.expiration ?? undefined,
			);
		}

		this.name_error = name_result.error ?? null;
		this.expiration_error = expiration_result.error ?? null;

		return name_result.is_valid && expiration_result.is_valid;
	}
}
