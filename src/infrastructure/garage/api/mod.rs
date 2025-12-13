//! Garage API types
//!
//! 按照 Garage Admin API v2 分類組織

pub mod access_key;
pub mod admin_token;
pub mod bucket;
pub mod block;
pub mod cluster;
pub mod node;
pub mod worker;

pub use access_key::*;
pub use admin_token::*;
pub use bucket::*;
pub use block::*;
pub use cluster::*;
pub use node::*;
pub use worker::*;
