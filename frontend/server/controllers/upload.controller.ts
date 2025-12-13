import type { UploadInitContract } from "@shared/contracts";
import type { Context } from "hono";
import type {
	UploadChunkRequest,
	UploadChunkResponse,
} from "../generated/object";

import { filter, firstValueFrom, Subject, take } from "rxjs";

import { object_client } from "../grpc/object.client";
import { thumbnail_service } from "../services/thumbnail.service";
import { download_and_generate_thumbnails } from "../utils/thumbnail.util";

interface UploadSession {
	upload_id: string;
	bucket: string;
	key: string;
	content_type: string;
	content_length: number;
	grpc_stream: Subject<UploadChunkRequest>;
	response_subject: Subject<UploadChunkResponse>;
	chunks_received: number;
	bytes_received: number;
}

const SESSIONS: Map<string, UploadSession> = new Map<string, UploadSession>();

// Session timeout: 5 minutes
const SESSION_TIMEOUT_MS: number = 5 * 60 * 1000;

class UploadController {
	/**
	 * Initialize upload session and start gRPC bidirectional stream
	 */
	public async init(context: Context): Promise<Response> {
		const payload = context.get("parse_payload") as UploadInitContract;

		const grpc_stream = new Subject<UploadChunkRequest>();
		const response_subject = new Subject<UploadChunkResponse>();

		// Start gRPC bidirectional streaming connection
		object_client.UploadObject(grpc_stream.asObservable()).subscribe({
			complete: () => response_subject.complete(),
			error: (err: Error) => response_subject.error(err),
			next: (response: UploadChunkResponse) => {
				response_subject.next(response);
			},
		});

		// Send metadata as first message
		const metadata_msg: UploadChunkRequest = {
			metadata: {
				bucket: payload.bucket_name,
				content_length: payload.content_length,
				content_type: payload.content_type,
				key: payload.key,
			},
		};
		grpc_stream.next(metadata_msg);

		const response = await firstValueFrom(
			response_subject.pipe(
				filter((r) => r.initiated !== undefined),
				take(1),
			),
		);

		if (!response.initiated) {
			return context.json({ error: "Upload init failed" }, 500);
		}

		const upload_id = response.initiated.upload_id;
		const session: UploadSession = {
			bucket: payload.bucket_name,
			bytes_received: 0,
			chunks_received: 0,
			content_length: payload.content_length,
			content_type: payload.content_type,
			grpc_stream,
			key: payload.key,
			response_subject,
			upload_id,
		};
		SESSIONS.set(upload_id, session);

		// Auto-cleanup after timeout
		setTimeout(() => {
			const s = SESSIONS.get(upload_id);
			if (s) {
				s.grpc_stream.complete();
				SESSIONS.delete(upload_id);
			}
		}, SESSION_TIMEOUT_MS);

		return context.json({
			upload_id,
		});
	}

	/**
	 * Abort an upload session
	 */
	public async abort(context: Context): Promise<Response> {
		const upload_id = context.req.param("upload_id");
		const session = SESSIONS.get(upload_id);

		if (!session) {
			return context.json({ error: "Upload session not found" }, 404);
		}

		// Close local session stream
		session.grpc_stream.complete();

		// If we have a gRPC upload_id, call AbortUpload on the server
		try {
			await object_client.AbortUpload({
				bucket: session.bucket,
				key: session.key,
				upload_id: session.upload_id,
			});

			return context.json({});
		} catch (error) {
			const message =
				error instanceof Error ? error.message : "Abort failed";
			return context.json({ error: message }, 500);
		} finally {
			SESSIONS.delete(upload_id);
		}
	}

