import { Type } from "class-transformer";
import {
	ArrayMinSize,
	ArrayNotEmpty,
	IsArray,
	IsBoolean,
	IsInt,
	IsOptional,
	IsString,
	MaxLength,
	MinLength,
	ValidateNested,
} from "class-validator";

import { Pagination } from "./utility.contract";

// ============ Bucket Contracts ============

export class CreateBucketContract {
	@IsString()
	@MinLength(3, { message: "Bucket alias must be at least 3 characters" })
	@MaxLength(63, { message: "Bucket alias must be at most 63 characters" })
	public readonly global_alias!: string;

	@IsArray()
	@ValidateNested({ each: true })
	@ArrayMinSize(1, {
		message: "Bucket must have at least one key permission",
	})
	@Type(() => BucketKeyPermissionContract)
	public readonly key_permissions!: BucketKeyPermissionContract[];
}

export class ListBucketContract extends Pagination {}

export class BucketQuotasContract {
	@IsOptional()
	@IsInt()
	public readonly max_size?: number;

	@IsOptional()
	@IsInt()
	public readonly max_objects?: number;
}

export class UpdateBucketContract {
	@IsOptional()
	@IsArray()
	@IsString({ each: true })
	public readonly global_aliases?: string[];

	@IsOptional()
	@IsBoolean()
	public readonly website_access?: boolean;

	@IsOptional()
	@ValidateNested()
	@Type(() => BucketQuotasContract)
	public readonly quotas?: BucketQuotasContract;

	@IsOptional()
	@IsArray()
	@ValidateNested({ each: true })
	@Type(() => BucketKeyPermissionContract)
	public readonly key_permissions?: BucketKeyPermissionContract[];
}

export class DeleteBucketContract {
	@IsArray()
	@ArrayNotEmpty()
	@IsString({ each: true })
	public readonly id!: string[];
}

export class BucketKeyPermissionsContract {
	@IsOptional()
	@IsBoolean()
	public readonly read?: boolean;

	@IsOptional()
	@IsBoolean()
	public readonly write?: boolean;

	@IsOptional()
	@IsBoolean()
	public readonly owner?: boolean;
}

export class BucketKeyPermissionContract {
	@IsString()
	public readonly access_key_id!: string;

	@ValidateNested()
	@Type(() => BucketKeyPermissionsContract)
	public readonly permissions!: BucketKeyPermissionsContract;
}
