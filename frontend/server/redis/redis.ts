import type { RedisClientType } from "redis";

import { createClient } from "redis";

import { config } from "../config";

class RedisProvider {
	public readonly client: RedisClientType;

	public constructor() {
		this.client = createClient({
			url: config.redis_uri,
		});

		this.client.on("error", (err: Error) =>
			console.error("[Redis] 錯誤:", err),
		);

		this.client.on("connect", () => {
			console.log(`[Redis: ${config.redis_uri}] 已連接`);
		});

		if (!this.client.isOpen) {
			this.client.connect();
		}
	}

	/**
	 * 獲取分布式鎖
	 */
	public async acquire_lock(key: string, ttl: number = 30): Promise<boolean> {
		try {
			const result = await this.client.set(key, "1", {
				// biome-ignore lint/style/useNamingConvention: redis config
				EX: ttl,
				// biome-ignore lint/style/useNamingConvention: redis config
				NX: true,
			});
			return result === "OK";
		} catch (error) {
			console.error(`[Redis Lock] 獲取鎖失敗 ${key}:`, error);
			return false;
		}
	}

	/**
	 * 釋放鎖
	 */
	public async release_lock(key: string): Promise<void> {
		try {
			await this.client.del(key);
		} catch (error) {
			console.error(`[Redis Lock] 釋放鎖失敗 ${key}:`, error);
		}
	}

	/**
	 * 批次處理工具
	 */
	public async *process_batch<T>(
		items: T[],
		batch_size: number,
		processor: (item: T) => Promise<void>,
	): AsyncGenerator<number, void, unknown> {
		for (let i = 0; i < items.length; i += batch_size) {
			const batch = items.slice(i, i + batch_size);
			await Promise.all(batch.map(processor));
			yield i + batch.length;
		}
	}

	/**
	 * 檢查 key 是否存在
	 */
	public async exists(key: string): Promise<number> {
		try {
			return await this.client.exists(key);
		} catch (error) {
			console.error(`[Redis] exists 失敗 ${key}:`, error);
			return 0;
		}
	}

	/**
	 * 獲取 Buffer 數據
	 */
	public async get_buffer(key: string): Promise<Buffer | null> {
		try {
			const value = await this.client.get(key);
			if (!value) {
				return null;
			}

			return Buffer.from(value, "base64");
		} catch (error) {
			console.error(`[Redis] get_buffer 失敗 ${key}:`, error);
			return null;
		}
	}

	/**
	 * 設置 key 的 TTL（秒）
	 */
	public async expire(key: string, seconds: number): Promise<number> {
		try {
			return await this.client.expire(key, seconds);
		} catch (error) {
			console.error(`[Redis] expire 失敗 ${key}:`, error);
			return 0;
		}
	}

	public async set_buffer_base64(
		key: string,
		value: string,
		ttl: number,
	): Promise<boolean> {
		try {
			await this.client.setEx(key, ttl, value);
			return true;
		} catch (error) {
			console.error(`[Redis] set_ex 失敗 ${key}:`, error);
			return false;
		}
	}

	public get_client(): RedisClientType {
		return this.client;
	}
}

// biome-ignore lint/style/useNamingConvention: singleton
export const redis: RedisProvider = new RedisProvider();
