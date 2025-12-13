import type { AccessKeyAggregate } from "../../../domain/aggregates/access-key.aggregate";

import { Injectable, inject } from "@angular/core";

import { AccessKeyRepository } from "../../../infrastructure/repositories/access-key.repository";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class GetAccessKeyQuery {
	private readonly repository = inject(AccessKeyRepository);

	public async execute(id: string): Promise<AccessKeyAggregate> {
		return await this.repository.find_by_id(id);
	}
}
