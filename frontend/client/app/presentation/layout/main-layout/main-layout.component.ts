import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";

import { BottomDockComponent } from "../bottom-dock/bottom-dock.component";
import { TopBarComponent } from "../top-bar/top-bar.component";

@Component({
	imports: [
		RouterOutlet,
		TopBarComponent,
		BottomDockComponent,
	],
	selector: "app-main-layout",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./main-layout.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./main-layout.component.html",
})
export class MainLayoutComponent {}
