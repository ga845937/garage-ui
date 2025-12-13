// import { OTLPTraceExporter } from "@opentelemetry/exporter-trace-otlp-http";
// import { GrpcInstrumentation } from "@opentelemetry/instrumentation-grpc";
// import { NodeSDK } from "@opentelemetry/sdk-node";
// import { SimpleSpanProcessor } from "@opentelemetry/sdk-trace-node";

// let sdk: NodeSDK | undefined;

// export function init_telemetry(): void {
// 	if (sdk) {
// 		return;
// 	}

// 	const trace_exporter = new OTLPTraceExporter({
// 		url:
// 			process.env.OTEL_EXPORTER_OTLP_ENDPOINT ||
// 			"http://localhost:4318/v1/traces",
// 	});

// 	sdk = new NodeSDK({
// 		instrumentations: [new GrpcInstrumentation()],
// 		serviceName: process.env.OTEL_SERVICE_NAME || "garage-ui-bff",
// 		spanProcessor: new SimpleSpanProcessor(trace_exporter),
// 	});

// 	sdk.start();
// 	console.log("🔭 OpenTelemetry initialized");

// 	process.on("SIGTERM", async () => {
// 		await sdk?.shutdown();
// 	});
// }
