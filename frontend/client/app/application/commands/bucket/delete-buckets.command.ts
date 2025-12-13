import { Injectable, inject } from "@angular/core";

import { BucketRepository } from "../../../infrastructure/repositories";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class DeleteBucketsCommand {
	private readonly repository = inject(BucketRepository);

	public async execute(ids: string[]): Promise<void> {
		await this.repository.delete(ids);
	}
}
