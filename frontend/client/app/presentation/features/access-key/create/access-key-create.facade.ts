import type { CreateAccessKeyContract } from "@shared/contracts";

import { Injectable, inject } from "@angular/core";

import { CreateAccessKeyCommand } from "../../../../application/commands";
import { NavigationService } from "../../../../infrastructure/navigation";
import { LayoutService } from "../../../layout/layout.service";

@Injectable()
export class AccessKeyCreateFacade {
	private readonly create_command = inject(CreateAccessKeyCommand);
	private readonly navigation = inject(NavigationService);
	private readonly layout = inject(LayoutService);

	public async create(data: CreateAccessKeyContract): Promise<boolean> {
		this.layout.set_loading(true);
		this.layout.set_error(null);

		try {
			const result = await this.create_command.execute(data);

			if (result) {
				this.navigation.to_access_key_detail(result.id);
				return true;
			}
			return false;
		} catch (e) {
			const message =
				e instanceof Error ? e.message : "Failed to create access key";
			this.layout.set_error(message);
			return false;
		} finally {
			this.layout.set_loading(false);
		}
	}

	public cancel(): void {
		this.navigation.to_access_key_list();
	}
}
