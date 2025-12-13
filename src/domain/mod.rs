//! Domain layer - Core business logic and entities
//! 
//! This module contains:
//! - Aggregates: Cluster of domain objects treated as a single unit
//! - Entities: Core domain objects with identity
//! - Value Objects: Immutable objects without identity
//! - Domain Events: Events that occur within the domain
//! - Repositories: Interfaces for data access (implemented in infrastructure)

pub mod aggregates;
pub mod entities;
pub mod events;
pub mod repositories;
pub mod value_objects;
pub mod errors;
