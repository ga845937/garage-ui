import { Injectable, inject } from "@angular/core";
import { Router } from "@angular/router";
import { BucketRoute, build_route, KeyRoute, ObjectRoute } from "@shared/route-paths";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class NavigationService {
	private readonly router = inject(Router);

	// Access Key
	public to_access_key_list(): void {
		this.router.navigateByUrl(`/${KeyRoute.LIST}`);
	}

	public to_access_key_detail(id: string): void {
		this.router.navigateByUrl(build_route(KeyRoute.DETAIL, { id }));
	}

	public to_access_key_create(): void {
		this.router.navigateByUrl(`/${KeyRoute.CREATE}`);
	}

	// Bucket
	public to_bucket_list(): void {
		this.router.navigateByUrl(`/${BucketRoute.LIST}`);
	}

	public to_bucket_detail(id: string): void {
		this.router.navigateByUrl(build_route(BucketRoute.DETAIL, { id }));
	}

	public to_bucket_create(): void {
		this.router.navigateByUrl(`/${BucketRoute.CREATE}`);
	}

	// Object
	public to_bucket_objects(bucket_id: string): void {
		this.router.navigateByUrl(build_route(ObjectRoute.LIST, { bucket_id }));
	}
}
