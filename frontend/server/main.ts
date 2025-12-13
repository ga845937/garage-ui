import "reflect-metadata";

import type { Hono } from "hono";

import { serve } from "@hono/node-server";

import { create_app } from "./app";
import { config } from "./config";
// import { init_telemetry } from "./grpc";
import { redis } from "./redis/redis";

void config;
void redis;

// Initialize OpenTelemetry before any gRPC calls
// init_telemetry();

const APP: Hono = create_app();

console.log(`🚀 BFF Server running on http://localhost:${config.http_port}`);

serve({
	fetch: APP.fetch,
	port: config.http_port,
});

// biome-ignore lint/style/noDefaultExport: Hono convention
export default APP;
