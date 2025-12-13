//! Garage Repositories Implementation
//!
//! Infrastructure 層的 Repository 具體實現
//! Repository trait 定義在 domain::repositories

pub mod access_key_repository;
pub mod admin_token_repository;
pub mod block_repository;
pub mod bucket_repository;
pub mod cluster_repository;
pub mod node_repository;
pub mod object_repository;
pub mod worker_repository;

// Re-export 具體實現（不是 trait，trait 在 domain 層）
pub use access_key_repository::{GarageAccessKeyCommandRepository, GarageAccessKeyQueryRepository};
pub use admin_token_repository::GarageAdminTokenRepository;
pub use block_repository::GarageBlockRepository;
pub use bucket_repository::GarageBucketRepository;
pub use cluster_repository::GarageClusterRepository;
pub use node_repository::GarageNodeRepository;
pub use object_repository::GarageObjectRepository;
pub use worker_repository::GarageWorkerRepository;
