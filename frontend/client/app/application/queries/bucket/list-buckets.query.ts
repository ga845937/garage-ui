import type { ListBucketContract } from "@shared/contracts/bucket.contract";
import type { BucketListItemAggregate } from "../../../domain/aggregates/bucket.aggregate";

import { Injectable, inject } from "@angular/core";

import { BucketRepository } from "../../../infrastructure/repositories";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class ListBucketsQuery {
	private readonly repository = inject(BucketRepository);

	public async execute(
		params: ListBucketContract,
	): Promise<{ rows: BucketListItemAggregate[]; total: number }> {
		return await this.repository.find_all(params);
	}
}
