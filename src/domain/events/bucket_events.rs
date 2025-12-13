//! Bucket domain events

use chrono::{DateTime, Utc};

/// Events related to bucket lifecycle
#[derive(Debug, Clone)]
pub enum BucketEvent {
    Created(BucketCreatedEvent),
    Updated(BucketUpdatedEvent),
    Deleted(BucketDeletedEvent),
    AliasAdded(BucketAliasAddedEvent),
    AliasRemoved(BucketAliasRemovedEvent),
    KeyAllowed(BucketKeyAllowedEvent),
    KeyDenied(BucketKeyDeniedEvent),
}

#[derive(Debug, Clone)]
pub struct BucketCreatedEvent {
    pub bucket_id: String,
    pub global_alias: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct BucketUpdatedEvent {
    pub bucket_id: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct BucketDeletedEvent {
    pub bucket_id: String,
    pub deleted_at: DateTime<Utc>,
}

impl BucketCreatedEvent {
    pub fn new(bucket_id: String, global_alias: Option<String>) -> Self {
        Self {
            bucket_id,
            global_alias,
            created_at: Utc::now(),
        }
    }
}

impl BucketUpdatedEvent {
    pub fn new(bucket_id: String) -> Self {
        Self {
            bucket_id,
            updated_at: Utc::now(),
        }
    }
}

impl BucketDeletedEvent {
    pub fn new(bucket_id: String) -> Self {
        Self {
            bucket_id,
            deleted_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BucketAliasAddedEvent {
    pub bucket_id: String,
    pub alias: String,
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct BucketAliasRemovedEvent {
    pub bucket_id: String,
    pub alias: String,
    pub removed_at: DateTime<Utc>,
}

impl BucketAliasAddedEvent {
    pub fn new(bucket_id: String, alias: String) -> Self {
        Self {
            bucket_id,
            alias,
            added_at: Utc::now(),
        }
    }
}

impl BucketAliasRemovedEvent {
    pub fn new(bucket_id: String, alias: String) -> Self {
        Self {
            bucket_id,
            alias,
            removed_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BucketKeyAllowedEvent {
    pub bucket_id: String,
    pub access_key_id: String,
    pub read: bool,
    pub write: bool,
    pub owner: bool,
    pub allowed_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct BucketKeyDeniedEvent {
    pub bucket_id: String,
    pub access_key_id: String,
    pub read: bool,
    pub write: bool,
    pub owner: bool,
    pub denied_at: DateTime<Utc>,
}

impl BucketKeyAllowedEvent {
    pub fn new(bucket_id: String, access_key_id: String, read: bool, write: bool, owner: bool) -> Self {
        Self {
            bucket_id,
            access_key_id,
            read,
            write,
            owner,
            allowed_at: Utc::now(),
        }
    }
}

impl BucketKeyDeniedEvent {
    pub fn new(bucket_id: String, access_key_id: String, read: bool, write: bool, owner: bool) -> Self {
        Self {
            bucket_id,
            access_key_id,
            read,
            write,
            owner,
            denied_at: Utc::now(),
        }
    }
}
