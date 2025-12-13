//! Event Handler trait and implementations

use async_trait::async_trait;
use super::{
    DomainEvent, BucketEvent, AccessKeyEvent, AdminTokenEvent,
    ClusterEvent, NodeEvent, BlockEvent, WorkerEvent,
};

/// Trait for handling domain events
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle a domain event
    async fn handle(&self, event: &DomainEvent);
}

/// Logging event handler - logs all events
pub struct LoggingEventHandler;

#[async_trait]
impl EventHandler for LoggingEventHandler {
    async fn handle(&self, event: &DomainEvent) {
        match event {
            DomainEvent::Bucket(e) => Self::handle_bucket_event(e),
            DomainEvent::AccessKey(e) => Self::handle_access_key_event(e),
            DomainEvent::AdminToken(e) => Self::handle_admin_token_event(e),
            DomainEvent::Cluster(e) => Self::handle_cluster_event(e),
            DomainEvent::Node(e) => Self::handle_node_event(e),
            DomainEvent::Block(e) => Self::handle_block_event(e),
            DomainEvent::Worker(e) => Self::handle_worker_event(e),
        }
    }
}

impl LoggingEventHandler {
    fn handle_bucket_event(event: &BucketEvent) {
        match event {
            BucketEvent::Created(e) => {
                tracing::info!(
                    "[INFO] Bucket created | bucket_id: {} | global_alias: {:?} | created_at: {}",
                    e.bucket_id,
                    e.global_alias,
                    e.created_at
                );
            }
            BucketEvent::Updated(e) => {
                tracing::info!(
                    "[INFO] Bucket updated | bucket_id: {} | updated_at: {}",
                    e.bucket_id,
                    e.updated_at
                );
            }
            BucketEvent::Deleted(e) => {
                tracing::info!(
                    "[INFO] Bucket deleted | bucket_id: {} | deleted_at: {}",
                    e.bucket_id,
                    e.deleted_at
                );
            }
            BucketEvent::AliasAdded(e) => {
                tracing::info!(
                    "[INFO] Bucket alias added | bucket_id: {} | alias: {} | added_at: {}",
                    e.bucket_id,
                    e.alias,
                    e.added_at
                );
            }
            BucketEvent::AliasRemoved(e) => {
                tracing::info!(
                    "[INFO] Bucket alias removed | bucket_id: {} | alias: {} | removed_at: {}",
                    e.bucket_id,
                    e.alias,
                    e.removed_at
                );
            }
            BucketEvent::KeyAllowed(e) => {
                tracing::info!(
                    "[INFO] Bucket key allowed | bucket_id: {} | access_key_id: {} | read: {} | write: {} | owner: {} | allowed_at: {}",
                    e.bucket_id,
                    e.access_key_id,
                    e.read,
                    e.write,
                    e.owner,
                    e.allowed_at
                );
            }
            BucketEvent::KeyDenied(e) => {
                tracing::info!(
                    "[INFO] Bucket key denied | bucket_id: {} | access_key_id: {} | read: {} | write: {} | owner: {} | denied_at: {}",
                    e.bucket_id,
                    e.access_key_id,
                    e.read,
                    e.write,
                    e.owner,
                    e.denied_at
                );
            }
        }
    }

    fn handle_access_key_event(event: &AccessKeyEvent) {
        match event {
            AccessKeyEvent::Created(e) => {
                tracing::info!(
                    "[INFO] Access Key created | key_id: {} | name: {:?} | created_at: {}",
                    e.id,
                    e.name,
                    e.created_at
                );
            }
            AccessKeyEvent::Updated(e) => {
                tracing::info!(
                    "[INFO] Access Key updated | key_id: {} | name: {:?} | updated_at: {}",
                    e.id,
                    e.name,
                    e.updated_at
                );
            }
            AccessKeyEvent::Deleted(e) => {
                tracing::info!(
                    "[INFO] Access Key deleted | key_id: {} | deleted_at: {}",
                    e.id,
                    e.deleted_at
                );
            }
        }
    }

