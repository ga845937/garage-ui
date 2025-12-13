import type { ObjectList, S3Object } from "@shared/entity/object.entity";

import { Injectable, inject } from "@angular/core";

import { BucketService } from "../../../infrastructure/api/bucket.service";

export interface ListObjectsResult extends ObjectList {
	objects: S3Object[];
	prefixes: string[];
	folder_stats: Record<string, { count: number; size: number }>;
}

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class ListObjectsQuery {
	private readonly service = inject(BucketService);

	public async execute(
		bucket_name: string,
		prefix?: string,
		continuation_token?: string,
		max_keys?: number,
	): Promise<ListObjectsResult> {
		const result = await this.service.list_objects(
			bucket_name,
			prefix,
			continuation_token,
			max_keys,
		);

		// Extract objects that are files (not prefixes) and are direct children
		const current_prefix = prefix ?? "";
		const objects = result.data.filter((obj) => {
			// Skip folder markers
			if (obj.key.endsWith("/")) {
				return false;
			}
			// Only show files that are direct children of current prefix
			const relative_key = obj.key.substring(current_prefix.length);
			return !relative_key.includes("/");
		});

		// Extract unique prefixes (folders) from object keys
		const prefixes_set = new Set<string>();
		// Calculate stats for each folder
		const folder_stats: Record<string, { count: number; size: number }> =
			{};

		for (const obj of result.data) {
			const relative_key = obj.key.substring(current_prefix.length);
			const slash_index = relative_key.indexOf("/");

			if (slash_index > 0) {
				const folder_key =
					current_prefix + relative_key.substring(0, slash_index + 1);
				prefixes_set.add(folder_key);

				if (!folder_stats[folder_key]) {
					folder_stats[folder_key] = { count: 0, size: 0 };
				}
				// Only count actual files (not folder markers themselves if they have size 0)
				if (!obj.key.endsWith("/")) {
					folder_stats[folder_key].count++;
					folder_stats[folder_key].size += obj.size || 0;
				}
			}
		}

		return {
			...result,
			folder_stats,
			objects,
			prefixes: Array.from(prefixes_set),
		};
	}
}
