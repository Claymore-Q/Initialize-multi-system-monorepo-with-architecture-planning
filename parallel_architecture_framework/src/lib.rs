//! Parallel Architecture Framework
//!
//! Framework for distributed computation orchestration.

#![warn(missing_docs)]
#![warn(clippy::all)]

use shared_core::{Result, SystemError};

pub mod api;
pub mod communication;
pub mod config;
pub mod core;
pub mod executor;
pub mod scheduler;

/// Framework configuration
#[derive(Debug, Clone)]
pub struct FrameworkConfig {
    /// Number of worker threads
    pub workers: usize,
}

impl Default for FrameworkConfig {
    fn default() -> Self {
        Self {
            workers: num_cpus::get(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = FrameworkConfig::default();
        assert!(config.workers > 0);
    }
}
