//! Cross-Domain Auto Learner
//!
//! Adaptive machine learning system for pattern recognition across domains.

#![warn(missing_docs)]
#![warn(clippy::all)]

use shared_core::{Result, SystemError};

pub mod api;
pub mod config;
pub mod core;
pub mod inference;
pub mod models;
pub mod training;

/// Auto learner configuration
#[derive(Debug, Clone)]
pub struct AutoLearnerConfig {
    /// Model type
    pub model_type: String,
}

impl Default for AutoLearnerConfig {
    fn default() -> Self {
        Self {
            model_type: "default".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = AutoLearnerConfig::default();
        assert_eq!(config.model_type, "default");
    }
}
