//! Cluster domain events

use chrono::{DateTime, Utc};

/// Events related to cluster lifecycle
#[derive(Debug, Clone)]
pub enum ClusterEvent {
    NodesConnected(ClusterNodesConnectedEvent),
    LayoutUpdated(ClusterLayoutUpdatedEvent),
    LayoutApplied(ClusterLayoutAppliedEvent),
    LayoutReverted(ClusterLayoutRevertedEvent),
    DeadNodesSkipped(ClusterDeadNodesSkippedEvent),
}

#[derive(Debug, Clone)]
pub struct ClusterNodesConnectedEvent {
    pub node_addresses: Vec<String>,
    pub successful_count: usize,
    pub connected_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ClusterLayoutUpdatedEvent {
    pub version: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ClusterLayoutAppliedEvent {
    pub version: i64,
    pub message: Vec<String>,
    pub applied_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ClusterLayoutRevertedEvent {
    pub version: i64,
    pub reverted_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ClusterDeadNodesSkippedEvent {
    pub version: i64,
    pub allow_missing_data: bool,
    pub skipped_at: DateTime<Utc>,
}

impl ClusterNodesConnectedEvent {
    pub fn new(node_addresses: Vec<String>, successful_count: usize) -> Self {
        Self {
            node_addresses,
            successful_count,
            connected_at: Utc::now(),
        }
    }
}

impl ClusterLayoutUpdatedEvent {
    pub fn new(version: i64) -> Self {
        Self {
            version,
            updated_at: Utc::now(),
        }
    }
}

impl ClusterLayoutAppliedEvent {
    pub fn new(version: i64, message: Vec<String>) -> Self {
        Self {
            version,
            message,
            applied_at: Utc::now(),
        }
    }
}

impl ClusterLayoutRevertedEvent {
    pub fn new(version: i64) -> Self {
        Self {
            version,
            reverted_at: Utc::now(),
        }
    }
}

impl ClusterDeadNodesSkippedEvent {
    pub fn new(version: i64, allow_missing_data: bool) -> Self {
        Self {
            version,
            allow_missing_data,
            skipped_at: Utc::now(),
        }
    }
}
