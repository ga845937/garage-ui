import type { AccessKeyAggregate } from "../../domain/aggregates/access-key.aggregate";

import { Injectable, inject } from "@angular/core";

import { AccessKeyDetailState } from "../states/access-key-detail.state";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class AccessKeyDetailStore {
	private readonly state = inject(AccessKeyDetailState);

	// Public read-only signals
	public readonly selected_item = this.state.selected_item;

	// ========================================
	// State Mutators
	// ========================================

	public set_item(item: AccessKeyAggregate | null): void {
		this.state.selected_item.set(item);
	}

	public clear(): void {
		this.state.selected_item.set(null);
	}
}
