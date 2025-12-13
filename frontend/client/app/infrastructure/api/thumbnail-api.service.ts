import type { ThumbnailSize } from "@shared/constants/thumbnail.constants";
import type { Observable } from "rxjs";

import { HttpClient } from "@angular/common/http";
import { Injectable, inject } from "@angular/core";
import { ObjectPath } from "@shared/api-paths";
import { is_image } from "@shared/utility/thumbnail.utility";
import { map } from "rxjs/operators";

@Injectable({
	// biome-ignore lint/style/useNamingConvention: Angular
	providedIn: "root",
})
export class ThumbnailApiService {
	private readonly http = inject(HttpClient);

	/**
	 * Get thumbnail for an object
	 * Returns a blob URL that can be used in img src
	 */
	public get_thumbnail(
		bucket_name: string,
		key: string,
		etag: string,
		size: ThumbnailSize = "grid",
	): Observable<string> {
		const params = { bucket_name, etag, key, size };

		return this.http
			.get(ObjectPath.THUMBNAIL, {
				params,
				// biome-ignore lint/style/useNamingConvention: http
				responseType: "blob",
			})
			.pipe(map((blob) => URL.createObjectURL(blob)));
	}

	/**
	 * Check if filename is an image based on extension
	 */
	public is_image(filename: string): boolean {
		return is_image(filename);
	}

	/**
	 * Cleanup blob URL to prevent memory leaks
	 */
	public revoke_url(url: string): void {
		if (url.startsWith("blob:")) {
			URL.revokeObjectURL(url);
		}
	}
}
