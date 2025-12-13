import type { BucketAggregate } from "../../../domain/aggregates/bucket.aggregate";

import { Injectable, inject } from "@angular/core";

import { BucketRepository } from "../../../infrastructure/repositories";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class GetBucketQuery {
	private readonly repository = inject(BucketRepository);

	public async execute(id: string): Promise<BucketAggregate> {
		return await this.repository.find_by_id(id);
	}
}
