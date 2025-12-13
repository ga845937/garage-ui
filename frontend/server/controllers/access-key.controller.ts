import type {
	CreateAccessKeyContract,
	DeleteAccessKeysContract,
	ListAccessKeyContract,
	ReadAccessKeyContract,
	UpdateAccessKeyContract,
} from "@shared/contracts";
import type {
	AccessKey,
	AccessKeyListItem,
} from "@shared/entity/access-key.entity";
import type { Context } from "hono";

import { access_key_client, grpc_service } from "../grpc";

function map_key_to_entity(key: {
	id: string;
	name: string;
	permissions?: { create_bucket: boolean };
	buckets?: Array<{
		id: string;
		global_aliases: string[];
		local_aliases: string[];
		permissions?: { read: boolean; write: boolean; owner: boolean };
	}>;
	secret_access_key?: string;
	expiration?: string;
	created?: string;
}): AccessKey {
	return {
		buckets: (key.buckets ?? []).map((b) => ({
			global_aliases: b.global_aliases,
			id: b.id,
			local_aliases: b.local_aliases,
			permissions: {
				owner: b.permissions?.owner ?? false,
				read: b.permissions?.read ?? false,
				write: b.permissions?.write ?? false,
			},
		})),
		created: key.created,
		expiration: key.expiration,
		id: key.id,
		name: key.name,
		permissions: {
			owner: key.permissions?.create_bucket ?? false,
			read: true,
			write: true,
		},
		secret_access_key: key.secret_access_key ?? "",
	};
}

class AccessKeyController {
	public async create(context: Context): Promise<Response> {
		const payload = context.get("parse_payload") as CreateAccessKeyContract;

		const response = await access_key_client.CreateKey({
			allow_create_bucket: payload.allow_create_bucket,
			expiration: payload.expiration,
			name: payload.name,
		});

		const key = response.data;
		if (!key) {
			return context.json({ error: "Failed to create access key" }, 500);
		}

		return context.json(key, 201);
	}

	// GET /api/access-keys/:id
	public async get(context: Context): Promise<Response> {
		const payload = context.get("parse_payload") as ReadAccessKeyContract;

		const response = await access_key_client.ReadKey({ id: payload.id });

		const key = response.data;
		if (!key) {
			return context.json({ error: "Access key not found" }, 404);
		}

		return context.json(key);
	}

	// GET /api/access-keys
	public async list(context: Context): Promise<Response> {
		const payload = context.get("parse_payload") as ListAccessKeyContract;

		const response = await access_key_client.ListKey({
			name: payload.search,
			pagination: {
				page: payload.page,
				page_size: payload.page_size,
			},
		});

		const rows: AccessKeyListItem[] = response.data.map((key) => ({
			created: key.created,
			expiration: key.expiration,
			id: key.id,
			name: key.name,
			secret_access_key: key.secret_access_key,
		}));

		return context.json({ rows, total: response.total });
	}

	// PUT /api/access-keys/:id
	public async update(context: Context): Promise<Response> {
		const id = context.req.param("id");
		const payload = context.get("parse_payload") as UpdateAccessKeyContract;

		const response = await access_key_client.UpdateKey({
			allow_create_bucket: payload.allow_create_bucket,
			expiration: grpc_service.to_null_able(payload.expiration),
			id,
			name: payload.name,
		});

		if (!response.data) {
			return context.json({ error: "Access key not found" }, 404);
		}

		return context.json(map_key_to_entity(response.data));
	}

	// POST /api/access-keys/delete
	public async delete(context: Context): Promise<Response> {
		const payload = context.get(
			"parse_payload",
		) as DeleteAccessKeysContract;

		const response = await access_key_client.DeleteKey({ id: payload.id });

		return context.json({ ids: response.id });
	}
}

// biome-ignore lint/style/useNamingConvention: singleton
export const access_key_controller: AccessKeyController =
	new AccessKeyController();
