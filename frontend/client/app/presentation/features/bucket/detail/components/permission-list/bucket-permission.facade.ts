import { Injectable, inject } from "@angular/core";
import { Router } from "@angular/router";
import { BucketRoute, build_route } from "@shared/route-paths";

import { BucketObjectStore } from "../../../../../../application/stores";
import { BucketRepository } from "../../../../../../infrastructure/repositories";
import { LayoutService } from "../../../../../layout/layout.service";

@Injectable()
export class BucketPermissionFacade {
	private readonly store = inject(BucketObjectStore);
	private readonly repository = inject(BucketRepository);
	private readonly layout = inject(LayoutService);
	private readonly router = inject(Router);

	public init(id: string): void {
		this.load_bucket(id);
	}

	public destroy(): void {
		// Cleanup if needed
	}

	private async load_bucket(id: string): Promise<void> {
		this.layout.set_loading(true);
		try {
			const bucket = await this.repository.find_by_id(id);
			this.store.set_selected_bucket(bucket);
		} catch (e) {
			const message =
				e instanceof Error ? e.message : "Failed to load bucket";
			this.layout.set_error(message);
		} finally {
			this.layout.set_loading(false);
		}
	}

	public async refresh(id: string): Promise<void> {
		await this.load_bucket(id);
	}

	public go_back(id: string): void {
		const route = build_route(BucketRoute.DETAIL, { id });
		this.router.navigate([route]);
	}
}
