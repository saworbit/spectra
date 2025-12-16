# S.P.E.C.T.R.A. Architecture & Vision

**Scalable Platform for Enterprise Content Topology & Resource Analytics**

> *"From Bytes to Wisdom."*

---

## 1. The Vision

Spectra is not just a disk usage analyzer. It is a **Trojan Horse for Data Intelligence**.

The modern data stack is broken. Governance tools (Alation, Collibra) are top-heavy, expensive, and require massive manual implementation. Cleanup tools (WizTree, TreeSize) are fast and tactical but lack memory and intelligence.

**Spectra bridges this gap.**

- **Phase 1 (The Hook):** We provide the world's fastest, most beautiful storage visualization tool. We solve the immediate pain of "Why is my disk full?" to gain ubiquity on every laptop and server.

- **Phase 2 (The Graph):** We use that foothold to silently index content, build semantic graphs, and track data lineage locally at the edge.

- **Phase 3 (The Brain):** We federate these edge insights into a centralized Data Intelligence Platform that governs security, cost, and compliance without moving the raw data.

## 2. The Core Philosophy

### A. The "Trojan Horse" Strategy

We do not sell governance; we sell **visibility**.

- **User Value:** "I can instantly see what is eating my storage."
- **Enterprise Value:** "We can instantly map where our Sensitive Data (PII) lives and how much 'Rot' (Redundant, Obsolete, Trivial data) is costing us."

### B. Speed as a Feature

If the scan takes 10 minutes, the user walks away. If it takes 10 seconds, they use it daily.

**Technical Implication:** No Garbage Collection (Rust over Go). No Interpreted Code (Native Binaries). Parallelism by default.

### C. Privacy by Design (Edge Intelligence)

We do not upload files to the cloud.

**The Model:** We send the code to the data, not the data to the code.

The Agent scans locally, generates vector embeddings and metadata summaries, and only transmits lightweight Insights to the control plane.

## 3. System Architecture

The system is composed of three decoupled layers: **The Agent** (Edge), **The Transport** (Nervous System), and **The Platform** (Brain).

### Current Implementation: Dual-Crate Workspace

**Architecture:** The project is structured as a Rust workspace with two independent crates:

**1. The CLI Agent (`spectra-cli`):**
- **Location:** `cli/` directory
- **Purpose:** Headless, high-performance scanning engine
- **Deployment:** Single binary executable for server/automation use
- **Stack:** Pure Rust with minimal dependencies (jwalk, serde, clap)

**2. The GUI Application (`app`):**
- **Location:** `app/` directory
- **Purpose:** Desktop visualization and user interface
- **Deployment:** Native cross-platform application (Windows, macOS, Linux)
- **Stack:** Tauri v2 (Rust backend) + React 18 + TypeScript (frontend)

### Layer 1: The Agent (Rust)

**Role:** The high-performance collector.

**Responsibility:**
- Direct MFT/Ext4 Parsing: Bypasses OS APIs for raw speed.
- Vectorization: Generates content fingerprints locally.
- Resource Profiling: Calculates "Cost to Keep" vs. "Cost to Delete."

**Current Stack:**
- CLI: Rust, jwalk (Parallelism), serde (Serialization), clap (CLI Parsing)
- GUI: Tauri v2, React 18, TypeScript, Vite (Development)

**Future Stack:** Rusqlite (Local State), rust-bert (Content Classification).

### Layer 2: The Transport (Orbit)

**Role:** The secure, asynchronous communication layer.

**Responsibility:**
- Telemetry streaming.
- Command & Control (telling agents to "Deep Scan" specific sectors).

**Stack:** gRPC / NATS / Flight (Apache Arrow).

### Layer 3: The Platform (Spectra Core)

**Role:** The centralized intelligence and visualization layer.

**Responsibility:**
- Topology Mapping: Visualizing the network of data flows.
- Global Search: "Find all contracts with 'Force Majeure' clauses across 5,000 laptops."
- Risk Heatmaps: Visualizing PII hotspots.

**Stack:** Tauri (Frontend), SurrealDB/Neo4j (Graph Storage), Vector DB.

## 4. Technical Roadmap

### Phase 1: The "Hyper-Fast" Analyzer (Completed)

**Goal:** Prove technical superiority.

- [x] Core Engine: Parallel multi-threaded walker (Rust).
- [x] Analytics: Extension grouping and "Heavy Hitter" identification.
- [x] Visuals: Tauri v2 + React + TypeScript foundation established.
- [x] Architecture: Dual-crate workspace (CLI + GUI separation).
- [ ] Persistence: SQLite/DuckDB integration for history tracking.
- [ ] Advanced Visuals: WebGL/Wasm Treemap visualization implementation.

### Phase 2: The Semantic Bridge (Implemented ✅)

**Goal:** Move from "Size" to "Meaning."

We have transitioned from pure topology (size/location) to typography (meaning/risk). Spectra now possesses "Sight."

#### Tiered Inspection Architecture

