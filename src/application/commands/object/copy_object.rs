//! Copy Object Command

use crate::domain::errors::DomainError;

/// Command to copy an object from one location to another
#[derive(Debug, Clone)]
pub struct CopyObjectCommand {
    source_bucket: String,
    source_key: String,
    dest_bucket: String,
    dest_key: String,
}

impl CopyObjectCommand {
    /// Create a new CopyObjectCommand
    pub fn new(
        source_bucket: String,
        source_key: String,
        dest_bucket: String,
        dest_key: String,
    ) -> Result<Self, DomainError> {
        let command = Self {
            source_bucket,
            source_key,
            dest_bucket,
            dest_key,
        };
        command.validate()?;
        Ok(command)
    }

    /// Validate the command
    fn validate(&self) -> Result<(), DomainError> {
        if self.source_bucket.is_empty() {
            return Err(DomainError::ValidationError(
                "Source bucket name is required".to_string(),
            ));
        }
        if self.source_key.is_empty() {
            return Err(DomainError::ValidationError(
                "Source object key is required".to_string(),
            ));
        }
        if self.dest_bucket.is_empty() {
            return Err(DomainError::ValidationError(
                "Destination bucket name is required".to_string(),
            ));
        }
        if self.dest_key.is_empty() {
            return Err(DomainError::ValidationError(
                "Destination object key is required".to_string(),
            ));
        }
        Ok(())
    }

    /// Get the source bucket name
    pub fn source_bucket(&self) -> &str {
        &self.source_bucket
    }

    /// Get the source object key
    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    /// Get the destination bucket name
    pub fn dest_bucket(&self) -> &str {
        &self.dest_bucket
    }

    /// Get the destination object key
    pub fn dest_key(&self) -> &str {
        &self.dest_key
    }
}
