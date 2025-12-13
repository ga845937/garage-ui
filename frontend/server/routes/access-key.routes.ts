import { KeyPath } from "@shared/api-paths";
import {
	CreateAccessKeyContract,
	DeleteAccessKeysContract,
	ListAccessKeyContract,
	ReadAccessKeyContract,
	UpdateAccessKeyContract,
} from "@shared/contracts";
import { Hono } from "hono";

import { access_key_controller } from "../controllers/access-key.controller";
import { validate_payload } from "../middleware/validator";

// biome-ignore lint/style/useNamingConvention: singleton
export const access_key_route: Hono = new Hono();

// POST /api/access-keys
access_key_route.post(
	KeyPath.BASE,
	validate_payload(CreateAccessKeyContract),
	access_key_controller.create,
);

// GET /api/access-keys
access_key_route.get(
	KeyPath.BASE,
	validate_payload(ListAccessKeyContract),
	access_key_controller.list,
);

// GET /api/access-keys/:id
access_key_route.get(
	KeyPath.DETAIL,
	validate_payload(ReadAccessKeyContract),
	access_key_controller.get,
);

// PUT /api/access-keys/:id
access_key_route.put(
	KeyPath.DETAIL,
	validate_payload(UpdateAccessKeyContract),
	access_key_controller.update,
);

// POST /api/access-keys/delete
access_key_route.post(
	KeyPath.DELETE,
	validate_payload(DeleteAccessKeysContract),
	access_key_controller.delete,
);
