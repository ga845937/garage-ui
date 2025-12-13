import type { BucketAggregate } from "../../domain/aggregates/bucket.aggregate";

import { Injectable, inject } from "@angular/core";

import { BucketDetailState } from "../states/bucket-detail.state";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class BucketDetailStore {
	private readonly state = inject(BucketDetailState);

	// Public read-only signals
	public readonly selected_item = this.state.selected_item;

	// ========================================
	// State Mutators
	// ========================================

	public set_item(item: BucketAggregate | null): void {
		this.state.selected_item.set(item);
	}

	public clear(): void {
		this.state.selected_item.set(null);
	}
}
