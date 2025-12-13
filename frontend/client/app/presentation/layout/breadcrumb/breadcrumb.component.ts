import { CommonModule } from "@angular/common";
import { Component, EventEmitter, Input, Output } from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatIconModule } from "@angular/material/icon";
import { MatTooltipModule } from "@angular/material/tooltip";
import { NgxTolgeeModule } from "@tolgee/ngx";

@Component({
	imports: [
		CommonModule,
		MatButtonModule,
		MatIconModule,
		MatTooltipModule,
		NgxTolgeeModule,
	],
	selector: "app-breadcrumb",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./breadcrumb.component.scss",
    // biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./breadcrumb.component.html",
})
export class BreadcrumbComponent {
	@Input() public prefix = "";
	@Input() public bucket_name = "";
	@Input() public can_go_back = false;
	@Input() public can_go_forward = false;
	@Output() public navigate_to = new EventEmitter<string>();
	@Output() public go_back = new EventEmitter<void>();
	@Output() public go_forward = new EventEmitter<void>();

	public get path_parts(): string[] {
		if (!this.prefix) {
			return [];
		}
		return this.prefix.split("/").filter((p) => p);
	}

	public on_part_click(index: number): void {
		const path = `${this.path_parts.slice(0, index + 1).join("/")}/`;
		this.navigate_to.emit(path);
	}
}
