import type { UpdateAccessKeyContract } from "@shared/contracts";
import type { AccessKeyAggregate } from "../../../domain/aggregates/access-key.aggregate";

import { Injectable, inject } from "@angular/core";

import { AccessKeyRepository } from "../../../infrastructure/repositories/access-key.repository";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class UpdateAccessKeyCommand {
	private readonly repository = inject(AccessKeyRepository);

	public async execute(
		contract: UpdateAccessKeyContract,
	): Promise<AccessKeyAggregate> {
		return await this.repository.update(contract);
	}
}
