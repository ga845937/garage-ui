import { Injectable, signal } from "@angular/core";

export type DateFormatOption = "short" | "medium" | "long" | "iso" | "relative";

export interface DateFormatConfig {
	format: DateFormatOption;
	locale: string;
}

const DATE_FORMAT_KEY = "garage_date_format";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({
	// biome-ignore lint/style/useNamingConvention: Angular
	providedIn: "root",
})
export class DateFormatService {
	private readonly format_signal = signal<DateFormatOption>(this.load_format());
	private readonly locale_signal = signal<string>("zh-TW");

	public readonly format_option = this.format_signal.asReadonly();
	public readonly locale = this.locale_signal.asReadonly();

	public readonly format_options: {
		value: DateFormatOption;
		label: string;
		example: string;
	}[] = [
		{ example: "1/8/26", label: "Short", value: "short" },
		{ example: "Jan 8, 2026", label: "Medium", value: "medium" },
		{ example: "January 8, 2026", label: "Long", value: "long" },
		{ example: "2026-01-08", label: "ISO 8601", value: "iso" },
		{ example: "2 days ago", label: "Relative", value: "relative" },
	];

	private load_format(): DateFormatOption {
		const saved = localStorage.getItem(DATE_FORMAT_KEY);
		return (saved as DateFormatOption) || "medium";
	}

	public set_format(format: DateFormatOption): void {
		this.format_signal.set(format);
		localStorage.setItem(DATE_FORMAT_KEY, format);
	}

	public set_locale(locale: string): void {
		this.locale_signal.set(locale);
	}

	public format(date: string | Date | undefined): string {
		if (!date) {
			return "-";
		}

		const d = typeof date === "string" ? new Date(date) : date;
		if (Number.isNaN(d.getTime())) {
			return "-";
		}

		const current_locale = this.locale_signal();

		switch (this.format_signal()) {
			case "short":
				return d.toLocaleDateString(current_locale, {
					day: "numeric",
					month: "numeric",
					year: "2-digit",
				});
			case "medium":
				return d.toLocaleDateString(current_locale, {
					day: "numeric",
					month: "short",
					year: "numeric",
				});
			case "long":
				return d.toLocaleDateString(current_locale, {
					day: "numeric",
					month: "long",
					year: "numeric",
				});
			case "iso":
				return d.toISOString().split("T")[0];
			case "relative":
				return this.get_relative_time(d);
			default:
				return d.toLocaleDateString(current_locale);
		}
	}

	/** Returns full datetime string with time for tooltips */
	public format_full(date: string | Date | undefined): string {
		if (!date) {
			return "-";
		}

		const d = typeof date === "string" ? new Date(date) : date;
		if (Number.isNaN(d.getTime())) {
			return "-";
		}

		const year = d.getFullYear();
		const month = String(d.getMonth() + 1).padStart(2, "0");
		const day = String(d.getDate()).padStart(2, "0");
		const hours = String(d.getHours()).padStart(2, "0");
		const minutes = String(d.getMinutes()).padStart(2, "0");
		const seconds = String(d.getSeconds()).padStart(2, "0");

		return `${year}/${month}/${day} ${hours}:${minutes}:${seconds}`;
	}

	private get_relative_time(date: Date): string {
		const now = new Date();
		const diff_ms = now.getTime() - date.getTime();
		const diff_days = Math.floor(diff_ms / (1000 * 60 * 60 * 24));

		if (diff_days === 0) {
			return "Today";
		}
		if (diff_days === 1) {
			return "Yesterday";
		}
		if (diff_days < 7) {
			return `${diff_days} days ago`;
		}
		if (diff_days < 30) {
			return `${Math.floor(diff_days / 7)} weeks ago`;
		}
		if (diff_days < 365) {
			return `${Math.floor(diff_days / 30)} months ago`;
		}
		return `${Math.floor(diff_days / 365)} years ago`;
	}
}
