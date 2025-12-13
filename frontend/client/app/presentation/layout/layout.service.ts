import { Injectable, inject, signal } from "@angular/core";
import { MatSnackBar } from "@angular/material/snack-bar";

import { I18nService } from "../../infrastructure/i18n/i18n.service";

export interface ContextAction {
	label_key?: string; // Translation key for label
	label?: string; // Literal label (overrides label_key)
	icon: string;
	action: () => void;
	variant?: "primary" | "danger" | "default";
	disabled?: boolean;
}

import { LayoutState } from "../../application/states/layout.state";

@Injectable({
	// biome-ignore lint/style/useNamingConvention: Angular
	providedIn: "root",
})
export class LayoutService {
	private readonly snackbar = inject(MatSnackBar);
	private readonly i18n = inject(I18nService);
	private readonly state = inject(LayoutState);

	// Global UI State exposed from LayoutState
	public readonly loading = this.state.loading;
	public readonly error = this.state.error;

	// Store translation key for reactive i18n (used by most pages)
	public page_title_key = signal<string>("");
	// Store literal title for dynamic values like bucket names
	public page_title_literal = signal<string>("");
	public context_actions = signal<ContextAction[]>([]);

	public set_title_key(key: string): void {
		this.page_title_key.set(key);
		this.page_title_literal.set(""); // Clear literal when using key
	}

	public set_title_literal(title: string): void {
		this.page_title_literal.set(title);
		this.page_title_key.set(""); // Clear key when using literal
	}

	public set_actions(actions: ContextAction[]): void {
		this.context_actions.set(actions);
	}

	public set_loading(is_loading: boolean): void {
		this.state.loading.set(is_loading);
	}

	public set_error(error: string | null): void {
		this.state.error.set(error);
	}

	public clear_actions(): void {
		this.context_actions.set([]);
	}

	public copy_to_clipboard(text: string): void {
		navigator.clipboard.writeText(text).then(() => {
			this.snackbar.open(
				this.i18n.t("common.copy_success"),
				this.i18n.t("common.confirm"),
				{ duration: 2000 },
			);
		});
	}
}
