//! Delete Objects Command (Batch)

use crate::domain::errors::DomainError;

/// Command to delete multiple objects from a bucket
#[derive(Debug, Clone)]
pub struct DeleteObjectsCommand {
    bucket: String,
    keys: Vec<String>,
}

impl DeleteObjectsCommand {
    /// Create a new DeleteObjectsCommand
    pub fn new(bucket: String, keys: Vec<String>) -> Result<Self, DomainError> {
        let command = Self { bucket, keys };
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
        if self.keys.is_empty() {
            return Err(DomainError::ValidationError(
                "At least one object key is required".to_string(),
            ));
        }
        // Check for empty keys
        for key in &self.keys {
            if key.is_empty() {
                return Err(DomainError::ValidationError(
                    "Object key cannot be empty".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Get the bucket name
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Get the object keys
    pub fn keys(&self) -> &[String] {
        &self.keys
    }

    /// Consume and return the keys
    pub fn into_keys(self) -> Vec<String> {
        self.keys
    }
}
