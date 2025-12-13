import type { DownloadChunkResponse } from "../generated/object";

import { object_client } from "../grpc";

/**
 * 從 gRPC 下載物件
 *
 * @param bucket - Bucket name
 * @param key - Object key
 * @returns Downloaded object buffer and metadata
 */
export async function download_object(
	bucket: string,
	key: string,
): Promise<{ buffer: Buffer; metadata?: DownloadChunkResponse["metadata"] }> {
	console.log(`[Download] Downloading ${key} from bucket ${bucket}...`);

	const stream = object_client.DownloadObject({ bucket, key });
	const chunks: Buffer[] = [];
	let metadata: DownloadChunkResponse["metadata"] | undefined;

	await new Promise<void>((resolve, reject) => {
		stream.subscribe({
			complete: () => {
				console.log(
					`[Download] ✓ Download complete: ${key}, chunks: ${chunks.length}`,
				);
				resolve();
			},
			error: (err: Error) => {
				console.error(`[Download] ✗ Download failed for ${key}:`, err);
				reject(err);
			},
			next: (response: DownloadChunkResponse) => {
				if (response.chunk !== undefined) {
					chunks.push(Buffer.from(response.chunk));
				}
				if (response.metadata !== undefined) {
					metadata = response.metadata;
				}
			},
		});
	});

	if (chunks.length === 0) {
		throw new Error(`No data received for ${key}`);
	}

	const buffer = Buffer.concat(chunks);
	console.log(`[Download] Total size: ${buffer.length} bytes`);

	return { buffer, metadata };
}

/**
 * 串流下載物件（不暫存在記憶體中）
 *
 * @param bucket - Bucket name
 * @param key - Object key
 * @returns ReadableStream and metadata for streaming response
 */
export async function stream_object(
	bucket: string,
	key: string,
): Promise<{
	stream: ReadableStream<Uint8Array>;
	metadata?: DownloadChunkResponse["metadata"];
}> {
	console.log(`[Stream] Streaming ${key} from bucket ${bucket}...`);

	const grpc_stream = object_client.DownloadObject({ bucket, key });
	let metadata: DownloadChunkResponse["metadata"] | undefined;
	let chunk_count = 0;
	let first_chunk: Uint8Array | undefined;

	// 等待第一個 response 以獲取 metadata
	await new Promise<void>((resolve, reject) => {
		let resolved = false;

		grpc_stream.subscribe({
			complete: () => {
				if (!resolved) {
					resolved = true;
					resolve();
				}
			},
			error: (err: Error) => {
				if (!resolved) {
					resolved = true;
					reject(err);
				}
			},
			next: (response: DownloadChunkResponse) => {
				if (response.metadata !== undefined) {
					metadata = response.metadata;
				}
				if (response.chunk !== undefined && !first_chunk) {
					first_chunk = new Uint8Array(response.chunk);
					chunk_count++;
					// 收到第一個 chunk 後就可以返回了
					if (!resolved) {
						resolved = true;
						resolve();
					}
				}
			},
		});
	});

	// 創建 ReadableStream 來串流剩餘數據
	const stream = new ReadableStream<Uint8Array>({
		cancel(): void {
			console.log(`[Stream] Stream cancelled for ${key}`);
		},
		start(controller: ReadableStreamDefaultController<Uint8Array>): void {
			// 先發送第一個 chunk（如果有）
			if (first_chunk) {
				controller.enqueue(first_chunk);
			}

			// 繼續接收剩餘 chunks
			grpc_stream.subscribe({
				complete: () => {
					console.log(
						`[Stream] ✓ Stream complete: ${key}, chunks: ${chunk_count}`,
					);
					controller.close();
				},
				error: (err: Error) => {
					console.error(`[Stream] ✗ Stream error for ${key}:`, err);
					controller.error(err);
				},
				next: (response: DownloadChunkResponse) => {
					if (response.chunk !== undefined) {
						chunk_count++;
						// 跳過第一個 chunk（已經發送了）
						if (chunk_count > 1) {
							controller.enqueue(new Uint8Array(response.chunk));
						}
					}
				},
			});
		},
	});

	return { metadata, stream };
}
