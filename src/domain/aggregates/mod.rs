//! Aggregates module
//!
//! 聚合根 - DDD 核心概念
//! 每個 Aggregate 負責維護其內部實體的一致性和業務規則

mod access_key;
mod admin_token;
mod bucket;
mod cluster;

pub use access_key::{AccessKeyAggregate, BucketVO, BucketPermissionVO};
pub use admin_token::{AdminTokenAggregate, AdminTokenScope};
pub use bucket::BucketAggregate;
pub use cluster::{ClusterAggregate, NodeRoleConfig};
