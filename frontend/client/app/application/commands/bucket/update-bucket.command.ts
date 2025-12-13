import type { BucketAggregate } from "../../../domain/aggregates/bucket.aggregate";
import type { UpdateBucketData } from "../../../infrastructure/repositories";

import { Injectable, inject } from "@angular/core";

import { BucketRepository } from "../../../infrastructure/repositories";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class UpdateBucketCommand {
	private readonly repository = inject(BucketRepository);

	public async execute(id: string, data: UpdateBucketData): Promise<BucketAggregate> {
		return await this.repository.update(id, data);
	}
}
