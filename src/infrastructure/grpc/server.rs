//! gRPC server

use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;
use tracing::info;

use crate::domain::events::EventBus;
use crate::infrastructure::config::S3Config;
use crate::infrastructure::garage::GarageClient;

use super::generated::bucket::bucket_service_server::BucketServiceServer;
use super::generated::access_key::access_key_service_server::AccessKeyServiceServer;
use super::generated::cluster::cluster_service_server::ClusterServiceServer;
use super::generated::node::node_service_server::NodeServiceServer;
use super::generated::block::block_service_server::BlockServiceServer;
use super::generated::object::object_service_server::ObjectServiceServer;
use super::generated::worker::worker_service_server::WorkerServiceServer;

use super::composition::{
    AccessKeyServiceBuilder, BucketServiceBuilder, ClusterServiceBuilder,
    NodeServiceBuilder, BlockServiceBuilder, ObjectServiceBuilder, WorkerServiceBuilder,
};
use super::middleware::LoggingLayer;

pub struct GrpcServer {
    addr: SocketAddr,
    garage_client: GarageClient,
    event_bus: Arc<dyn EventBus>,
    s3_config: S3Config,
}

impl GrpcServer {
    pub fn new(
        addr: SocketAddr,
        garage_client: GarageClient,
        event_bus: Arc<dyn EventBus>,
        s3_config: S3Config,
    ) -> Self {
        Self {
            addr,
            garage_client,
            event_bus,
            s3_config,
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let bucket_service = BucketServiceBuilder::new(
            self.garage_client.clone(),
            self.event_bus.clone(),
        ).build();

        let access_key_service = AccessKeyServiceBuilder::new(self.garage_client.clone()).build();

        let cluster_service = ClusterServiceBuilder::new(self.garage_client.clone()).build();

        let node_service = NodeServiceBuilder::new(self.garage_client.clone()).build();

        let block_service = BlockServiceBuilder::new(self.garage_client.clone()).build();

        let worker_service = WorkerServiceBuilder::new(self.garage_client.clone()).build();

        let object_service = ObjectServiceBuilder::new(self.s3_config).build().await;

        info!(
            "Starting gRPC server |\n addr: {}",
            self.addr
        );        

        Server::builder()
            .layer(LoggingLayer)
            .add_service(BucketServiceServer::new(bucket_service))
            .add_service(AccessKeyServiceServer::new(access_key_service))
            .add_service(ClusterServiceServer::new(cluster_service))
            .add_service(NodeServiceServer::new(node_service))
            .add_service(BlockServiceServer::new(block_service))
            .add_service(ObjectServiceServer::new(object_service))
            .add_service(WorkerServiceServer::new(worker_service))
            .serve(self.addr)
            .await?;

        Ok(())
    }
}
