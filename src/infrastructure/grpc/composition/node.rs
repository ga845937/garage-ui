//! Node Service Composition
//!
//! 負責組合 NodeGrpcService 及其所有 handlers

use std::sync::Arc;

use crate::infrastructure::garage::{GarageClient, GarageNodeRepository};
use crate::application::commands::node::handlers::{
    CreateMetadataSnapshotHandler, LaunchRepairHandler,
};
use crate::application::queries::node::handlers::{
    GetNodeInfoHandler, GetNodeStatisticsHandler,
};
use crate::infrastructure::grpc::services::NodeGrpcService;

/// Node Service 的依賴建構器
pub struct NodeServiceBuilder {
    client: GarageClient,
}

impl NodeServiceBuilder {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }

    pub fn build(self) -> NodeGrpcService {
        let repository = Arc::new(GarageNodeRepository::new(self.client));

        // Command Handlers
        let create_metadata_snapshot_handler = Arc::new(CreateMetadataSnapshotHandler::new(repository.clone()));
        let launch_repair_handler = Arc::new(LaunchRepairHandler::new(repository.clone()));

        // Query Handlers
        let get_node_info_handler = Arc::new(GetNodeInfoHandler::new(repository.clone()));
        let get_node_statistics_handler = Arc::new(GetNodeStatisticsHandler::new(repository));

        NodeGrpcService::new(
            create_metadata_snapshot_handler,
            launch_repair_handler,
            get_node_info_handler,
            get_node_statistics_handler,
        )
    }
}
