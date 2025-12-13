import { AccessKeyServiceClientImpl } from "../generated/access_key";
import { grpc_adapter } from "./grpc-rpc.adapter";

// biome-ignore lint/style/useNamingConvention: singleton
export const access_key_client: AccessKeyServiceClientImpl =
	new AccessKeyServiceClientImpl(grpc_adapter);
