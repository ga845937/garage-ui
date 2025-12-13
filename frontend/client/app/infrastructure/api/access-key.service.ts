// Access Key Service
// 透過 HTTP 抽象層存取 Access Key API

import type {
	CreateAccessKeyContract,
	UpdateAccessKeyContract,
} from "@shared/contracts";
import type {
	AccessKey,
	AccessKeyListItem,
} from "@shared/entity/access-key.entity";
import type { ListResponse } from "@shared/entity/response.entity";
import type { HttpClient } from "../http";

import { Injectable, inject } from "@angular/core";
import { build_path, KeyPath } from "@shared/api-paths";

import { HTTP_CLIENT } from "../http";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class AccessKeyService {
	private readonly http = inject<HttpClient>(HTTP_CLIENT);

	public list_keys(
		page: number,
		page_size: number,
		search?: string,
	): Promise<ListResponse<AccessKeyListItem>> {
		const params = new URLSearchParams({
			page: page.toString(),
			page_size: page_size.toString(),
		});
		if (search) {
			params.set("search", search);
		}

		return this.http.get(KeyPath.BASE, { params });
	}

	public get_key(id: string): Promise<AccessKey> {
		const url = build_path(KeyPath.DETAIL, { id });
		return this.http.get(url);
	}

	public create_key(request: CreateAccessKeyContract): Promise<AccessKey> {
		return this.http.post(KeyPath.BASE, request);
	}

	public update_key(request: UpdateAccessKeyContract): Promise<AccessKey> {
		const url = build_path(KeyPath.DETAIL, { id: request.id });
		return this.http.put(url, request);
	}

	public async delete(id: string[]): Promise<string[]> {
		const result = await this.http.post<{ ids?: string[] }>(
			KeyPath.DELETE,
			{
				id,
			},
		);
		return result?.ids ?? id;
	}
}