	/**
	 * Stream upload - receive file as stream and forward to gRPC
	 * Uses SSE for progress updates (forwarded from gRPC server)
	 * Requires init() to be called first to get upload_id
	 */
	public stream(context: Context): Response {
		// Get upload_id from URL param
		const upload_id = context.req.param("upload_id");
		if (!upload_id) {
			return context.json({ error: "Missing upload_id" }, 400);
		}

		// Get session from SESSIONS
		const session = SESSIONS.get(upload_id);
		if (!session) {
			return context.json({ error: "Upload session not found" }, 404);
		}

		// Use session data
		const { bucket, key, grpc_stream, response_subject, content_length } =
			session;

		// Create SSE response stream
		const encoder = new TextEncoder();
		const stream = new ReadableStream({
			// biome-ignore lint/complexity/noExcessiveCognitiveComplexity: streaming logic
			async start(
				controller: ReadableStreamDefaultController<Uint8Array>,
			): Promise<void> {
				// Send initiated event
				controller.enqueue(
					encoder.encode(
						`data: ${JSON.stringify({ type: "initiated", upload_id })}\n\n`,
					),
				);

				// Subscribe to gRPC responses for progress updates
				const grpc_subscription = response_subject.subscribe({
					error: (err: Error) => {
						controller.enqueue(
							encoder.encode(
								`data: ${JSON.stringify({ message: err.message, type: "error" })}\n\n`,
							),
						);
					},
					next: (response: UploadChunkResponse) => {
						// Forward progress events from gRPC server
						if (response.progress) {
							const { bytes_uploaded, total_bytes } =
								response.progress;
							const progress =
								total_bytes > 0
									? Math.round(
											(bytes_uploaded / total_bytes) *
												100,
										)
									: 0;
							controller.enqueue(
								encoder.encode(
									`data: ${JSON.stringify({
										bytes_uploaded,
										progress,
										total_bytes,
										type: "progress",
									})}\n\n`,
								),
							);
						}
						// Forward result event from gRPC server
						if (response.result) {
							const result = response.result;
							// Generate thumbnails for images
							if (
								result.etag &&
								thumbnail_service.is_image(key)
							) {
								download_and_generate_thumbnails(
									bucket,
									key,
									result.etag,
									["grid", "list"],
								).catch((err: Error) =>
									console.error(
										`[Upload] Thumbnail generation failed:`,
										err,
									),
								);
							}
							controller.enqueue(
								encoder.encode(
									`data: ${JSON.stringify({
										etag: result.etag,
										size: result.size,
										type: "completed",
									})}\n\n`,
								),
							);
							controller.close();
							SESSIONS.delete(upload_id);
						}
					},
				});

				// Read request body stream and forward to gRPC
				const body = context.req.raw.body;
				if (!body) {
					controller.enqueue(
						encoder.encode(
							`data: ${JSON.stringify({ message: "No request body", type: "error" })}\n\n`,
						),
					);
					controller.close();
					grpc_stream.complete();
					grpc_subscription.unsubscribe();
					return;
				}

				const reader = body.getReader();
				const chunk_size = 64 * 1024; // 64KB chunks for gRPC
				let buffer = new Uint8Array(0);

				try {
					while (true) {
						const { done, value } = await reader.read();

						if (done) {
							// Send any remaining buffer
							if (buffer.length > 0) {
								const chunk_msg: UploadChunkRequest = {
									chunk: buffer,
								};
								grpc_stream.next(chunk_msg);
							}
							break;
						}

						// Append to buffer
						const new_buffer = new Uint8Array(
							buffer.length + value.length,
						);
						new_buffer.set(buffer);
						new_buffer.set(value, buffer.length);
						buffer = new_buffer;

						// Send chunks when buffer is large enough
						while (buffer.length >= chunk_size) {
							const chunk = buffer.slice(0, chunk_size);
							buffer = buffer.slice(chunk_size);

							const chunk_msg: UploadChunkRequest = {
								chunk,
							};
							grpc_stream.next(chunk_msg);
						}
					}

					// Complete gRPC stream (triggers server to send result)
					grpc_stream.complete();
				} catch (error) {
					grpc_subscription.unsubscribe();
					const message =
						error instanceof Error
							? error.message
							: "Upload failed";
					controller.enqueue(
						encoder.encode(
							`data: ${JSON.stringify({ message, type: "error" })}\n\n`,
						),
					);
					controller.close();
				}
			},
		});

		return new Response(stream, {
			headers: {
				"Cache-Control": "no-cache",
				// biome-ignore lint/style/useNamingConvention: SSE
				Connection: "keep-alive",
				"Content-Type": "text/event-stream",
			},
		});
	}
}

// biome-ignore lint/style/useNamingConvention: singleton
export const upload_controller: UploadController = new UploadController();
