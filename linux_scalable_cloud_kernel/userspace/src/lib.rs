//! Linux Scalable Cloud Kernel - Userspace Tools
//!
//! Userspace management tools for cloud-scale Linux kernel operations.

#![warn(missing_docs)]
#![warn(clippy::all)]

use shared_core::{Result, SystemError};

pub mod api;
pub mod config;
pub mod core;
pub mod monitoring;
pub mod orchestration;

/// Cloud kernel configuration
#[derive(Debug, Clone)]
pub struct CloudKernelConfig {
    /// Enable advanced features
    pub advanced_features: bool,
}

impl Default for CloudKernelConfig {
    fn default() -> Self {
        Self {
            advanced_features: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = CloudKernelConfig::default();
        assert!(!config.advanced_features);
    }
}
