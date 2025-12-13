//! Connect nodes command handler

use std::sync::Arc;
use crate::application::commands::cluster::ConnectNodesCommand;
use crate::domain::entities::ConnectNodeResult;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ClusterRepository;

/// Handler for connecting nodes to the cluster
pub struct ConnectNodesHandler {
    repository: Arc<dyn ClusterRepository>,
}

impl ConnectNodesHandler {
    pub fn new(repository: Arc<dyn ClusterRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: ConnectNodesCommand) -> Result<Vec<ConnectNodeResult>, DomainError> {
        self.repository.connect_nodes(command.node_addresses).await
    }
}
