//! Event Bus for domain events

use async_trait::async_trait;
use tokio::sync::mpsc;
use super::{
    AccessKeyEvent, AdminTokenEvent, BlockEvent, BucketEvent, 
    ClusterEvent, NodeEvent, WorkerEvent,
};

/// 統一的領域事件類型
#[derive(Debug, Clone)]
pub enum DomainEvent {
    Bucket(BucketEvent),
    AccessKey(AccessKeyEvent),
    AdminToken(AdminTokenEvent),
    Cluster(ClusterEvent),
    Node(NodeEvent),
    Block(BlockEvent),
    Worker(WorkerEvent),
}

/// Event Bus trait for publishing domain events
#[async_trait]
pub trait EventBus: Send + Sync {
    /// Publish an event to all subscribers
    async fn publish(&self, event: DomainEvent);
    
    /// Publish a bucket event (convenience method)
    async fn publish_bucket(&self, event: BucketEvent) {
        self.publish(DomainEvent::Bucket(event)).await;
    }
    
    /// Publish an access key event (convenience method)
    async fn publish_access_key(&self, event: AccessKeyEvent) {
        self.publish(DomainEvent::AccessKey(event)).await;
    }
    
    /// Publish an admin token event (convenience method)
    async fn publish_admin_token(&self, event: AdminTokenEvent) {
        self.publish(DomainEvent::AdminToken(event)).await;
    }
    
    /// Publish a cluster event (convenience method)
    async fn publish_cluster(&self, event: ClusterEvent) {
        self.publish(DomainEvent::Cluster(event)).await;
    }
    
    /// Publish a node event (convenience method)
    async fn publish_node(&self, event: NodeEvent) {
        self.publish(DomainEvent::Node(event)).await;
    }
    
    /// Publish a block event (convenience method)
    async fn publish_block(&self, event: BlockEvent) {
        self.publish(DomainEvent::Block(event)).await;
    }
    
    /// Publish a worker event (convenience method)
    async fn publish_worker(&self, event: WorkerEvent) {
        self.publish(DomainEvent::Worker(event)).await;
    }
}

/// Channel-based Event Bus implementation
pub struct ChannelEventBus {
    sender: mpsc::UnboundedSender<DomainEvent>,
}

impl ChannelEventBus {
    /// Create a new EventBus with an unbounded channel
    pub fn new() -> (Self, mpsc::UnboundedReceiver<DomainEvent>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        (Self { sender }, receiver)
    }
}

impl Default for ChannelEventBus {
    fn default() -> Self {
        Self::new().0
    }
}

#[async_trait]
impl EventBus for ChannelEventBus {
    async fn publish(&self, event: DomainEvent) {
        // Using unbounded channel, send should not fail unless receiver is dropped
        // In production, you might want to log errors or use a dead letter queue
        let _ = self.sender.send(event);
    }
}
