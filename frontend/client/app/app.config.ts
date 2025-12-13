import type { ApplicationConfig } from "@angular/core";
import type { TolgeeInstance } from "@tolgee/ngx";

import { provideHttpClient, withFetch } from "@angular/common/http";
import {
	importProvidersFrom,
	provideBrowserGlobalErrorListeners,
} from "@angular/core";
import { provideRouter, withComponentInputBinding } from "@angular/router";
import {
	DevTools,
	FormatSimple,
	NgxTolgeeModule,
	TOLGEE_INSTANCE,
	Tolgee,
} from "@tolgee/ngx";

import { app_route } from "./app.routes";
import { ENVIRONMENT } from "./environments/environment";
import { provide_fetch_client } from "./infrastructure/http";

// Tolgee 設定 (使用預設值，實際值會由 server 注入)
const TOLGEE: TolgeeInstance = Tolgee()
	.use(DevTools())
	.use(FormatSimple())
	.init({
		// biome-ignore lint/style/useNamingConvention: Tolgee
		apiKey: ENVIRONMENT.tolgee_api_key || undefined,
		// biome-ignore lint/style/useNamingConvention: Tolgee
		apiUrl: ENVIRONMENT.tolgee_api_url || undefined,
		// biome-ignore lint/style/useNamingConvention: Tolgee
		availableLanguages: ENVIRONMENT.language,
		language: ENVIRONMENT.default_language,
		// biome-ignore lint/style/useNamingConvention: Tolgee
		staticData: {
			en: () => import("@shared/i18n/en.json"),
			"zh-TW": () => import("@shared/i18n/zh-TW.json"),
		},
	});

// biome-ignore lint/style/useNamingConvention: singleton
export const app_config: ApplicationConfig = {
	providers: [
		provideBrowserGlobalErrorListeners(),
		provideHttpClient(withFetch()),
		provide_fetch_client(),
		provideRouter(app_route, withComponentInputBinding()),
		importProvidersFrom(NgxTolgeeModule),
		// biome-ignore lint/style/useNamingConvention: Angular
		{ provide: TOLGEE_INSTANCE, useValue: TOLGEE },
	],
};
