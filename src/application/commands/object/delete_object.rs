//! Delete Object Command

use crate::domain::errors::DomainError;

/// Command to delete a single object from a bucket
#[derive(Debug, Clone)]
pub struct DeleteObjectCommand {
    bucket: String,
    key: String,
}

impl DeleteObjectCommand {
    /// Create a new DeleteObjectCommand
    pub fn new(bucket: String, key: String) -> Result<Self, DomainError> {
        let command = Self { bucket, key };
        command.validate()?;
        Ok(command)
    }

    /// Validate the command
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
