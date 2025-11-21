# System Packaging Guide

**Version:** 1.0.0
**Last Updated:** 2025-11-19

---

## Overview

This guide explains how each system in the monorepo is packaged as:
1. **Standalone Library** - Reusable Rust crate
2. **CLI Tool** - Command-line interface
3. **API Documentation** - Comprehensive API docs
4. **Examples** - Usage demonstrations

---

## Table of Contents

- [General Packaging Structure](#general-packaging-structure)
- [System-by-System Packaging](#system-by-system-packaging)
  - [1. Chaos Engine](#1-chaos-engine)
  - [2. Contract Executable Compiler](#2-contract-executable-compiler)
  - [3. Cross-Domain Auto Learner](#3-cross-domain-auto-learner)
  - [4. Dynamic Semantic Lattice Engine](#4-dynamic-semantic-lattice-engine)
  - [5. Linux Scalable Cloud Kernel](#5-linux-scalable-cloud-kernel)
  - [6. Parallel Architecture Framework](#6-parallel-architecture-framework)
  - [7. Symbolic Reduction Modeler](#7-symbolic-reduction-modeler)
  - [8. Synthetic Pipeline Engine](#8-synthetic-pipeline-engine)
  - [9. Universal Attestation Authority](#9-universal-attestation-authority)
  - [10. Cross-Domain Autoblocker Ledger](#10-cross-domain-autoblocker-ledger)
- [Shared Core Library](#shared-core-library)
- [Publishing to crates.io](#publishing-to-cratesio)
- [Docker Packaging](#docker-packaging)
- [Binary Distribution](#binary-distribution)

---

## General Packaging Structure

Each system follows a consistent packaging pattern:

```
<system_name>/
├── src/
│   ├── lib.rs              # Library API (public interface)
│   ├── main.rs             # CLI entry point (optional)
│   ├── api.rs              # API types and traits
│   ├── core.rs             # Core domain logic
│   ├── config.rs           # Configuration
│   └── [modules].rs        # Domain-specific modules
├── examples/
│   ├── basic_usage.rs      # Simple usage example
│   ├── advanced.rs         # Advanced features
│   └── integration.rs      # Integration with other systems
├── tests/
│   ├── unit/               # Unit tests (inline with code)
│   └── integration/        # Integration tests
├── docs/
│   ├── SPEC.md             # Formal specification
│   ├── ARCHITECTURE.md     # Architecture guide
│   └── API.md              # API documentation
├── benches/
│   └── benchmarks.rs       # Performance benchmarks
├── Cargo.toml              # Package metadata
└── README.md               # System-specific README
```

---

## System-by-System Packaging

### 1. Chaos Engine

**Purpose:** Fault injection and resilience testing for distributed systems

#### Library Usage

```rust
// In your Cargo.toml
[dependencies]
chaos_engine = { path = "../chaos_engine" }
shared_core = { path = "../shared_core" }
tokio = { version = "1.35", features = ["full"] }
```

```rust
use chaos_engine::{ChaosEngine, ChaosConfig, FaultType};
use shared_core::resource_governor::{ResourceGovernor, ResourceConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure resource governor
    let resource_config = ResourceConfig {
        cpu_percent_cap: Some(80.0),
        ram_limit_bytes: Some(2_000_000_000), // 2GB
        max_io_ops_per_sec: Some(1000.0),
        deterministic: false,
        sandbox_mode: false,
    };
    let governor = ResourceGovernor::new(resource_config);

    // Initialize chaos engine
    let config = ChaosConfig::default();
    let mut engine = ChaosEngine::new(config, governor).await?;

    // Inject CPU fault
    engine.inject_fault(FaultType::CpuHog {
        duration_sec: 10
    }).await?;

    // Inject network fault
    engine.inject_fault(FaultType::NetworkLatency {
        delay_ms: 500,
        jitter_ms: 100
    }).await?;

    // Generate report
    let report = engine.generate_report().await?;
    println!("Chaos Report:\n{:#?}", report);

    Ok(())
}
```

#### CLI Tool

```bash
# Build CLI
cd chaos_engine
cargo build --release --bin chaos-cli

# Run fault injection
./target/release/chaos-cli inject \
  --fault cpu \
  --duration 10 \
  --target-service my-app

# List active faults
./target/release/chaos-cli list

# Generate report
./target/release/chaos-cli report --format json > report.json

# Stop all faults
./target/release/chaos-cli stop-all
```

#### CLI Implementation

Create `chaos_engine/src/bin/chaos-cli.rs`:

```rust
use clap::{Parser, Subcommand};
use chaos_engine::{ChaosEngine, ChaosConfig, FaultType};
use shared_core::resource_governor::{ResourceGovernor, ResourceConfig};

#[derive(Parser)]
#[command(name = "chaos-cli")]
#[command(about = "Chaos engineering CLI for fault injection")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inject a fault
    Inject {
        #[arg(long)]
        fault: String,
        #[arg(long)]
        duration: u64,
        #[arg(long)]
        target_service: Option<String>,
    },
    /// List active faults
    List,
    /// Generate report
    Report {
        #[arg(long, default_value = "text")]
        format: String,
    },
    /// Stop all faults
    StopAll,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Implementation here...

    Ok(())
}
```

#### API Documentation

Generate docs:
```bash
cd chaos_engine
cargo doc --no-deps --open
```

Key API types:
- `ChaosEngine` - Main engine struct
- `FaultType` - Enum of fault types
- `ChaosConfig` - Configuration
- `FaultReport` - Fault execution report

---

### 2. Contract Executable Compiler

**Purpose:** DSL compiler for executable smart contracts with formal verification

#### Library Usage

```rust
use contract_executable_compiler::{Compiler, CompilerConfig, SourceCode};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize compiler
    let config = CompilerConfig {
        optimization_level: 2,
        target_arch: "x86_64".to_string(),
        enable_verification: true,
    };
    let compiler = Compiler::new(config)?;

    // Load contract source
    let source = SourceCode::from_file(Path::new("contracts/example.ctr"))?;

    // Compile contract
    let compiled = compiler.compile(source).await?;

    // Verify correctness
    let verification = compiler.verify(&compiled).await?;
    println!("Verification: {:?}", verification);

    // Generate executable
    compiled.write_to_file(Path::new("build/example.exe"))?;

    Ok(())
}
```

#### CLI Tool

```bash
# Compile contract
contract-cli compile \
  --input contracts/example.ctr \
  --output build/example.exe \
  --optimize 2 \
  --verify

# Verify contract
contract-cli verify build/example.exe

# Run contract
contract-cli execute build/example.exe \
  --args '{"amount": 100}'

# Decompile for inspection
contract-cli decompile build/example.exe > example.ctr
```

#### Example Contract

```
// example.ctr - Contract DSL
contract TransferFunds {
  state {
    balances: Map<Address, u64>;
  }

  constraints {
    // Balance never goes negative
    forall addr: balances[addr] >= 0;

    // Total supply is conserved
    sum(balances.values()) == INITIAL_SUPPLY;
  }

  function transfer(from: Address, to: Address, amount: u64) {
    require(balances[from] >= amount);
    balances[from] -= amount;
    balances[to] += amount;
    emit TransferEvent(from, to, amount);
  }
}
```

---

### 3. Cross-Domain Auto Learner

**Purpose:** Adaptive machine learning across heterogeneous domains

#### Library Usage

```rust
use cross_domain_auto_learner::{AutoLearner, Domain, TrainingConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize learner
    let config = TrainingConfig {
        learning_rate: 0.001,
        batch_size: 32,
        max_epochs: 100,
    };
    let mut learner = AutoLearner::new(config)?;

    // Define domains
    let domain_a = Domain::new("image_classification", "data/images");
    let domain_b = Domain::new("text_classification", "data/text");

    // Train on domain A
    learner.train_on_domain(domain_a).await?;

    // Transfer learning to domain B
    learner.adapt_to_domain(domain_b).await?;

    // Make predictions
    let prediction = learner.predict(input_data).await?;
    println!("Prediction: {:?}", prediction);

    // Export model
    learner.export_model("models/trained.model")?;

    Ok(())
}
```

#### CLI Tool

```bash
# Train model
auto-learner train \
  --domain images \
  --data data/images \
  --output models/image_model.bin

# Adapt to new domain
auto-learner adapt \
  --source-model models/image_model.bin \
  --target-domain text \
  --data data/text \
  --output models/text_model.bin

# Evaluate model
auto-learner evaluate \
  --model models/text_model.bin \
  --test-data data/test

# Serve model via API
auto-learner serve \
  --model models/text_model.bin \
  --port 8080
```

---

### 4. Dynamic Semantic Lattice Engine

**Purpose:** Semantic reasoning engine with lattice-based inference

#### Library Usage

```rust
use dynamic_semantic_lattice_engine::{LatticeEngine, Query, Ontology};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load ontology
    let ontology = Ontology::from_file("ontologies/knowledge.owl")?;

    // Initialize engine
    let mut engine = LatticeEngine::new(ontology)?;

    // Add knowledge
    engine.add_fact("Person", "John", "age", 30).await?;
    engine.add_fact("Person", "John", "occupation", "Engineer").await?;

    // Query knowledge
    let query = Query::new("SELECT ?name WHERE { ?name occupation Engineer }");
    let results = engine.execute_query(query).await?;

    for result in results {
        println!("Result: {:?}", result);
    }

    // Infer new knowledge
    let inferred = engine.infer_all().await?;
    println!("Inferred {} new facts", inferred.len());

    Ok(())
}
```

#### CLI Tool

```bash
# Start interactive REPL
lattice-cli repl --ontology ontologies/knowledge.owl

# Execute query
lattice-cli query \
  --ontology ontologies/knowledge.owl \
  --query "SELECT ?x WHERE { ?x type Person }"

# Load and infer
lattice-cli infer \
  --ontology ontologies/knowledge.owl \
  --output ontologies/inferred.owl

# Validate ontology
lattice-cli validate ontologies/knowledge.owl
```

---

### 5. Linux Scalable Cloud Kernel

**Purpose:** Kernel modules for cloud-scale Linux operations

#### Library Usage

```rust
use linux_scalable_cloud_kernel::{KernelModule, ModuleConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load kernel module (requires root)
    let config = ModuleConfig {
        name: "cloud_scheduler".to_string(),
        params: vec![
            ("max_tasks", "10000"),
            ("scheduler_policy", "fair"),
        ],
    };

    let module = KernelModule::load(config).await?;

    // Monitor module
    let stats = module.get_stats().await?;
    println!("Module stats: {:#?}", stats);

    // Unload module
    module.unload().await?;

    Ok(())
}
```

#### CLI Tool

```bash
# Load kernel module (requires sudo)
sudo cloud-kernel load \
  --module cloud_scheduler \
  --param max_tasks=10000

# List loaded modules
cloud-kernel list

# Get module stats
cloud-kernel stats cloud_scheduler

# Unload module
sudo cloud-kernel unload cloud_scheduler

# Build and install eBPF program
cloud-kernel ebpf \
  --program network_filter.bpf.c \
  --attach xdp \
  --interface eth0
```

---

### 6. Parallel Architecture Framework

**Purpose:** Distributed computation orchestration with work-stealing

#### Library Usage

```rust
use parallel_architecture_framework::{Framework, Task, TaskGraph};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize framework
    let framework = Framework::new().await?;

    // Build task graph
    let mut graph = TaskGraph::new();

    let task_a = Task::new("compute_a", || {
        // Computation here
        42
    });

    let task_b = Task::new("compute_b", || {
        // Computation here
        100
    });

    let task_c = Task::new("combine", |a, b| {
        a + b
    }).depends_on(vec![task_a.id(), task_b.id()]);

    graph.add_task(task_a);
    graph.add_task(task_b);
    graph.add_task(task_c);

    // Execute graph
    let results = framework.execute(graph).await?;
    println!("Results: {:?}", results);

    Ok(())
}
```

#### CLI Tool

```bash
# Submit task graph
parallel-cli submit \
  --graph tasks/workflow.yaml \
  --workers 8

# Monitor execution
parallel-cli status --job-id abc123

# Get results
parallel-cli results --job-id abc123

# Cancel job
parallel-cli cancel --job-id abc123
```

---

### 7. Symbolic Reduction Modeler

**Purpose:** Symbolic computation and algebraic simplification

#### Library Usage

```rust
use symbolic_reduction_modeler::{Expression, Modeler, SimplificationRule};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse expression
    let expr = Expression::parse("(x + 0) * (y / y)")?;

    // Create modeler
    let modeler = Modeler::new();

    // Simplify
    let simplified = modeler.simplify(expr)?;
    println!("Simplified: {}", simplified); // Output: x

    // Apply custom rules
    let rule = SimplificationRule::new("x^2 - y^2", "(x+y)*(x-y)");
    let result = modeler.apply_rule(expr, rule)?;

    Ok(())
}
```

#### CLI Tool

```bash
# Simplify expression
symbolic-cli simplify "(x + 0) * (y / y)"

# Apply custom rules
symbolic-cli reduce \
  --expr "x^2 - y^2" \
  --rules rules/algebra.txt

# Verify equivalence
symbolic-cli verify \
  --left "(x+y)^2" \
  --right "x^2 + 2xy + y^2"

# Interactive REPL
symbolic-cli repl
```

---

### 8. Synthetic Pipeline Engine

**Purpose:** Data pipeline orchestration with stream processing

#### Library Usage

```rust
use synthetic_pipeline_engine::{Pipeline, Stage, DataStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create pipeline
    let mut pipeline = Pipeline::new("etl_pipeline");

    // Add stages
    pipeline
        .add_stage(Stage::source("kafka", "topic-input"))
        .add_stage(Stage::transform("parse_json"))
        .add_stage(Stage::filter(|record| record["status"] == "active"))
        .add_stage(Stage::transform("enrich"))
        .add_stage(Stage::sink("postgres", "table-output"));

    // Configure backpressure
    pipeline.set_backpressure_limit(1000);

    // Start pipeline
    pipeline.start().await?;

    // Monitor metrics
    let metrics = pipeline.get_metrics().await?;
    println!("Throughput: {} records/sec", metrics.throughput);

    // Stop pipeline
    pipeline.stop().await?;

    Ok(())
}
```

#### CLI Tool

```bash
# Start pipeline
pipeline-cli start \
  --config pipelines/etl.yaml \
  --workers 4

# Monitor pipeline
pipeline-cli monitor --pipeline etl_pipeline

# Pause pipeline
pipeline-cli pause --pipeline etl_pipeline

# Resume pipeline
pipeline-cli resume --pipeline etl_pipeline

# Get metrics
pipeline-cli metrics --pipeline etl_pipeline --format json
```

---

### 9. Universal Attestation Authority

**Purpose:** Cryptographic attestation service with TPM integration

#### Library Usage

```rust
use universal_attestation_authority::{AttestationAuthority, AttestationRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize authority
    let authority = AttestationAuthority::new().await?;

    // Create attestation request
    let request = AttestationRequest {
        nonce: authority.generate_nonce()?,
        pcr_selection: vec![0, 1, 2, 3],
        quote_type: "TPM2.0".to_string(),
    };

    // Generate attestation
    let attestation = authority.attest(request).await?;

    // Verify attestation
    let verified = authority.verify(&attestation).await?;
    println!("Attestation valid: {}", verified);

    // Export certificate
    attestation.export_certificate("certs/device.pem")?;

    Ok(())
}
```

#### CLI Tool

```bash
# Initialize TPM
attestation-cli init --tpm /dev/tpm0

# Generate attestation
attestation-cli attest \
  --output attestation.json \
  --pcrs 0,1,2,3

# Verify attestation
attestation-cli verify attestation.json

# Export certificate
attestation-cli export-cert \
  --attestation attestation.json \
  --output device.pem

# Start attestation server
attestation-cli serve \
  --port 8443 \
  --tls-cert server.crt \
  --tls-key server.key
```

---

### 10. Cross-Domain Autoblocker Ledger

**Purpose:** Distributed security event ledger with Byzantine consensus

#### Library Usage

```rust
use cross_domain_autoblocker_ledger::{Ledger, SecurityEvent, ConsensusConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize ledger
    let config = ConsensusConfig {
        quorum_size: 3,
        timeout_ms: 5000,
    };
    let ledger = Ledger::new(config).await?;

    // Record security event
    let event = SecurityEvent {
        timestamp: chrono::Utc::now(),
        event_type: "malicious_ip".to_string(),
        source: "192.168.1.100".to_string(),
        severity: "high".to_string(),
        metadata: serde_json::json!({"attempts": 5}),
    };

    ledger.record_event(event).await?;

    // Query events
    let events = ledger.query()
        .filter_by_type("malicious_ip")
        .since(chrono::Utc::now() - chrono::Duration::hours(24))
        .execute()
        .await?;

    for event in events {
        println!("Event: {:?}", event);
    }

    // Sync with other nodes
    ledger.sync_with_peers().await?;

    Ok(())
}
```

#### CLI Tool

```bash
# Start ledger node
autoblocker-cli node \
  --port 9000 \
  --peers node1:9000,node2:9000,node3:9000

# Record event
autoblocker-cli record \
  --type malicious_ip \
  --source 192.168.1.100 \
  --severity high

# Query events
autoblocker-cli query \
  --type malicious_ip \
  --since "24h ago" \
  --format json

# Export ledger
autoblocker-cli export \
  --output ledger_backup.json

# Verify ledger integrity
autoblocker-cli verify
```

---

## Shared Core Library

**Purpose:** Common infrastructure used by all systems

### Library Usage

```rust
use shared_core::{
    error::SystemError,
    logging::init_logging,
    crypto::{sign, verify, hash},
    config::Config,
    resource_governor::{ResourceGovernor, ResourceConfig},
    plugin::{Plugin, PluginRegistry},
};

#[tokio::main]
async fn main() -> Result<(), SystemError> {
    // Initialize logging
    init_logging("info")?;

    // Load configuration
    let config = Config::from_env()?;

    // Create resource governor
    let resource_config = ResourceConfig {
        cpu_percent_cap: Some(75.0),
        ram_limit_bytes: Some(4_000_000_000),
        max_io_ops_per_sec: Some(5000.0),
        deterministic: false,
        sandbox_mode: false,
    };
    let governor = ResourceGovernor::new(resource_config);

    // Use cryptography
    let message = b"Hello, World!";
    let signature = sign(message)?;
    let valid = verify(message, &signature)?;
    assert!(valid);

    let hash = hash(message);
    println!("Hash: {}", hex::encode(hash));

    Ok(())
}
```

### API Documentation

All shared_core modules:
- `error` - Unified error handling
- `logging` - Structured logging
- `telemetry` - OpenTelemetry integration
- `crypto` - Cryptographic primitives
- `config` - Configuration management
- `types` - Common types
- `resource_governor` - Resource throttling
- `plugin` - Plugin system

---

## Publishing to crates.io

### Prerequisites

1. **Create crates.io account**: https://crates.io/
2. **Login via cargo**:
   ```bash
   cargo login <your-api-token>
   ```

### Publishing Shared Core

```bash
cd shared_core
cargo publish
```

### Publishing Systems

```bash
# Publish systems in dependency order
cd chaos_engine
cargo publish

cd ../contract_executable_compiler
cargo publish

# ... repeat for all systems
```

### Versioning

Follow Semantic Versioning 2.0:
- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible functionality
- **PATCH**: Backwards-compatible bug fixes

Example:
```toml
[package]
name = "chaos_engine"
version = "1.0.0"  # Start at 1.0.0 for initial release
```

---

## Docker Packaging

### Multi-Stage Dockerfile

Create `Dockerfile` at repository root:

```dockerfile
# Build stage
FROM rust:1.81 as builder

WORKDIR /app
COPY . .

RUN cargo build --release --workspace

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binaries
COPY --from=builder /app/target/release/chaos-cli /usr/local/bin/
COPY --from=builder /app/target/release/contract-cli /usr/local/bin/
COPY --from=builder /app/target/release/auto-learner /usr/local/bin/
# ... copy other binaries

# Create non-root user
RUN useradd -m -u 1000 appuser
USER appuser

ENTRYPOINT ["/bin/bash"]
```

### Build Docker Image

```bash
docker build -t multi-system-monorepo:1.0.0 .
```

### Run System in Docker

```bash
# Run chaos engine
docker run -it multi-system-monorepo:1.0.0 \
  chaos-cli inject --fault cpu --duration 10

# Run contract compiler
docker run -v $(pwd)/contracts:/contracts \
  multi-system-monorepo:1.0.0 \
  contract-cli compile --input /contracts/example.ctr
```

### Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  chaos-engine:
    image: multi-system-monorepo:1.0.0
    command: chaos-cli serve --port 8001
    ports:
      - "8001:8001"

  attestation-authority:
    image: multi-system-monorepo:1.0.0
    command: attestation-cli serve --port 8443
    ports:
      - "8443:8443"
    volumes:
      - ./certs:/certs

  autoblocker-ledger:
    image: multi-system-monorepo:1.0.0
    command: autoblocker-cli node --port 9000
    ports:
      - "9000:9000"
```

---

## Binary Distribution

### GitHub Releases

Use GitHub Actions to build binaries for multiple platforms:

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: 1.81

      - name: Build
        run: cargo build --release --workspace

      - name: Package
        run: |
          tar czf binaries-${{ matrix.os }}.tar.gz \
            -C target/release \
            chaos-cli \
            contract-cli \
            # ... other binaries

      - name: Upload
        uses: actions/upload-artifact@v3
        with:
          name: binaries-${{ matrix.os }}
          path: binaries-${{ matrix.os }}.tar.gz
```

### Manual Binary Build

```bash
# Build for current platform
cargo build --release --workspace

# Cross-compile for other platforms
cargo install cross
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-apple-darwin
cross build --release --target x86_64-pc-windows-gnu
```

---

## Summary

Each system in the monorepo is packaged as:

1. **Library** - Reusable via `Cargo.toml` dependencies
2. **CLI Tool** - Binary in `src/bin/` or `src/main.rs`
3. **API Docs** - Generated via `cargo doc`
4. **Examples** - In `examples/` directory
5. **Docker Image** - Multi-stage Dockerfile
6. **Binary Releases** - GitHub Actions CI/CD

**Next Steps:**
1. Implement CLI tools for each system
2. Add comprehensive examples
3. Generate API documentation
4. Create Docker images
5. Set up GitHub Actions for releases

---

*Last Updated: 2025-11-19*
*Document Version: 1.0.0*
