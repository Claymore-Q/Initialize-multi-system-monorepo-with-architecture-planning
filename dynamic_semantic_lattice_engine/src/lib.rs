//! Dynamic Semantic Lattice Engine
//!
//! Semantic reasoning engine with lattice-based knowledge representation.

#![warn(missing_docs)]
#![warn(clippy::all)]

use shared_core::{Result, SystemError};

pub mod api;
pub mod config;
pub mod core;
pub mod lattice;
pub mod query;
pub mod reasoning;

/// Lattice engine configuration
#[derive(Debug, Clone)]
pub struct LatticeConfig {
    /// Maximum nodes
    pub max_nodes: usize,
}

impl Default for LatticeConfig {
    fn default() -> Self {
        Self { max_nodes: 10000 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = LatticeConfig::default();
        assert_eq!(config.max_nodes, 10000);
    }
}
