//! Access Key domain events

use chrono::{DateTime, Utc};

/// Events related to access key lifecycle
#[derive(Debug, Clone)]
pub enum AccessKeyEvent {
    Created(AccessKeyCreatedEvent),
    Updated(AccessKeyUpdatedEvent),
    Deleted(AccessKeyDeletedEvent),
}

#[derive(Debug, Clone)]
pub struct AccessKeyCreatedEvent {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AccessKeyUpdatedEvent {
    pub id: String,
    pub name: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AccessKeyDeletedEvent {
    pub id: String,
    pub deleted_at: DateTime<Utc>,
}

impl AccessKeyCreatedEvent {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            created_at: Utc::now(),
        }
    }
}

impl AccessKeyUpdatedEvent {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            updated_at: Utc::now(),
        }
    }
}

impl AccessKeyDeletedEvent {
    pub fn new(id: String) -> Self {
        Self {
            id,
            deleted_at: Utc::now(),
        }
    }
}