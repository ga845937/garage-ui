import { serveStatic } from "@hono/node-server/serve-static";
import { UtilityPath } from "@shared/api-paths";
import { Hono } from "hono";
import { compress } from "hono/compress";
import { cors } from "hono/cors";
import { logger } from "hono/logger";

import { api_route } from "./routes";

export function create_app(): Hono {
	const app = new Hono();

	// Middleware
	app.use("*", logger());
	app.use("*", compress());

	// CORS for development
	app.use(
		"/api/*",
		cors({
			// biome-ignore lint/style/useNamingConvention: hono
			allowHeaders: ["Content-Type", "Authorization", "X-Trace-Id"],
			// biome-ignore lint/style/useNamingConvention: hono
			allowMethods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"],
			origin: ["http://localhost:4200"],
		}),
	);

	// Health check
	app.get(UtilityPath.HEALTH, (c) =>
		c.json({ status: "ok", timestamp: new Date().toISOString() }),
	);


	// Register API routes
	app.route("/", api_route);

	// Serve Static Files (Angular)
	// Match any file in dist/garage-ui/browser
	app.use("/*", serveStatic({ root: "./dist/garage-ui/browser" }));

	// SPA Fallback
	// If no file found, serve index.html
	app.get("*", serveStatic({ path: "./dist/garage-ui/browser/index.html" }));

	return app;
}
