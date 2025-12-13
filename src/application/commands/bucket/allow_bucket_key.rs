//! Bucket key permission input types

/// Permissions to set on a bucket for a key
#[derive(Debug, Clone, Default)]
pub struct BucketKeyPermissionInput {
    pub read: bool,
    pub write: bool,
    pub owner: bool,
}
