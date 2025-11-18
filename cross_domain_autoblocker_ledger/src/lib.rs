//! Cross-Domain Autoblocker Ledger
//!
//! Distributed ledger for cross-domain security event blocking.

#![warn(missing_docs)]
#![warn(clippy::all)]

use shared_core::{Result, SystemError};

pub mod api;
pub mod blocking;
pub mod config;
pub mod consensus;
pub mod core;
pub mod ledger;

/// Ledger configuration
#[derive(Debug, Clone)]
pub struct LedgerConfig {
    /// Consensus timeout in milliseconds
    pub consensus_timeout_ms: u64,
}

impl Default for LedgerConfig {
    fn default() -> Self {
        Self {
            consensus_timeout_ms: 5000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = LedgerConfig::default();
        assert_eq!(config.consensus_timeout_ms, 5000);
    }
}
