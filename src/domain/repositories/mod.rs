//! Repository interfaces
//! 
//! Abstract interfaces for data access, implemented in infrastructure layer
//! 
//! Domain 層定義 Repository trait（抽象介面）
//! Infrastructure 層提供具體實現

pub mod access_key_repository;
pub mod admin_token_repository;
pub mod block_repository;
pub mod bucket_repository;
pub mod cluster_repository;
pub mod node_repository;
pub mod object_repository;
pub mod worker_repository;

pub use access_key_repository::*;
pub use admin_token_repository::*;
pub use block_repository::*;
pub use bucket_repository::*;
pub use cluster_repository::*;
pub use node_repository::*;
pub use object_repository::*;
pub use worker_repository::*;
