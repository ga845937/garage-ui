//! List Objects Query

use crate::domain::errors::DomainError;

/// Query to list objects in a bucket
#[derive(Debug, Clone)]
pub struct ListObjectsQuery {
    bucket: String,
    prefix: Option<String>,
    continuation_token: Option<String>,
    max_keys: Option<i32>,
}

impl ListObjectsQuery {
    /// Create a new ListObjectsQuery
    pub fn new(
        bucket: String,
        prefix: Option<String>,
        continuation_token: Option<String>,
        max_keys: Option<i32>,
    ) -> Result<Self, DomainError> {
        let query = Self {
            bucket,
            prefix,
            continuation_token,
            max_keys,
        };
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
        if let Some(max) = self.max_keys {
            if max <= 0 || max > 1000 {
                return Err(DomainError::ValidationError(
                    "max_keys must be between 1 and 1000".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Get the bucket name
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Get the prefix filter
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    /// Get the continuation token
    pub fn continuation_token(&self) -> Option<&str> {
        self.continuation_token.as_deref()
    }

    /// Get the max keys
    pub fn max_keys(&self) -> Option<i32> {
        self.max_keys
    }
}
