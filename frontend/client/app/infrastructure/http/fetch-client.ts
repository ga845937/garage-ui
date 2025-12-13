// Fetch Client Implementation
// 使用原生 fetch 的 HttpClient 實作

import type { HttpClient, RequestOptions } from "./http-client.interface";

import { Injectable, InjectionToken } from "@angular/core";

// DI Token
export const HTTP_CLIENT: InjectionToken<HttpClient> = new InjectionToken<HttpClient>("HTTP_CLIENT");

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class FetchClient implements HttpClient {
	private async request<T>(
		url: string,
		options: RequestInit & { params?: URLSearchParams },
	): Promise<T> {
		const full_url = options.params ? `${url}?${options.params}` : url;

		const response = await fetch(full_url, {
			...options,
			headers: {
				"Content-Type": "application/json",
				...options.headers,
			},
		});

		if (!response.ok) {
			throw new Error(`HTTP ${response.status}: ${response.statusText}`);
		}

		// Handle empty responses
		const text = await response.text();
		return text ? (JSON.parse(text) as T) : (undefined as T);
	}

	public get<T>(url: string, options?: RequestOptions): Promise<T> {
		return this.request(url, {
			headers: options?.headers,
			method: "GET",
			params: options?.params,
		});
	}

	public post<T>(
		url: string,
		body: unknown,
		options?: RequestOptions,
	): Promise<T> {
		return this.request(url, {
			body: JSON.stringify(body),
			headers: options?.headers,
			method: "POST",
			params: options?.params,
		});
	}

	public put<T>(
		url: string,
		body: unknown,
		options?: RequestOptions,
	): Promise<T> {
		return this.request(url, {
			body: JSON.stringify(body),
			headers: options?.headers,
			method: "PUT",
			params: options?.params,
		});
	}

	public patch<T>(
		url: string,
		body: unknown,
		options?: RequestOptions,
	): Promise<T> {
		return this.request(url, {
			body: JSON.stringify(body),
			headers: options?.headers,
			method: "PATCH",
			params: options?.params,
		});
	}

	public delete<T>(url: string, options?: RequestOptions): Promise<T> {
		return this.request(url, {
			headers: options?.headers,
			method: "DELETE",
			params: options?.params,
		});
	}
}

// Provider function for app.config.ts
export function provide_fetch_client(): {
	provide: InjectionToken<HttpClient>;
	// biome-ignore lint/style/useNamingConvention: Angular
	useClass: typeof FetchClient;
} {
	return {
		provide: HTTP_CLIENT,
		// biome-ignore lint/style/useNamingConvention: Angular
		useClass: FetchClient,
	};
}