To maintain our <20MB binary and sub-second scan times while introducing AI capabilities, we implement a three-tier analysis model:

- **Tier 0 (Metadata):** Size, Extension, Path. (Nanoseconds) - Already implemented in Phase 1.
- **Tier 1 (Heuristic):** Filename regex patterns, Shannon entropy calculation on first 8KB. (Microseconds)
- **Tier 2 (Semantic):** LLM-based content classification using rust-bert. (Milliseconds) - Optional feature.

#### Implementation Details

**Module Structure:** `cli/src/analysis/`
- `entropy.rs` - Shannon entropy calculation for detecting encryption/compression
- `heuristics.rs` - Pattern-based risk analysis for sensitive files
- `semantic.rs` - Optional AI-based content classification
- `mod.rs` - Public API for analysis capabilities

**Key Features:**

1. **Entropy Profiling:**
   - Calculates Shannon Entropy on file headers (first 8KB only)
   - Low Entropy (0-4): Text, Source Code, XML
   - High Entropy (7-8): Compressed archives, Encrypted volumes, Random keys
   - Helps identify obfuscated malware or encrypted sensitive data

2. **Heuristic Risk Scoring:**
   - Regex-based identification of "Toxic Assets" (keys, passwords, credentials)
   - Five-level risk classification: None, Low, Medium, High, Critical
   - Patterns include: `.pem`, `.p12`, `password`, `secret`, `token`, `.env`, `.kdbx`
   - No file content read required - filename analysis only

3. **Neural Classification (Optional):**
   - Integration of rust-bert DistilBERT zero-shot classification
   - Classifies content as: Legal Contract, Source Code, Financial Invoice, Log File, etc.
   - Only applies to low-entropy (text) files
   - Gated behind `semantic` feature flag to keep base binary small
   - Requires LibTorch (~500MB) - not included in default build

**Safety & Performance:**

- All analysis is **read-only** on the first 8KB of files
- No data leaves the local machine
- Analysis is **opt-in** via `--analyze` and `--semantic` flags
- Default scan remains ultra-fast with no overhead
- Feature flags prevent binary bloat

**Usage:**

```bash
# Standard fast scan (Phase 1 only)
cargo run -p spectra-cli -- --path ./

# With heuristic analysis (entropy + risk)
cargo run -p spectra-cli -- --path ./ --analyze

# With full AI classification (requires 'semantic' feature)
cargo build -p spectra-cli --features semantic
cargo run -p spectra-cli --features semantic -- --path ./ --semantic
```

#### Implementation Status

- [x] Entropy calculation engine
- [x] Risk pattern detection
- [x] Semantic classification framework
- [x] Feature flag architecture
- [x] CLI integration with `--analyze` and `--semantic` flags
- [x] Enhanced reporting with risk icons and entropy scores
- [ ] Batch processing optimization for large file sets
- [ ] Custom pattern configuration files
- [ ] Machine learning model fine-tuning for enterprise domains

### Phase 3: The Enterprise Mesh (Implemented ✅)

**Goal:** Federated Governance, Historical Analytics, and Active Policy Enforcement.

We have transitioned from Local-Only to Local-First, Cloud-Aware. Spectra now possesses "Memory" and "Hands."

#### Hub & Spoke Architecture

The system now operates as a distributed control plane with central coordination:

- **The Hub (spectra-server):** High-performance Rust server (Axum + SurrealDB) that:
  - Ingests telemetry snapshots from distributed agents
  - Stores historical topology data for time-travel analysis
  - Distributes governance policies to all connected agents
  - Provides REST API for analytics and control

- **The Spoke (spectra-cli):** Enhanced agent with "Beacon" capabilities:
  - Pushes compressed JSON snapshots to the Hub
  - Pulls governance policies from the Hub
  - Executes policies with built-in safety mechanisms
  - Maintains local-first operation (works offline)

#### Module Structure

**Server Crate:** `server/`
- `src/main.rs` - HTTP/2 API server with Axum
- `/api/v1/ingest` - Receives agent snapshots (time-series data)
- `/api/v1/policies` - Distributes governance rules to agents

**CLI Governance Module:** `cli/src/governance/`
- `engine.rs` - Policy evaluation and execution engine
- `tests.rs` - Comprehensive safety tests for governance actions
- `mod.rs` - Module interface

#### Key Features

1. **Time-Travel Analytics:**
   - Agents send periodic snapshots (metadata only, not raw files)
   - Server stores snapshots with timestamps
   - Calculate "Data Velocity" by comparing snapshots at T₀ vs T₁
   - Answer questions like: "How fast is our data growing?" "What changed this month?"
   - Top extensions tracking across the fleet

2. **Active Governance Engine:**
   - Rule-based policy evaluation (extension, size, age thresholds)
   - Three action types: Report, Delete, Archive
   - **Safety-First Design:**
     - Defaults to DRY-RUN mode (reports only)
     - Requires explicit `--enforce` flag for destructive actions
     - Double-checks before file deletion
     - Comprehensive test coverage
   - Policies distributed from central server
   - Local policy configuration support

