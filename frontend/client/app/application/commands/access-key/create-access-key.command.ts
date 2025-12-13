import type { CreateAccessKeyContract } from "@shared/contracts";
import type { AccessKeyAggregate } from "../../../domain/aggregates/access-key.aggregate";

import { Injectable, inject } from "@angular/core";

import { AccessKeyRepository } from "../../../infrastructure/repositories/access-key.repository";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class CreateAccessKeyCommand {
	private readonly repository = inject(AccessKeyRepository);

	public async execute(
		data: CreateAccessKeyContract,
	): Promise<AccessKeyAggregate> {
		return await this.repository.create(data);
	}
}
