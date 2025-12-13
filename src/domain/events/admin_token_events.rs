//! Admin Token domain events

use chrono::{DateTime, Utc};

/// Events related to admin token lifecycle
#[derive(Debug, Clone)]
pub enum AdminTokenEvent {
    Created(AdminTokenCreatedEvent),
    Updated(AdminTokenUpdatedEvent),
    Deleted(AdminTokenDeletedEvent),
}

#[derive(Debug, Clone)]
pub struct AdminTokenCreatedEvent {
    pub token_id: String,
    pub name: Option<String>,
    pub scope: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AdminTokenUpdatedEvent {
    pub token_id: String,
    pub name: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AdminTokenDeletedEvent {
    pub token_id: String,
    pub deleted_at: DateTime<Utc>,
}

impl AdminTokenCreatedEvent {
    pub fn new(token_id: String, name: Option<String>, scope: Vec<String>) -> Self {
        Self {
            token_id,
            name,
            scope,
            created_at: Utc::now(),
        }
    }
}

impl AdminTokenUpdatedEvent {
    pub fn new(token_id: String, name: Option<String>) -> Self {
        Self {
            token_id,
            name,
            updated_at: Utc::now(),
        }
    }
}

impl AdminTokenDeletedEvent {
    pub fn new(token_id: String) -> Self {
        Self {
            token_id,
            deleted_at: Utc::now(),
        }
    }
}
