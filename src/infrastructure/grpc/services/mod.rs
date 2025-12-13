//! gRPC service implementations
//!
//! This module contains all the gRPC service implementations that connect
//! the protobuf-generated service traits to the application layer handlers.

mod access_key_service;
mod block_service;
mod bucket_service;
mod cluster_service;
mod node_service;
mod object_service;
mod worker_service;

pub use access_key_service::AccessKeyGrpcService;
pub use block_service::BlockGrpcService;
pub use bucket_service::BucketGrpcService;
pub use cluster_service::ClusterGrpcService;
pub use node_service::NodeGrpcService;
pub use object_service::ObjectGrpcService;
pub use worker_service::WorkerGrpcService;
