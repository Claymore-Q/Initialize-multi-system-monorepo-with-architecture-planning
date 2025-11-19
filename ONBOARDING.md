# Developer Onboarding Guide

**Version:** 1.0.0
**Last Updated:** 2025-11-19
**Estimated Time:** 2-4 hours to complete

---

## Welcome!

Welcome to the **Multi-System Monorepo** project! This guide will help you get started as a new developer contributing to this Rust-based distributed systems platform.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Day 1: Environment Setup](#day-1-environment-setup)
- [Day 2: Codebase Overview](#day-2-codebase-overview)
- [Day 3: Your First Contribution](#day-3-your-first-contribution)
- [Common Tasks Reference](#common-tasks-reference)
- [Architecture Deep Dive](#architecture-deep-dive)
- [Development Workflow](#development-workflow)
- [Debugging Guide](#debugging-guide)
- [Getting Help](#getting-help)

---

## Prerequisites

### Required Knowledge

- **Rust:** Intermediate level (async/await, traits, lifetimes, error handling)
- **Git:** Basic commands (clone, commit, push, pull request)
- **Linux/Unix:** Command line proficiency
- **Distributed Systems:** Basic concepts (helpful but not required)

### Recommended Reading

1. **The Rust Programming Language** - https://doc.rust-lang.org/book/
2. **Async Programming in Rust** - https://rust-lang.github.io/async-book/
3. **Tokio Tutorial** - https://tokio.rs/tokio/tutorial
4. **Distributed Systems (optional)** - Martin Kleppmann's "Designing Data-Intensive Applications"

### Hardware Requirements

- **CPU:** 4+ cores (8+ recommended)
- **RAM:** 8 GB minimum (16 GB recommended)
- **Disk:** 20 GB free space
- **OS:** Linux, macOS, or Windows with WSL2

---

## Day 1: Environment Setup

### Step 1: Install Rust

```bash
# Install rustup (Rust toolchain manager)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart shell or source environment
source $HOME/.cargo/env

# Verify installation
rustc --version
# Expected: rustc 1.81.0 or newer

# Install required components
rustup component add rustfmt clippy
```

### Step 2: Install Development Tools

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    curl
```

**macOS:**
```bash
# Install Homebrew if not present
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install openssl pkg-config
```

**Windows (WSL2):**
```bash
# Follow Linux instructions above in WSL2 Ubuntu
```

### Step 3: Clone Repository

```bash
# Clone the repository
git clone <repository-url>
cd Initialize-multi-system-monorepo-with-architecture-planning

# Create your development branch
git checkout -b feature/your-name-first-feature
```

### Step 4: Build the Project

```bash
# Build entire workspace (takes 5-10 minutes first time)
./scripts/build_all.sh

# Or build manually
cargo build --workspace

# Expected output:
# Compiling shared_core v0.1.0
# Compiling chaos_engine v0.1.0
# ... (all 11 crates)
# Finished dev [unoptimized + debuginfo] target(s)
```

### Step 5: Run Tests

```bash
# Run all tests
./scripts/test_all.sh

# Or run manually
cargo test --workspace

# Expected: 31+ tests passing
# Expected time: 30-60 seconds
```

### Step 6: Set Up Editor

**VS Code (Recommended):**

```bash
# Install VS Code
# Download from https://code.visualstudio.com/

# Install Rust extensions
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension serayuzgur.crates
code --install-extension tamasfe.even-better-toml

# Open project
code .
```

**Configuration:** Create `.vscode/settings.json`:
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.allFeatures": true,
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

**Vim/Neovim:**
```bash
# Install rust.vim
# Add to .vimrc:
Plug 'rust-lang/rust.vim'

# Install coc-rust-analyzer
:CocInstall coc-rust-analyzer
```

### Step 7: Verify Setup

```bash
# Lint codebase
./scripts/lint.sh

# Expected: No errors, some warnings OK

# Format code
cargo fmt --all

# Run a specific system
cd chaos_engine
cargo run --example basic_usage
```

**✅ Checkpoint:** If all steps passed, your environment is ready!

---

## Day 2: Codebase Overview

### Repository Structure

```
/
├── shared_core/              # ⭐ START HERE - Shared infrastructure
│   ├── src/
│   │   ├── lib.rs           # Public API
│   │   ├── error.rs         # Error types
│   │   ├── logging.rs       # Logging setup
│   │   ├── crypto.rs        # Cryptography
│   │   ├── resource_governor.rs  # Resource control
│   │   └── plugin.rs        # Plugin system
│   └── tests/               # Shared core tests
│
├── chaos_engine/            # Fault injection system
├── contract_executable_compiler/  # DSL compiler
├── cross_domain_auto_learner/     # Machine learning
├── dynamic_semantic_lattice_engine/  # Semantic reasoning
├── linux_scalable_cloud_kernel/   # Kernel modules
├── parallel_architecture_framework/  # Distributed computing
├── symbolic_reduction_modeler/    # Symbolic computation
├── synthetic_pipeline_engine/     # Pipeline orchestration
├── universal_attestation_authority/  # Cryptographic attestation
├── cross_domain_autoblocker_ledger/  # Security ledger
│
├── specs/                   # Formal specifications
├── scripts/                 # Build and test scripts
├── ROOT_ARCHITECTURE.md     # 📖 Architecture guide
├── README.md                # 📖 Quick start
└── Cargo.toml               # Workspace definition
```

### Key Files to Read (in order)

**Hour 1: High-Level Overview**
1. `README.md` (15 min) - Project overview
2. `ROOT_ARCHITECTURE.md` (30 min) - Architecture principles
3. `RELEASE_NOTES.md` (15 min) - Recent changes

**Hour 2: Shared Infrastructure**
4. `shared_core/src/lib.rs` (10 min) - Core API
5. `shared_core/src/error.rs` (10 min) - Error handling patterns
6. `shared_core/src/resource_governor.rs` (20 min) - Resource management
7. `shared_core/src/plugin.rs` (20 min) - Plugin architecture

**Hour 3: Pick a System**
8. Choose one system that interests you:
   - **Chaos Engine** - If you like reliability engineering
   - **Contract Compiler** - If you like compilers/PLT
   - **Auto Learner** - If you like machine learning
   - **Semantic Lattice** - If you like knowledge graphs
   - **Parallel Framework** - If you like distributed systems

9. Read that system's:
   - `docs/SPEC.md` - Specification
   - `src/lib.rs` - Public API
   - `tests/integration/` - Integration tests

### Code Navigation Tips

**Find a function:**
```bash
# Using grep
grep -r "fn inject_fault" .

# Using ripgrep (faster)
rg "fn inject_fault"

# In rust-analyzer (VS Code)
# Press F12 on a function name to go to definition
# Press Shift+F12 to find all references
```

**Understand module structure:**
```bash
# Generate documentation
cargo doc --workspace --no-deps --open

# Opens browser with full API documentation
```

**Find examples:**
```bash
# List all examples
find . -path "*/examples/*.rs"

# Run an example
cargo run -p chaos_engine --example basic_usage
```

### Common Patterns

#### 1. Error Handling

```rust
use shared_core::error::SystemError;

// Function returning Result
pub fn risky_operation() -> Result<String, SystemError> {
    // Use ? operator for error propagation
    let data = read_file(path)?;

    // Map errors to SystemError variants
    parse_data(&data)
        .map_err(|e| SystemError::Serialization(e.to_string()))
}
```

#### 2. Logging

```rust
use tracing::{info, warn, error, debug};

pub async fn process_data(id: &str) -> Result<(), SystemError> {
    info!(id = %id, "Starting data processing");

    match dangerous_operation(id).await {
        Ok(result) => {
            debug!(id = %id, result = ?result, "Operation succeeded");
            Ok(())
        }
        Err(e) => {
            error!(id = %id, error = %e, "Operation failed");
            Err(e)
        }
    }
}
```

#### 3. Async Operations

```rust
use tokio;

#[tokio::main]
async fn main() -> Result<(), SystemError> {
    // Initialize runtime
    shared_core::logging::init_logging("info")?;

    // Run async operations
    let result = async_operation().await?;

    Ok(())
}

async fn async_operation() -> Result<String, SystemError> {
    // Spawn concurrent tasks
    let handle1 = tokio::spawn(async { task1().await });
    let handle2 = tokio::spawn(async { task2().await });

    // Wait for both
    let (r1, r2) = tokio::try_join!(handle1, handle2)?;

    Ok(format!("{} {}", r1?, r2?))
}
```

#### 4. Configuration

```rust
use shared_core::config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
struct MyConfig {
    timeout_ms: u64,
    max_retries: u32,
}

fn load_config() -> Result<MyConfig, SystemError> {
    let config = Config::from_env()?;
    let my_config = config.get::<MyConfig>("my_service")?;
    Ok(my_config)
}
```

### Testing Patterns

#### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operation() {
        let result = my_function(42);
        assert_eq!(result, 84);
    }

    #[tokio::test]
    async fn test_async_operation() {
        let result = async_function().await.unwrap();
        assert!(result.is_valid());
    }
}
```

#### Integration Tests

```rust
// tests/integration/my_test.rs
use my_crate::{System, Config};

#[tokio::test]
async fn test_full_workflow() {
    // Setup
    let config = Config::default();
    let mut system = System::new(config).await.unwrap();

    // Execute
    system.start().await.unwrap();
    let result = system.process(input).await.unwrap();

    // Assert
    assert_eq!(result.status, "success");

    // Cleanup
    system.shutdown().await.unwrap();
}
```

---

## Day 3: Your First Contribution

### Choose a Starter Task

**Good First Issues:**

1. **Add a test** - Improve test coverage in any system
2. **Fix documentation** - Improve comments or README
3. **Add example** - Create example program for a system
4. **Add validation** - Improve error messages or validation

**Example: Add a test to Chaos Engine**

### Step-by-Step: Add a Test

#### 1. Find existing tests

```bash
cd chaos_engine
cat tests/integration/basic_workflow.rs
```

#### 2. Identify gap

Look for untested functionality:
- Edge cases
- Error conditions
- Different parameter combinations

#### 3. Write test

Create `tests/integration/error_handling.rs`:

```rust
use chaos_engine::{ChaosEngine, ChaosConfig, FaultType};
use shared_core::resource_governor::{ResourceGovernor, ResourceConfig};

#[tokio::test]
async fn test_invalid_fault_parameters() {
    let governor = ResourceGovernor::new(ResourceConfig::default());
    let config = ChaosConfig::default();
    let mut engine = ChaosEngine::new(config, governor).await.unwrap();

    // Test: CPU fault with invalid duration should fail
    let result = engine.inject_fault(FaultType::CpuHog {
        duration_sec: 0  // Invalid: zero duration
    }).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid duration"));
}

#[tokio::test]
async fn test_concurrent_fault_limit() {
    let governor = ResourceGovernor::new(ResourceConfig::default());
    let config = ChaosConfig {
        max_concurrent_faults: 2,
        ..Default::default()
    };
    let mut engine = ChaosEngine::new(config, governor).await.unwrap();

    // Inject 2 faults (should succeed)
    engine.inject_fault(FaultType::CpuHog { duration_sec: 10 }).await.unwrap();
    engine.inject_fault(FaultType::CpuHog { duration_sec: 10 }).await.unwrap();

    // Third fault should fail due to limit
    let result = engine.inject_fault(FaultType::CpuHog { duration_sec: 10 }).await;
    assert!(result.is_err());
}
```

#### 4. Run test

```bash
cargo test --test error_handling

# Expected output:
# running 2 tests
# test test_invalid_fault_parameters ... ok
# test test_concurrent_fault_limit ... ok
```

#### 5. Commit changes

```bash
git add tests/integration/error_handling.rs
git commit -m "Add error handling tests for Chaos Engine

- Test invalid fault parameters
- Test concurrent fault limits
- Improve test coverage for edge cases"
```

#### 6. Create pull request

```bash
git push origin feature/your-name-first-feature

# Then open PR on GitHub with description:
# - What: Added error handling tests
# - Why: Improve test coverage
# - How: Created new integration test file
```

### Code Review Checklist

Before submitting PR:

- [ ] Tests pass: `cargo test --workspace`
- [ ] Linting passes: `cargo clippy --workspace`
- [ ] Formatting correct: `cargo fmt --all`
- [ ] Documentation updated (if needed)
- [ ] Commit message is descriptive
- [ ] No compiler warnings
- [ ] Code follows project conventions

---

## Common Tasks Reference

### Building

```bash
# Build everything
cargo build --workspace

# Build specific crate
cargo build -p chaos_engine

# Build with optimizations
cargo build --release --workspace

# Build single binary
cargo build -p chaos_engine --bin chaos-cli
```

### Testing

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p shared_core

# Specific test
cargo test -p chaos_engine test_name

# Integration tests only
cargo test --test integration_test_name

# Show output (even for passing tests)
cargo test -- --nocapture

# Run tests in parallel (default) or serial
cargo test -- --test-threads=1
```

### Linting & Formatting

```bash
# Check formatting
cargo fmt --all -- --check

# Auto-format
cargo fmt --all

# Run Clippy (linter)
cargo clippy --workspace

# Clippy with all features
cargo clippy --workspace --all-features

# Fix some Clippy warnings automatically
cargo clippy --fix
```

### Documentation

```bash
# Generate docs
cargo doc --workspace --no-deps

# Generate and open in browser
cargo doc --workspace --no-deps --open

# Check doc examples
cargo test --doc

# Document private items (for internal dev)
cargo doc --workspace --document-private-items
```

### Running Examples

```bash
# List examples
cargo run -p chaos_engine --example

# Run specific example
cargo run -p chaos_engine --example basic_usage

# Run with arguments
cargo run -p chaos_engine --example cli -- --help
```

### Benchmarking

```bash
# Run all benchmarks
cargo bench --workspace

# Specific benchmark
cargo bench -p chaos_engine

# Generate flamegraph
cargo install flamegraph
cargo flamegraph --bench benchmark_name
```

### Dependency Management

```bash
# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update

# Add dependency to specific crate
cd chaos_engine
cargo add tokio --features full

# Remove dependency
cargo remove tokio
```

### Debugging

```bash
# Build with debug symbols
cargo build

# Run with debugger (GDB)
rust-gdb target/debug/my_binary

# Run with debugger (LLDB, macOS)
rust-lldb target/debug/my_binary

# Print backtrace on panic
RUST_BACKTRACE=1 cargo test

# Full backtrace
RUST_BACKTRACE=full cargo run
```

---

## Architecture Deep Dive

### Shared Core Library

**Purpose:** Provide common infrastructure for all systems

**Key Modules:**

1. **Error Handling** (`error.rs`)
   - Unified `SystemError` enum
   - Conversion from common error types
   - Serialization support

2. **Resource Governor** (`resource_governor.rs`)
   - CPU throttling
   - Memory limits
   - I/O rate limiting
   - Deterministic execution
   - Sandbox mode

3. **Plugin System** (`plugin.rs`)
   - Dynamic plugin loading
   - Lifecycle management
   - Type-safe downcasting
   - State transitions

4. **Cryptography** (`crypto.rs`)
   - Ed25519 signatures
   - BLAKE3 hashing
   - AES-256-GCM encryption

5. **Logging** (`logging.rs`)
   - Structured logging via `tracing`
   - Configurable levels
   - JSON output

### System Architecture Patterns

**1. Async-First Design**
- All I/O operations are async
- Use Tokio runtime
- Structured concurrency with `tokio::spawn`

**2. Error Propagation**
- Use `Result<T, SystemError>` for all fallible operations
- `?` operator for propagation
- Context-rich errors

**3. Configuration**
- Environment variables
- TOML config files
- Defaults in code
- Hierarchical override

**4. Testing Strategy**
- 70% unit tests (inline with code)
- 25% integration tests (`tests/integration/`)
- 5% end-to-end tests

---

## Development Workflow

### Branching Strategy

```
main
  ├── feature/add-new-fault-type
  ├── bugfix/fix-memory-leak
  └── docs/improve-onboarding
```

**Branch Naming:**
- `feature/description` - New features
- `bugfix/description` - Bug fixes
- `docs/description` - Documentation
- `refactor/description` - Refactoring
- `test/description` - Test improvements

### Commit Message Format

```
<type>: <subject>

<body>

<footer>
```

**Example:**
```
feat: Add network latency fault injection

Implement NetworkLatency fault type for Chaos Engine.
Supports configurable delay and jitter parameters.

Closes #123
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `test`: Tests
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `chore`: Maintenance

### Pull Request Process

1. **Create branch** from main
2. **Make changes** with clear commits
3. **Push branch** to origin
4. **Open PR** with description
5. **Address review** comments
6. **Squash merge** when approved

**PR Template:**

```markdown
## Description
Brief description of changes

## Motivation
Why are these changes needed?

## Changes
- Added X
- Modified Y
- Removed Z

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests pass
- [ ] Manual testing performed

## Checklist
- [ ] Code follows style guide
- [ ] Documentation updated
- [ ] No new warnings
- [ ] All tests pass
```

---

## Debugging Guide

### Common Issues

#### 1. Compilation Errors

**Issue:** Borrow checker errors

```rust
// ❌ Wrong: value moved
let data = vec![1, 2, 3];
process(data);
process(data);  // Error: value used after move

// ✅ Correct: clone or borrow
let data = vec![1, 2, 3];
process(data.clone());
process(data);

// Or use references
let data = vec![1, 2, 3];
process_ref(&data);
process_ref(&data);
```

#### 2. Async Runtime Errors

**Issue:** Blocking in async context

```rust
// ❌ Wrong: blocking call in async
async fn bad_async() {
    std::thread::sleep(Duration::from_secs(1));  // Blocks entire runtime!
}

// ✅ Correct: use async sleep
async fn good_async() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

#### 3. Test Failures

**Issue:** Flaky async tests

```rust
// ❌ Wrong: timing assumptions
#[tokio::test]
async fn flaky_test() {
    spawn_background_task();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(task_completed());  // May fail randomly
}

// ✅ Correct: wait for condition
#[tokio::test]
async fn reliable_test() {
    let handle = spawn_background_task();
    handle.await.unwrap();
    assert!(task_completed());
}
```

### Debugging Tools

**1. Print Debugging**
```rust
println!("Debug: value = {:?}", value);
dbg!(value);  // Prints with file/line info
```

**2. Logging**
```rust
tracing::debug!("Processing item: {:?}", item);
```

**3. Debugger (LLDB/GDB)**
```bash
# Build with debug info
cargo build

# Run with debugger
rust-lldb target/debug/my_binary

# Set breakpoint
(lldb) breakpoint set --name main
(lldb) run

# Step through
(lldb) next
(lldb) step
(lldb) continue

# Inspect variables
(lldb) frame variable
(lldb) print my_var
```

**4. Tokio Console**
```bash
# Add dependency
cargo add tokio --features tracing

# Run with console
cargo run --features tokio-console

# In another terminal
tokio-console
```

---

## Getting Help

### Resources

**Documentation:**
- 📖 `README.md` - Quick start
- 📖 `ROOT_ARCHITECTURE.md` - Architecture guide
- 📖 `RELEASE_NOTES.md` - Changelog
- 📖 `cargo doc --open` - API docs

**Communication:**
- 💬 GitHub Discussions - Questions and ideas
- 🐛 GitHub Issues - Bug reports
- 📧 Email - maintainer@example.com (replace)

**Learning:**
- 🦀 [Rust Book](https://doc.rust-lang.org/book/)
- 🌊 [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- 🔬 [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Office Hours

**Virtual Office Hours:** (Configure as needed)
- **When:** Tuesdays 2-3 PM PST
- **Where:** Zoom/Discord link
- **What:** Ask questions, pair programming, code reviews

### Mentorship

New contributors are paired with experienced developers for:
- Code review guidance
- Architecture questions
- Career development

---

## Next Steps

### Week 1 Goals

- [ ] Complete environment setup
- [ ] Read core documentation
- [ ] Build and test successfully
- [ ] Make first contribution (small PR)

### Month 1 Goals

- [ ] Contribute to 3+ PRs
- [ ] Review others' PRs
- [ ] Deep dive into one system
- [ ] Propose a feature or improvement

### Beyond

- Lead a feature implementation
- Mentor new contributors
- Present at team meeting
- Write blog post about your work

---

## Congratulations!

You're now ready to contribute to the Multi-System Monorepo! 🎉

**Remember:**
- Ask questions early and often
- Read existing code for patterns
- Start small and iterate
- Tests are your friend
- Have fun building distributed systems!

---

*Last Updated: 2025-11-19*
*Document Version: 1.0.0*
*Questions? Open a GitHub Discussion!*
