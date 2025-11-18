//! Synthetic Pipeline Engine
//!
//! Data pipeline orchestration with synthetic data generation.

#![warn(missing_docs)]
#![warn(clippy::all)]

use shared_core::{Result, SystemError};

pub mod api;
pub mod config;
pub mod core;
pub mod generators;
pub mod pipeline;
pub mod transformers;

/// Pipeline configuration
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Maximum pipeline stages
    pub max_stages: usize,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self { max_stages: 100 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = PipelineConfig::default();
        assert_eq!(config.max_stages, 100);
    }
}
