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

### Phase 2: The Semantic Bridge

**Goal:** Move from "Size" to "Meaning."

- **Content Fingerprinting:** Integration of local LLM/Vector models (e.g., rust-bert) to classify file content (Contract, Invoice, Code) without reading the file.
- **Risk Scoring:** Automated heuristic analysis (e.g., "High entropy file named 'passwords.txt'").

### Phase 3: The Enterprise Mesh

**Goal:** Federated Governance.

- **The "Spectra Server":** A central dashboard that aggregates metadata from all deployed agents.
- **Time-Travel:** "Show me how our data footprint changed over the last 6 months."
- **Active Governance:** Policy enforcement (e.g., "Automatically archive .log files older than 90 days").

## 5. Build & Development Workflow

### Workspace Structure

The project uses Cargo's workspace feature to manage both crates:

```toml
[workspace]
members = [
    "cli",
    "app/src-tauri"
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

### Deployment Targets

**CLI Agent:**
- Single binary: `target/release/spectra-cli` (or `.exe` on Windows)
- No runtime dependencies
- Suitable for: Server deployments, automation scripts, CI/CD pipelines

**GUI Application:**
- Platform-specific installers: `.msi` (Windows), `.dmg` (macOS), `.deb`/`.appimage` (Linux)
- Native performance with web UI flexibility
- Suitable for: Desktop users, data analysts, administrators

## 6. Coding Standards & Principles

- **Zero-Cost Abstractions:** If a feature slows down the scan, it is disabled by default.
- **Unsafe Where Needed:** We use unsafe Rust only when interacting with raw filesystem headers (MFT) for performance, but it must be heavily documented.
- **Metadata as Code:** All schemas and reports must be exportable as JSON/YAML.
- **Single Binary (CLI):** The Agent must remain a single, portable executable with no external dependencies (no Python runtime, no JVM).
- **Native Performance (GUI):** The Tauri application bundles the Rust backend with the frontend, maintaining near-native performance.

---

> *"We are building the Google Earth for Enterprise Data. Start by mapping the backyard, end by mapping the world."*