//! Resource Governor
//!
//! Provides resource management and throttling capabilities for all systems.
//! Supports CPU caps, RAM limits, I/O throttling, deterministic mode, and sandbox mode.

use crate::{Result, SystemError};
use rand::{RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Notify, Semaphore, RwLock};
use tokio::time::sleep;

/// Zero-allocation RNG wrapper for deterministic and non-deterministic modes
pub enum GovernorRng {
    /// Deterministic RNG with fixed seed
    Deterministic(rand::rngs::StdRng),
    /// Non-deterministic thread RNG
    Random(rand::rngs::ThreadRng),
}

impl RngCore for GovernorRng {
    fn next_u32(&mut self) -> u32 {
        match self {
            GovernorRng::Deterministic(rng) => rng.next_u32(),
            GovernorRng::Random(rng) => rng.next_u32(),
        }
    }

    fn next_u64(&mut self) -> u64 {
        match self {
            GovernorRng::Deterministic(rng) => rng.next_u64(),
            GovernorRng::Random(rng) => rng.next_u64(),
        }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        match self {
            GovernorRng::Deterministic(rng) => rng.fill_bytes(dest),
            GovernorRng::Random(rng) => rng.fill_bytes(dest),
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> std::result::Result<(), rand::Error> {
        match self {
            GovernorRng::Deterministic(rng) => rng.try_fill_bytes(dest),
            GovernorRng::Random(rng) => rng.try_fill_bytes(dest),
        }
    }
}

/// Resource governor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceGovernorConfig {
    /// Maximum CPU percentage (0-100), None = unlimited
    pub cpu_cap_percent: Option<u8>,

    /// Maximum RAM usage in bytes, None = unlimited
    pub ram_cap_bytes: Option<u64>,

    /// Maximum I/O operations per second, None = unlimited
    pub io_ops_per_second: Option<u64>,

    /// Enable deterministic execution mode (fixed seeds, ordered execution)
    pub deterministic_mode: bool,

    /// Enable sandbox mode (restricted system access)
    pub sandbox_mode: bool,

    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,
}

impl Default for ResourceGovernorConfig {
    fn default() -> Self {
        Self {
            cpu_cap_percent: None,
            ram_cap_bytes: None,
            io_ops_per_second: None,
            deterministic_mode: false,
            sandbox_mode: false,
            max_concurrent_operations: 1000,
        }
    }
}

impl ResourceGovernorConfig {
    /// Create a configuration for testing with strict limits
    pub fn testing() -> Self {
        Self {
            cpu_cap_percent: Some(50),
            ram_cap_bytes: Some(512 * 1024 * 1024), // 512MB
            io_ops_per_second: Some(100),
            deterministic_mode: true,
            sandbox_mode: true,
            max_concurrent_operations: 10,
        }
    }

    /// Create a configuration for production with moderate limits
    pub fn production() -> Self {
        Self {
            cpu_cap_percent: Some(80),
            ram_cap_bytes: Some(4 * 1024 * 1024 * 1024), // 4GB
            io_ops_per_second: Some(10000),
            deterministic_mode: false,
            sandbox_mode: false,
            max_concurrent_operations: 1000,
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if let Some(cpu) = self.cpu_cap_percent {
            if cpu > 100 {
                return Err(SystemError::Config {
                    message: "CPU cap percentage must be <= 100".into(),
                    key: Some("cpu_cap_percent".into()),
                });
            }
        }

        if self.max_concurrent_operations == 0 {
            return Err(SystemError::Config {
                message: "max_concurrent_operations must be > 0".into(),
                key: Some("max_concurrent_operations".into()),
            });
        }

        Ok(())
    }
}

/// Resource governor for managing and throttling system resources
pub struct ResourceGovernor {
    config: ResourceGovernorConfig,

    // CPU tracking
    cpu_usage_percent: Arc<AtomicU64>,
    last_cpu_check: Arc<RwLock<Instant>>,

    // RAM tracking
    ram_usage_bytes: Arc<AtomicU64>,

    // I/O throttling
    io_ops_count: Arc<AtomicU64>,
    io_window_start: Arc<RwLock<Instant>>,

    // Concurrency control
    operation_semaphore: Arc<Semaphore>,

