//! Get access key query

/// Query to get an access key by ID
#[derive(Debug, Clone)]
pub struct ReadKeyQuery {
    /// The access key ID
    pub id: String,
}

impl ReadKeyQuery {
    pub fn new(id: String) -> Self {
        Self { 
            id,
        }
    }
}