    fn handle_admin_token_event(event: &AdminTokenEvent) {
        match event {
            AdminTokenEvent::Created(e) => {
                tracing::info!(
                    "[INFO] Admin Token created | token_id: {} | name: {:?} | scope: {:?} | created_at: {}",
                    e.token_id,
                    e.name,
                    e.scope,
                    e.created_at
                );
            }
            AdminTokenEvent::Updated(e) => {
                tracing::info!(
                    "[INFO] Admin Token updated | token_id: {} | name: {:?} | updated_at: {}",
                    e.token_id,
                    e.name,
                    e.updated_at
                );
            }
            AdminTokenEvent::Deleted(e) => {
                tracing::info!(
                    "[INFO] Admin Token deleted | token_id: {} | deleted_at: {}",
                    e.token_id,
                    e.deleted_at
                );
            }
        }
    }

    fn handle_cluster_event(event: &ClusterEvent) {
        match event {
            ClusterEvent::NodesConnected(e) => {
                tracing::info!(
                    "[INFO] Cluster nodes connected | nodes: {:?} | successful: {} | connected_at: {}",
                    e.node_addresses,
                    e.successful_count,
                    e.connected_at
                );
            }
            ClusterEvent::LayoutUpdated(e) => {
                tracing::info!(
                    "[INFO] Cluster layout updated | version: {} | updated_at: {}",
                    e.version,
                    e.updated_at
                );
            }
            ClusterEvent::LayoutApplied(e) => {
                tracing::info!(
                    "[INFO] Cluster layout applied | version: {} | message: {:?} | applied_at: {}",
                    e.version,
                    e.message,
                    e.applied_at
                );
            }
            ClusterEvent::LayoutReverted(e) => {
                tracing::info!(
                    "[INFO] Cluster layout reverted | version: {} | reverted_at: {}",
                    e.version,
                    e.reverted_at
                );
            }
            ClusterEvent::DeadNodesSkipped(e) => {
                tracing::warn!(
                    "[WARN] Cluster dead nodes skipped | version: {} | allow_missing_data: {} | skipped_at: {}",
                    e.version,
                    e.allow_missing_data,
                    e.skipped_at
                );
            }
        }
    }

    fn handle_node_event(event: &NodeEvent) {
        match event {
            NodeEvent::MetadataSnapshotCreated(e) => {
                tracing::info!(
                    "[INFO] Node metadata snapshot created | node_id: {} | created_at: {}",
                    e.node_id,
                    e.created_at
                );
            }
            NodeEvent::RepairLaunched(e) => {
                tracing::info!(
                    "[INFO] Node repair launched | node_id: {} | repair_type: {} | launched_at: {}",
                    e.node_id,
                    e.repair_type,
                    e.launched_at
                );
            }
        }
    }

    fn handle_block_event(event: &BlockEvent) {
        match event {
            BlockEvent::Purged(e) => {
                tracing::info!(
                    "[INFO] Blocks purged | node_id: {} | blocks: {} | objects: {} | uploads: {} | purged_at: {}",
                    e.node_id,
                    e.blocks_purged,
                    e.objects_deleted,
                    e.uploads_deleted,
                    e.purged_at
                );
            }
            BlockEvent::ResyncRetried(e) => {
                tracing::info!(
                    "[INFO] Block resync retried | node_id: {} | blocks_retried: {} | retried_at: {}",
                    e.node_id,
                    e.blocks_retried,
                    e.retried_at
                );
            }
        }
    }

    fn handle_worker_event(event: &WorkerEvent) {
        match event {
            WorkerEvent::VariableSet(e) => {
                tracing::info!(
                    "[INFO] Worker variable set | node_id: {} | variable: {} | old: {} | new: {} | set_at: {}",
                    e.node_id,
                    e.variable,
                    e.old_value,
                    e.new_value,
                    e.set_at
                );
            }
        }
    }
}

/// Event processor - processes events from a channel with multiple handlers
pub struct EventProcessor {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventProcessor {
    /// Create a new event processor with handlers
    pub fn new(handlers: Vec<Box<dyn EventHandler>>) -> Self {
        Self { handlers }
    }
    
    /// Process events from a receiver
    pub async fn run(self, mut receiver: tokio::sync::mpsc::UnboundedReceiver<DomainEvent>) {
        while let Some(event) = receiver.recv().await {
            // Process event with all handlers concurrently
            let futures: Vec<_> = self
                .handlers
                .iter()
                .map(|handler| handler.handle(&event))
                .collect();
            
            // Wait for all handlers to complete
            futures::future::join_all(futures).await;
        }
        
        tracing::info!("Event processor stopped - channel closed");
    }
}
