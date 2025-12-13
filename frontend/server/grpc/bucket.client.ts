import { BucketServiceClientImpl } from "../generated/bucket";
import { grpc_adapter } from "./grpc-rpc.adapter";

// biome-ignore lint/style/useNamingConvention: singleton
export const bucket_client: BucketServiceClientImpl =
	new BucketServiceClientImpl(grpc_adapter);
