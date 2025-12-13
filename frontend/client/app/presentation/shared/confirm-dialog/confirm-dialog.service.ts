import type { ConfirmDialogData } from "./confirm-dialog.component";

import { Injectable, inject } from "@angular/core";
import { MatDialog } from "@angular/material/dialog";
import { firstValueFrom } from "rxjs";

import { I18nService } from "../../../infrastructure/i18n/i18n.service";
import { ConfirmDialogComponent } from "./confirm-dialog.component";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class ConfirmDialogService {
	private readonly dialog = inject(MatDialog);
	private readonly i18n = inject(I18nService);

	/**
	 * Show a confirmation dialog
	 * @returns Promise that resolves to true if confirmed, false if cancelled
	 */
	public async confirm(options: ConfirmDialogData): Promise<boolean> {
		const dialog_ref = this.dialog.open(ConfirmDialogComponent, {
			data: options,
			// biome-ignore lint/style/useNamingConvention: Angular Material
			panelClass: "confirm-dialog-panel",
		});

		const result = await firstValueFrom(dialog_ref.afterClosed());
		return result === true;
	}

	/**
	 * Convenience method for delete confirmations
	 * @param item_type - The type of item being deleted (e.g., "access_key", "bucket")
	 * @param count - Number of items being deleted (default: 1)
	 * @returns Promise that resolves to true if confirmed
	 */
	public confirm_delete(
		item_type: string,
		count: number = 1,
	): Promise<boolean> {
		const title = this.i18n.t("common.delete");
		const message =
			count > 1
				? this.i18n.t(`${item_type}.delete_confirm_multiple`, { count })
				: this.i18n.t(`${item_type}.delete_confirm`);

		return this.confirm({
			cancel_text: "common.cancel",
			confirm_text: "common.delete",
			message,
			title,
			type: "danger",
		});
	}
}
