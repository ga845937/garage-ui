import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatIconModule } from "@angular/material/icon";
import { MatTooltipModule } from "@angular/material/tooltip";
import { RouterLink, RouterLinkActive } from "@angular/router";
import { BucketRoute, KeyRoute, SettingsRoute } from "@shared/route-paths";
import { NgxTolgeeModule } from "@tolgee/ngx";

@Component({
	imports: [
		CommonModule,
		MatButtonModule,
		MatIconModule,
		MatTooltipModule,
		RouterLink,
		RouterLinkActive,
		NgxTolgeeModule,
	],
	selector: "app-bottom-dock",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./bottom-dock.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./bottom-dock.component.html",
})
export class BottomDockComponent {
	public readonly key_route = KeyRoute;
	public readonly bucket_route = BucketRoute;
	public readonly settings_route = SettingsRoute;
}
