import type { BucketListItemAggregate } from "../../domain/aggregates/bucket.aggregate";

import { Injectable, inject } from "@angular/core";

import { BucketListState } from "../states/bucket-list.state";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class BucketListStore {
	private readonly state = inject(BucketListState);

	// Public read-only signals
	public readonly items = this.state.items;
	public readonly total = this.state.total;
	public readonly page = this.state.page;
	public readonly page_size = this.state.page_size;
	public readonly selected_ids = this.state.selected_ids;
	public readonly has_selection = this.state.has_selection;
	public readonly all_selected = this.state.all_selected;

	// ========================================
	// State Mutators
	// ========================================

	public set_items(items: BucketListItemAggregate[]): void {
		this.state.items.set(items);
	}

	public append_items(items: BucketListItemAggregate[]): void {
		this.state.items.update((curr) => [...curr, ...items]);
	}

	public remove_items(ids: string[]): void {
		this.state.items.update((items) =>
			items.filter((item) => !ids.includes(item.id)),
		);
	}

	public set_total(total: number): void {
		this.state.total.set(total);
	}

	public set_page(page: number): void {
		this.state.page.set(page);
	}

	// ========================================
	// Selection Methods
	// ========================================

	public toggle_select(id: string): void {
		if (this.state.selected_ids().includes(id)) {
			this.state.selected_ids.update((ids) => ids.filter((i) => i !== id));
		} else {
			this.state.selected_ids.update((ids) => [...ids, id]);
		}
	}

	public select_all(): void {
		this.state.selected_ids.set(this.state.items().map((item) => item.id));
	}

	public clear_selection(): void {
		this.state.selected_ids.set([]);
	}

	public clear_selected_ids(ids: string[]): void {
		this.state.selected_ids.update((selected) =>
			selected.filter((id) => !ids.includes(id)),
		);
	}
}
