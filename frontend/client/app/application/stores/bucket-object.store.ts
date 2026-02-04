import type { S3Object } from "@shared/entity/object.entity";
import type { BucketAggregate } from "../../domain/aggregates/bucket.aggregate";

import { Injectable, inject } from "@angular/core";

import { BucketObjectState } from "../states/bucket-object.state";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class BucketObjectStore {
	private readonly state = inject(BucketObjectState);

	// Public read-only signals
	public readonly selected_bucket = this.state.selected_bucket;

	// Object Browser
	public readonly objects = this.state.objects;
	public readonly prefixes = this.state.prefixes;
	public readonly current_prefix = this.state.current_prefix;
	public readonly loading_objects = this.state.loading_objects;
	public readonly upload_progress = this.state.upload_progress;
	public readonly folder_stats = this.state.folder_stats;
	public readonly has_uploads = this.state.has_uploads;

	// Navigation
	public readonly prefix_history = this.state.prefix_history;
	public readonly can_go_back = this.state.can_go_back;
	public readonly can_go_forward = this.state.can_go_forward;

	// ========================================
	// State Mutators
	// ========================================

	public set_selected_bucket(bucket: BucketAggregate | null): void {
		this.state.selected_bucket.set(bucket);
	}

	public set_objects(objects: S3Object[]): void {
		this.state.objects.set(objects);
	}

	public remove_objects(keys: string[]): void {
		this.state.objects.update((objects) =>
			objects.filter((o) => !keys.includes(o.key)),
		);
	}

	public set_prefixes(prefixes: string[]): void {
		this.state.prefixes.set(prefixes);
	}

	public set_current_prefix(prefix: string): void {
		this.state.current_prefix.set(prefix);
	}

	public set_folder_stats(
		stats: Record<string, { count: number; size: number; is_truncated: boolean }>,
	): void {
		this.state.folder_stats.set(stats);
	}

	public set_loading_objects(loading: boolean): void {
		this.state.loading_objects.set(loading);
	}

	public clear_state(): void {
		this.state.selected_bucket.set(null);
		this.state.objects.set([]);
		this.state.prefixes.set([]);
		this.state.current_prefix.set("");
		this.state.folder_stats.set({});
		this.reset_navigation_history();
	}

	// ========================================
	// Upload Progress
	// ========================================

	public update_upload_progress(key: string, progress: number): void {
		this.state.upload_progress.update((curr) => ({
			...curr,
			[key]: progress,
		}));
	}

	public remove_upload_progress(key: string): void {
		this.state.upload_progress.update((curr) => {
			const { [key]: _, ...next } = curr;
			return next;
		});
	}

	public clear_upload_progress(): void {
		this.state.upload_progress.set({});
	}

	// ========================================
	// Navigation History
	// ========================================

	public reset_navigation_history(): void {
		this.state.prefix_history.set([""]);
		this.state.history_index.set(0);
	}

	public push_to_history(prefix: string): void {
		const current_index = this.state.history_index();
		const history = this.state.prefix_history();

		// If navigating from middle of history, truncate forward history
		const new_history = [...history.slice(0, current_index + 1), prefix];
		this.state.prefix_history.set(new_history);
		this.state.history_index.set(new_history.length - 1);
	}

	public navigate_back(): string | null {
		const index = this.state.history_index();
		if (index > 0) {
			const new_index = index - 1;
			this.state.history_index.set(new_index);
			return this.state.prefix_history()[new_index];
		}
		return null;
	}

	public navigate_forward(): string | null {
		const index = this.state.history_index();
		const history = this.state.prefix_history();
		if (index < history.length - 1) {
			const new_index = index + 1;
			this.state.history_index.set(new_index);
			return history[new_index];
		}
		return null;
	}
}
