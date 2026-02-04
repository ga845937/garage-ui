import {
	ArrayNotEmpty,
	IsArray,
	IsInt,
	IsOptional,
	IsString,
	Min,
	MinLength,
} from "class-validator";

export class ListObjectsRequestContract {
	@IsString()
	@MinLength(1)
	public bucket_name!: string;

	@IsString()
	@MinLength(1)
	@IsOptional()
	public prefix?: string;

	@IsString()
	@MinLength(1)
	@IsOptional()
	public continuation_token?: string;

	@IsInt()
	@Min(1)
	@IsOptional()
	public max_keys?: number;

	@IsString()
	@IsOptional()
	public delimiter?: string;
}

export class DeleteObjectRequestContract {
	@IsString()
	@MinLength(1)
	public bucket_name!: string;

	@IsArray()
	@ArrayNotEmpty()
	@IsString({ each: true })
	public key!: string[];
}

export class UploadObjectRequestContract {
	@IsString()
	@MinLength(1)
	public bucket_name!: string;

	@IsString()
	@MinLength(1)
	public key!: string;
}

// Chunked Upload Contracts
export class UploadInitContract {
	@IsString()
	@MinLength(1)
	public bucket_name!: string;

	@IsString()
	@MinLength(1)
	public key!: string;

	@IsString()
	public content_type!: string;

	@IsInt()
	public content_length!: number;
}