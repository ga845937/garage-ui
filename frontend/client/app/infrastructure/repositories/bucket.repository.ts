import type {
	BucketKeyPermissionContract,
	ListBucketContract,
	UpdateBucketContract,
} from "@shared/contracts/bucket.contract";
import type { ObjectList } from "@shared/entity/object.entity";
import type { ListResponse } from "@shared/entity/response.entity";

import { Injectable, inject } from "@angular/core";

import {
	BucketAggregate,
	BucketListItemAggregate,
} from "../../domain/aggregates/bucket.aggregate";
import { BucketService } from "../api/bucket.service";

export interface UpdateBucketData {
	global_aliases?: string[];
	quotas?: {
		max_size?: number | null;
		max_objects?: number | null;
	};
	website_access?: boolean;
	// Permission changes
	key_permissions?: Array<{
		access_key_id: string;
		permissions: { read: boolean; write: boolean; owner: boolean };
	}>;
}

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class BucketRepository {
	private readonly api = inject(BucketService);

	public async find_all(
		params: ListBucketContract,
	): Promise<ListResponse<BucketListItemAggregate>> {
		const response = await this.api.list_buckets(
			params.page,
			params.page_size,
		);
		return {
			rows: response.rows.map((entity) =>
				BucketListItemAggregate.from(entity),
			),
			total: response.total,
		};
	}

	public async find_by_id(id: string): Promise<BucketAggregate> {
		const entity = await this.api.get_bucket(id);
		return BucketAggregate.from(entity);
	}

	public async create(
		global_alias: string,
		key_permissions: BucketKeyPermissionContract[],
	): Promise<BucketAggregate> {
		const entity = await this.api.create_bucket(
			global_alias,
			key_permissions,
		);
		return BucketAggregate.from(entity);
	}

	public async update(
		id: string,
		data: UpdateBucketData,
	): Promise<BucketAggregate> {
		const contract: UpdateBucketContract = {
			global_aliases: data.global_aliases,
			key_permissions: data.key_permissions,
			quotas: data.quotas
				? {
						max_objects: data.quotas.max_objects ?? undefined,
						max_size: data.quotas.max_size ?? undefined,
					}
				: undefined,
			website_access: data.website_access,
		};
		const entity = await this.api.update_bucket(id, contract);
		return BucketAggregate.from(entity);
	}

	public async delete(id: string[]): Promise<void> {
		await this.api.delete_bucket(id);
	}

	public async list_objects(
		bucket_name: string,
		prefix?: string,
	): Promise<ObjectList> {
		return await this.api.list_objects(bucket_name, prefix);
	}
}
