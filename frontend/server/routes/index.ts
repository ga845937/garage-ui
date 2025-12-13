import { Hono } from "hono";

import { access_key_route } from "./access-key.routes";
import { bucket_route } from "./bucket.routes";
import { object_route } from "./object.routes";
import { upload_route } from "./upload.routes";

// biome-ignore lint/style/useNamingConvention: Hono
export const api_route: Hono = new Hono();

// Register all sub-routes
api_route.route("/", access_key_route);
api_route.route("/", bucket_route);
api_route.route("/", object_route);
api_route.route("/", upload_route);
