import { IsEnum, IsNotEmpty, IsOptional, IsString } from "class-validator";

export class GetThumbnailContract {
	@IsString()
	@IsNotEmpty()
	public readonly bucket_name!: string;

	@IsString()
	@IsNotEmpty()
	public readonly key!: string;

	@IsString()
	@IsNotEmpty()
	public readonly etag!: string;

	@IsEnum(["grid", "list"])
	@IsOptional()
	public readonly size?: "grid" | "list";
}
