import type {
	AccessKey,
	AccessKeyListItem,
} from "@shared/entity/access-key.entity";

import { CommonModule } from "@angular/common";
import { Component, EventEmitter, Input, inject, Output, signal } from "@angular/core";
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
	selector: "app-access-key-card",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./access-key-card.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./access-key-card.component.html",
})
export class AccessKeyCardComponent {
	@Input({ required: true }) public key!: AccessKey | AccessKeyListItem;
	@Input() public selection_mode = false;
	@Input() public selected = false;
	@Output() public card_click = new EventEmitter<void>();
	@Output() public selection_change = new EventEmitter<boolean>();

	private readonly layout = inject(LayoutService);
	public readonly date_format = inject(DateFormatService);

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

	public copy_id(): void {
		this.layout.copy_to_clipboard(this.key.id);
	}

	public is_expired(): boolean {
		if (!this.key.expiration) {
			return false;
		}
		return new Date(this.key.expiration) < new Date();
	}

	// Secret key functionality
	public readonly show_secret = signal(false);

	public toggle_secret(): void {
		this.show_secret.update((v) => !v);
	}

	public copy_secret(): void {
		this.layout.copy_to_clipboard(this.key.secret_access_key);
	}

	public get_secret(): string {
		return this.key.secret_access_key || "";
	}
}
