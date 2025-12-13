import type { ValidationError } from "class-validator";
import type { Context, Next } from "hono";

import { plainToInstance } from "class-transformer";
import { validate } from "class-validator";

type ClassConstructor<T> = new () => T;

declare module "hono" {
	interface ContextVariableMap {
		parse_payload: unknown;
	}
}

interface IValidationError {
	details: {
		constraints?: Record<string, string>;
		property: string;
	}[];
	error: string;
}

function format_errors(errors: ValidationError[]): IValidationError {
	return {
		details: errors.map((error) => ({
			constraints: error.constraints,
			property: error.property,
		})),
		error: "Validation failed",
	};
}

/**
 * 統一驗證 middleware
 * 自動合併 params + query + body/form 到 parse_payload
 *
 * 合併順序（後者覆蓋前者）：
 * 1. URL params (/api/users/:id)
 * 2. Query string (?page=1&size=20)
 * 3. Request body (JSON) 或 form data (multipart)
 *
 * @example
 * routes.get("/:id", validate_payload(GetUserDto), (c) => {
 *   const payload = context.get("parse_payload") as GetUserDto;
 *   // payload.id 來自 params
 *   // payload.include 來自 query
 * });
 *
 * routes.post("/", validate_payload(CreateUserDto), (c) => {
 *   const payload = context.get("parse_payload") as CreateUserDto;
 *   // payload.name, payload.email 來自 body
 * });
 */
export function validate_payload<T extends object>(
	dto_class: ClassConstructor<T>,
) {
	// biome-ignore lint/suspicious/noConfusingVoidType: Hono middleware can return void or Response
	return async (context: Context, next: Next): Promise<Response | void> => {
		// 1. 取得 URL params
		const params = context.req.param() as Record<string, string>;

		// 2. 取得 query string
		const query = context.req.query();

		// 3. 取得 body (JSON 或 form data)
		let body: Record<string, unknown> = {};
		const content_type = context.req.header("content-type") || "";

		if (content_type.includes("application/json")) {
			try {
				body = await context.req.json();
			} catch {
				// 沒有 body 或解析失敗
			}
		} else if (
			content_type.includes("multipart/form-data") ||
			content_type.includes("application/x-www-form-urlencoded")
		) {
			try {
				const form_data = await context.req.parseBody();
				body = form_data as Record<string, unknown>;
			} catch {
				// 沒有 form data
			}
		}

		// 4. 取得 headers
		// const headers = context.req.header();

		// 合併所有來源：params < query < body < headers
		const merged_data = { ...params, ...query, ...body };

		// 轉換並驗證（只保留 DTO 中用 @Expose() 標註的欄位）
		const dto_instance = plainToInstance(dto_class, merged_data);
		const errors = await validate(dto_instance);

		if (errors.length > 0) {
			return context.json(format_errors(errors), 400);
		}

		context.set("parse_payload", dto_instance);
		return await next();
	};
}
