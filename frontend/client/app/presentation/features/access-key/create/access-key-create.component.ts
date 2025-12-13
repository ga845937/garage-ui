import type { OnDestroy, OnInit } from "@angular/core";
import type { FormGroup } from "@angular/forms";

import { Component, inject, signal } from "@angular/core";
import { FormBuilder, ReactiveFormsModule, Validators } from "@angular/forms";
import { MatButtonModule } from "@angular/material/button";
import { MatCardModule } from "@angular/material/card";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { MatFormFieldModule } from "@angular/material/form-field";
import { MatIconModule } from "@angular/material/icon";
import { MatInputModule } from "@angular/material/input";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { MatTooltipModule } from "@angular/material/tooltip";
import { KeyRoute } from "@shared/route-paths";
import { NgxTolgeeModule } from "@tolgee/ngx";

import { I18nService } from "../../../../infrastructure/i18n/i18n.service";
import { LayoutService } from "../../../layout/layout.service";
import { DatetimePickerComponent } from "../../../shared/datetime-picker/datetime-picker.component";
import { AccessKeyCreateFacade } from "./access-key-create.facade";

@Component({
	imports: [
		ReactiveFormsModule,
		DatetimePickerComponent,
		MatButtonModule,
		MatCardModule,
		MatCheckboxModule,
		MatFormFieldModule,
		MatIconModule,
		MatInputModule,
		MatProgressSpinnerModule,
		MatTooltipModule,
		NgxTolgeeModule,
	],
	providers: [AccessKeyCreateFacade],
	selector: "app-access-key-create",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./access-key-create.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./access-key-create.component.html",
})
export class AccessKeyCreateComponent implements OnInit, OnDestroy {
	public readonly key_route = KeyRoute;

	// UI State
	public readonly submitting = signal(false);

	private readonly facade = inject(AccessKeyCreateFacade);
	private readonly layout = inject(LayoutService);
	private readonly fb = inject(FormBuilder);
	protected readonly i18n = inject(I18nService);

	public form: FormGroup = this.fb.group({
		allow_create_bucket: [false],
		expiration: [""],
		name: ["", [Validators.required, Validators.minLength(1)]],
	});

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnInit(): void {
		this.layout.set_title_key("access_key.create_title");
		this.update_actions();
	}

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnDestroy(): void {
		this.layout.clear_actions();
	}

	public async on_submit(): Promise<void> {
		if (this.form.valid) {
			const { name, expiration, allow_create_bucket } = this.form.value;
			let rfc3339_expiration: string | undefined;
			if (expiration) {
				const date = new Date(expiration);
				rfc3339_expiration = date.toISOString();
			}

			this.submitting.set(true);
			await this.facade.create({
				allow_create_bucket,
				expiration: rfc3339_expiration,
				name,
			});
			this.submitting.set(false);
		}
	}

	public cancel(): void {
		this.facade.cancel();
	}

	public on_expiration_change(value: string | null): void {
		this.form.patchValue({ expiration: value ?? "" });
	}

	private update_actions(): void {
		this.layout.set_actions([
			{
				action: () => this.cancel(),
				icon: "close",
				label_key: "common.cancel",
			},
			{
				action: () => this.on_submit(),
				icon: "save",
				label_key: "common.save",
				variant: "primary",
			},
		]);
	}
}
