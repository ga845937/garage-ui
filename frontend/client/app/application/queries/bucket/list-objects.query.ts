import type { ObjectList, S3Object } from "@shared/entity/object.entity";

import { Injectable, inject } from "@angular/core";

import { BucketService } from "../../../infrastructure/api/bucket.service";

export interface ListObjectsResult extends ObjectList {
	objects: S3Object[];
	prefixes: string[];
}

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class ListObjectsQuery {
	private readonly service = inject(BucketService);

	public async execute(
		bucket_name: string,
		prefix?: string,
		continuation_token?: string,
		max_keys?: number,
	): Promise<ListObjectsResult> {
		// 使用 delimiter="/" 來獲取虛擬資料夾結構
		const result = await this.service.list_objects(
			bucket_name,
			prefix,
			continuation_token,
			max_keys,
			"/", // delimiter - 啟用虛擬資料夾模式
		);

		// 過濾掉資料夾標記（以 / 結尾的空物件）
		// 當使用 delimiter 時，後端已經只回傳當前層級的檔案
		const objects = result.data.filter((obj) => !obj.key.endsWith("/"));

		// 後端回傳的 common_prefixes 就是虛擬資料夾列表
		const prefixes = result.common_prefixes;

		return {
			...result,
			objects,
			prefixes,
		};
	}
}
