import type { AccessKeyAggregate } from "../../domain/aggregates/access-key.aggregate";

import { Injectable, signal } from "@angular/core";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class AccessKeyDetailState {
	// Domain state signals
	public readonly selected_item = signal<AccessKeyAggregate | null>(null);
}
