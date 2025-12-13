import type { BucketKeyPermissionContract } from "@shared/contracts/bucket.contract";

import { Injectable, inject, signal } from "@angular/core";

import { CreateBucketCommand } from "../../../../application/commands/bucket/create-bucket.command";
import { NavigationService } from "../../../../infrastructure/navigation";
import { LayoutService } from "../../../layout/layout.service";

@Injectable()
export class BucketCreateFacade {
	private readonly create_command = inject(CreateBucketCommand);
	private readonly navigation = inject(NavigationService);
	private readonly layout = inject(LayoutService);

	// UI State
	public readonly submitting = signal(false);

	public init(): void {
        // No cleanup needed
	}

	public destroy(): void {
        // No cleanup needed
	}

	public async create(
		global_alias: string,
		key_permissions: BucketKeyPermissionContract[],
	): Promise<boolean> {
		this.submitting.set(true);
		this.layout.set_error(null);

		try {
			await this.create_command.execute(global_alias, key_permissions);
			this.navigation.to_bucket_list();
			return true;
		} catch (e) {
			const message =
				e instanceof Error ? e.message : "Failed to create bucket";
			this.layout.set_error(message);
			return false;
		} finally {
			this.submitting.set(false);
		}
	}

	public cancel(): void {
		this.navigation.to_bucket_list();
	}
}
