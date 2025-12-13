//! Infrastructure layer - External services and implementations
//! 
//! This module contains:
//! - gRPC server implementation
//! - Garage API client
//! - S3 client for object operations
//! - Repository implementations
//! - Configuration
//! - Logging
//!
//! Note: `trace_id` and `context` have been moved to `crate::shared`

pub mod grpc;
pub mod garage;
pub mod s3;
pub mod config;
pub mod logging;
