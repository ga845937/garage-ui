import { Type } from "class-transformer";
import { IsInt, IsOptional, Min } from "class-validator";

export class Pagination {
	@IsOptional()
	@Type(() => Number)
	@IsInt()
	@Min(1)
	public page: number = 1;

	@IsOptional()
	@Type(() => Number)
	@IsInt()
	@Min(1)
	public page_size: number = 20;
}
