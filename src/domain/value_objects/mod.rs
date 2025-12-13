//! Value Objects module
//!
//! Domain 專用的 Value Objects
//! 

mod alias;
mod quotas;

pub use alias::{GlobalAlias, LocalAlias};
pub use quotas::Quotas;
