import { Injectable, inject } from "@angular/core";

import { BucketService } from "../../../infrastructure/api/bucket.service";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class DeleteObjectCommand {
	private readonly service = inject(BucketService);

	public async execute(bucket_name: string, key: string[]): Promise<void> {
		await this.service.delete_object(bucket_name, key);
	}
}
