import type {
	BucketKeyPermissionContract,
	UpdateBucketContract,
} from "@shared/contracts/bucket.contract";
import type { Bucket, BucketListItem } from "@shared/entity/bucket.entity";
import type { ObjectList } from "@shared/entity/object.entity";
import type { ListResponse } from "@shared/entity/response.entity";
import type { HttpClient } from "../http";

import { Injectable, inject } from "@angular/core";
import {
	BucketPath,
	build_path,
	ObjectPath,
	UploadPath,
} from "@shared/api-paths";

import { HTTP_CLIENT } from "../http";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class BucketService {
	private readonly http = inject<HttpClient>(HTTP_CLIENT);

	public async list_buckets(
		page: number,
		page_size: number,
	): Promise<ListResponse<BucketListItem>> {
		const params = new URLSearchParams({
			page: page.toString(),
			page_size: page_size.toString(),
		});
		return await this.http.get(BucketPath.BASE, { params });
	}

	public get_bucket(id: string): Promise<Bucket> {
		const url = build_path(BucketPath.DETAIL, { id });
		return this.http.get(url);
	}

	public create_bucket(
		global_alias: string,
		key_permissions: BucketKeyPermissionContract[],
	): Promise<Bucket> {
		return this.http.post(BucketPath.BASE, {
			global_alias,
			key_permissions,
		});
	}

	public async update_bucket(
		id: string,
		data: UpdateBucketContract,
	): Promise<Bucket> {
		const url = build_path(BucketPath.DETAIL, { id });
		return await this.http.put(url, data);
	}

	public async delete_bucket(id: string[]): Promise<void> {
		return await this.http.post(BucketPath.DELETE, { id });
	}

	public async list_objects(
		bucket_name: string,
		prefix?: string,
		continuation_token?: string,
		max_keys?: number,
		delimiter?: string,
	): Promise<ObjectList> {
		const params = new URLSearchParams();
		if (prefix) {
			params.set("prefix", prefix);
		}
		if (continuation_token) {
			params.set("continuation_token", continuation_token);
		}
		if (max_keys) {
			params.set("max_keys", max_keys.toString());
		}
		if (delimiter) {
			params.set("delimiter", delimiter);
		}

		const url = build_path(ObjectPath.BASE, { bucket_name });
		return await this.http.get(url, { params: params });
	}

	/**
	 * Stream upload - two-step flow:
	 * 1. Call init to get upload_id and establish gRPC connection
	 * 2. Call stream to upload file content with SSE progress
	 */
	// biome-ignore lint/complexity/noExcessiveCognitiveComplexity: SSE parsing logic
	public async upload_file_stream(
		bucket_name: string,
		key: string,
		file: File,
		on_progress?: (progress: number) => void,
		signal?: AbortSignal,
	): Promise<void> {
		const content_type = file.type || "application/octet-stream";

		// Step 1: Initialize upload session
		const init_response = await this.http.post<{ upload_id: string }>(
			UploadPath.INIT,
			{
				bucket_name,
				content_length: file.size,
				content_type: content_type,
				key,
			},
		);

		const { upload_id } = init_response;
		if (upload_id === "folder_upload") {
			return;
		}

		// Step 2: Stream file content
		const stream_url = build_path(UploadPath.STREAM, { upload_id });
		const file_buffer = await file.arrayBuffer();

		const response = await fetch(stream_url, {
			body: file_buffer,
			headers: {
				"Content-Type": "application/octet-stream",
			},
			method: "POST",
			signal,
		});

		if (!response.ok) {
			throw new Error(`Upload failed: ${response.statusText}`);
		}

		if (!response.body) {
			throw new Error("No response body");
		}

		// Parse SSE response
		const reader = response.body.getReader();
		const decoder = new TextDecoder();
		let buffer = "";

		while (true) {
			const { done, value } = await reader.read();

			if (done) {
				break;
			}

			buffer += decoder.decode(value, { stream: true });

			// Parse SSE events
			const lines = buffer.split("\n\n");
			buffer = lines.pop() || "";

			for (const line of lines) {
				if (line.startsWith("data: ")) {
					const data = JSON.parse(line.slice(6)) as {
						type: string;
						progress?: number;
						message?: string;
					};

					if (
						data.type === "progress" &&
						on_progress &&
						data.progress !== undefined
					) {
						on_progress(data.progress);
					} else if (data.type === "error") {
						throw new Error(data.message || "Upload failed");
					} else if (data.type === "completed") {
						return;
					}
				}
			}
		}
	}

	/**
	 * Create a ReadableStream from a File, reading in chunks
	 */
	private create_file_stream(
		file: File,
		chunk_size: number,
	): ReadableStream<Uint8Array> {
		let offset = 0;

		return new ReadableStream<Uint8Array>({
			pull: async (
				controller: ReadableStreamDefaultController<Uint8Array>,
			): Promise<void> => {
				if (offset >= file.size) {
					controller.close();
					return;
				}

				const end = Math.min(offset + chunk_size, file.size);
				const blob = file.slice(offset, end);
				const buffer = await blob.arrayBuffer();

				controller.enqueue(new Uint8Array(buffer));
				offset = end;
			},
		});
	}

	public async delete_object(
		bucket_name: string,
		key: string[],
	): Promise<void> {
		const url = build_path(ObjectPath.DELETE, { bucket_name });
		await this.http.post(url, { key });
	}

	public get_download_url(bucket_name: string, key: string): string {
		return `${build_path(ObjectPath.BASE, { bucket_name })}/${encodeURIComponent(key)}`;
	}
}
