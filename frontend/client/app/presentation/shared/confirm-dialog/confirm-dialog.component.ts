import { CommonModule } from "@angular/common";
import { Component, inject } from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import {
	MAT_DIALOG_DATA,
	MatDialogModule,
	MatDialogRef,
} from "@angular/material/dialog";
import { MatIconModule } from "@angular/material/icon";
import { NgxTolgeeModule } from "@tolgee/ngx";

export interface ConfirmDialogData {
	title: string;
	message: string;
	confirm_text?: string;
	cancel_text?: string;
	type?: "danger" | "warning" | "info";
}

@Component({
	imports: [
		CommonModule,
		MatButtonModule,
		MatDialogModule,
		MatIconModule,
		NgxTolgeeModule,
	],
	selector: "app-confirm-dialog",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./confirm-dialog.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./confirm-dialog.component.html",
})
export class ConfirmDialogComponent {
	private readonly dialog_ref = inject(MatDialogRef<ConfirmDialogComponent>);
	public readonly data = inject<ConfirmDialogData>(MAT_DIALOG_DATA);

	public get icon(): string {
		switch (this.data.type) {
			case "danger":
				return "warning";
			case "warning":
				return "error_outline";
			default:
				return "help_outline";
		}
	}

	public get confirm_text(): string {
		return this.data.confirm_text || "common.confirm";
	}

	public get cancel_text(): string {
		return this.data.cancel_text || "common.cancel";
	}

	public confirm(): void {
		this.dialog_ref.close(true);
	}

	public cancel(): void {
		this.dialog_ref.close(false);
	}
}
