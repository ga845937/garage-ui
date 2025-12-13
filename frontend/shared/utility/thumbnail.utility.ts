import { IMAGE_EXTENSIONS } from "../constants/thumbnail.constants";

/**
 * Check if a filename is an image based on its extension
 */
export function is_image(filename: string): boolean {
	const ext = filename.split(".").pop()?.toLowerCase() || "";
	return IMAGE_EXTENSIONS.includes(ext as (typeof IMAGE_EXTENSIONS)[number]);
}
