//! Get Object Metadata Query

use crate::domain::errors::DomainError;

/// Query to get object metadata (HEAD request)
#[derive(Debug, Clone)]
pub struct GetObjectMetadataQuery {
    bucket: String,
    key: String,
}

impl GetObjectMetadataQuery {
    /// Create a new GetObjectMetadataQuery
    pub fn new(bucket: String, key: String) -> Result<Self, DomainError> {
        let query = Self { bucket, key };
        query.validate()?;
        Ok(query)
    }

    /// Validate the query
    fn validate(&self) -> Result<(), DomainError> {
        if self.bucket.is_empty() {
            return Err(DomainError::ValidationError(
                "Bucket name is required".to_string(),
            ));
        }
        if self.key.is_empty() {
            return Err(DomainError::ValidationError(
                "Object key is required".to_string(),
            ));
        }
        Ok(())
    }

    /// Get the bucket name
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Get the object key
    pub fn key(&self) -> &str {
        &self.key
    }
}
