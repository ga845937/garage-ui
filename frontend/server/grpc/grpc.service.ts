class GrpcService {
	public to_null_able<T>(
		value: T | undefined,
	): { value: T } | { value: undefined } {
		return value ? { value } : { value: undefined };
	}
}

// biome-ignore lint/style/useNamingConvention: singleton
export const grpc_service: GrpcService = new GrpcService();
