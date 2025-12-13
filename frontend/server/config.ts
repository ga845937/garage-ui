import { env } from "node:process";

interface Config {
	http_port: number;
	grpc_uri: string;
	redis_uri: string;
}

// biome-ignore-start lint/complexity/useLiteralKeys: environment
// biome-ignore lint/style/useNamingConvention: singleton
export const config: Config = {
	grpc_uri: env["GRPC_URI"] || "http://localhost:50051",
	http_port: Number(env["HTTP_PORT"] || 3000),
	redis_uri: env["REDIS_URI"] || "redis://localhost:6379",
};
// biome-ignore-end lint/complexity/useLiteralKeys: environment
