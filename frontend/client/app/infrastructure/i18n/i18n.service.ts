import { Injectable, inject } from "@angular/core";
import { TranslateService } from "@tolgee/ngx";

/**
 * I18n 工具 Service
 * 用於 standalone component 中取得翻譯文字
 */
// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class I18nService {
	private readonly translate = inject(TranslateService);

	/**
	 * 取得翻譯文字 (同步，需確保翻譯已載入)
	 */
	public t(key: string, params?: Record<string, string | number>): string {
		return this.translate.instant(key, params);
	}

	/**
	 * 取得目前語言
	 */
	public get language(): string {
		return this.translate.language;
	}

	/**
	 * 切換語言
	 */
	public set_language(lang: string): void {
		this.translate.changeLanguage(lang);
	}
}
