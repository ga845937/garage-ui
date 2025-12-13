import type { BucketAggregate } from "../../domain/aggregates/bucket.aggregate";

import { Injectable, signal } from "@angular/core";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class BucketDetailState {
	// Domain state signals
	public readonly selected_item = signal<BucketAggregate | null>(null);
}
