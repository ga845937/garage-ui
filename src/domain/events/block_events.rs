//! Block domain events

use chrono::{DateTime, Utc};

/// Events related to block operations
#[derive(Debug, Clone)]
pub enum BlockEvent {
    Purged(BlocksPurgedEvent),
    ResyncRetried(BlockResyncRetriedEvent),
}

#[derive(Debug, Clone)]
pub struct BlocksPurgedEvent {
    pub node_id: String,
    pub blocks_purged: i32,
    pub objects_deleted: i32,
    pub uploads_deleted: i32,
    pub purged_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct BlockResyncRetriedEvent {
    pub node_id: String,
    pub blocks_retried: i32,
    pub retried_at: DateTime<Utc>,
}

impl BlocksPurgedEvent {
    pub fn new(node_id: String, blocks_purged: i32, objects_deleted: i32, uploads_deleted: i32) -> Self {
        Self {
            node_id,
            blocks_purged,
            objects_deleted,
            uploads_deleted,
            purged_at: Utc::now(),
        }
    }
}

impl BlockResyncRetriedEvent {
    pub fn new(node_id: String, blocks_retried: i32) -> Self {
        Self {
            node_id,
            blocks_retried,
            retried_at: Utc::now(),
        }
    }
}
