import type {
	ChannelCredentials,
	Client,
	ServiceDefinition,
} from "@grpc/grpc-js";
import type { Observable } from "rxjs";

import { credentials, makeGenericClientConstructor } from "@grpc/grpc-js";
import { Subject } from "rxjs";

import { config } from "../config";

interface Rpc {
	request(
		service: string,
		method: string,
		data: Uint8Array,
	): Promise<Uint8Array>;
	// biome-ignore lint/style/useNamingConvention: gprc-js
	clientStreamingRequest(
		service: string,
		method: string,
		data: Observable<Uint8Array>,
	): Promise<Uint8Array>;
	// biome-ignore lint/style/useNamingConvention: gprc-js
	serverStreamingRequest(
		service: string,
		method: string,
		data: Uint8Array,
	): Observable<Uint8Array>;
	// biome-ignore lint/style/useNamingConvention: gprc-js
	bidirectionalStreamingRequest(
		service: string,
		method: string,
		data: Observable<Uint8Array>,
	): Observable<Uint8Array>;
}
interface GenericServiceDefinition {
	[key: string]: {
		path: string;
		request_serialize: (value: Uint8Array) => Buffer;
		request_deserialize: (value: Buffer) => Uint8Array;
		response_serialize: (value: Uint8Array) => Buffer;
		response_deserialize: (value: Buffer) => Uint8Array;
	};
}

class GrpcAdapter implements Rpc {
	private readonly channel_credentials: ChannelCredentials;
	private readonly grpc_address: string;
	private readonly client_cache: Map<string, Client> = new Map();

	public constructor() {
		const url = new URL(config.grpc_uri);
		this.grpc_address = `${url.hostname}:${url.port}`;

		// Use insecure credentials for now (can be configured for TLS later)
		this.channel_credentials = credentials.createInsecure();
	}

	/**
	 * Client Streaming Request - client sends stream of messages, server responds with single message
	 * Used for file uploads where client streams chunks
	 */ // biome-ignore lint/style/useNamingConvention: gprc-js
	public clientStreamingRequest(
		service: string,
		method: string,
		data: Observable<Uint8Array>,
	): Promise<Uint8Array> {
		const client = this.get_or_create_client(service, method);
		const method_path = `/${service}/${method}`;

		return new Promise((resolve, reject) => {
			const call = client.makeClientStreamRequest(
				method_path,
				(value: Uint8Array) => Buffer.from(value),
				(value: Buffer) => new Uint8Array(value),
				(error, response) => {
					if (error) {
						reject(error);
					} else if (response) {
						resolve(response);
					} else {
						reject(new Error("No response received"));
					}
				},
			);

			// Subscribe to the observable and write each chunk to the stream
			data.subscribe({
				complete: () => call.end(),
				error: (err: Error) => {
					call.cancel();
					reject(err);
				},
				next: (chunk: Uint8Array) => call.write(chunk),
			});
		});
	}

	/**
	 * Server Streaming Request - client sends single message, server responds with stream of messages
	 * Used for file downloads where server streams chunks
	 */
	// biome-ignore lint/style/useNamingConvention: gprc-js
	public serverStreamingRequest(
		service: string,
		method: string,
		data: Uint8Array,
	): Observable<Uint8Array> {
		const client = this.get_or_create_client(service, method);
		const method_path = `/${service}/${method}`;
		const subject = new Subject<Uint8Array>();

		const call = client.makeServerStreamRequest(
			method_path,
			(value: Uint8Array) => Buffer.from(value),
			(value: Buffer) => new Uint8Array(value),
			data,
		);

		call.on("data", (chunk: Uint8Array) => {
			subject.next(chunk);
		});

		call.on("end", () => {
			subject.complete();
		});

		call.on("error", (error: Error) => {
			subject.error(error);
		});

		return subject.asObservable();
	}

	/**
	 * Bidirectional Streaming Request - both client and server stream messages
	 */
	// biome-ignore lint/style/useNamingConvention: gprc-js
	public bidirectionalStreamingRequest(
		service: string,
		method: string,
		data: Observable<Uint8Array>,
	): Observable<Uint8Array> {
		const client = this.get_or_create_client(service, method);
		const method_path = `/${service}/${method}`;
		const subject = new Subject<Uint8Array>();

		const call = client.makeBidiStreamRequest(
			method_path,
			(value: Uint8Array) => Buffer.from(value),
			(value: Buffer) => new Uint8Array(value),
		);

		// Handle incoming data from server
		call.on("data", (chunk: Uint8Array) => {
			subject.next(chunk);
		});

		call.on("end", () => {
			subject.complete();
		});

		call.on("error", (error: Error) => {
			subject.error(error);
		});

		// Subscribe to the observable and write each chunk to the stream
		data.subscribe({
			complete: () => call.end(),
			error: (err: Error) => {
				call.cancel();
				subject.error(err);
			},
			next: (chunk: Uint8Array) => call.write(chunk),
		});

		return subject.asObservable();
	}

	public request(
		service: string,
		method: string,
		data: Uint8Array,
	): Promise<Uint8Array> {
		const client = this.get_or_create_client(service, method);
		const method_path = `/${service}/${method}`;

		return new Promise((resolve, reject) => {
			client.makeUnaryRequest(
				method_path,
				(value: Uint8Array) => Buffer.from(value),
				(value: Buffer) => new Uint8Array(value),
				data,
				(error, response) => {
					if (error) {
						reject(error);
					} else if (response) {
						resolve(response);
					} else {
						reject(new Error("No response received"));
					}
				},
			);
		});
	}

	private get_or_create_client(service: string, method: string): Client {
		const cache_key = service;

		let client = this.client_cache.get(cache_key);
		if (client) {
			return client;
		}

		// Create a generic service definition for dynamic calls
		const service_definition: GenericServiceDefinition = {
			[method]: {
				path: `/${service}/${method}`,
				request_deserialize: (value: Buffer) => new Uint8Array(value),
				request_serialize: (value: Uint8Array) => Buffer.from(value),
				response_deserialize: (value: Buffer) => new Uint8Array(value),
				response_serialize: (value: Uint8Array) => Buffer.from(value),
			},
		};

		const generic_client = makeGenericClientConstructor(
			service_definition as unknown as ServiceDefinition,
			service,
			{},
		);

		client = new generic_client(
			this.grpc_address,
			this.channel_credentials,
		);
		this.client_cache.set(cache_key, client);

		return client;
	}
}

// biome-ignore lint/style/useNamingConvention: singleton
export const grpc_adapter: GrpcAdapter = new GrpcAdapter();
