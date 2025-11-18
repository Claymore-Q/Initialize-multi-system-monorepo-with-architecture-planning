//! Shared Core Library
//!
//! This crate provides common functionality used across all systems in the monorepo.
//!
//! # Modules
//!
//! - `error`: Unified error types and error handling utilities
//! - `logging`: Structured logging configuration and utilities
//! - `telemetry`: OpenTelemetry integration for distributed tracing and metrics
//! - `crypto`: Cryptographic primitives and utilities
//! - `config`: Configuration management and parsing
//! - `types`: Common types and traits used across systems

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]

pub mod config;
pub mod crypto;
pub mod error;
pub mod logging;
pub mod telemetry;
pub mod types;

// Re-export commonly used items
pub use error::{Result, SystemError};
pub use types::*;
