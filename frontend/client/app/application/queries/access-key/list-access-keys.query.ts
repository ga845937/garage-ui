import type { ListAccessKeyContract } from "@shared/contracts/access-key.contract";
import type { ListResponse } from "@shared/entity/response.entity";
import type { AccessKeyListItemAggregate } from "../../../domain/aggregates/access-key.aggregate";

import { Injectable, inject } from "@angular/core";

import { AccessKeyRepository } from "../../../infrastructure/repositories/access-key.repository";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class ListAccessKeysQuery {
	private readonly repository = inject(AccessKeyRepository);

	public async execute(
		params: ListAccessKeyContract,
	): Promise<ListResponse<AccessKeyListItemAggregate>> {
		return await this.repository.find_all(params);
	}
}
