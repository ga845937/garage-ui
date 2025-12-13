import type { Task } from "../task-progress.service";

import { CommonModule } from "@angular/common";
import { Component, inject } from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatIconModule } from "@angular/material/icon";
import { MatMenuModule } from "@angular/material/menu";
import { MatProgressBarModule } from "@angular/material/progress-bar";
import { MatTooltipModule } from "@angular/material/tooltip";
import { Router, RouterLink } from "@angular/router";
import { build_route, ObjectRoute, SettingsRoute } from "@shared/route-paths";
import { NgxTolgeeModule, TranslateService } from "@tolgee/ngx";

import { LayoutService } from "../layout.service";
import { TaskProgressService } from "../task-progress.service";

@Component({
	imports: [
		CommonModule,
		MatButtonModule,
		MatIconModule,
		MatMenuModule,
		MatTooltipModule,
		MatProgressBarModule,
		RouterLink,
		NgxTolgeeModule,
	],
	selector: "app-top-bar",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./top-bar.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./top-bar.component.html",
})
export class TopBarComponent {
	public layout_service = inject(LayoutService);
	public task_service = inject(TaskProgressService);
	private translate_service = inject(TranslateService);
	private router = inject(Router);

	public readonly settings_route = SettingsRoute;

	public readonly license_url = "https://github.com/ga845937/garage-ui/blob/master/LICENSE.md";
	public readonly github_url = "https://github.com/ga845937/garage-ui";

	public toggle_language(): void {
		const current_lang = this.translate_service.language;
		const new_lang = current_lang === "zh-TW" ? "en" : "zh-TW";
		this.translate_service.changeLanguage(new_lang);
	}

	public on_task_click(task: Task): void {
		if (task.bucket_id) {
			const path = build_route(ObjectRoute.LIST, {
				bucket_id: task.bucket_id,
			});
			this.router.navigate([path]);
		}
	}

	public on_cancel_task(event: Event, task_id: string): void {
		event.stopPropagation();
		this.task_service.cancel_task(task_id);
	}
}
