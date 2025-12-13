import { ObjectServiceClientImpl } from "../generated/object";
import { grpc_adapter } from "./grpc-rpc.adapter";

// biome-ignore lint/style/useNamingConvention: singleton
export const object_client: ObjectServiceClientImpl =
	new ObjectServiceClientImpl(grpc_adapter);
