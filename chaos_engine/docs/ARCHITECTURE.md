# Chaos Engine Architecture

**Version:** 0.1.0
**Last Updated:** 2025-11-17

---

## Table of Contents

1. [System Overview](#system-overview)
2. [Module Breakdown](#module-breakdown)
3. [Internal Services](#internal-services)
4. [Plugin System](#plugin-system)
5. [Algorithms](#algorithms)
6. [Schema Diagrams](#schema-diagrams)
7. [Decision Boundaries](#decision-boundaries)
8. [Concurrency Model](#concurrency-model)
9. [Data Flow](#data-flow)
10. [Fault Tolerance](#fault-tolerance)

---

## 1. System Overview

### 1.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                       Chaos Engine                               │
│                                                                  │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐                │
│  │    API     │  │    Core    │  │  Storage   │                │
│  │  Gateway   │──│   Engine   │──│   Layer    │                │
│  └────────────┘  └────────────┘  └────────────┘                │
│         │              │                 │                       │
│         │              │                 │                       │
│  ┌──────┴──────┐  ┌───┴───┐  ┌──────────┴───────┐             │
│  │             │  │       │  │                  │              │
│  ▼             ▼  ▼       ▼  ▼                  ▼              │
│  ┌────────┐  ┌────────┐  ┌────────┐  ┌────────────┐           │
│  │Strategy│  │Injector│  │Observer│  │  Reporter  │           │
│  │Manager │  │ Pool   │  │ Pool   │  │  Service   │           │
│  └────────┘  └────────┘  └────────┘  └────────────┘           │
│       │           │           │              │                  │
└───────┼───────────┼───────────┼──────────────┼──────────────────┘
        │           │           │              │
        ▼           ▼           ▼              ▼
   ┌────────┐  ┌────────┐  ┌────────┐    ┌────────┐
   │Backends│  │Targets │  │Metrics │    │  Logs  │
   └────────┘  └────────┘  └────────┘    └────────┘
```

### 1.2 Component Responsibilities

- **API Gateway**: HTTP/gRPC interface for external interactions
- **Core Engine**: Experiment lifecycle management, state machine
- **Storage Layer**: Experiment definitions, results, audit logs
- **Strategy Manager**: Fault injection strategy selection and configuration
- **Injector Pool**: Concurrent fault injection workers
- **Observer Pool**: System state monitoring workers
- **Reporter Service**: Experiment report generation and analysis

---

## 2. Module Breakdown

### 2.1 Core Module (`src/core/`)

#### 2.1.1 Engine (`engine.rs`)
```rust
/// Main chaos engine coordinator
pub struct ChaosEngine {
    /// Configuration
    config: Arc<EngineConfig>,
    /// Experiment state store
    state_store: Arc<StateStore>,
    /// Injector pool
    injectors: Arc<InjectorPool>,
    /// Observer pool
    observers: Arc<ObserverPool>,
    /// Strategy registry
    strategies: Arc<StrategyRegistry>,
    /// Safety monitor
    safety_monitor: Arc<SafetyMonitor>,
}

impl ChaosEngine {
    /// Create a new chaos engine
    pub async fn new(config: EngineConfig) -> Result<Self>;

    /// Start the engine
    pub async fn start(&self) -> Result<()>;

    /// Shutdown gracefully
    pub async fn shutdown(&self) -> Result<()>;

    /// Submit an experiment
    pub async fn submit_experiment(&self, exp: Experiment) -> Result<ExperimentId>;

    /// Execute an experiment (internal)
    async fn execute_experiment(&self, exp: Experiment) -> Result<ExperimentReport>;
}
```

#### 2.1.2 State Machine (`state_machine.rs`)
```rust
/// Experiment state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExperimentState {
    Pending,
    Validating,
    Injecting,
    Observing,
    Cleaning,
    Completed,
    Aborted,
}

/// State machine for experiment lifecycle
pub struct StateMachine {
    state: Arc<RwLock<ExperimentState>>,
    transitions: HashMap<(ExperimentState, Event), ExperimentState>,
}

/// Events that trigger state transitions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    SubmitExperiment,
    ValidationPassed,
    ValidationFailed,
    InjectionComplete,
    ObservationComplete,
    CleanupComplete,
    EmergencyStop,
    SafetyTriggered,
}
```

#### 2.1.3 Experiment Model (`experiment.rs`)
```rust
/// Experiment execution context
pub struct ExperimentContext {
    pub id: ExperimentId,
    pub definition: Experiment,
    pub state: ExperimentState,
    pub start_time: Timestamp,
    pub end_time: Option<Timestamp>,
    pub targets: Vec<Target>,
    pub injections: Vec<InjectionRecord>,
    pub observations: Vec<Observation>,
    pub errors: Vec<ErrorRecord>,
}
```

### 2.2 Strategies Module (`src/strategies/`)

#### 2.2.1 Strategy Trait
```rust
#[async_trait]
pub trait FaultStrategy: Send + Sync {
    /// Strategy name
    fn name(&self) -> &str;

    /// Validate strategy parameters
    fn validate(&self, params: &FaultParameters) -> Result<()>;

    /// Inject fault
    async fn inject(&self, target: &Target, params: &FaultParameters) -> Result<InjectionHandle>;

    /// Remove fault
    async fn remove(&self, handle: &InjectionHandle) -> Result<()>;

    /// Check if fault is still active
    async fn is_active(&self, handle: &InjectionHandle) -> Result<bool>;
}
```

#### 2.2.2 Network Strategies
```rust
/// Network latency injection
pub struct NetworkLatencyStrategy {
    backend: Arc<dyn NetworkBackend>,
}

impl NetworkLatencyStrategy {
    async fn inject_tc_rules(&self, target: &Target, delay_ms: u64, jitter_ms: u64) -> Result<()> {
        // Uses Linux tc (traffic control) to add latency
        // tc qdisc add dev eth0 root netem delay 100ms 10ms
    }
}

/// Network partition injection
pub struct NetworkPartitionStrategy {
    backend: Arc<dyn NetworkBackend>,
}

impl NetworkPartitionStrategy {
    async fn inject_iptables_rules(&self, target: &Target, blocked: &[String]) -> Result<()> {
        // Uses iptables to block specific destinations
        // iptables -A OUTPUT -d <destination> -j DROP
    }
}
```

#### 2.2.3 Process Strategies
```rust
/// Process kill strategy
pub struct ProcessKillStrategy;

impl ProcessKillStrategy {
    async fn send_signal(&self, pid: u32, signal: Signal) -> Result<()> {
        // Send SIGKILL, SIGTERM, or custom signal
        use nix::sys::signal;
        signal::kill(Pid::from_raw(pid as i32), signal)?;
    }
}

/// Process pause strategy (SIGSTOP/SIGCONT)
pub struct ProcessPauseStrategy;
```

#### 2.2.4 Resource Strategies
```rust
/// CPU throttling using cgroups
pub struct CpuThrottleStrategy {
    cgroup_manager: Arc<CgroupManager>,
}

/// Memory limit using cgroups
pub struct MemoryLimitStrategy {
    cgroup_manager: Arc<CgroupManager>,
}

/// Disk I/O throttling
pub struct DiskIoThrottleStrategy {
    cgroup_manager: Arc<CgroupManager>,
}
```

### 2.3 Observers Module (`src/observers/`)

#### 2.3.1 Observer Trait
```rust
#[async_trait]
pub trait Observer: Send + Sync {
    /// Observer name
    fn name(&self) -> &str;

    /// Start observing
    async fn start(&self, targets: &[Target]) -> Result<ObserverHandle>;

    /// Collect current observation
    async fn observe(&self, handle: &ObserverHandle) -> Result<Observation>;

    /// Stop observing
    async fn stop(&self, handle: &ObserverHandle) -> Result<()>;
}
```

#### 2.3.2 Metric Observer
```rust
/// Observes Prometheus metrics
pub struct MetricObserver {
    prometheus_client: PrometheusClient,
    queries: Vec<MetricQuery>,
}

#[derive(Debug, Clone)]
pub struct MetricQuery {
    pub name: String,
    pub query: String,  // PromQL query
    pub evaluation_interval_ms: u64,
}
```

#### 2.3.3 Health Observer
```rust
/// Observes HTTP health endpoints
pub struct HealthObserver {
    http_client: Client,
    endpoints: Vec<HealthEndpoint>,
}

#[derive(Debug, Clone)]
pub struct HealthEndpoint {
    pub url: String,
    pub expected_status: u16,
    pub timeout_ms: u64,
}
```

#### 2.3.4 Process Observer
```rust
/// Observes process state
pub struct ProcessObserver;

impl ProcessObserver {
    async fn get_process_info(&self, pid: u32) -> Result<ProcessInfo> {
        // Read /proc/{pid}/stat, /proc/{pid}/status
    }
}

#[derive(Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub state: ProcessState,
    pub cpu_percent: f64,
    pub memory_bytes: u64,
    pub threads: u32,
}
```

### 2.4 Reporters Module (`src/reporters/`)

```rust
/// Report generator
pub struct ReportGenerator {
    template_engine: TemplateEngine,
}

impl ReportGenerator {
    pub fn generate_report(&self, context: &ExperimentContext) -> Result<ExperimentReport> {
        // Analyze steady-state results
        let steady_state_analysis = self.analyze_steady_state(context)?;

        // Aggregate observations
        let observation_summary = self.aggregate_observations(context)?;

        // Generate conclusion
        let conclusion = self.generate_conclusion(context, &steady_state_analysis)?;

        Ok(ExperimentReport {
            experiment_id: context.id.clone(),
            status: context.state.clone(),
            start_time: context.start_time,
            end_time: context.end_time,
            steady_state_results: steady_state_analysis,
            observations: observation_summary,
            conclusion,
        })
    }

    fn analyze_steady_state(&self, context: &ExperimentContext) -> Result<SteadyStateAnalysis> {
        // Compare before, during, after measurements
    }
}
```

---

## 3. Internal Services

### 3.1 Injector Pool

```rust
/// Pool of concurrent fault injectors
pub struct InjectorPool {
    workers: Vec<InjectorWorker>,
    task_queue: Arc<Mutex<VecDeque<InjectionTask>>>,
    semaphore: Arc<Semaphore>,
}

impl InjectorPool {
    pub async fn new(config: InjectorPoolConfig) -> Result<Self> {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));
        let task_queue = Arc::new(Mutex::new(VecDeque::new()));

        let mut workers = Vec::new();
        for i in 0..config.worker_count {
            let worker = InjectorWorker::new(
                format!("injector-{}", i),
                Arc::clone(&task_queue),
                Arc::clone(&semaphore),
            );
            workers.push(worker);
        }

        Ok(Self { workers, task_queue, semaphore })
    }

    pub async fn submit(&self, task: InjectionTask) -> Result<()> {
        self.task_queue.lock().await.push_back(task);
        Ok(())
    }
}

struct InjectorWorker {
    id: String,
    task_queue: Arc<Mutex<VecDeque<InjectionTask>>>,
    semaphore: Arc<Semaphore>,
}

impl InjectorWorker {
    async fn run(&self) {
        loop {
            let _permit = self.semaphore.acquire().await;

            let task = {
                let mut queue = self.task_queue.lock().await;
                queue.pop_front()
            };

            if let Some(task) = task {
                self.execute_task(task).await;
            } else {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }

    async fn execute_task(&self, task: InjectionTask) {
        tracing::info!(worker_id = %self.id, task = ?task, "Executing injection task");

        match task.strategy.inject(&task.target, &task.parameters).await {
            Ok(handle) => {
                // Record successful injection
            }
            Err(e) => {
                tracing::error!(error = ?e, "Injection failed");
            }
        }
    }
}
```

### 3.2 Safety Monitor

```rust
/// Monitors system safety and triggers rollbacks
pub struct SafetyMonitor {
    config: SafetyConfig,
    metric_store: Arc<MetricStore>,
}

impl SafetyMonitor {
    pub async fn check_safety(&self, context: &ExperimentContext) -> Result<SafetyStatus> {
        // Check blast radius
        if self.exceeds_blast_radius(context)? {
            return Ok(SafetyStatus::Violation {
                reason: "Blast radius exceeded".to_string(),
            });
        }

        // Check metric thresholds
        if self.breaches_threshold(context).await? {
            return Ok(SafetyStatus::Violation {
                reason: "Metric threshold breached".to_string(),
            });
        }

        // Check concurrent experiments
        if self.too_many_concurrent().await? {
            return Ok(SafetyStatus::Violation {
                reason: "Too many concurrent experiments".to_string(),
            });
        }

        Ok(SafetyStatus::Safe)
    }

    fn exceeds_blast_radius(&self, context: &ExperimentContext) -> Result<bool> {
        let total_targets = self.get_total_targets(&context.definition.target)?;
        let affected_count = context.targets.len();
        let percentage = (affected_count as f64) / (total_targets as f64);

        Ok(percentage > self.config.max_blast_radius_percentage)
    }

    async fn breaches_threshold(&self, context: &ExperimentContext) -> Result<bool> {
        for trigger in &context.definition.rollback.triggers {
            match trigger {
                RollbackTrigger::MetricThreshold { metric, threshold } => {
                    let current_value = self.metric_store.get_current_value(metric).await?;
                    if current_value > *threshold {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }
}

#[derive(Debug)]
pub enum SafetyStatus {
    Safe,
    Violation { reason: String },
}
```

### 3.3 State Store

```rust
/// Persistent storage for experiment state
pub struct StateStore {
    db: Arc<Database>,
}

impl StateStore {
    pub async fn save_experiment(&self, context: &ExperimentContext) -> Result<()> {
        let json = serde_json::to_string(context)?;
        self.db.put(&context.id.to_string(), json.as_bytes()).await?;
        Ok(())
    }

    pub async fn load_experiment(&self, id: &ExperimentId) -> Result<ExperimentContext> {
        let data = self.db.get(&id.to_string()).await?;
        let context = serde_json::from_slice(&data)?;
        Ok(context)
    }

    pub async fn list_experiments(&self, filter: ExperimentFilter) -> Result<Vec<ExperimentSummary>> {
        // Query database with filter
    }
}
```

---

## 4. Plugin System

### 4.1 Plugin Architecture

```rust
/// Plugin trait for extensibility
#[async_trait]
pub trait ChaosPlugin: Send + Sync {
    /// Plugin name
    fn name(&self) -> &str;

    /// Plugin version
    fn version(&self) -> Version;

    /// Initialize plugin
    async fn initialize(&mut self, config: &PluginConfig) -> Result<()>;

    /// Register strategies
    fn strategies(&self) -> Vec<Box<dyn FaultStrategy>>;

    /// Register observers
    fn observers(&self) -> Vec<Box<dyn Observer>>;

    /// Shutdown plugin
    async fn shutdown(&mut self) -> Result<()>;
}
```

### 4.2 Plugin Registry

```rust
/// Registry for managing plugins
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn ChaosPlugin>>,
}

impl PluginRegistry {
    pub fn register(&mut self, plugin: Box<dyn ChaosPlugin>) -> Result<()> {
        let name = plugin.name().to_string();
        if self.plugins.contains_key(&name) {
            return Err(SystemError::AlreadyExists {
                resource_type: "plugin".to_string(),
                identifier: name,
            });
        }
        self.plugins.insert(name, plugin);
        Ok(())
    }

    pub fn get_strategies(&self) -> Vec<Box<dyn FaultStrategy>> {
        self.plugins
            .values()
            .flat_map(|p| p.strategies())
            .collect()
    }
}
```

### 4.3 Built-in Plugins

- **Kubernetes Plugin**: Kubernetes-specific fault injection (pod deletion, namespace isolation)
- **Docker Plugin**: Docker container manipulation
- **Network Plugin**: Advanced network fault injection
- **Storage Plugin**: Disk and filesystem faults

---

## 5. Algorithms

### 5.1 Progressive Rollout Algorithm

```rust
/// Progressively inject faults into targets
pub async fn progressive_rollout(
    targets: Vec<Target>,
    strategy: Arc<dyn FaultStrategy>,
    params: FaultParameters,
    config: ProgressiveConfig,
) -> Result<Vec<InjectionHandle>> {
    let mut handles = Vec::new();
    let batch_size = (targets.len() as f64 * config.initial_percentage).ceil() as usize;
    let batch_size = batch_size.max(1);

    let mut remaining = targets;
    let mut batch_number = 1;

    while !remaining.is_empty() {
        let batch: Vec<_> = remaining.drain(..batch_size.min(remaining.len())).collect();

        tracing::info!(
            batch_number,
            batch_size = batch.len(),
            remaining = remaining.len(),
            "Injecting fault into batch"
        );

        // Inject into batch
        for target in &batch {
            let handle = strategy.inject(target, &params).await?;
            handles.push(handle);
        }

        // Wait and observe
        tokio::time::sleep(Duration::from_millis(config.observation_delay_ms)).await;

        // Check if safe to continue
        if !config.continue_on_success {
            // Wait for user approval or automated check
        }

        batch_number += 1;
    }

    Ok(handles)
}

pub struct ProgressiveConfig {
    pub initial_percentage: f64,
    pub observation_delay_ms: u64,
    pub continue_on_success: bool,
}
```

### 5.2 Steady-State Validation Algorithm

```rust
/// Validate steady-state hypothesis
pub async fn validate_steady_state(
    hypothesis: &SteadyStateHypothesis,
    observers: &[Arc<dyn Observer>],
    duration_ms: u64,
) -> Result<SteadyStateResult> {
    let mut results = Vec::new();

    let start = Instant::now();
    let mut interval = interval(Duration::from_millis(100));

    while start.elapsed().as_millis() < duration_ms as u128 {
        interval.tick().await;

        for probe in &hypothesis.probes {
            let result = evaluate_probe(probe, observers).await?;
            results.push(result);
        }
    }

    // Analyze results
    let success_count = results.iter().filter(|r| r.passed).count();
    let total_count = results.len();
    let success_rate = (success_count as f64) / (total_count as f64);

    Ok(SteadyStateResult {
        hypothesis_title: hypothesis.title.clone(),
        passed: success_rate >= 0.95,  // 95% threshold
        success_rate,
        probe_results: results,
    })
}

async fn evaluate_probe(
    probe: &Probe,
    observers: &[Arc<dyn Observer>],
) -> Result<ProbeResult> {
    match probe {
        Probe::Http { url, expected_status, tolerance } => {
            let response = reqwest::get(url).await?;
            let status_ok = response.status().as_u16() == *expected_status;

            Ok(ProbeResult {
                probe_name: url.clone(),
                passed: status_ok,
                details: format!("Status: {}", response.status()),
            })
        }
        Probe::Metric { name, query, tolerance } => {
            // Query metrics from observers
            // Compare against tolerance
            todo!()
        }
    }
}
```

### 5.3 Automatic Cleanup Algorithm

```rust
/// Cleanup daemon that ensures all faults are eventually removed
pub struct CleanupDaemon {
    injection_registry: Arc<RwLock<HashMap<InjectionId, InjectionMetadata>>>,
}

impl CleanupDaemon {
    pub async fn run(&self) {
        let mut interval = interval(Duration::from_secs(10));

        loop {
            interval.tick().await;

            let expired = self.find_expired_injections().await;

            for (id, metadata) in expired {
                tracing::warn!(
                    injection_id = %id,
                    "Cleaning up expired injection"
                );

                if let Err(e) = self.cleanup_injection(&metadata).await {
                    tracing::error!(
                        injection_id = %id,
                        error = ?e,
                        "Failed to cleanup injection"
                    );
                }
            }
        }
    }

    async fn find_expired_injections(&self) -> Vec<(InjectionId, InjectionMetadata)> {
        let now = Timestamp::now();
        let registry = self.injection_registry.read().await;

        registry
            .iter()
            .filter(|(_, metadata)| {
                metadata.ttl.as_millis() > 0 &&
                now.as_millis() > metadata.created_at.as_millis() + metadata.ttl.as_millis()
            })
            .map(|(id, metadata)| (id.clone(), metadata.clone()))
            .collect()
    }

    async fn cleanup_injection(&self, metadata: &InjectionMetadata) -> Result<()> {
        metadata.strategy.remove(&metadata.handle).await?;

        let mut registry = self.injection_registry.write().await;
        registry.remove(&metadata.id);

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct InjectionMetadata {
    id: InjectionId,
    handle: InjectionHandle,
    strategy: Arc<dyn FaultStrategy>,
    created_at: Timestamp,
    ttl: Duration,
}
```

---

## 6. Schema Diagrams

### 6.1 Database Schema

```sql
-- Experiments table
CREATE TABLE experiments (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    definition JSON NOT NULL,
    state TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    started_at INTEGER,
    completed_at INTEGER,
    created_by TEXT NOT NULL,
    INDEX idx_state (state),
    INDEX idx_created_at (created_at)
);

-- Injection records
CREATE TABLE injections (
    id TEXT PRIMARY KEY,
    experiment_id TEXT NOT NULL,
    target TEXT NOT NULL,
    fault_type TEXT NOT NULL,
    parameters JSON NOT NULL,
    injected_at INTEGER NOT NULL,
    removed_at INTEGER,
    status TEXT NOT NULL,
    FOREIGN KEY (experiment_id) REFERENCES experiments(id),
    INDEX idx_experiment (experiment_id),
    INDEX idx_status (status)
);

-- Observations
CREATE TABLE observations (
    id TEXT PRIMARY KEY,
    experiment_id TEXT NOT NULL,
    observer_name TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    data JSON NOT NULL,
    FOREIGN KEY (experiment_id) REFERENCES experiments(id),
    INDEX idx_experiment_timestamp (experiment_id, timestamp)
);

-- Audit log
CREATE TABLE audit_log (
    id TEXT PRIMARY KEY,
    timestamp INTEGER NOT NULL,
    user_id TEXT NOT NULL,
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id TEXT NOT NULL,
    details JSON NOT NULL,
    INDEX idx_timestamp (timestamp),
    INDEX idx_user (user_id)
);
```

### 6.2 Experiment Lifecycle

```
┌─────────────────────────────────────────────────────────────────┐
│ Experiment Lifecycle Timeline                                    │
└─────────────────────────────────────────────────────────────────┘

Time ────────────────────────────────────────────────────────────▶

  │         │         │           │            │          │
  │ Submit  │ Validate│  Inject   │  Observe   │ Cleanup  │ Report
  │         │         │           │            │          │
  ▼         ▼         ▼           ▼            ▼          ▼

┌────┐  ┌────┐  ┌────────┐  ┌──────────┐  ┌───────┐  ┌────────┐
│Queue│─▶│Pre │─▶│ Inject │─▶│  Monitor │─▶│Remove │─▶│Generate│
│    │  │Check│  │ Faults │  │  System  │  │ Faults│  │ Report │
└────┘  └────┘  └────────┘  └──────────┘  └───────┘  └────────┘
   │       │         │            │            │           │
   │       │         │            │            │           │
   └───────┴─────────┴────────────┴────────────┴───────────┘
              Store state at each step in database
```

---

## 7. Decision Boundaries

### 7.1 Rollback Decision Tree

```
                    Start Experiment
                          │
                          ▼
                    Validate Baseline
                          │
             ┌────────────┼────────────┐
             │                         │
             ▼                         ▼
         Baseline OK              Baseline Failed
             │                         │
             ▼                         └──▶ ABORT
      Inject Fault
             │
             ▼
       Monitor Metrics
             │
    ┌────────┼────────┐
    │                 │
    ▼                 ▼
Metrics OK      Metrics Breach Threshold
    │                 │
    ▼                 ├──▶ Automatic Rollback? ─┐
Continue                                         │
    │                                            │
    │                 ┌──────────────────────────┘
    │                 │
    │                 ▼
    │              Yes: Emergency Rollback
    │                 │
    │                 └──▶ Remove Faults
    │                      │
    ▼                      ▼
Complete Duration      ABORTED
    │
    ▼
Remove Faults
    │
    ▼
COMPLETED
```

### 7.2 Target Selection Logic

```rust
/// Select targets based on criteria
pub fn select_targets(
    all_targets: &[Target],
    selector: &TargetSelector,
    safety_config: &SafetyConfig,
) -> Result<Vec<Target>> {
    // Filter by selector
    let mut selected: Vec<Target> = all_targets
        .iter()
        .filter(|t| selector.matches(t))
        .cloned()
        .collect();

    // Apply safety limits
    let max_count = (selected.len() as f64 * safety_config.max_blast_radius_percentage).ceil() as usize;
    if selected.len() > max_count {
        // Select randomly or by priority
        selected.shuffle(&mut thread_rng());
        selected.truncate(max_count);
    }

    // Ensure minimum count
    if selected.len() < selector.min_count.unwrap_or(1) {
        return Err(SystemError::validation(
            "target_count",
            "Not enough targets match selector",
            None,
        ));
    }

    Ok(selected)
}
```

---

## 8. Concurrency Model

### 8.1 Async Runtime

- **Runtime**: Tokio with multi-threaded scheduler
- **Worker Threads**: Configurable (default: CPU count)
- **Task Distribution**: Work-stealing scheduler

### 8.2 Concurrency Primitives

```rust
// Experiment concurrency control
pub struct ExperimentExecutor {
    // Semaphore limits concurrent experiments
    experiment_semaphore: Arc<Semaphore>,

    // Each experiment runs in its own task
    active_experiments: Arc<RwLock<HashMap<ExperimentId, JoinHandle<()>>>>,
}

impl ExperimentExecutor {
    pub async fn submit(&self, experiment: Experiment) -> Result<ExperimentId> {
        // Acquire permit (blocks if at max concurrency)
        let _permit = self.experiment_semaphore.acquire().await?;

        let id = ExperimentId::generate();
        let handle = tokio::spawn(async move {
            self.execute(experiment).await
        });

        self.active_experiments.write().await.insert(id.clone(), handle);

        Ok(id)
    }
}
```

### 8.3 Shared State Management

```rust
// State shared across tasks
pub struct SharedState {
    // Concurrent hashmap for fast reads
    experiments: Arc<DashMap<ExperimentId, ExperimentContext>>,

    // Injections registry
    injections: Arc<RwLock<HashMap<InjectionId, InjectionMetadata>>>,

    // Metrics cache
    metrics: Arc<RwLock<MetricsCache>>,
}
```

---

## 9. Data Flow

### 9.1 Experiment Submission Flow

```
User ─▶ HTTP API ─▶ API Gateway ─▶ Validation ─▶ Core Engine
                          │               │             │
                          │               │             ▼
                          │               │        State Store
                          │               │             │
                          │               ▼             │
                          │          Authorization      │
                          │               │             │
                          │               ▼             │
                          │          Experiment Queue   │
                          │               │             │
                          ▼               ▼             ▼
                     Metrics ◀────── Executor ────▶ Database
```

### 9.2 Fault Injection Flow

```
Core Engine ─▶ Strategy Manager ─▶ Select Strategy ─▶ Injector Pool
     │                                                      │
     │                                                      ▼
     │                                                  Execute
     │                                                      │
     │                                                      ▼
     │                                                Backend API
     │                                                      │
     │                                                      ▼
     └─────────────────── Record ◀───────────── Apply Fault
```

---

## 10. Fault Tolerance

### 10.1 Component Failures

| Component | Failure Impact | Recovery Strategy |
|-----------|----------------|-------------------|
| API Gateway | Cannot accept new experiments | Horizontal scaling, load balancer |
| Core Engine | Running experiments may stall | Persist state, resume on restart |
| Injector Worker | Some injections fail | Retry logic, alternative workers |
| Observer | Missing metrics | Continue experiment, log warning |
| State Store | Cannot save/load state | Replicated storage, backups |

### 10.2 Cleanup Guarantees

```rust
/// Ensure cleanup even if engine crashes
impl Drop for ExperimentContext {
    fn drop(&mut self) {
        // Best-effort cleanup
        for injection in &self.injections {
            if let Err(e) = block_on(injection.remove()) {
                eprintln!("Failed to cleanup injection: {}", e);
            }
        }
    }
}

// Additionally, cleanup daemon runs independently
// and cleans up based on TTL
```

### 10.3 Idempotency

All operations are idempotent:
- Injecting the same fault twice has the same effect as once
- Removing a non-existent fault succeeds
- Submitting the same experiment with same ID is rejected

---

## Appendix: Performance Characteristics

### Time Complexity

- **Submit Experiment**: O(1)
- **Validate Targets**: O(n) where n = target count
- **Inject Fault**: O(m) where m = number of targets
- **Observe**: O(p) where p = number of probes
- **Generate Report**: O(k) where k = observation count

### Space Complexity

- **Experiment Context**: O(n + m + k) for targets, injections, observations
- **State Store**: O(total_experiments)
- **Metrics Cache**: O(metric_count × time_window)

---

**End of Architecture Document**
