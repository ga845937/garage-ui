//! Worker domain events

use chrono::{DateTime, Utc};

/// Events related to worker operations
#[derive(Debug, Clone)]
pub enum WorkerEvent {
    VariableSet(WorkerVariableSetEvent),
}

#[derive(Debug, Clone)]
pub struct WorkerVariableSetEvent {
    pub node_id: String,
    pub variable: String,
    pub old_value: String,
    pub new_value: String,
    pub set_at: DateTime<Utc>,
}

impl WorkerVariableSetEvent {
    pub fn new(node_id: String, variable: String, old_value: String, new_value: String) -> Self {
        Self {
            node_id,
            variable,
            old_value,
            new_value,
            set_at: Utc::now(),
        }
    }
}
