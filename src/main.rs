//! garage-ui - Main entry point

use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

use garage_ui::infrastructure::{
    config::AppConfig,
    garage::GarageClient,
    grpc::GrpcServer,
    logging::init_logging,
};
use garage_ui::domain::events::{ChannelEventBus, EventProcessor, LoggingEventHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration first
    let config = AppConfig::from_env()?;
    
    // Initialize dual logging (console + file)
    // Guard must be kept alive for the duration of the program
    let _guard = init_logging(&config.log_dir);
    
    info!(
        "Starting garage-ui backend |\n garage_api_url: {} |\n grpc_server_addr: {} |\n log_dir: {} |\n s3_endpoint: {}",
        config.garage_api_url,
        config.grpc_server_addr,
        config.log_dir,
        config.s3_config.endpoint_url
    );

    // Create event bus and processor
    let (event_bus, receiver) = ChannelEventBus::new();
    let event_bus = Arc::new(event_bus);
    
    // Start event processor in background
    let event_processor = EventProcessor::new(vec![
        Box::new(LoggingEventHandler),
        // Add more event handlers here
    ]);
    tokio::spawn(async move {
        event_processor.run(receiver).await;
    });
    info!("Event processor started");

    // Create Garage client
    let garage_client = GarageClient::new(config.garage_api_url, config.garage_api_key);

    // Parse server address
    let addr: SocketAddr = config.grpc_server_addr.parse()?;

    // Create and run gRPC server with S3 config for object operations
    let server = GrpcServer::new(addr, garage_client, event_bus, config.s3_config);
    server.run().await?;

    Ok(())
}
