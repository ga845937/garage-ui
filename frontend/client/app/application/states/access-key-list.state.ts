import type { AccessKeyListItemAggregate } from "../../domain/aggregates/access-key.aggregate";

import { computed, Injectable, signal } from "@angular/core";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class AccessKeyListState {
	// Domain state signals
	public readonly items = signal<AccessKeyListItemAggregate[]>([]);
	public readonly selected_ids = signal<string[]>([]);
	public readonly total = signal(0);
	public readonly page = signal(1);
	public readonly page_size = signal(20);

	// Computed signals
	public readonly has_selection = computed(
		() => this.selected_ids().length > 0,
	);
	public readonly selection_count = computed(
		() => this.selected_ids().length,
	);
	public readonly all_selected = computed(
		() =>
			this.items().length > 0 &&
			this.selected_ids().length === this.items().length,
	);
}
