//! Symbolic Reduction Modeler
//!
//! Symbolic computation engine for algebraic reduction.

#![warn(missing_docs)]
#![warn(clippy::all)]

use shared_core::{Result, SystemError};

pub mod api;
pub mod config;
pub mod core;
pub mod optimization;
pub mod reduction;
pub mod symbolic;

/// Modeler configuration
#[derive(Debug, Clone)]
pub struct ModelerConfig {
    /// Enable optimizations
    pub optimize: bool,
}

impl Default for ModelerConfig {
    fn default() -> Self {
        Self { optimize: true }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = ModelerConfig::default();
        assert!(config.optimize);
    }
}
