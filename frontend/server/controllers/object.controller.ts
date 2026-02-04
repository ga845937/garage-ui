import type {
	DeleteObjectRequestContract,
	ListObjectsRequestContract,
} from "@shared/contracts/object.contract";
import type { GetThumbnailContract } from "@shared/contracts/thumbnail.contract";
import type { ObjectList } from "@shared/entity/object.entity";
import type { Context } from "hono";

import { object_client } from "../grpc";
import { thumbnail_service } from "../services/thumbnail.service";
import { stream_object } from "../utils/download.util";
import { download_and_generate_thumbnails } from "../utils/thumbnail.util";

class ObjectController {
	public async download(context: Context): Promise<Response> {
		try {
			const bucket_name = context.req.param("bucket_name");
			const key = context.req.param("key");

			console.log(
				`[Object] Streaming download: ${key} from bucket ${bucket_name}`,
			);

			// 使用 streaming 下載，等待 metadata
			const { stream, metadata } = await stream_object(bucket_name, key);

			// 設置響應標頭
			const headers: Record<string, string> = {};

			if (metadata?.content_type) {
				headers["Content-Type"] = metadata.content_type;
			}

			if (metadata?.content_length) {
				headers["Content-Length"] = metadata.content_length.toString();
			}

			// 設置下載檔名
			headers["Content-Disposition"] =
				`attachment; filename="${encodeURIComponent(key)}"`;

			// 返回 streaming response
			return new Response(stream, { headers });
		} catch (error) {
			console.error("[Object] Download error:", error);
			return context.json({ error: "下載失敗" }, 500);
		}
	}

	public async list(context: Context): Promise<Response> {
		const payload = context.get(
			"parse_payload",
		) as ListObjectsRequestContract;

		const response = await object_client.ListObjects({
			bucket: payload.bucket_name,
			continuation_token: payload.continuation_token,
			delimiter: payload.delimiter,
			max_keys: payload.max_keys,
			prefix: payload.prefix,
		});

		const data: ObjectList = {
			common_prefixes: response.common_prefixes,
			data: response.data,
			folder_stats: response.folder_stats,
			is_truncated: response.is_truncated,
			next_continuation_token: response.next_continuation_token,
		};

		return context.json(data);
	}

	/**
	 * 獲取縮圖（優先快取，降級生成）
	 */
	public async get_thumbnail(context: Context): Promise<Response> {
		try {
			const payload = context.get(
				"parse_payload",
			) as GetThumbnailContract;
			const { bucket_name, key, etag, size = "grid" } = payload;

			if (!thumbnail_service.is_image(key)) {
				return context.json({ error: "不是圖片檔案" }, 400);
			}

			// 1. 優先從快取讀取
			let thumbnail = await thumbnail_service.get_cached_thumbnail(
				etag,
				size,
			);

			if (thumbnail) {
				console.log(`[Thumbnail] ✓ Cache hit: ${etag} (${size})`);

				// 刷新 TTL 至 30 天
				await thumbnail_service.refresh_ttl(etag, size);

				return new Response(thumbnail, {
					headers: {
						"Cache-Control": "public, max-age=2592000", // 30 days
						"Content-Type": "image/jpeg",
					},
				});
			}

			// 2. 快取未命中，使用共用函數下載並生成
			console.log(
				`[Thumbnail] ✗ Cache miss, generating: ${etag} (${size})`,
			);

			const thumbnails = await download_and_generate_thumbnails(
				bucket_name,
				key,
				etag,
				[size],
			);

			thumbnail = thumbnails[size];

			return new Response(thumbnail, {
				headers: {
					"Cache-Control": "public, max-age=2592000",
					"Content-Type": "image/jpeg",
				},
			});
		} catch (error) {
			console.error("[Thumbnail] 錯誤:", error);
			return context.json({ error: "縮圖生成失敗" }, 500);
		}
	}

	public async delete(context: Context): Promise<Response> {
		const payload = context.get(
			"parse_payload",
		) as DeleteObjectRequestContract;

		await object_client.DeleteObject({
			bucket: payload.bucket_name,
			keys: payload.key,
		});

		return context.json({});
	}
}

// biome-ignore lint/style/useNamingConvention: singleton
export const object_controller: ObjectController = new ObjectController();
