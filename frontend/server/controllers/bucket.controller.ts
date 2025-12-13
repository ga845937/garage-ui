import type {
	BucketKeyPermissionContract,
	CreateBucketContract,
	ListBucketContract,
	UpdateBucketContract,
} from "@shared/contracts";
import type { BucketListItem } from "@shared/entity/bucket.entity";
import type { Context } from "hono";
import type {
	BucketAliasItem,
	BucketKeyPermissionItem,
} from "../generated/bucket";

import { bucket_client, object_client } from "../grpc";

class BucketController {
	public async create(context: Context): Promise<Response> {
		const payload = context.get("parse_payload") as CreateBucketContract;

		const response = await bucket_client.CreateBucket({
			global_alias: payload.global_alias,
		});

		if (!response.data) {
			return context.json({ error: "Failed to create bucket" }, 500);
		}

		// Apply permissions immediately if provided
		if (payload.key_permissions && payload.key_permissions.length > 0) {
			await this.apply_permission_changes(
				response.data.id,
				payload.key_permissions,
			);
		}

		return context.json({ id: response.data.id }, 201);
	}

	public async get(context: Context): Promise<Response> {
		const id = context.req.param("id");

		try {
			const response = await bucket_client.ReadBucket({ id });

			const bucket = response.data;
			if (!bucket) {
				return context.json({ error: "Bucket not found" }, 404);
			}

			return context.json(bucket);
		} catch (error: unknown) {
			// Handle gRPC NOT_FOUND error (code 5)
			if (error instanceof Error && "code" in error && error.code === 5) {
				return context.json({ error: "Bucket not found" }, 404);
			}
			throw error;
		}
	}

	public async list(context: Context): Promise<Response> {
		const payload = context.get("parse_payload") as ListBucketContract;

		const response = await bucket_client.ListBucket({
			pagination: {
				page: payload.page,
				page_size: payload.page_size,
			},
		});

		if (!response.data) {
			return context.json({ rows: [] });
		}

		const rows: BucketListItem[] = response.data.map((b) => ({
			bytes: b.bytes,
			created: b.created,
			global_aliases: b.global_aliases,
			id: b.id,
			local_aliases: b.local_aliases,
			objects: b.objects,
		}));

		return context.json({ rows, total: response.total });
	}

	public async update(context: Context): Promise<Response> {
		const id = context.req.param("id");
		const payload = context.get("parse_payload") as UpdateBucketContract;

		// 1. Get current bucket state for alias diff calculation
		const current_response = await bucket_client.ReadBucket({ id });
		if (!current_response.data) {
			return context.json({ error: "Bucket not found" }, 404);
		}
		const current_aliases = current_response.data.global_aliases ?? [];

		// 2. Calculate alias diff
		const desired_aliases = payload.global_aliases ?? current_aliases;
		const current_set = new Set(current_aliases);

		const add_alias = desired_aliases.reduce<BucketAliasItem[]>(
			(acc, alias) => {
				if (current_set.has(alias)) {
					current_set.delete(alias);
				} else {
					acc.push({
						bucket_id: id,
						global_alias: alias,
					});
				}
				return acc;
			},
			[],
		);

		const remove_alias: BucketAliasItem[] = Array.from(current_set).map(
			(alias) => ({
				bucket_id: id,
				global_alias: alias,
			}),
		);

		const task = [];
		if (add_alias.length > 0) {
			task.push(
				bucket_client.AddBucketAlias({
					items: add_alias,
				}),
			);
		}

		if (remove_alias.length > 0) {
			task.push(
				bucket_client.RemoveBucketAlias({
					items: remove_alias,
				}),
			);
		}

		// 3. Permission changes
		if (payload.key_permissions && payload.key_permissions.length > 0) {
			await this.apply_permission_changes(id, payload.key_permissions);
		}
		
		await Promise.all(task);

		// 4. Update other bucket settings (quotas, website_access)
		const response = await bucket_client.UpdateBucket({
			id,
			quotas: payload.quotas,
			website_access: payload.website_access
				? { value: payload.website_access }
				: undefined,
		});

		const bucket = response.data;
		if (!bucket) {
			return context.json({ error: "Bucket not found" }, 404);
		}

		return context.json(bucket);
	}

	private async apply_permission_changes(
		bucket_id: string,
		permissions: BucketKeyPermissionContract[],
	): Promise<void> {
		interface IPermissionGroup {
			to_allow: BucketKeyPermissionContract[];
			to_deny: BucketKeyPermissionContract[];
		}
		const { to_allow, to_deny } = permissions.reduce<IPermissionGroup>(
			(acc, item) => {
				if (
					item.permissions.read ||
					item.permissions.write ||
					item.permissions.owner
				) {
					acc.to_allow.push(item);
				}

				if (
					!item.permissions.read ||
					!item.permissions.write ||
					!item.permissions.owner
				) {
					acc.to_deny.push(item);
				}
				return acc;
			},
			{ to_allow: [], to_deny: [] } as IPermissionGroup,
		);

		const task = [];

		if (to_allow.length > 0) {
			task.push(
				bucket_client.AllowBucketKey({
					items: this.to_permission_items(bucket_id, to_allow),
				}),
			);
		}

		if (to_deny.length > 0) {
			task.push(
				bucket_client.DenyBucketKey({
					items: this.to_permission_items(bucket_id, to_deny, true),
				}),
			);
		}

		await Promise.all(task);
	}

	public async delete(context: Context): Promise<Response> {
		const payload = context.get("parse_payload") as { id: string[] };

		// Empty buckets first (concurrently)
		await Promise.all(payload.id.map((id) => this.empty_bucket(id)));

		// Delete buckets
		const response = await bucket_client.DeleteBucket({ id: payload.id });

		return context.json({ id: response.id ?? [] });
	}

	private async empty_bucket(bucket_id: string): Promise<void> {
		let is_truncated = true;
		let continuation_token: string | undefined;

		const response = await bucket_client.ReadBucket({ id: bucket_id });

		const bucket = response.data;
		if (!bucket) {
			return;
		}

		const bucket_name = bucket.global_aliases[0];
		
		while (is_truncated) {
			const list_response = await object_client.ListObjects({
				bucket: bucket_name,
				continuation_token,
			});

			if (list_response.data.length > 0) {
				const keys = list_response.data.map((obj) => obj.key);
				await object_client.DeleteObject({
					bucket: bucket_name,
					keys,
				});
			}

			is_truncated = list_response.is_truncated;
			continuation_token = list_response.next_continuation_token;
		}
	}

	private to_permission_items(
		bucket_id: string,
		permissions: BucketKeyPermissionContract[],
		to_deny: boolean = false,
	): BucketKeyPermissionItem[] {
		return permissions.map((item) => ({
			access_key_id: item.access_key_id,
			bucket_id,
			permissions: {
				owner: to_deny
					? !(item.permissions.owner ?? false)
					: (item.permissions.owner ?? false),
				read: to_deny
					? !(item.permissions.read ?? false)
					: (item.permissions.read ?? false),
				write: to_deny
					? !(item.permissions.write ?? false)
					: (item.permissions.write ?? false),
			},
		}));
	}
}

// biome-ignore lint/style/useNamingConvention: singleton
export const bucket_controller: BucketController = new BucketController();
