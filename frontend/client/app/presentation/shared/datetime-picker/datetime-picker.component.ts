import { Component, EventEmitter, Input, Output } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { MatButtonModule } from "@angular/material/button";
import { MatFormFieldModule } from "@angular/material/form-field";
import { MatIconModule } from "@angular/material/icon";
import { MatInputModule } from "@angular/material/input";
import { MatTooltipModule } from "@angular/material/tooltip";
import { NgxTolgeeModule } from "@tolgee/ngx";

@Component({
	imports: [
		FormsModule,
		MatButtonModule,
		MatFormFieldModule,
		MatIconModule,
		MatInputModule,
		MatTooltipModule,
		NgxTolgeeModule,
	],
	selector: "app-datetime-picker",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./datetime-picker.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./datetime-picker.component.html",
})
export class DatetimePickerComponent {
	@Input()
	public label = "";

	@Input()
	public value: string | null | undefined = null;

	@Input()
	public hint = "";

	@Input()
	public clearable = true;

	@Output()
	public readonly value_change = new EventEmitter<string | null>();

	public get min_datetime(): string {
		return new Date().toISOString().slice(0, 19);
	}

	public on_input(event: Event): void {
		const input = event.target as HTMLInputElement;
		this.value_change.emit(input.value || null);
	}

	public clear(): void {
		this.value_change.emit(null);
	}

	public show_picker(input: HTMLInputElement): void {
		input.showPicker();
	}
}
