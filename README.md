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

### Phase 2: Visual Interface (Current)
- **Desktop Application:** Native cross-platform GUI built with Tauri v2 + React + TypeScript.
- **Dual-Crate Architecture:** Separation of headless agent (CLI) and visual interface (GUI).
- **Modern Stack:** Vite-powered development with hot module replacement for rapid iteration.

## 🚀 Quick Start

### Installation

```bash
git clone https://github.com/YOUR_USERNAME/spectra.git
cd spectra
```

### Running the GUI Application

```bash
cd app
npm install
npm run tauri dev
```

This will launch the Tauri desktop application with the React frontend.

### Running the CLI Agent (Headless)

```bash
# Build the CLI agent
cargo build --release -p spectra-cli

# Scan current directory (Human Readable)
./target/release/spectra-cli --path ./

# Scan entire drive and output JSON for analysis (Agent Mode)
./target/release/spectra-cli --path / --json > scan_results.json
```

## 🏗 Architecture

### Project Structure

```
spectra/
├── cli/                 # Headless Rust agent (spectra-cli)
│   ├── src/
│   │   └── main.rs     # High-performance scanning engine
│   └── Cargo.toml
├── app/                 # Tauri + React GUI application
│   ├── src/            # React/TypeScript frontend
│   ├── src-tauri/      # Tauri Rust backend
│   └── package.json
├── Cargo.toml          # Workspace manifest
└── ARCHITECTURE.md     # Detailed technical documentation
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