import { UploadPath } from "@shared/api-paths";
import {
	UploadInitContract,
} from "@shared/contracts";
import { Hono } from "hono";

import { upload_controller } from "../controllers/upload.controller";
import { validate_payload } from "../middleware/validator";

// biome-ignore lint/style/useNamingConvention: Hono
export const upload_route: Hono = new Hono();

upload_route.post(
	UploadPath.INIT,
	validate_payload(UploadInitContract),
	upload_controller.init,
);

upload_route.post(
	UploadPath.ABORT,
	upload_controller.abort,
);

upload_route.post(
	UploadPath.STREAM,
	upload_controller.stream,
);
