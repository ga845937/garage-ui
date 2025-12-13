export const IMAGE_EXTENSIONS = [
	"jpg",
	"jpeg",
	"png",
	"gif",
	"webp",
	"bmp",
	"svg",
] as const;

export const THUMBNAIL_SIZES = {
	grid: { height: 48, width: 48 },
	list: { height: 24, width: 24 },
} as const;

export type ThumbnailSize = keyof typeof THUMBNAIL_SIZES;
