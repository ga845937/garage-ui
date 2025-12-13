import type { UpdateAccessKeyContract } from "@shared/contracts";
import type {
	AccessKey,
	AccessKeyBucket,
	AccessKeyListItem,
	AccessKeyPermissions,
} from "@shared/entity/access-key.entity";

/**
 * AccessKeyListItemAggregate - Aggregate for List View
 */
export class AccessKeyListItemAggregate {
	private constructor(private readonly entity: AccessKeyListItem) {}

	public static from(entity: AccessKeyListItem): AccessKeyListItemAggregate {
		return new AccessKeyListItemAggregate(entity);
	}

	public get id(): string {
		return this.entity.id;
	}

	public get name(): string {
		return this.entity.name;
	}

	public get created(): string | undefined {
		return this.entity.created;
	}

	public get expiration(): string | undefined {
		return this.entity.expiration;
	}

	public get secret_access_key(): string {
		return this.entity.secret_access_key;
	}

	// Computed properties for list view
	public get display_name(): string {
		return this.name || this.id.substring(0, 8);
	}

	public get is_expired(): boolean {
		if (!this.entity.expiration) {
			return false;
		}
		return new Date(this.entity.expiration) < new Date();
	}
}

/**
 * AccessKeyAggregate - Pure TypeScript Domain Aggregate
 *
 * This class encapsulates business logic for AccessKey without any Angular dependencies.
 * It provides validation rules, computed properties, and conversion methods.
 *
 * Usage:
 * ```typescript
 * const aggregate = AccessKeyAggregate.from(entity);
 * if (aggregate.validate_name("new name").is_valid) {
 *   // proceed
 * }
 * ```
 */
export class AccessKeyAggregate {
	private constructor(private readonly entity: AccessKey) {}

	// ========================================
	// Factory Method
	// ========================================

	public static from(entity: AccessKey): AccessKeyAggregate {
		return new AccessKeyAggregate(entity);
	}

	// ========================================
	// Getters (Read-only access to entity)
	// ========================================

	public get id(): string {
		return this.entity.id;
	}

	public get name(): string {
		return this.entity.name;
	}

	public get secret_access_key(): string {
		return this.entity.secret_access_key;
	}

	public get permissions(): AccessKeyPermissions {
		return this.entity.permissions;
	}

	public get buckets(): AccessKeyBucket[] {
		return this.entity.buckets;
	}

	public get created(): string | undefined {
		return this.entity.created;
	}

	public get expiration(): string | undefined {
		return this.entity.expiration;
	}

	// ========================================
	// Computed Properties
	// ========================================

	public get allow_create_bucket(): boolean {
		return this.permissions.owner || this.permissions.write;
	}

	public get expiration_date(): Date | null {
		return this.expiration ? new Date(this.expiration) : null;
	}

	public get created_date(): Date | null {
		return this.created ? new Date(this.created) : null;
	}

	public get is_expired(): boolean {
		const exp = this.expiration_date;
		return exp !== null && exp < new Date();
	}

	public get expiration_formatted(): string {
		const date = this.expiration_date;
		return date ? date.toISOString().slice(0, 16) : "";
	}

	public get created_formatted(): string {
		const date = this.created_date;
		return date ? date.toISOString().slice(0, 16) : "";
	}

	// ========================================
	// Validation Methods
	// ========================================

	public static validate_name(name: string): ValidationResult {
		if (!name || name.trim().length === 0) {
			return { error: "Name is required", is_valid: false };
		}
		if (name.length > 100) {
			return {
				error: "Name must be at most 100 characters",
				is_valid: false,
			};
		}
		return { is_valid: true };
	}

	public static validate_expiration(
		expiration: string | undefined,
	): ValidationResult {
		if (!expiration) {
			return { is_valid: true }; // Optional field
		}
		const date = new Date(expiration);
		if (Number.isNaN(date.getTime())) {
			return { error: "Invalid date format", is_valid: false };
		}
		if (date < new Date()) {
			return {
				error: "Expiration date must be in the future",
				is_valid: false,
			};
		}
		return { is_valid: true };
	}

	// ========================================
	// Conversion Methods
	// ========================================

	public to_update_contract(changes: {
		name?: string;
		expiration?: string;
		allow_create_bucket?: boolean;
	}): UpdateAccessKeyContract {
		return {
			allow_create_bucket: changes.allow_create_bucket,
			expiration: changes.expiration,
			id: this.id,
			name: changes.name,
		};
	}

	public to_entity(): AccessKey {
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
