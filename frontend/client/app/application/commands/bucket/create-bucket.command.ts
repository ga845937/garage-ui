import type { BucketKeyPermissionContract } from "@shared/contracts/bucket.contract";
import type { BucketAggregate } from "../../../domain/aggregates/bucket.aggregate";

import { Injectable, inject } from "@angular/core";

import { BucketRepository } from "../../../infrastructure/repositories";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class CreateBucketCommand {
	private readonly repository = inject(BucketRepository);

	public async execute(
		global_alias: string,
		key_permissions: BucketKeyPermissionContract[],
	): Promise<BucketAggregate> {
		return await this.repository.create(global_alias, key_permissions);
	}
}