3. **Federation Protocol:**
   - REST over HTTP/2 for deployment ease
   - MessagePack for compressed payloads (future enhancement)
   - Secure agent authentication (planned)
   - Offline-capable agents with local policy cache

#### Implementation Details

**Agent Snapshot Format:**
```rust
{
    "agent_id": "agent_1734394726",
    "timestamp": 1734394726,
    "hostname": "DESKTOP-XYZ",
    "total_size_bytes": 524288000,
    "file_count": 15234,
    "top_extensions": [
        ("log", 125829120),
        ("tmp", 89653248),
        ("json", 45678912)
    ]
}
```

**Policy Structure:**
```rust
Policy {
    name: "Cleanup Old Logs",
    rule: Rule {
        extension: Some("log"),
        min_size_bytes: None,
        min_age_days: Some(90)
    },
    action: Action::Report  // or Delete, Archive
}
```

#### Safety & Security

- **No Raw Data Upload:** Only metadata summaries are transmitted
- **Dry-Run by Default:** Governance actions require explicit `--enforce` flag
- **Audit Trail:** All policy executions are logged
- **Local Autonomy:** Agents continue operating if server is unreachable
- **Policy Validation:** Server-side and client-side policy validation

#### Usage

**Starting the Hub:**
```bash
cd server
cargo run

# Server starts on http://0.0.0.0:3000
```

**Agent with Server Connection:**
```bash
# Scan and upload snapshot (dry-run governance)
cargo run -p spectra-cli -- --path ./ --server http://localhost:3000

# Scan with active policy enforcement
cargo run -p spectra-cli -- --path ./ --server http://localhost:3000 --enforce

# Analyze with governance
cargo run -p spectra-cli -- --path ./ --server http://localhost:3000 --analyze --enforce
```

#### Implementation Status

- [x] Spectra Server scaffolding (Axum + basic endpoints)
- [x] Agent snapshot ingestion API
- [x] Policy distribution API
- [x] Governance engine with rule evaluation
- [x] Safety mechanisms (dry-run mode)
- [x] CLI integration with `--server` and `--enforce` flags
- [x] Snapshot upload from agents
- [x] Comprehensive governance tests
- [ ] SurrealDB integration for persistent storage
- [ ] Time-travel query interface
- [ ] Agent authentication and authorization
- [ ] Policy management UI
- [ ] Growth rate analytics and alerting
- [ ] Archive functionality implementation
- [ ] Custom policy configuration files

## 5. Build & Development Workflow

### Workspace Structure

The project uses Cargo's workspace feature to manage all three crates:

```toml
[workspace]
members = [
    "cli",
    "app/src-tauri",
    "server"
]
resolver = "2"
```

### Development Commands

**CLI Development:**
```bash
# Build the CLI agent
cargo build -p spectra-cli

# Run the CLI agent
cargo run -p spectra-cli -- --path ./

# Release build
cargo build --release -p spectra-cli
```

**GUI Development:**
```bash
# Navigate to app directory
cd app

# Install frontend dependencies
npm install

# Run in development mode (hot reload)
npm run tauri dev

# Build production application
npm run tauri build
```

**Server Development:**
```bash
# Build the server
cargo build -p spectra-server

# Run the server (listens on port 3000)
cargo run -p spectra-server

# Release build
cargo build --release -p spectra-server

# Run tests
cargo test -p spectra-server
```

### Deployment Targets

**CLI Agent:**
- Single binary: `target/release/spectra-cli` (or `.exe` on Windows)
- No runtime dependencies
- Suitable for: Server deployments, automation scripts, CI/CD pipelines, distributed agents

**GUI Application:**
- Platform-specific installers: `.msi` (Windows), `.dmg` (macOS), `.deb`/`.appimage` (Linux)
- Native performance with web UI flexibility
- Suitable for: Desktop users, data analysts, administrators

**Spectra Server:**
- Single binary: `target/release/spectra-server` (or `.exe` on Windows)
- Requires network access (listens on port 3000 by default)
- Suitable for: Central control plane, fleet management, historical analytics
- Deployment options: Docker container, systemd service, Windows service

## 6. Coding Standards & Principles

- **Zero-Cost Abstractions:** If a feature slows down the scan, it is disabled by default.
- **Unsafe Where Needed:** We use unsafe Rust only when interacting with raw filesystem headers (MFT) for performance, but it must be heavily documented.
- **Metadata as Code:** All schemas and reports must be exportable as JSON/YAML.
- **Single Binary (CLI):** The Agent must remain a single, portable executable with no external dependencies (no Python runtime, no JVM).
- **Native Performance (GUI):** The Tauri application bundles the Rust backend with the frontend, maintaining near-native performance.

---

> *"We are building the Google Earth for Enterprise Data. Start by mapping the backyard, end by mapping the world."*