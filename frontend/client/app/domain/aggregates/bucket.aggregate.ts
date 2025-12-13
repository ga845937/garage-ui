import type { UpdateBucketContract } from "@shared/contracts/bucket.contract";
import type {
	Bucket,
	BucketKey,
	BucketListItem,
	LocalAlias,
	Quotas,
} from "@shared/entity/bucket.entity";

// S3 bucket naming rules: lowercase, numbers, hyphens
const GLOBAL_ALIAS_PATTERN = /^[a-z0-9][a-z0-9-]*[a-z0-9]$/;

/**
 * BucketListItemAggregate - Aggregate for List View
 */
export class BucketListItemAggregate {
	private constructor(private readonly entity: BucketListItem) {}

	public static from(entity: BucketListItem): BucketListItemAggregate {
		return new BucketListItemAggregate(entity);
	}

	public get id(): string {
		return this.entity.id;
	}

	public get global_aliases(): string[] {
		return this.entity.global_aliases;
	}

	public get local_aliases(): LocalAlias[] {
		return this.entity.local_aliases;
	}

	public get objects(): number {
		return this.entity.objects;
	}

	public get bytes(): number {
		return this.entity.bytes;
	}

	public get created(): string {
		return this.entity.created;
	}

	// Computed properties for list view
	public get display_name(): string {
		return this.global_aliases[0] || this.id.substring(0, 8);
	}

	public get formatted_size(): string {
		const units = ["B", "KB", "MB", "GB", "TB"];
		let size = this.bytes;
		let unit_index = 0;
		while (size >= 1024 && unit_index < units.length - 1) {
			size /= 1024;
			unit_index++;
		}
		return `${size.toFixed(unit_index === 0 ? 0 : 2)} ${units[unit_index]}`;
	}

	public get created_date(): Date | null {
		return this.created ? new Date(this.created) : null;
	}

	public get formatted_date(): string {
		const date = this.created_date;
		return date ? date.toISOString().slice(0, 16) : "";
	}
}

/**
 * BucketAggregate - Pure TypeScript Domain Aggregate
 *
 * This class encapsulates business logic for Bucket without any Angular dependencies.
 * It provides validation rules, computed properties, and conversion methods.
 *
 * Usage:
 * ```typescript
 * const aggregate = BucketAggregate.from(entity);
 * if (aggregate.validate_global_alias("new-alias").is_valid) {
 *   // proceed
 * }
 * ```
 */
export class BucketAggregate {
	private constructor(private readonly entity: Bucket) {}

	// ========================================
	// Factory Method
	// ========================================

	public static from(entity: Bucket): BucketAggregate {
		return new BucketAggregate(entity);
	}

	// ========================================
	// Getters (Read-only access to entity)
	// ========================================

	public get id(): string {
		return this.entity.id;
	}

	public get global_aliases(): string[] {
		return this.entity.global_aliases;
	}

	public get local_aliases(): LocalAlias[] {
		return this.entity.local_aliases;
	}

	public get objects(): number {
		return this.entity.objects;
	}

	public get bytes(): number {
		return this.entity.bytes;
	}

	public get created(): string {
		return this.entity.created;
	}

	public get website_access(): boolean {
		return this.entity.website_access;
	}

	public get website_config(): string | undefined {
		return this.entity.website_config;
	}

	public get keys(): BucketKey[] {
		return this.entity.keys;
	}

	public get quotas(): Quotas {
		return this.entity.quotas;
	}

	// ========================================
	// Computed Properties
	// ========================================

	public get display_name(): string {
		return this.global_aliases[0] || this.id.substring(0, 8);
	}

	public get formatted_size(): string {
		const units = ["B", "KB", "MB", "GB", "TB"];
		let size = this.bytes;
		let unit_index = 0;
		while (size >= 1024 && unit_index < units.length - 1) {
			size /= 1024;
			unit_index++;
		}
		return `${size.toFixed(unit_index === 0 ? 0 : 2)} ${units[unit_index]}`;
	}

	public get created_date(): Date | null {
		return this.created ? new Date(this.created) : null;
	}

	public get formatted_date(): string {
		const date = this.created_date;
		return date ? date.toISOString().slice(0, 16) : "";
	}

	public get allow_website(): boolean {
		return this.website_access;
	}

	public get has_quotas(): boolean {
		return (
			this.quotas.max_size !== undefined ||
			this.quotas.max_objects !== undefined
		);
	}

	public get key_count(): number {
		return this.keys.length;
	}

	// ========================================
	// Validation Methods
	// ========================================

	public static validate_global_alias(alias: string): ValidationResult {
		if (!alias || alias.trim().length === 0) {
			return { error: "Global alias is required", is_valid: false };
		}
		if (alias.length < 3) {
			return {
				error: "Global alias must be at least 3 characters",
				is_valid: false,
			};
		}
		if (alias.length > 63) {
			return {
				error: "Global alias must be at most 63 characters",
				is_valid: false,
			};
		}
		// S3 bucket naming rules: lowercase, numbers, hyphens
		if (!GLOBAL_ALIAS_PATTERN.test(alias) && alias.length > 2) {
			return {
				error: "Global alias must contain only lowercase letters, numbers, and hyphens",
				is_valid: false,
			};
		}
		return { is_valid: true };
	}

	// ========================================
	// Conversion Methods
	// ========================================

	public to_update_contract(changes: {
		global_aliases?: string[];
		quotas?: { max_size?: number | null; max_objects?: number | null };
		website_access?: boolean;
		key_permissions?: Array<{
			access_key_id: string;
			permissions: { read: boolean; write: boolean; owner: boolean };
		}>;
	}): UpdateBucketContract {
		return {
			global_aliases: changes.global_aliases,
			key_permissions: changes.key_permissions,
			quotas: changes.quotas
				? {
						max_objects: changes.quotas.max_objects ?? undefined,
						max_size: changes.quotas.max_size ?? undefined,
					}
				: undefined,
			website_access: changes.website_access,
		};
	}

	public to_entity(): Bucket {
		return { ...this.entity };
	}
}

// ========================================
// Supporting Types
// ========================================

export interface ValidationResult {
	is_valid: boolean;
	error?: string;
}
