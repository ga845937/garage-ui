import { Injectable, signal } from "@angular/core";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class LayoutState {
	// Global page loading state
	public readonly loading = signal(false);
	// Global page error state
	public readonly error = signal<string | null>(null);
}
