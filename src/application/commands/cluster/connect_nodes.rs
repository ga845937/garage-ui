//! Connect nodes command

/// Command to connect nodes to the cluster
#[derive(Debug, Clone)]
pub struct ConnectNodesCommand {
    /// Node addresses to connect (format: node_id@address:port)
    pub node_addresses: Vec<String>,
}

impl ConnectNodesCommand {
    pub fn new(node_addresses: Vec<String>) -> Self {
        Self { node_addresses }
    }
}