    // State
    is_paused: Arc<AtomicBool>,
    pause_notify: Arc<Notify>,
    total_operations: Arc<AtomicU64>,
    throttled_operations: Arc<AtomicU64>,
}

impl ResourceGovernor {
    /// Create a new resource governor
    pub fn new(config: ResourceGovernorConfig) -> Result<Self> {
        config.validate()?;

        Ok(Self {
            operation_semaphore: Arc::new(Semaphore::new(config.max_concurrent_operations)),
            config,
            cpu_usage_percent: Arc::new(AtomicU64::new(0)),
            last_cpu_check: Arc::new(RwLock::new(Instant::now())),
            ram_usage_bytes: Arc::new(AtomicU64::new(0)),
            io_ops_count: Arc::new(AtomicU64::new(0)),
            io_window_start: Arc::new(RwLock::new(Instant::now())),
            is_paused: Arc::new(AtomicBool::new(false)),
            pause_notify: Arc::new(Notify::new()),
            total_operations: Arc::new(AtomicU64::new(0)),
            throttled_operations: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Acquire a permit to execute an operation
    pub async fn acquire_permit(&self) -> Result<OperationPermit> {
        self.total_operations.fetch_add(1, Ordering::Relaxed);

        // Efficient pause handling using Notify instead of busy-wait
        while self.is_paused.load(Ordering::Relaxed) {
            self.pause_notify.notified().await;
        }

        // Acquire concurrency permit
        let permit = self
            .operation_semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| SystemError::Concurrency {
                message: format!("Failed to acquire permit: {}", e),
                thread_id: None,
            })?;

        // Check CPU throttling
        if let Some(cpu_cap) = self.config.cpu_cap_percent {
            let current_cpu = self.cpu_usage_percent.load(Ordering::Relaxed);
            if current_cpu > u64::from(cpu_cap) {
                self.throttled_operations.fetch_add(1, Ordering::Relaxed);
                let sleep_duration = Duration::from_millis(10);
                sleep(sleep_duration).await;
            }
        }

        // Check RAM limit
        if let Some(ram_cap) = self.config.ram_cap_bytes {
            let current_ram = self.ram_usage_bytes.load(Ordering::Relaxed);
            if current_ram > ram_cap {
                return Err(SystemError::Validation {
                    field: "ram_usage".into(),
                    reason: format!(
                        "RAM limit exceeded: {} bytes > {} bytes cap",
                        current_ram, ram_cap
                    ),
                    value: Some(current_ram.to_string()),
                });
            }
        }

        Ok(OperationPermit {
            _permit: permit,
            governor: self.clone(),
            start_time: Instant::now(),
        })
    }

    /// Throttle I/O operation if needed
    pub async fn throttle_io(&self) -> Result<()> {
        if let Some(ops_limit) = self.config.io_ops_per_second {
            let mut window_start = self.io_window_start.write().await;
            let elapsed = window_start.elapsed();

            // Reset window if 1 second has passed
            if elapsed >= Duration::from_secs(1) {
                self.io_ops_count.store(0, Ordering::Relaxed);
                *window_start = Instant::now();
            } else {
                let current_ops = self.io_ops_count.fetch_add(1, Ordering::Relaxed);

                if current_ops >= ops_limit {
                    // Sleep until next window
                    let sleep_duration = Duration::from_secs(1) - elapsed;
                    self.throttled_operations.fetch_add(1, Ordering::Relaxed);
                    sleep(sleep_duration).await;

                    // Reset for new window
                    self.io_ops_count.store(1, Ordering::Relaxed);
                    *window_start = Instant::now();
                }
            }
        }

        Ok(())
    }

    /// Update CPU usage percentage
    pub fn update_cpu_usage(&self, percent: u8) {
        self.cpu_usage_percent
            .store(u64::from(percent.min(100)), Ordering::Relaxed);
    }

    /// Track RAM allocation
    pub fn track_ram_allocation(&self, bytes: u64) {
        self.ram_usage_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Track RAM deallocation
    pub fn track_ram_deallocation(&self, bytes: u64) {
        self.ram_usage_bytes.fetch_sub(bytes, Ordering::Relaxed);
    }

    /// Get current RAM usage
    pub fn current_ram_usage(&self) -> u64 {
        self.ram_usage_bytes.load(Ordering::Relaxed)
    }

    /// Get current CPU usage
    pub fn current_cpu_usage(&self) -> u64 {
        self.cpu_usage_percent.load(Ordering::Relaxed)
    }

    /// Pause all operations
    pub fn pause(&self) {
        self.is_paused.store(true, Ordering::Relaxed);
    }

    /// Resume operations
    pub fn resume(&self) {
        self.is_paused.store(false, Ordering::Relaxed);
        // Notify all waiting tasks that we've resumed
        self.pause_notify.notify_waiters();
    }

    /// Check if in deterministic mode
    pub fn is_deterministic(&self) -> bool {
        self.config.deterministic_mode
    }

    /// Check if in sandbox mode
    pub fn is_sandboxed(&self) -> bool {
        self.config.sandbox_mode
    }

    /// Get statistics
    pub fn statistics(&self) -> GovernorStatistics {
        GovernorStatistics {
            total_operations: self.total_operations.load(Ordering::Relaxed),
            throttled_operations: self.throttled_operations.load(Ordering::Relaxed),
            current_cpu_usage: self.cpu_usage_percent.load(Ordering::Relaxed),
            current_ram_usage: self.ram_usage_bytes.load(Ordering::Relaxed),
            is_paused: self.is_paused.load(Ordering::Relaxed),
        }
    }

    /// Reset statistics
    pub fn reset_statistics(&self) {
        self.total_operations.store(0, Ordering::Relaxed);
        self.throttled_operations.store(0, Ordering::Relaxed);
    }

    /// Get random number generator (deterministic if in deterministic mode)
    /// Returns enum-based RNG to avoid heap allocation
    pub fn get_rng(&self) -> GovernorRng {
        if self.config.deterministic_mode {
            // Use seeded RNG for deterministic execution
            GovernorRng::Deterministic(rand::rngs::StdRng::seed_from_u64(42))
        } else {
            // Use thread RNG for non-deterministic execution
            GovernorRng::Random(rand::thread_rng())
        }
    }
}

impl Clone for ResourceGovernor {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            cpu_usage_percent: Arc::clone(&self.cpu_usage_percent),
            last_cpu_check: Arc::clone(&self.last_cpu_check),
            ram_usage_bytes: Arc::clone(&self.ram_usage_bytes),
            io_ops_count: Arc::clone(&self.io_ops_count),
            io_window_start: Arc::clone(&self.io_window_start),
            operation_semaphore: Arc::clone(&self.operation_semaphore),
            is_paused: Arc::clone(&self.is_paused),
            pause_notify: Arc::clone(&self.pause_notify),
            total_operations: Arc::clone(&self.total_operations),
            throttled_operations: Arc::clone(&self.throttled_operations),
        }
    }
}

/// Permit for executing an operation under resource governance
pub struct OperationPermit {
    _permit: tokio::sync::OwnedSemaphorePermit,
    governor: ResourceGovernor,
    start_time: Instant,
}

impl OperationPermit {
    /// Get the resource governor
    pub fn governor(&self) -> &ResourceGovernor {
        &self.governor
    }

