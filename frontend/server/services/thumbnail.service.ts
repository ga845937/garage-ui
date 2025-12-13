import type { ThumbnailSize } from "@shared/constants/thumbnail.constants";

import { THUMBNAIL_SIZES } from "@shared/constants/thumbnail.constants";
import { is_image } from "@shared/utility/thumbnail.utility";
import sharp from "sharp";

import { redis } from "../redis/redis";

const CACHE_TTL: number = 60 * 60 * 24 * 30; // 30 天

export class ThumbnailService {
	/**
	 * 生成 Redis key（只使用 ETag）
	 */
	private generate_cache_key(
		etag: string,
		size: ThumbnailSize,
	): string {
		return `thumbnail:${etag}:${size}`;
	}

	public is_image(filename: string): boolean {
		return is_image(filename);
	}

	/**
	 * 刷新快取的 TTL 至 30 天
	 */
	public async refresh_ttl(etag: string, size: ThumbnailSize): Promise<void> {
		const key = this.generate_cache_key(etag, size);
		try {
			await redis.expire(key, CACHE_TTL);
		} catch (error) {
			console.error("[Thumbnail] TTL refresh failed:", error);
		}
	}

	public async has_cached_thumbnail(
		etag: string,
		size: ThumbnailSize,
	): Promise<boolean> {
		const key = this.generate_cache_key(etag, size);
		const exists = await redis.exists(key);
		return exists === 1;
	}

	public async get_cached_thumbnail(
		etag: string,
		size: ThumbnailSize,
	): Promise<Buffer | null> {
		const key = this.generate_cache_key(etag, size);

		try {
			const buffer = await redis.get_buffer(key);
			return buffer;
		} catch (error) {
			console.error("[Thumbnail] Redis 讀取失敗:", error);
			return null;
		}
	}

	public async cache_thumbnail(
		etag: string,
		size: ThumbnailSize,
		thumbnail: Buffer,
	): Promise<void> {
		const key = this.generate_cache_key(etag, size);

		try {
			const success = await redis.set_buffer_base64(key, thumbnail.toString("base64"), CACHE_TTL);
			if (success) {
				console.log(`[Thumbnail] ✓ 快取已存儲: ${key}`);
			}
		} catch (error) {
			console.error("[Thumbnail] Redis 寫入失敗:", error);
		}
	}

	public async generate_thumbnail(
		image_buffer: Buffer,
		size: ThumbnailSize,
	): Promise<Buffer> {
		const { width, height } = THUMBNAIL_SIZES[size];

		try {
			const thumbnail = await sharp(image_buffer)
				.resize(width, height, {
					fit: "cover",
					position: "center",
				})
				.jpeg({ quality: 80 })
				.toBuffer();

			return thumbnail;
		} catch (error) {
			console.error("[Thumbnail] 生成失敗:", error);
			throw new Error("縮圖生成失敗");
		}
	}

}

// biome-ignore lint/style/useNamingConvention: singleton
export const thumbnail_service: ThumbnailService = new ThumbnailService();
