// Route Paths
// 前端路由 path enums

export enum KeyRoute {
	LIST = "access-key",
	CREATE = "access-key/create",
	DETAIL = "access-key/:id",
}

export enum BucketRoute {
	LIST = "bucket",
	CREATE = "bucket/create",
	DETAIL = "bucket/:id",
	PERMISSION = "bucket/:id/permission",
}

export enum ObjectRoute {
	LIST = ":bucket_id/object",
}

export enum SettingsRoute {
	HOME = "settings",
}

export function build_route(
	path: string,
	params: Record<string, string>,
): string {
	return Object.entries(params).reduce(
		(p, [k, v]) => p.replace(`:${k}`, v),
		path,
	);
}
