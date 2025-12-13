import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";

@Component({
	imports: [RouterOutlet],
	selector: "app-root",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./app.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./app.component.html",
})
export class App {}
