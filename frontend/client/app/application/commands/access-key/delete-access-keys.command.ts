import { Injectable, inject } from "@angular/core";

import { AccessKeyRepository } from "../../../infrastructure/repositories/access-key.repository";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class DeleteAccessKeysCommand {
	private readonly repository = inject(AccessKeyRepository);

	public async execute(ids: string[]): Promise<void> {
		return await this.repository.delete(ids);
	}
}
