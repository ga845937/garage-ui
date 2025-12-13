export enum UtilityPath {
	HEALTH = "/health",
}

export enum KeyPath {
	BASE = "/api/access-key",
	DETAIL = "/api/access-key/:id",
	DELETE = "/api/access-key/delete",
}

export enum BucketPath {
	BASE = "/api/bucket",
	DETAIL = "/api/bucket/:id",
	DELETE = "/api/bucket/delete",
	ALIAS = "/api/bucket/:bucket_id/alias",
}

export enum ObjectPath {
	BASE = "/api/:bucket_name/object",
	OPERATE = "/api/:bucket_name/object/:key{.+}",
	DELETE = "/api/:bucket_name/object/delete",
	THUMBNAIL = "/api/:bucket_name/object/thumbnail"
}

export enum UploadPath {
	INIT = "/api/upload/init",
	CHUNK = "/api/upload/:upload_id/chunk",
	COMPLETE = "/api/upload/:upload_id/complete",
	ABORT = "/api/upload/:upload_id/abort",
	STREAM = "/api/upload/:upload_id/stream",
}

export function build_path(
	path: string,
	params: Record<string, string>,
): string {
	return Object.entries(params).reduce(
		(p, [k, v]) => p.replace(`:${k}`, v),
		path,
	);
}