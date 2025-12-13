import { BucketPath } from "@shared/api-paths";
import {
	CreateBucketContract,
	DeleteBucketContract,
	ListBucketContract,
	UpdateBucketContract,
} from "@shared/contracts";
import { Hono } from "hono";

import { bucket_controller } from "../controllers/bucket.controller";
import { validate_payload } from "../middleware/validator";

// biome-ignore lint/style/useNamingConvention: singleton
export const bucket_route: Hono = new Hono();

// POST /api/buckets
bucket_route.post(
	BucketPath.BASE,
	validate_payload(CreateBucketContract),
	(c) => bucket_controller.create(c),
);

bucket_route.get(
	BucketPath.BASE,
	validate_payload(ListBucketContract),
	bucket_controller.list,
);

bucket_route.get(BucketPath.DETAIL, bucket_controller.get);

bucket_route.put(
	BucketPath.DETAIL,
	validate_payload(UpdateBucketContract),
	(c) => bucket_controller.update(c),
);

bucket_route.post(
	BucketPath.DELETE,
	validate_payload(DeleteBucketContract),
	(c) => bucket_controller.delete(c),
);
