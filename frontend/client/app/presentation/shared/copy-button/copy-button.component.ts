import { Component, EventEmitter, Input, Output } from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatIconModule } from "@angular/material/icon";
import { MatTooltipModule } from "@angular/material/tooltip";
import { NgxTolgeeModule } from "@tolgee/ngx";

@Component({
	imports: [MatButtonModule, MatIconModule, MatTooltipModule, NgxTolgeeModule],
	selector: "app-copy-button",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./copy-button.component.scss",
	template: `
		<button
			mat-icon-button
			class="copy-btn"
			(click)="on_click($event)"
			[matTooltip]="tooltip"
		>
			<mat-icon>content_copy</mat-icon>
		</button>
	`,
})
export class CopyButtonComponent {
	@Input() public tooltip = "";
	@Output() public copy_click = new EventEmitter<void>();

	public on_click(event: Event): void {
		event.stopPropagation();
		this.copy_click.emit();
	}
}
