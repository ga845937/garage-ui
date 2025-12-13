import type { OnInit } from "@angular/core";
import type { FormGroup } from "@angular/forms";
import type { BucketKeyPermissionContract } from "@shared/contracts/bucket.contract";

import { Component, inject, signal } from "@angular/core";
import { FormBuilder, ReactiveFormsModule, Validators } from "@angular/forms";
import { MatButtonModule } from "@angular/material/button";
import { MatCardModule } from "@angular/material/card";
import { MatFormFieldModule } from "@angular/material/form-field";
import { MatIconModule } from "@angular/material/icon";
import { MatInputModule } from "@angular/material/input";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { BucketRoute } from "@shared/route-paths";

import { I18nService } from "../../../../infrastructure/i18n/i18n.service";
import { LayoutService } from "../../../layout/layout.service";
import { BucketCreateFacade } from "./bucket-create.facade";
import { BucketCreateKeyListComponent } from "./components/key-list/bucket-create-key-list.component";

const BUCKET_NAME_REGEX = /^[a-z0-9][a-z0-9-]*[a-z0-9]$/;

@Component({
	imports: [
		ReactiveFormsModule,
		MatButtonModule,
		MatCardModule,
		MatFormFieldModule,
		MatIconModule,
		MatInputModule,
		MatProgressSpinnerModule,
		BucketCreateKeyListComponent,
	],
	providers: [BucketCreateFacade],
	selector: "app-bucket-create",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./bucket-create.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./bucket-create.component.html",
})
export class BucketCreateComponent implements OnInit {
	protected readonly facade = inject(BucketCreateFacade);
	private readonly form_builder = inject(FormBuilder);
	protected readonly i18n = inject(I18nService);

	protected readonly key_permissions = signal<BucketKeyPermissionContract[]>(
		[],
	);

	private readonly layout = inject(LayoutService);

	// biome-ignore lint/style/useNamingConvention: Angular
	public ngOnInit(): void {
		this.facade.init();
		this.layout.set_title_key("bucket.create_title");
		this.update_actions();
	}

	public readonly bucket_route = BucketRoute;

	public form: FormGroup = this.form_builder.group({
		global_alias: [
			"",
			[
				Validators.required,
				Validators.minLength(3),
				Validators.maxLength(63),
				Validators.pattern(BUCKET_NAME_REGEX),
			],
		],
	});

	public on_key_selection(permissions: BucketKeyPermissionContract[]): void {
		this.key_permissions.set(permissions);
	}

	public async on_submit(): Promise<void> {
		if (this.form.valid) {
			const { global_alias } = this.form.value;
			await this.facade.create(global_alias, this.key_permissions());
		}
	}

	public cancel(): void {
		this.facade.cancel();
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
