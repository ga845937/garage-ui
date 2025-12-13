//! Garage API client
//!
//! 按照 Garage Admin API v2 分類組織

pub mod api;
pub mod client;
pub mod endpoints;
pub mod repositories;

pub use api::*;
pub use client::*;
pub use endpoints::*;
pub use repositories::*;
