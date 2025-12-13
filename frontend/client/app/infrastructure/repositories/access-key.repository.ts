import type {
	CreateAccessKeyContract,
	ListAccessKeyContract,
	UpdateAccessKeyContract,
} from "@shared/contracts";
import type { ListResponse } from "@shared/entity/response.entity";

import { Injectable, inject } from "@angular/core";

import {
	AccessKeyAggregate,
	AccessKeyListItemAggregate,
} from "../../domain/aggregates/access-key.aggregate";
import { AccessKeyService } from "../api/access-key.service";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class AccessKeyRepository {
	private readonly api = inject(AccessKeyService);

	public async find_all(
		params: ListAccessKeyContract,
	): Promise<ListResponse<AccessKeyListItemAggregate>> {
		const { page = 1, page_size = 20, search } = params;
		const response = await this.api.list_keys(page, page_size, search);
		return {
			rows: response.rows.map((entity) =>
				AccessKeyListItemAggregate.from(entity),
			),
			total: response.total,
		};
	}

	public async find_by_id(id: string): Promise<AccessKeyAggregate> {
		const entity = await this.api.get_key(id);
		return AccessKeyAggregate.from(entity);
	}

	public async create(
		data: CreateAccessKeyContract,
	): Promise<AccessKeyAggregate> {
		const entity = await this.api.create_key(data);
		return AccessKeyAggregate.from(entity);
	}

	public async update(
		data: UpdateAccessKeyContract,
	): Promise<AccessKeyAggregate> {
		const entity = await this.api.update_key(data);
		return AccessKeyAggregate.from(entity);
	}

	public async delete(id: string[]): Promise<void> {
		await this.api.delete(id);
	}
}
