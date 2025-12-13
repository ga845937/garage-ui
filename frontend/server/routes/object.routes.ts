import { ObjectPath } from "@shared/api-paths";
import {
	DeleteObjectRequestContract,
	ListObjectsRequestContract,
} from "@shared/contracts";
import { GetThumbnailContract } from "@shared/contracts/thumbnail.contract";
import { Hono } from "hono";

import { object_controller } from "../controllers/object.controller";
import { validate_payload } from "../middleware/validator";

// biome-ignore lint/style/useNamingConvention: singleton
export const object_route: Hono = new Hono();

object_route.get(
	ObjectPath.BASE,
	validate_payload(ListObjectsRequestContract),
	(c) => object_controller.list(c),
);

object_route.get(
	ObjectPath.THUMBNAIL,
	validate_payload(GetThumbnailContract),
	(c) => object_controller.get_thumbnail(c),
);

object_route.post(
	ObjectPath.DELETE,
	validate_payload(DeleteObjectRequestContract),
	(c) => object_controller.delete(c),
);

object_route.get(ObjectPath.OPERATE, (c) => object_controller.download(c));
