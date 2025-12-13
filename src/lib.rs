//! garage-ui - A Rust backend for Garage storage management
//! 
//! Architecture: DDD (Domain-Driven Design) + CQRS
//! Protocol: gRPC

pub mod shared;
pub mod domain;
pub mod application;
pub mod infrastructure;
