//! Update layout command handler

use std::sync::Arc;
use crate::application::commands::cluster::UpdateLayoutCommand;
use crate::domain::entities::ClusterLayout;
use crate::domain::errors::DomainError;
use crate::domain::repositories::{ClusterRepository, UpdateLayoutInput};

/// Handler for updating cluster layout
pub struct UpdateLayoutHandler {
    repository: Arc<dyn ClusterRepository>,
}

impl UpdateLayoutHandler {
    pub fn new(repository: Arc<dyn ClusterRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: UpdateLayoutCommand) -> Result<ClusterLayout, DomainError> {
        let roles: Vec<UpdateLayoutInput> = command.role_changes.into_iter().map(|rc| {
            UpdateLayoutInput {
                node_id: rc.node_id,
                zone: rc.zone,
                capacity: rc.capacity.map(|c| c as u64),
                tags: rc.tags,
            }
        }).collect();
        
        self.repository.update_layout(roles).await
    }
}