    /// Get operation duration
    pub fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// Resource governor statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernorStatistics {
    /// Total operations processed
    pub total_operations: u64,

    /// Operations that were throttled
    pub throttled_operations: u64,

    /// Current CPU usage percentage
    pub current_cpu_usage: u64,

    /// Current RAM usage in bytes
    pub current_ram_usage: u64,

    /// Whether governor is paused
    pub is_paused: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let mut config = ResourceGovernorConfig::default();
        assert!(config.validate().is_ok());

        config.cpu_cap_percent = Some(150);
        assert!(config.validate().is_err());

        config.cpu_cap_percent = Some(50);
        config.max_concurrent_operations = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_governor_creation() {
        let config = ResourceGovernorConfig::default();
        let governor = ResourceGovernor::new(config);
        assert!(governor.is_ok());
    }

    #[tokio::test]
    async fn test_acquire_permit() {
        let config = ResourceGovernorConfig::default();
        let governor = ResourceGovernor::new(config).unwrap();

        let permit = governor.acquire_permit().await;
        assert!(permit.is_ok());
    }

    #[tokio::test]
    async fn test_cpu_tracking() {
        let config = ResourceGovernorConfig::default();
        let governor = ResourceGovernor::new(config).unwrap();

        governor.update_cpu_usage(50);
        assert_eq!(governor.current_cpu_usage(), 50);
    }

    #[tokio::test]
    async fn test_ram_tracking() {
        let config = ResourceGovernorConfig::default();
        let governor = ResourceGovernor::new(config).unwrap();

        governor.track_ram_allocation(1024);
        assert_eq!(governor.current_ram_usage(), 1024);

        governor.track_ram_deallocation(512);
        assert_eq!(governor.current_ram_usage(), 512);
    }

    #[tokio::test]
    async fn test_pause_resume() {
        let config = ResourceGovernorConfig::default();
        let governor = ResourceGovernor::new(config).unwrap();

        governor.pause();
        let stats = governor.statistics();
        assert!(stats.is_paused);

        governor.resume();
        let stats = governor.statistics();
        assert!(!stats.is_paused);
    }

    #[tokio::test]
    async fn test_io_throttling() {
        let mut config = ResourceGovernorConfig::default();
        config.io_ops_per_second = Some(10);
        let governor = ResourceGovernor::new(config).unwrap();

        // First few operations should be fast
        for _ in 0..5 {
            let result = governor.throttle_io().await;
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_deterministic_mode() {
        let mut config = ResourceGovernorConfig::default();
        config.deterministic_mode = true;
        let governor = ResourceGovernor::new(config).unwrap();

        assert!(governor.is_deterministic());
        assert!(!governor.is_sandboxed());
    }

    #[test]
    fn test_sandbox_mode() {
        let mut config = ResourceGovernorConfig::default();
        config.sandbox_mode = true;
        let governor = ResourceGovernor::new(config).unwrap();

        assert!(governor.is_sandboxed());
    }

    #[tokio::test]
    async fn test_statistics() {
        let config = ResourceGovernorConfig::default();
        let governor = ResourceGovernor::new(config).unwrap();

        let _permit = governor.acquire_permit().await.unwrap();

        let stats = governor.statistics();
        assert_eq!(stats.total_operations, 1);

        governor.reset_statistics();
        let stats = governor.statistics();
        assert_eq!(stats.total_operations, 0);
    }

    #[test]
    fn test_preset_configs() {
        let testing = ResourceGovernorConfig::testing();
        assert!(testing.deterministic_mode);
        assert!(testing.sandbox_mode);
        assert_eq!(testing.cpu_cap_percent, Some(50));

        let production = ResourceGovernorConfig::production();
        assert!(!production.deterministic_mode);
        assert!(!production.sandbox_mode);
        assert_eq!(production.cpu_cap_percent, Some(80));
    }
}
