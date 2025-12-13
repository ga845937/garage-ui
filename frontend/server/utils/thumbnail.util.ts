import { thumbnail_service } from "../services/thumbnail.service";
import { download_object } from "./download.util";

/**
 * 從 gRPC 下載物件並生成縮圖
 *
 * @param bucket - Bucket name
 * @param key - Object key
 * @param etag - Object ETag
 * @param sizes - Thumbnail sizes to generate (default: both "grid" and "list")
 * @returns Generated thumbnails keyed by size
 */
export async function download_and_generate_thumbnails(
	bucket: string,
	key: string,
	etag: string,
	sizes: ("grid" | "list")[] = ["grid", "list"],
): Promise<Record<string, Buffer>> {
	console.log(`[Thumbnail] Downloading ${key} to generate thumbnails...`);

	const { buffer: image_buffer } = await download_object(bucket, key);

	const thumbnails: Record<string, Buffer> = {};

	// Generate thumbnails for each requested size
	for (const size of sizes) {
		const thumbnail = await thumbnail_service.generate_thumbnail(
			image_buffer,
			size,
		);
		thumbnails[size] = thumbnail;

		// Cache the generated thumbnail
		await thumbnail_service.cache_thumbnail(etag, size, thumbnail);
		console.log(
			`[Thumbnail] ✓ Generated and cached ${size} thumbnail for ${key}`,
		);
	}

	return thumbnails;
}
