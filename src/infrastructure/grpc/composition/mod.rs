//! Service Composition
//!
//! 負責組合各個 gRPC Service，將 DI (Dependency Injection) 邏輯
//! 從 server.rs 分離出來，使 server 專注於啟動與路由配置

mod access_key;
mod bucket;
mod cluster;
mod node;
mod block;
mod object;
mod worker;

pub use access_key::AccessKeyServiceBuilder;
pub use bucket::BucketServiceBuilder;
pub use cluster::ClusterServiceBuilder;
pub use node::NodeServiceBuilder;
pub use block::BlockServiceBuilder;
pub use object::ObjectServiceBuilder;
pub use worker::WorkerServiceBuilder;
