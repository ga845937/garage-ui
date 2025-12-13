//! Delete access key command

use futures::future::try_join_all;

use crate::domain::errors::DomainError;
use crate::domain::aggregates::AccessKeyAggregate;

/// Command to delete an access key
#[derive(Debug, Clone)]
pub struct DeleteKeyCommand {
    id: Vec<String>,
}

impl DeleteKeyCommand {
    pub fn new(id: Vec<String>) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &Vec<String> {
        &self.id
    }

    pub async fn validate(&self) -> Result<(), DomainError> {
        let task = self.id.iter().map(|key_id| {
            let key_id = key_id.clone();

            async move {
                AccessKeyAggregate::validate_id(key_id.as_str())
            }
        });

        try_join_all(task).await?;

        Ok(())
    }
}
