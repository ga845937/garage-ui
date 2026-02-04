import type { S3Object } from "@shared/entity/object.entity";
import type { BucketAggregate } from "../../domain/aggregates/bucket.aggregate";

import { computed, Injectable, signal } from "@angular/core";

export interface UploadProgress {
	filename: string;
	progress: number;
	status: "pending" | "uploading" | "done" | "error";
}

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class BucketObjectState {
	// Bucket Detail
	public readonly selected_bucket = signal<BucketAggregate | null>(null);

	// Object Browser
	public readonly objects = signal<S3Object[]>([]);
	public readonly prefixes = signal<string[]>([]);
	public readonly current_prefix = signal("");
	public readonly loading_objects = signal(false);
	public readonly upload_progress = signal<Record<string, number>>({});
	public readonly folder_stats = signal<
		Record<string, { count: number; size: number; is_truncated: boolean }>
	>({});

	// Navigation history
	public readonly prefix_history = signal<string[]>([""]);
	public readonly history_index = signal(0);

	// Computed signals
	public readonly has_uploads = computed(
		() => Object.keys(this.upload_progress()).length > 0,
	);
	public readonly can_go_back = computed(() => this.history_index() > 0);
	public readonly can_go_forward = computed(
		() => this.history_index() < this.prefix_history().length - 1,
	);
}
