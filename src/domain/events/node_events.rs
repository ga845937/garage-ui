//! Node domain events

use chrono::{DateTime, Utc};

/// Events related to node operations
#[derive(Debug, Clone)]
pub enum NodeEvent {
    MetadataSnapshotCreated(NodeMetadataSnapshotCreatedEvent),
    RepairLaunched(NodeRepairLaunchedEvent),
}

#[derive(Debug, Clone)]
pub struct NodeMetadataSnapshotCreatedEvent {
    pub node_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NodeRepairLaunchedEvent {
    pub node_id: String,
    pub repair_type: String,
    pub launched_at: DateTime<Utc>,
}

impl NodeMetadataSnapshotCreatedEvent {
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            created_at: Utc::now(),
        }
    }
}

impl NodeRepairLaunchedEvent {
    pub fn new(node_id: String, repair_type: String) -> Self {
        Self {
            node_id,
            repair_type,
            launched_at: Utc::now(),
        }
    }
}
