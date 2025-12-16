# S.P.E.C.T.R.A.

> **Scalable Platform for Enterprise Content Topology & Resource Analytics**

[![Rust](https://img.shields.io/badge/built_with-Rust-d05c44.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-pre--alpha-red.svg)]()

---

## 📡 The Mission

**Spectra** is the intelligence layer of the **Orbit/SPACE Ecosystem**.

While **Orbit** handles the movement of data and **SPACE** handles the environment, **Spectra** provides the vision. It is a high-performance data cartography agent designed to map the "Dark Matter" of enterprise storage.

It starts as a hyper-fast storage profiler (like WizTree/ncdu) but is architected to evolve into a distributed semantic governance brain (like Alation).

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
- **Risk Treemaps:** Interactive hierarchical visualization where color = entropy risk and size = data volume.
- **Visual Risk Encoding:**
  - 🟩 Green (0-3.0): Low entropy - Text, code, configs
  - 🟨 Yellow (3.0-6.0): Medium - Binaries, media
  - 🟧 Orange (6.0-7.5): High - Compressed data
  - 🟥 Red (7.5-8.0): Critical - Encryption, high randomness
- **Desktop Application:** Native cross-platform GUI built with Tauri v2 + React + TypeScript.
- **Nivo Integration:** D3-powered treemap visualizations with hover tooltips and click inspection.
- **Dark-Themed Interface:** Modern UI optimized for data density visualization.
- **One-Click Launch:** Sophisticated scripts for instant startup on Windows and Unix systems.

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

The application will open in a new window with the interactive risk treemap visualization. See [app/README.md](app/README.md) for detailed GUI documentation.

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

### Convenience Scripts

**Cross-Platform Launch Scripts:**
```bash
# Windows
launch-vision.bat       # Launch Spectra Vision GUI (Phase 4)
run-server.bat          # Start the Hub server (Phase 3)
run-agent.bat           # Run federated agent - dry-run (Phase 3)
build-release.bat       # Build all release binaries
test-all.bat            # Run full test suite

# Unix/Linux/macOS
./launch-vision.sh      # Launch Spectra Vision GUI (Phase 4)
```

## 🏗 Architecture

### Project Structure

```
spectra/
├── cli/                        # Headless Rust agent (spectra-cli)
│   ├── src/
│   │   ├── main.rs            # High-performance scanning engine
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
│   │   ├── components/        # React components
│   │   │   └── RiskTreemap.tsx     # Nivo treemap visualization
│   │   ├── App.tsx            # Main application
│   │   └── App.css            # Dark-themed styling
│   ├── src-tauri/
│   │   └── src/
│   │       └── lib.rs         # TreeNode scanning & entropy
│   ├── launch-spectra-vision.bat   # Windows launcher
│   ├── launch-spectra-vision.sh    # Unix launcher
│   ├── package.json
│   └── README.md              # GUI documentation
├── Cargo.toml                  # Workspace manifest
├── ARCHITECTURE.md             # Detailed technical documentation
├── PHASE3_GUIDE.md             # Phase 3 quick start guide
├── CHANGELOG.md                # Version history
├── launch-vision.bat           # Launch GUI (Windows)
├── launch-vision.sh            # Launch GUI (Unix)
├── run-server.bat              # Start Hub server (Windows)
├── run-agent.bat               # Run federated agent (Windows)
├── build-release.bat           # Build all binaries (Windows)
└── test-all.bat                # Run test suite (Windows)
```

### Philosophy

Spectra adheres to the **"Trojan Horse"** philosophy:

- **The Hook:** Solve the immediate "Disk Full" pain point with superior speed/UI.
- **The Pivot:** Use the installed base to index content and map lineage.
- **The Platform:** Federate metadata to a central governance plane.

See [ARCHITECTURE.md](ARCHITECTURE.md) for the deep dive.

## 🤝 Contributing

We prioritize **Performance** and **Safety**. If it slows down the scan, it doesn't get merged.

See [CONTRIBUTING.md](CONTRIBUTING.md) for protocols.

---

*Part of the Orbit/SPACE Data Infrastructure Initiative.*