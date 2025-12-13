//! Cluster Service Composition
//!
//! 負責組合 ClusterGrpcService 及其所有 handlers

use std::sync::Arc;

use crate::infrastructure::garage::{GarageClient, GarageClusterRepository};
use crate::application::commands::cluster::handlers::{
    ConnectNodesHandler, UpdateLayoutHandler, ApplyLayoutHandler,
    RevertLayoutHandler, SkipDeadNodesHandler,
};
use crate::application::queries::cluster::handlers::{
    GetClusterStatusHandler, GetClusterHealthHandler, GetClusterLayoutHandler,
    GetLayoutHistoryHandler, PreviewLayoutChangesHandler,
};
use crate::infrastructure::grpc::services::ClusterGrpcService;

/// Cluster Service 的依賴建構器
pub struct ClusterServiceBuilder {
    client: GarageClient,
}

impl ClusterServiceBuilder {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }

    pub fn build(self) -> ClusterGrpcService {
        let repository = Arc::new(GarageClusterRepository::new(self.client));

        // Command Handlers
        let connect_nodes_handler = Arc::new(ConnectNodesHandler::new(repository.clone()));
        let update_layout_handler = Arc::new(UpdateLayoutHandler::new(repository.clone()));
        let apply_layout_handler = Arc::new(ApplyLayoutHandler::new(repository.clone()));
        let revert_layout_handler = Arc::new(RevertLayoutHandler::new(repository.clone()));
        let skip_dead_nodes_handler = Arc::new(SkipDeadNodesHandler::new(repository.clone()));

        // Query Handlers
        let get_cluster_status_handler = Arc::new(GetClusterStatusHandler::new(repository.clone()));
        let get_cluster_health_handler = Arc::new(GetClusterHealthHandler::new(repository.clone()));
        let get_cluster_layout_handler = Arc::new(GetClusterLayoutHandler::new(repository.clone()));
        let get_layout_history_handler = Arc::new(GetLayoutHistoryHandler::new(repository.clone()));
        let preview_layout_changes_handler = Arc::new(PreviewLayoutChangesHandler::new(repository));

        ClusterGrpcService::new(
            connect_nodes_handler,
            update_layout_handler,
            apply_layout_handler,
            revert_layout_handler,
            skip_dead_nodes_handler,
            get_cluster_status_handler,
            get_cluster_health_handler,
            get_cluster_layout_handler,
            get_layout_history_handler,
            preview_layout_changes_handler,
        )
    }
}
