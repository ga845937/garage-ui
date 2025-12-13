//! Import access key command

/// Command to import an existing access key
#[derive(Debug, Clone)]
pub struct ImportKeyCommand {
    /// Optional name for the key
    pub name: Option<String>,
    /// The access key ID
    pub access_key_id: String,
    /// The secret access key
    pub secret_access_key: String,
}
