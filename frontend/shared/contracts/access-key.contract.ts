import {
	ArrayNotEmpty,
	IsArray,
	IsBoolean,
	IsDateString,
	IsOptional,
	IsString,
	MaxLength,
	MinLength,
} from "class-validator";

import { Pagination } from "./utility.contract";

// ============ Access Key Contracts ============

export class CreateAccessKeyContract {
	@IsString()
	@MinLength(1, { message: "Name is required" })
	@MaxLength(100, { message: "Name must be at most 100 characters" })
	public readonly name!: string;

	@IsOptional()
	@IsDateString()
	public readonly expiration?: string;

	@IsOptional()
	@IsBoolean()
	public readonly allow_create_bucket?: boolean;
}

export class ReadAccessKeyContract {
	@IsString()
	public readonly id!: string;
}

export class ListAccessKeyContract extends Pagination {
	@IsString()
	@IsOptional()
	public readonly search?: string;
}

export class UpdateAccessKeyContract {
	@IsString()
	public readonly id!: string;

	@IsOptional()
	@IsString()
	@MinLength(1)
	@MaxLength(100)
	public readonly name?: string;

	@IsOptional()
	@IsDateString()
	public readonly expiration?: string;

	@IsOptional()
	@IsBoolean()
	public readonly allow_create_bucket?: boolean;
}

export class DeleteAccessKeysContract {
	@IsArray()
	@ArrayNotEmpty()
	@IsString({ each: true })
	public readonly id!: string[];
}
