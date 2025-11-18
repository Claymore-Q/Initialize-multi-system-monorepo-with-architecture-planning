# ROOT ARCHITECTURE
## Multi-System Monorepo: Enterprise-Grade Internal Tools

**Version:** 1.0.0
**Last Updated:** 2025-11-17
**Language:** Rust (Edition 2021+)
**Target Ecosystem:** 2027-2030

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Language Selection Rationale](#language-selection-rationale)
3. [Repository Structure](#repository-structure)
4. [Naming Conventions](#naming-conventions)
5. [Error Model](#error-model)
6. [Logging and Telemetry Philosophy](#logging-and-telemetry-philosophy)
7. [Concurrency Model](#concurrency-model)
8. [Versioning Rules](#versioning-rules)
9. [Folder Conventions](#folder-conventions)
10. [CI/CD Conventions](#cicd-conventions)
11. [Security Posture](#security-posture)
12. [Testing Philosophy](#testing-philosophy)
13. [Dependency Management](#dependency-management)
14. [Build and Deployment](#build-and-deployment)
15. [Inter-System Communication](#inter-system-communication)

---

## 1. Executive Summary

This monorepo houses 10 standalone internal systems designed for enterprise-grade operation across heterogeneous environments. Each system operates as an independent service while sharing common infrastructure, error handling, logging, and security primitives.

**Core Tenets:**
- **Isolation with Sharing**: Each system is independently deployable but shares core libraries
- **Fail-Safe by Default**: All systems must gracefully degrade under failure conditions
- **Observable**: Comprehensive telemetry at every boundary
- **Testable**: Tests are first-class artifacts, not afterthoughts
- **Portable**: Single binary deployment across Linux, macOS, Windows
- **Offline-First**: No runtime dependencies on external services for core functionality

**Systems Portfolio:**

1. **chaos_engine**: Fault injection and resilience testing framework
2. **contract_executable_compiler**: Domain-specific language compiler for executable contracts
3. **cross_domain_auto_learner**: Adaptive machine learning system for pattern recognition across domains
4. **dynamic_semantic_lattice_engine**: Semantic reasoning engine with lattice-based knowledge representation
5. **linux_scalable_cloud_kernel**: Kernel module and userspace toolkit for cloud-scale Linux operations
6. **parallel_architecture_framework**: Framework for distributed computation orchestration
7. **symbolic_reduction_modeler**: Symbolic computation engine for algebraic reduction
8. **synthetic_pipeline_engine**: Data pipeline orchestration with synthetic data generation
9. **universal_attestation_authority**: Cryptographic attestation and verification service
10. **cross_domain_autoblocker_ledger**: Distributed ledger for security event blocking

---

## 2. Language Selection Rationale

### Rust (Edition 2021+)

**Decision Criteria Analysis:**

#### 2.1 Cross-Platform Portability ✓✓✓
- **Native cross-compilation** via rustc targets (x86_64-unknown-linux-gnu, x86_64-apple-darwin, x86_64-pc-windows-msvc)
- **Single binary deployment** with static linking options
- **No runtime dependencies** (no JVM, no interpreter, optional C runtime)
- **Consistent behavior** across platforms due to strong type system

#### 2.2 Concurrency Primitives ✓✓✓
- **Fearless concurrency** through ownership and borrowing
- **Async/await** with zero-cost futures
- **Tokio runtime**: Battle-tested async runtime with work-stealing scheduler
- **Rayon**: Data parallelism for CPU-bound workloads
- **Crossbeam**: Lock-free data structures and channels
- **No data races** guaranteed at compile time

#### 2.3 Cryptographic Support ✓✓✓
- **ring**: High-performance, audited crypto primitives (ECDSA, RSA, AES-GCM, SHA-2/3)
- **rustls**: Memory-safe TLS implementation
- **ed25519-dalek**: EdDSA signatures
- **blake3**: Fast cryptographic hashing
- **x509-parser**: Certificate handling
- **zeroize**: Secure memory clearing
- **Constant-time operations** to prevent timing attacks

#### 2.4 Ecosystem Fit (2027-2030) ✓✓✓
- **Linux kernel adoption**: Rust officially supported in Linux 6.1+
- **Cloud-native**: Foundational language for Kubernetes, Linkerd, Firecracker
- **WASM target**: First-class WebAssembly support for edge deployment
- **Industry momentum**: Microsoft, Google, AWS investing heavily
- **Security focus**: Memory safety eliminates 70% of CVEs (Microsoft data)

#### 2.5 Offline Capability ✓✓✓
- **Cargo offline mode**: `--offline` flag for hermetic builds
- **Vendoring**: `cargo vendor` for complete dependency snapshots
- **No runtime package managers**: Everything compiled in
- **Reproducible builds**: Cargo.lock ensures deterministic dependencies

#### 2.6 Low Dependency Footprint ✓✓✓
- **Static binaries**: All dependencies compiled in
- **Minimal standard library**: Core + alloc + std modular design
- **Feature flags**: Fine-grained dependency control
- **Build time vs runtime**: Dependencies resolved at compile time
- **Strip symbols**: Production binaries < 5MB typical

**Alternatives Considered:**

| Language | Concurrency | Crypto | Portability | Footprint | 2027 Fit | Decision |
|----------|-------------|---------|-------------|-----------|----------|----------|
| Go | ✓✓ (GC pauses) | ✓✓ | ✓✓✓ | ✓✓ | ✓✓ | Strong alternative |
| C++ | ✓ (complex) | ✓✓ | ✓✓ | ✓ (large) | ✓ | Memory safety concerns |
| Python | ✗ (GIL) | ✓ | ✓ (runtime) | ✗ | ✓ | Runtime dependency |
| Zig | ✓✓ | ✓ | ✓✓✓ | ✓✓✓ | ✓ | Ecosystem immature |
| Rust | ✓✓✓ | ✓✓✓ | ✓✓✓ | ✓✓✓ | ✓✓✓ | **SELECTED** |

---

## 3. Repository Structure

```
/
├── chaos_engine/
│   ├── src/
│   │   ├── lib.rs                 # Public API
│   │   ├── core/                  # Core fault injection engine
│   │   ├── strategies/            # Injection strategies
│   │   ├── observers/             # System state observation
│   │   └── reporters/             # Chaos experiment reporting
│   ├── tests/
│   │   ├── integration/           # Integration tests
│   │   ├── unit/                  # Unit tests
│   │   └── fixtures/              # Test fixtures and data
│   ├── docs/
│   │   ├── SPEC.md                # System specification
│   │   ├── ARCHITECTURE.md        # Detailed architecture
│   │   ├── API.md                 # API documentation
│   │   └── diagrams/              # Architecture diagrams
│   ├── examples/
│   │   └── *.rs                   # Example usage
│   ├── benches/                   # Criterion benchmarks
│   └── Cargo.toml
│
├── contract_executable_compiler/
│   └── [same structure]
│
├── cross_domain_auto_learner/
│   └── [same structure]
│
├── dynamic_semantic_lattice_engine/
│   └── [same structure]
│
├── linux_scalable_cloud_kernel/
│   ├── kernel_module/             # Kernel-space C code
│   ├── userspace/                 # Rust userspace tools
│   └── [same structure]
│
├── parallel_architecture_framework/
│   └── [same structure]
│
├── symbolic_reduction_modeler/
│   └── [same structure]
│
├── synthetic_pipeline_engine/
│   └── [same structure]
│
├── universal_attestation_authority/
│   └── [same structure]
│
├── cross_domain_autoblocker_ledger/
│   └── [same structure]
│
├── shared_core/
│   ├── error/                     # Unified error types
│   ├── logging/                   # Structured logging
│   ├── telemetry/                 # OpenTelemetry integration
│   ├── crypto/                    # Shared crypto primitives
│   ├── config/                    # Configuration management
│   ├── types/                     # Common types and traits
│   └── Cargo.toml
│
├── scripts/
│   ├── build_all.sh               # Build all systems
│   ├── test_all.sh                # Run all tests
│   ├── bench_all.sh               # Run all benchmarks
│   ├── lint.sh                    # Clippy + rustfmt
│   ├── security_audit.sh          # cargo audit + deny
│   └── cross_compile.sh           # Cross-compilation script
│
├── specs/
│   ├── CHAOS_ENGINE_SPEC.md
│   ├── CONTRACT_COMPILER_SPEC.md
│   └── [... all system specs]
│
├── .github/
│   └── workflows/
│       ├── ci.yml                 # Continuous integration
│       ├── security.yml           # Security scanning
│       └── release.yml            # Release automation
│
├── Cargo.toml                     # Workspace configuration
├── Cargo.lock                     # Dependency lock file
├── rust-toolchain.toml            # Rust version specification
├── .clippy.toml                   # Clippy lints configuration
├── rustfmt.toml                   # Code formatting rules
├── deny.toml                      # cargo-deny configuration
└── ROOT_ARCHITECTURE.md           # This file
```

---

## 4. Naming Conventions

### 4.1 File and Directory Names
- **Lowercase with underscores**: `chaos_engine/`, `src/fault_injector.rs`
- **No hyphens in paths**: Underscores for multi-word names
- **Rust modules**: Match directory structure (module tree = file tree)

### 4.2 Rust Identifiers

#### Types and Traits
```rust
// PascalCase for types, structs, enums, traits
struct FaultInjectionStrategy { }
enum AttestationResult { }
trait SemanticLatticeNode { }
```

#### Functions and Methods
```rust
// snake_case for functions, methods, variables
fn inject_network_latency(target: &str, delay_ms: u64) -> Result<()> { }
async fn verify_attestation_chain(chain: &[Attestation]) -> AttestationResult { }
```

#### Constants and Statics
```rust
// SCREAMING_SNAKE_CASE for constants
const MAX_RETRY_ATTEMPTS: u32 = 5;
const DEFAULT_TIMEOUT_MS: u64 = 30_000;
static GLOBAL_CONFIG: OnceCell<Config> = OnceCell::new();
```

#### Modules
```rust
// snake_case for module names
mod fault_injection;
mod semantic_lattice;
pub mod crypto_primitives;
```

### 4.3 Error Type Naming
- **Pattern**: `<System><Operation>Error`
- **Examples**: `ChaosInjectionError`, `ContractCompilationError`, `AttestationVerificationError`

### 4.4 Configuration Keys
- **Environment variables**: `SCREAMING_SNAKE_CASE` with system prefix
  - `CHAOS_ENGINE_LOG_LEVEL=debug`
  - `ATTESTATION_AUTHORITY_KEY_PATH=/etc/keys`
- **TOML keys**: `snake_case`
  ```toml
  [chaos_engine]
  max_concurrent_faults = 10
  observer_poll_interval_ms = 100
  ```

---

## 5. Error Model

### 5.1 Error Philosophy

**Core Principles:**
1. **Errors are values**: Use `Result<T, E>` everywhere, never panic in library code
2. **Context-rich**: Every error carries enough information for debugging
3. **Typed**: Specific error types for each domain
4. **Actionable**: Error messages suggest remediation when possible
5. **Structured**: Errors are serializable for telemetry

### 5.2 Error Hierarchy

```rust
// shared_core/error/mod.rs

use thiserror::Error;
use serde::{Serialize, Deserialize};

/// Root error type for all systems
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum SystemError {
    #[error("I/O error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
        context: String,
    },

    #[error("Configuration error: {message}")]
    Config {
        message: String,
        key: Option<String>,
    },

    #[error("Cryptographic error: {operation}")]
    Crypto {
        operation: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Concurrency error: {message}")]
    Concurrency {
        message: String,
        thread_id: Option<String>,
    },

    #[error("Validation error: {field} - {reason}")]
    Validation {
        field: String,
        reason: String,
        value: Option<String>,
    },

    #[error("System-specific error: {0}")]
    SystemSpecific(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Result type alias used throughout the codebase
pub type Result<T> = std::result::Result<T, SystemError>;
```

### 5.3 Per-System Error Types

Each system defines its own error enum wrapping `SystemError`:

```rust
// chaos_engine/src/error.rs

#[derive(Debug, Error)]
pub enum ChaosError {
    #[error("Fault injection failed: {0}")]
    InjectionFailed(String),

    #[error("Invalid chaos strategy: {0}")]
    InvalidStrategy(String),

    #[error("Observer error: {0}")]
    Observer(String),

    #[error(transparent)]
    System(#[from] SystemError),
}
```

### 5.4 Error Handling Patterns

```rust
// Pattern 1: Context-rich error propagation
fn load_config(path: &Path) -> Result<Config> {
    std::fs::read_to_string(path)
        .map_err(|e| SystemError::Io {
            source: e,
            context: format!("Failed to read config from {:?}", path),
        })?;
    // ... parse config
}

// Pattern 2: Error conversion with context
fn parse_attestation(data: &[u8]) -> Result<Attestation> {
    serde_json::from_slice(data)
        .map_err(|e| SystemError::Validation {
            field: "attestation".to_string(),
            reason: format!("Invalid JSON: {}", e),
            value: None,
        })
}

// Pattern 3: Graceful degradation
async fn fetch_with_fallback(url: &str) -> Result<Data> {
    match fetch_primary(url).await {
        Ok(data) => Ok(data),
        Err(e) => {
            warn!("Primary fetch failed: {}, trying fallback", e);
            fetch_secondary(url).await
        }
    }
}
```

### 5.5 Panic Policy

**Library code**: NEVER panic. Always return `Result`.

**Binary code**: Panic only for:
- Irrecoverable initialization failures (e.g., cannot create runtime)
- Contract violations that indicate programmer error (use `debug_assert!`)

**Testing code**: Panic freely with descriptive messages.

---

## 6. Logging and Telemetry Philosophy

### 6.1 Structured Logging

**Framework**: `tracing` + `tracing-subscriber`

**Rationale**:
- Structured fields for filtering and aggregation
- Async-aware (tracks spans across await points)
- Zero-cost when disabled
- OpenTelemetry integration

### 6.2 Log Levels

```rust
use tracing::{trace, debug, info, warn, error};

// TRACE: Fine-grained debugging (disabled in production)
trace!(target = "chaos_engine::injector", "Injecting fault into PID {}", pid);

// DEBUG: Detailed operational information
debug!(operation = "attestation_verify", chain_length = chain.len());

// INFO: High-level operational events
info!(
    event = "service_started",
    version = env!("CARGO_PKG_VERSION"),
    "Chaos Engine service initialized"
);

// WARN: Recoverable errors, degraded operation
warn!(
    error = ?e,
    retry_attempt = attempt,
    "Failed to connect to observer, retrying"
);

// ERROR: Unrecoverable errors for this operation
error!(
    error = ?e,
    component = "contract_compiler",
    "Compilation failed"
);
```

### 6.3 Span Instrumentation

```rust
use tracing::{instrument, info_span};

#[instrument(skip(data), fields(data_len = data.len()))]
async fn process_attestation(data: &[u8]) -> Result<Attestation> {
    let _span = info_span!("parsing").entered();
    let parsed = parse(data)?;

    let _span = info_span!("verification").entered();
    verify(&parsed).await?;

    Ok(parsed)
}
```

### 6.4 Telemetry Architecture

```
┌─────────────┐
│   Service   │
└──────┬──────┘
       │ tracing spans/events
       ▼
┌─────────────────┐
│ tracing-subscriber │
│  ├─ fmt layer   │  → stderr (local dev)
│  ├─ json layer  │  → structured logs
│  └─ otel layer  │  → OpenTelemetry
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  OTEL Collector │
└────────┬────────┘
         │
    ┌────┴────┐
    ▼         ▼
┌────────┐ ┌─────────┐
│ Jaeger │ │Prometheus│
└────────┘ └─────────┘
```

### 6.5 Metrics

**Framework**: `metrics` crate with Prometheus exporter

```rust
use metrics::{counter, histogram, gauge};

// Counters for events
counter!("chaos_engine.faults_injected", 1, "type" => "network_latency");

// Histograms for durations
histogram!("attestation.verify_duration_ms", duration.as_millis() as f64);

// Gauges for current state
gauge!("pipeline.active_jobs", active_count as f64);
```

### 6.6 Telemetry Configuration

```toml
# config.toml
[telemetry]
log_level = "info"
log_format = "json"  # or "pretty" for development
enable_otel = true
otel_endpoint = "http://localhost:4317"

[telemetry.sampling]
trace_ratio = 0.1  # Sample 10% of traces in production
```

---

## 7. Concurrency Model

### 7.1 Async Runtime

**Primary**: Tokio with multi-threaded scheduler

```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
```

**Rationale**:
- Industry standard
- Work-stealing scheduler for balanced load
- Excellent ecosystem integration
- Production-proven at scale

### 7.2 CPU-Bound Parallelism

**Framework**: Rayon for data parallelism

```rust
use rayon::prelude::*;

// Parallel iteration
let results: Vec<_> = large_dataset
    .par_iter()
    .map(|item| expensive_computation(item))
    .collect();

// Parallel sorting
items.par_sort_by_key(|item| item.priority);
```

### 7.3 Async Patterns

#### Pattern 1: Concurrent Requests
```rust
use tokio::try_join;

async fn fetch_all_data() -> Result<(DataA, DataB, DataC)> {
    try_join!(
        fetch_data_a(),
        fetch_data_b(),
        fetch_data_c(),
    )
}
```

#### Pattern 2: Buffered Stream Processing
```rust
use futures::stream::{self, StreamExt};

async fn process_batch(items: Vec<Item>) -> Result<Vec<Output>> {
    stream::iter(items)
        .map(|item| async move { process_item(item).await })
        .buffered(10)  // Process 10 concurrently
        .try_collect()
        .await
}
```

#### Pattern 3: Timeout and Cancellation
```rust
use tokio::time::{timeout, Duration};

async fn with_timeout<T>(
    future: impl Future<Output = Result<T>>,
    duration: Duration,
) -> Result<T> {
    timeout(duration, future)
        .await
        .map_err(|_| SystemError::Concurrency {
            message: "Operation timed out".to_string(),
            thread_id: None,
        })?
}
```

### 7.4 Shared State

**Principles**:
1. Prefer message passing over shared state
2. Use channels (tokio::mpsc, crossbeam::channel) for communication
3. When shared state is necessary, use Arc<RwLock<T>> or Arc<Mutex<T>>
4. For high-contention scenarios, use lock-free structures (crossbeam)

```rust
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Clone)]
struct SharedCache {
    inner: Arc<RwLock<HashMap<String, Data>>>,
}

impl SharedCache {
    async fn get(&self, key: &str) -> Option<Data> {
        self.inner.read().await.get(key).cloned()
    }

    async fn insert(&self, key: String, value: Data) {
        self.inner.write().await.insert(key, value);
    }
}
```

### 7.5 Backpressure

Apply backpressure to prevent resource exhaustion:

```rust
use tokio::sync::Semaphore;

struct RateLimiter {
    semaphore: Arc<Semaphore>,
}

impl RateLimiter {
    fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    async fn acquire(&self) -> Result<SemaphorePermit> {
        self.semaphore.acquire().await
            .map_err(|_| SystemError::Concurrency {
                message: "Semaphore closed".to_string(),
                thread_id: None,
            })
    }
}
```

---

## 8. Versioning Rules

### 8.1 Semantic Versioning

All systems follow **SemVer 2.0.0**: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking API changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

### 8.2 Pre-1.0 Semantics

During initial development (0.x.y):
- **0.MINOR.PATCH**: MINOR bumps may include breaking changes
- **0.x.PATCH**: PATCH bumps are backward compatible

### 8.3 Version Synchronization

**Workspace Version**: All systems share a major version for coordinated releases

```toml
# Cargo.toml (workspace root)
[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.75"

[workspace.dependencies]
shared_core = { path = "./shared_core", version = "0.1.0" }
```

Individual systems can have independent minor/patch versions:
```toml
# chaos_engine/Cargo.toml
[package]
name = "chaos_engine"
version = "0.2.3"  # Independent versioning
```

### 8.4 API Stability Guarantees

**Stability Levels** (marked in rustdoc):

```rust
/// Stable API - will not change in breaking ways within major version
#[stability = "stable"]
pub fn verify_attestation(data: &[u8]) -> Result<Attestation> { }

/// Unstable API - may change without major version bump during 0.x
#[stability = "unstable"]
pub fn experimental_feature() { }

/// Deprecated API - will be removed in next major version
#[deprecated(since = "0.5.0", note = "Use verify_attestation_v2 instead")]
pub fn verify_attestation_v1(data: &[u8]) -> Result<Attestation> { }
```

### 8.5 Changelog

Each system maintains a `CHANGELOG.md` following [Keep a Changelog](https://keepachangelog.com/):

```markdown
# Changelog

## [Unreleased]
### Added
- New fault injection strategy: disk_corruption

## [0.2.0] - 2025-11-17
### Added
- Support for Kubernetes pod fault injection
- Observer for container metrics

### Changed
- Improved error messages for injection failures

### Fixed
- Race condition in observer polling

## [0.1.0] - 2025-11-01
### Added
- Initial release
```

---

## 9. Folder Conventions

### 9.1 System Layout

Every system follows identical structure:

```
<system_name>/
├── Cargo.toml           # Package manifest
├── README.md            # Quick start guide
├── CHANGELOG.md         # Version history
├── src/
│   ├── lib.rs           # Library root (pub API)
│   ├── main.rs          # Binary entry point (if applicable)
│   ├── core/            # Core business logic
│   ├── api/             # Public API types and traits
│   ├── config/          # Configuration structures
│   ├── error.rs         # Error types
│   └── [modules]/       # Domain-specific modules
├── tests/
│   ├── integration/     # Integration tests
│   │   └── *.rs
│   ├── unit/            # Additional unit tests
│   └── fixtures/        # Test data
│       ├── configs/
│       └── data/
├── benches/
│   └── *.rs             # Criterion benchmarks
├── docs/
│   ├── SPEC.md          # Formal specification
│   ├── ARCHITECTURE.md  # Architecture deep-dive
│   ├── API.md           # API documentation
│   ├── RUNBOOK.md       # Operations guide
│   └── diagrams/
│       └── *.svg
└── examples/
    └── *.rs             # Runnable examples
```

### 9.2 Module Organization

**Principle**: Modules represent domain concepts, not technical layers

**Good**:
```
src/
├── fault_injection/     # Domain: fault injection
│   ├── strategies/
│   ├── targets/
│   └── executor.rs
├── observation/         # Domain: system observation
│   ├── metrics/
│   └── state.rs
└── reporting/           # Domain: experiment reporting
```

**Avoid**:
```
src/
├── models/              # Technical layer, not domain
├── services/            # Technical layer
└── utils/               # Junk drawer anti-pattern
```

### 9.3 Test Organization

```rust
// Unit tests: Inline with implementation
// src/fault_injection/strategies.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_latency_strategy() {
        // ...
    }
}

// Integration tests: Separate files
// tests/integration/chaos_workflow.rs
#[tokio::test]
async fn test_full_chaos_experiment() {
    // Tests complete workflow across modules
}
```

---

## 10. CI/CD Conventions

### 10.1 Continuous Integration Pipeline

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta, nightly]

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}

      - name: Build
        run: cargo build --all-features --verbose

      - name: Test
        run: cargo test --all-features --verbose

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Format
        run: cargo fmt -- --check

  security:
    steps:
      - name: Security Audit
        run: cargo audit

      - name: Dependency Check
        run: cargo deny check

  coverage:
    steps:
      - name: Code Coverage
        run: cargo tarpaulin --out Xml

      - name: Upload to Codecov
        uses: codecov/codecov-action@v3
```

### 10.2 Pre-commit Hooks

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Format check
cargo fmt -- --check || {
    echo "Formatting issues found. Run: cargo fmt"
    exit 1
}

# Clippy lints
cargo clippy --all-targets --all-features -- -D warnings || {
    echo "Clippy lints failed"
    exit 1
}

# Fast tests
cargo test --lib || {
    echo "Tests failed"
    exit 1
}
```

### 10.3 Release Process

1. **Version Bump**: Update `Cargo.toml` versions
2. **Changelog**: Update `CHANGELOG.md` with release notes
3. **Tag**: Create Git tag `v<major>.<minor>.<patch>`
4. **Build**: Cross-compile for all platforms
5. **Test**: Run full test suite on release build
6. **Publish**: Create GitHub release with binaries

```bash
# scripts/release.sh
#!/bin/bash
set -euo pipefail

VERSION=$1

# Validate version format
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Invalid version format. Use: X.Y.Z"
    exit 1
fi

# Update versions
sed -i "s/^version = .*/version = \"$VERSION\"/" Cargo.toml

# Run tests
cargo test --release --all-features

# Build release binaries
./scripts/cross_compile.sh

# Create tag
git tag -a "v$VERSION" -m "Release v$VERSION"
git push origin "v$VERSION"

echo "Release v$VERSION ready"
```

### 10.4 Continuous Deployment

**Staging**: Automatic deployment on merge to `main`
**Production**: Manual approval required after staging validation

```yaml
# .github/workflows/deploy.yml
deploy-staging:
  if: github.ref == 'refs/heads/main'
  steps:
    - name: Deploy to Staging
      run: ./scripts/deploy.sh staging

deploy-production:
  if: startsWith(github.ref, 'refs/tags/v')
  environment: production
  steps:
    - name: Deploy to Production
      run: ./scripts/deploy.sh production
```

---

## 11. Security Posture

### 11.1 Threat Model

**Assumptions**:
- Attacker may have network access
- Attacker may provide malicious input
- Systems run with limited privileges
- Host OS and hardware are trusted

**Out of Scope**:
- Physical access attacks
- Hardware tampering
- Side-channel attacks (timing, power analysis)

### 11.2 Security Principles

1. **Least Privilege**: Run with minimal permissions
2. **Defense in Depth**: Multiple security layers
3. **Fail Secure**: Default to deny on error
4. **Input Validation**: Validate all external input
5. **Secure Defaults**: Safe configuration out-of-box

### 11.3 Cryptographic Standards

```rust
// AES-256-GCM for symmetric encryption
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};

// Ed25519 for signatures
use ed25519_dalek::{Keypair, Signature, Signer, Verifier};

// BLAKE3 for hashing (faster than SHA-256, secure)
use blake3::Hasher;

// TLS 1.3 only
use rustls::{ClientConfig, ServerConfig};
```

**Key Management**:
- Never hardcode keys
- Use OS keyring (keyring-rs) or HSM
- Rotate keys periodically
- Zeroize keys on drop

```rust
use zeroize::Zeroize;

struct SecretKey([u8; 32]);

impl Drop for SecretKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}
```

### 11.4 Dependency Security

```toml
# deny.toml
[advisories]
db-path = "~/.cargo/advisory-db"
db-urls = ["https://github.com/rustsec/advisory-db"]
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"

[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-3-Clause",
]
deny = [
    "GPL-3.0",  # Copyleft incompatible with proprietary use
]

[bans]
multiple-versions = "warn"
wildcards = "deny"  # No version wildcards
```

### 11.5 Input Validation

**Principle**: Validate early, validate strictly

```rust
use validator::Validate;

#[derive(Validate)]
struct AttestationRequest {
    #[validate(length(min = 1, max = 1024))]
    identity: String,

    #[validate(range(min = 0, max = 86400))]
    validity_seconds: u64,

    #[validate(custom = "validate_public_key")]
    public_key: Vec<u8>,
}

fn validate_public_key(key: &[u8]) -> Result<(), validator::ValidationError> {
    if key.len() != 32 {
        return Err(validator::ValidationError::new("invalid_key_length"));
    }
    Ok(())
}
```

---

## 12. Testing Philosophy

### 12.1 Test Pyramid

```
        ┌──────┐
        │  E2E │  ← 5% (End-to-end, full system)
        └──────┘
      ┌──────────┐
      │Integration│ ← 25% (Multiple modules)
      └──────────┘
    ┌──────────────┐
    │     Unit     │  ← 70% (Individual functions)
    └──────────────┘
```

### 12.2 Test Categories

#### Unit Tests
- **Purpose**: Test individual functions and modules
- **Location**: Inline with `#[cfg(test)]` or `tests/unit/`
- **Isolation**: Mock external dependencies
- **Speed**: Fast (<1ms per test)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fault_injection_validates_input() {
        let result = inject_fault("", FaultType::NetworkLatency);
        assert!(matches!(result, Err(ChaosError::InvalidStrategy(_))));
    }
}
```

#### Integration Tests
- **Purpose**: Test module interactions
- **Location**: `tests/integration/`
- **Isolation**: May use test fixtures
- **Speed**: Moderate (<100ms per test)

```rust
#[tokio::test]
async fn test_chaos_experiment_workflow() {
    let engine = ChaosEngine::new(Config::test_config());
    let experiment = Experiment::builder()
        .target("test-service")
        .fault(FaultType::NetworkLatency { delay_ms: 100 })
        .build();

    let result = engine.run_experiment(experiment).await;
    assert!(result.is_ok());
}
```

#### Property-Based Tests
- **Purpose**: Test invariants across input space
- **Framework**: `proptest` or `quickcheck`

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_attestation_round_trip(data in prop::collection::vec(any::<u8>(), 0..1024)) {
        let attestation = create_attestation(&data).unwrap();
        let verified = verify_attestation(&attestation).unwrap();
        prop_assert_eq!(verified.data, data);
    }
}
```

#### Fuzz Tests
- **Purpose**: Find edge cases and crashes
- **Framework**: `cargo-fuzz` (libFuzzer)

```rust
// fuzz/fuzz_targets/parse_contract.rs
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = contract_compiler::parse(data);
    // Should never panic, only return Err
});
```

### 12.3 Test Design Principles

**FIRST Principles**:
- **Fast**: Tests run quickly
- **Independent**: No test dependencies
- **Repeatable**: Same results every time
- **Self-validating**: Pass/fail, no manual inspection
- **Timely**: Written alongside code

**AAA Pattern** (Arrange-Act-Assert):
```rust
#[test]
fn test_semantic_lattice_merge() {
    // Arrange
    let lattice1 = SemanticLattice::new();
    let lattice2 = SemanticLattice::new();
    lattice1.insert("key", "value1");
    lattice2.insert("key", "value2");

    // Act
    let merged = lattice1.merge(&lattice2);

    // Assert
    assert_eq!(merged.get("key"), Some("value2"));
}
```

### 12.4 Failure Testing

**Principle**: Test both success and failure paths

```rust
#[tokio::test]
async fn test_network_fault_injection_succeeds() {
    let result = inject_network_latency("target-service", 100).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_network_fault_injection_fails_invalid_target() {
    let result = inject_network_latency("", 100).await;
    assert!(matches!(result, Err(ChaosError::InvalidTarget(_))));
}

#[tokio::test]
async fn test_network_fault_injection_fails_excessive_delay() {
    let result = inject_network_latency("target", u64::MAX).await;
    assert!(matches!(result, Err(ChaosError::InvalidParameter(_))));
}
```

### 12.5 Concurrency Testing

```rust
use std::sync::Arc;
use tokio::task;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_attestation_verification() {
    let authority = Arc::new(AttestationAuthority::new());
    let mut handles = vec![];

    for i in 0..100 {
        let authority = Arc::clone(&authority);
        handles.push(task::spawn(async move {
            let attestation = create_attestation(&format!("data-{}", i));
            authority.verify(&attestation).await
        }));
    }

    for handle in handles {
        assert!(handle.await.unwrap().is_ok());
    }
}
```

### 12.6 Load Testing

```rust
// benches/attestation_throughput.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn bench_attestation_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("attestation");
    group.throughput(Throughput::Elements(1));

    group.bench_function("verify", |b| {
        let attestation = create_test_attestation();
        b.iter(|| {
            verify_attestation(black_box(&attestation))
        });
    });

    group.finish();
}

criterion_group!(benches, bench_attestation_verification);
criterion_main!(benches);
```

---

## 13. Dependency Management

### 13.1 Dependency Philosophy

**Principles**:
1. **Minimize dependencies**: Each dependency is a liability
2. **Audit dependencies**: Run `cargo audit` regularly
3. **Pin versions**: Use exact versions in Cargo.lock
4. **Prefer battle-tested**: Avoid dependencies with <10k downloads
5. **Check licenses**: Ensure compatibility with our license

### 13.2 Workspace Dependencies

Centralize common dependencies in workspace:

```toml
# Cargo.toml (workspace root)
[workspace.dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }
futures = "0.3"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bincode = "1.3"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }

# Crypto
ring = "0.17"
ed25519-dalek = "2.1"
blake3 = "1.5"

# Testing
proptest = "1.4"
criterion = "0.5"
```

Individual systems reference workspace dependencies:

```toml
# chaos_engine/Cargo.toml
[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
thiserror = { workspace = true }

# System-specific dependencies
nix = "0.27"  # Unix system calls
```

### 13.3 Feature Flags

Use features for optional functionality:

```toml
[features]
default = ["std"]
std = []
async = ["tokio"]
crypto = ["ring", "ed25519-dalek"]
telemetry = ["tracing", "tracing-subscriber"]
full = ["async", "crypto", "telemetry"]
```

### 13.4 Dependency Auditing

```bash
# Install audit tools
cargo install cargo-audit
cargo install cargo-deny
cargo install cargo-outdated

# Regular audits
cargo audit          # Check for security advisories
cargo deny check     # Check licenses and bans
cargo outdated       # Check for updates
```

---

## 14. Build and Deployment

### 14.1 Build Profiles

```toml
# Cargo.toml
[profile.dev]
opt-level = 0
debug = true
incremental = true

[profile.release]
opt-level = 3
debug = false
lto = "fat"           # Link-time optimization
codegen-units = 1     # Better optimization
strip = "symbols"     # Strip debug symbols
panic = "abort"       # Smaller binary

[profile.bench]
inherits = "release"
debug = true          # Keep symbols for profiling
```

### 14.2 Cross-Compilation

```bash
# scripts/cross_compile.sh
#!/bin/bash

TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"  # Static Linux binary
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"       # Apple Silicon
    "x86_64-pc-windows-msvc"
)

for target in "${TARGETS[@]}"; do
    echo "Building for $target..."
    cargo build --release --target "$target" --all-features
done
```

### 14.3 Docker Deployment

```dockerfile
# Dockerfile (multi-stage build)
FROM rust:1.75-slim as builder

WORKDIR /build
COPY . .

RUN cargo build --release --bin chaos_engine

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/chaos_engine /usr/local/bin/

USER nobody
ENTRYPOINT ["chaos_engine"]
```

### 14.4 Configuration Management

**Hierarchy** (later sources override earlier):
1. Default values (hardcoded)
2. Config file (`/etc/app/config.toml`)
3. Environment variables
4. Command-line arguments

```rust
use config::{Config, ConfigError, Environment, File};

fn load_config() -> Result<AppConfig, ConfigError> {
    Config::builder()
        .set_default("log_level", "info")?
        .add_source(File::with_name("/etc/chaos_engine/config").required(false))
        .add_source(Environment::with_prefix("CHAOS_ENGINE"))
        .build()?
        .try_deserialize()
}
```

---

## 15. Inter-System Communication

### 15.1 Communication Patterns

**Within Process**:
- Direct function calls for systems in same binary
- Shared memory via `Arc<T>`

**Across Processes**:
- gRPC for synchronous RPC
- Message queues (NATS, Redis) for async events
- Shared database for persistence

### 15.2 gRPC Service Definition

```protobuf
// shared_core/proto/attestation.proto
syntax = "proto3";

package attestation;

service AttestationAuthority {
    rpc CreateAttestation(AttestationRequest) returns (AttestationResponse);
    rpc VerifyAttestation(VerificationRequest) returns (VerificationResponse);
}

message AttestationRequest {
    string identity = 1;
    bytes public_key = 2;
    uint64 validity_seconds = 3;
}

message AttestationResponse {
    bytes attestation = 1;
    string error = 2;
}
```

### 15.3 Event-Driven Architecture

```rust
use async_nats::Client;

#[derive(Serialize, Deserialize)]
struct ChaosEvent {
    event_type: String,
    target: String,
    timestamp: u64,
}

async fn publish_event(client: &Client, event: ChaosEvent) -> Result<()> {
    let payload = serde_json::to_vec(&event)?;
    client.publish("chaos.events", payload.into()).await?;
    Ok(())
}

async fn subscribe_events(client: Client) -> Result<()> {
    let mut subscriber = client.subscribe("chaos.events").await?;

    while let Some(msg) = subscriber.next().await {
        let event: ChaosEvent = serde_json::from_slice(&msg.payload)?;
        handle_event(event).await?;
    }

    Ok(())
}
```

---

## Conclusion

This architecture establishes a robust foundation for building, testing, and operating 10 interconnected systems at enterprise scale. The principles outlined here prioritize:

- **Safety**: Rust's type system prevents entire classes of bugs
- **Performance**: Zero-cost abstractions and async I/O
- **Observability**: Comprehensive logging and tracing
- **Maintainability**: Clear conventions and extensive testing
- **Portability**: Single-binary deployment across platforms

All systems must adhere to these standards to ensure consistency, quality, and long-term maintainability.

**Next Steps**:
1. Review and approve this architecture document
2. Create detailed SPEC.md for each system
3. Implement shared_core libraries
4. Build system scaffolds with comprehensive tests
5. Iterate based on real-world usage

---

**Document Control**

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0 | 2025-11-17 | Architecture Team | Initial release |

**Approval**

- [ ] Technical Lead
- [ ] Security Team
- [ ] Operations Team
- [ ] Development Team
