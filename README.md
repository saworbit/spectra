# Spectra

> **Scalable Platform for Enterprise Content Topology & Resource Analytics**

[![Rust](https://img.shields.io/badge/built_with-Rust-d05c44.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-pre--alpha-red.svg)]()
[![Rust CI](https://github.com/saworbit/spectra/workflows/Rust%20CI/badge.svg)](https://github.com/saworbit/spectra/actions/workflows/rust-ci.yml)
[![Frontend CI](https://github.com/saworbit/spectra/workflows/Frontend%20CI/badge.svg)](https://github.com/saworbit/spectra/actions/workflows/frontend-ci.yml)

---

## 📡 The Mission

**Spectra** is the intelligence layer of the **Orbit/SPACE Ecosystem**.

While **Orbit** handles the movement of data and **SPACE** handles the environment, **Spectra** provides the vision. It is a high-performance data cartography agent designed to map the "Dark Matter" of enterprise storage.

It starts as a hyper-fast storage profiler (like WizTree/ncdu) but is architected to evolve into a distributed semantic governance brain for enterprise data management.

## ⚡ Capabilities

### Phase 1: Core Engine
- **Zero-Latency Scanning:** Bypasses standard OS bottlenecks using parallelized walkers (`jwalk`).
- **Data Gravity Mapping:** Instantly identifies "Heavy Hitters" (top largest files) and "Data Sprawl" (extension distribution).
- **Edge-First Architecture:** Runs locally with a tiny memory footprint (<20MB RAM for 1M files).
- **Dual Mode:**
    - `Human Mode`: Pretty-printed CLI summaries for sysadmins.
    - `Agent Mode`: JSON streams for ingestion into the Spectra Brain.

### Phase 2: The Semantic Bridge ✅ (Implemented)
- **Intelligent Analysis:** Move beyond size to understand meaning and risk.
- **Entropy Profiling:** Shannon entropy calculation to detect encrypted/compressed files.
- **Risk Scoring:** 5-level risk classification for sensitive files (passwords, keys, credentials).
- **Content Classification:** Optional AI-powered categorization using rust-bert (legal, financial, technical).
- **Tiered Architecture:** Fast heuristics (microseconds) with optional deep analysis (milliseconds).
- **Feature Flags:** Keep base binary small; opt-in to heavy AI models.

### Phase 3: The Enterprise Mesh ✅ (Implemented)
- **Federation:** Hub & Spoke architecture for distributed agent coordination.
- **Spectra Server:** Central control plane (Axum + SurrealDB) for telemetry aggregation.
- **Time-Travel Analytics:** Historical snapshots to track data growth and velocity over time.
- **Active Governance:** Policy-based file management (Report, Delete, Archive actions).
- **Safety-First:** Dry-run mode by default; requires explicit `--enforce` flag for destructive actions.
- **Beacon Protocol:** Agents push snapshots and pull policies via REST API.
- **Local-First:** Agents work perfectly offline; federation is optional.

### Phase 4: The Lens (Visualization Layer) ✅ (Implemented)
- **Enterprise Dashboard:** Real-time statistics visualization for directory scans.
- **Data Insights:**
  - 📊 Overview Card: Total files, folders, size, and scan duration
  - 📈 Top Extensions: Top 5 file types by total size with file counts
  - 🐳 Heavy Hitters: Top 10 largest files with full path display
- **Desktop Application:** Native cross-platform GUI built with Tauri v2 + React + TypeScript.
- **Tauri Bridge:** Strongly-typed interface between React frontend and Rust backend.
- **Dark-Themed Interface:** Modern enterprise UI with grid-based card layout.
- **One-Click Launch:** Sophisticated scripts for instant startup on Windows and Unix systems.
- **Type Safety:** Full TypeScript support with interface contracts for scan results.

## 🚀 Quick Start

### Installation

```bash
git clone https://github.com/YOUR_USERNAME/spectra.git
cd spectra
```

### Running the GUI Application (Phase 4 - The Lens)

**Quick Launch (Recommended):**
```bash
# Windows
launch-vision.bat

# Unix/Linux/macOS
./launch-vision.sh
```

**Manual Launch:**
```bash
cd app
npm install
npm run tauri dev
```

The application will open in a new window with the enterprise dashboard. Enter a directory path and click "Deep Scan" to visualize file statistics. See [app/README.md](app/README.md) for detailed GUI documentation.

### Running the CLI Agent (Headless)

```bash
# Build the CLI agent
cargo build --release -p spectra-cli

# Basic scan (Phase 1 - topology only)
./target/release/spectra-cli --path ./

# Scan with semantic analysis (Phase 2 - entropy + risk detection)
./target/release/spectra-cli --path ./ --analyze

# Scan with full AI classification (requires semantic feature)
cargo build --release -p spectra-cli --features semantic
./target/release/spectra-cli --path ./ --semantic

# Output JSON for analysis (Agent Mode)
./target/release/spectra-cli --path ./ --analyze --json > scan_results.json
```

### Running the Server (Phase 3 - Federation)

```bash
# Start the Spectra Server (Hub)
run-server.bat          # Windows
# or
cargo run -p spectra-server

# The server listens on http://0.0.0.0:3000
```

### Running Federated Agents (Phase 3)

```bash
# Connect agent to server (dry-run governance)
run-agent.bat           # Windows
# or
cargo run -p spectra-cli -- --path ./ --server http://localhost:3000

# Agent with active policy enforcement (⚠️ CAUTION: Can delete files)
cargo run -p spectra-cli -- --path ./ --server http://localhost:3000 --enforce

# Full stack: Analysis + Governance + Federation
cargo run -p spectra-cli -- --path ./ --server http://localhost:3000 --analyze
```

## 🔧 Scripts & Tools

Spectra includes several convenience scripts to streamline development and deployment workflows.

### Quality Assurance

**`validate-refactor.bat`** (Windows)
- **Purpose**: Pre-alpha comprehensive validation suite for modular architecture
- **What it does**:
  - Code formatting check (`cargo fmt --check`)
  - Clippy linting with strict warnings (`-D warnings`)
  - Builds all crates (core, CLI, server, Tauri app) in release mode
  - Runs unit tests for each crate independently
  - Runs integration tests (CLI basic scan, CLI analysis scan)
  - Full workspace test suite
- **When to use**: Before committing changes, during refactoring, or after dependency updates
- **Usage**: `validate-refactor.bat`
- **Expected output**: Color-coded step-by-step validation with ✓/✗ indicators

### Build & Deployment

**`build-release.bat`** (Windows)
- **Purpose**: Build production-ready release binaries for all crates
- **What it does**: Runs `cargo build --release` for spectra-core, spectra-cli, spectra-server, and app
- **When to use**: When preparing binaries for distribution or benchmarking
- **Usage**: `build-release.bat`
- **Output location**: `target/release/`

### GUI Application

**`launch-vision.bat`** (Windows) / **`launch-vision.sh`** (Unix/Linux/macOS)
- **Purpose**: One-click launcher for Spectra Dashboard GUI (Phase 4 - The Lens)
- **What it does**:
  - Checks for dependencies (Node.js, Cargo)
  - Navigates to `app/` directory
  - Installs npm dependencies if needed
  - Launches Tauri development server (`npm run tauri dev`)
- **When to use**: To visualize directory scans with enterprise statistics dashboard
- **Usage**:
  - Windows: `launch-vision.bat`
  - Unix: `./launch-vision.sh`
- **Features**: Opens native GUI with file statistics, top extensions, and heavy hitters visualization

### Federation & Server

**`run-server.bat`** (Windows)
- **Purpose**: Start the Spectra Server (Phase 3 - Federation Hub)
- **What it does**: Runs `cargo run -p spectra-server` to start the central control plane
- **When to use**: When testing federated agent coordination or time-travel analytics
- **Usage**: `run-server.bat`
- **Endpoint**: Listens on `http://0.0.0.0:3000`
- **API endpoints**:
  - `POST /api/v1/ingest` - Receive agent snapshots
  - `GET /api/v1/policies` - Distribute governance policies

**`run-agent.bat`** (Windows)
- **Purpose**: Run federated CLI agent in dry-run mode
- **What it does**: Executes `cargo run -p spectra-cli -- --path ./ --server http://localhost:3000`
- **When to use**: Testing agent federation without policy enforcement
- **Usage**: `run-agent.bat` (ensure server is running first)
- **Safety**: Dry-run by default (reports governance actions without executing)
- **For enforcement**: Edit script to add `--enforce` flag (⚠️ CAUTION: Can delete files)

### Script Compatibility

| Script | Windows | Unix/Linux | macOS |
|--------|---------|------------|-------|
| validate-refactor.bat | ✅ | ❌ | ❌ |
| build-release.bat | ✅ | ❌ | ❌ |
| launch-vision.bat | ✅ | ❌ | ❌ |
| launch-vision.sh | ❌ | ✅ | ✅ |
| run-server.bat | ✅ | ❌ | ❌ |
| run-agent.bat | ✅ | ❌ | ❌ |

**Note**: Unix/Linux/macOS users can run equivalent Cargo commands directly. See individual sections above for command equivalents.

## 🏗 Architecture

### Project Structure

```
spectra/
├── spectra-core/              # 🆕 Core scanning library (NEW)
│   ├── src/
│   │   └── lib.rs            # Scanner, FileRecord, ScanStats
│   ├── Cargo.toml
│   └── README.md              # Usage guide
├── cli/                        # Headless Rust agent (spectra-cli)
│   ├── src/
│   │   ├── main.rs            # Thin client using spectra-core
│   │   ├── analysis/          # Phase 2: Semantic Bridge
│   │   │   ├── entropy.rs     # Shannon entropy calculation
│   │   │   ├── heuristics.rs  # Risk pattern detection
│   │   │   ├── semantic.rs    # AI content classification
│   │   │   └── mod.rs         # Analysis module API
│   │   └── governance/        # Phase 3: Active Governance
│   │       ├── engine.rs      # Policy evaluation & execution
│   │       ├── tests.rs       # Safety test suite
│   │       └── mod.rs         # Governance module API
│   └── Cargo.toml
├── server/                     # Phase 3: Central Hub (spectra-server)
│   ├── src/
│   │   └── main.rs            # Axum API server
│   └── Cargo.toml
├── app/                        # Phase 4: Tauri + React GUI (The Lens)
│   ├── src/
│   │   ├── types.ts           # TypeScript interfaces for scan results
│   │   ├── App.tsx            # Enterprise dashboard implementation
│   │   └── App.css            # Dark-themed enterprise styling
│   ├── src-tauri/
│   │   └── src/
│   │       └── lib.rs         # Tauri commands using spectra-core
│   ├── launch-spectra-vision.bat   # Windows launcher
│   ├── launch-spectra-vision.sh    # Unix launcher
│   ├── package.json
│   └── README.md              # GUI documentation
├── docs/                        # Documentation
│   ├── ARCHITECTURE.md         # Detailed technical documentation
│   ├── PHASE3_GUIDE.md         # Phase 3 quick start guide
│   ├── PHASE4_GUIDE.md         # Phase 4 visualization guide
│   ├── CONTRIBUTING.md         # Contribution guidelines
│   ├── DEVELOPMENT.md          # Developer setup guide
│   └── FAQ.md                  # Frequently asked questions
├── Cargo.toml                  # Workspace manifest
├── CHANGELOG.md                # Version history
├── validate-refactor.bat       # 🆕 QA validation suite (NEW)
├── launch-vision.bat           # Launch GUI (Windows)
├── launch-vision.sh            # Launch GUI (Unix)
├── run-server.bat              # Start Hub server (Windows)
├── run-agent.bat               # Run federated agent (Windows)
└── build-release.bat           # Build all binaries (Windows)
```

### 🏗️ Modular Architecture (Pre-Alpha)

Spectra recently underwent a **major refactoring** to establish a clean modular architecture:

```
┌─────────────────┐
│ spectra-server  │ ← Federation endpoint (Phase 3)
└────────┬────────┘
         │
         ↓
┌─────────────────┐
│  spectra-cli    │ ← Thin client
├─────────────────┤
│ • Analysis      │ ← Phase 2 (entropy, risk, semantic)
│ • Governance    │ ← Phase 3 (policies)
│ • Federation    │ ← Phase 3 (server comms)
└────────┬────────┘
         │ uses
         ↓
┌─────────────────┐
│ spectra-core    │ ← Shared scanning engine ⭐ NEW
├─────────────────┤
│ • Scanner       │ ← Phase 1 (jwalk, BinaryHeap)
│ • FileRecord    │ ← Simple (path, size)
│ • ScanStats     │ ← Aggregated results
└────────┬────────┘
         ↑ uses
         │
┌─────────────────┐
│   app (Tauri)   │ ← GUI application (Phase 4)
├─────────────────┤
│ • scan_directory│ ← Statistics dashboard
│ • React + TS    │ ← Strongly-typed frontend
└─────────────────┘
```

**Key Benefits:**
- 🔄 Code reuse between CLI and GUI
- ✅ Independently testable core library
- 🎯 Clear separation of concerns
- 🚀 No performance regression
- 🧩 Extensible for future use cases

**Status:** Pre-Alpha (API unstable)

### Philosophy

Spectra adheres to the **"Trojan Horse"** philosophy:

- **The Hook:** Solve the immediate "Disk Full" pain point with superior speed/UI.
- **The Pivot:** Use the installed base to index content and map lineage.
- **The Platform:** Federate metadata to a central governance plane.

See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for the deep dive.

## 🤝 Contributing

We prioritize **Performance** and **Safety**. If it slows down the scan, it doesn't get merged.

See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for protocols.

## 📄 License

Spectra is dual-licensed under your choice of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

This dual licensing provides maximum flexibility for both open-source and commercial use. You may choose whichever license best suits your needs.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.

See [LICENSE](LICENSE) for more details on the dual licensing model.

---

*Part of the Orbit/SPACE Data Infrastructure Initiative.*