// HTTP Client Interface
// 抽象 HTTP 客戶端，方便日後抽換實作

export interface RequestOptions {
	headers?: Record<string, string>;
	params?: URLSearchParams;
}

export interface HttpClient {
	get<T>(url: string, options?: RequestOptions): Promise<T>;
	post<T>(url: string, body: unknown, options?: RequestOptions): Promise<T>;
	put<T>(url: string, body: unknown, options?: RequestOptions): Promise<T>;
	patch<T>(url: string, body: unknown, options?: RequestOptions): Promise<T>;
	delete<T>(url: string, options?: RequestOptions): Promise<T>;
}
