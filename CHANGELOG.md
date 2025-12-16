Changelog
All notable changes to the S.P.E.C.T.R.A. project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

[Unreleased]
Planned
Persistence Layer: Integration of DuckDB or Rusqlite to dump scan results to a queryable local file.

Deep Scan: Content fingerprinting for identifying file types beyond simple extensions.

Semantic Bridge: Integration of local LLM/Vector models for content classification.

[0.2.0] - 2025-12-16
"The Dual Core" - Phase 2 Architecture Setup
This release transforms Spectra from a single-binary CLI tool into a dual-crate workspace architecture, separating the headless agent from the visual interface and establishing the foundation for the Tauri-based UI.

Added
Workspace Architecture: Converted project to Rust workspace with two distinct crates:
  - cli: High-performance headless agent (renamed to spectra-cli)
  - app: Tauri v2 + React + TypeScript GUI application

Tauri Frontend: Scaffolded complete Tauri v2 application with:
  - React 18 + TypeScript for UI development
  - Vite for fast development builds
  - Modern development tooling (ESLint, TypeScript compiler)

Project Structure: Established clear separation of concerns:
  - Backend Agent: c:\spectra\cli (Pure Rust, no GUI dependencies)
  - Frontend UI: c:\spectra\app (Tauri + React for visualization)
  - Workspace Root: Unified build system via Cargo workspace

Changed
CLI Binary Name: Renamed from spectra to spectra-cli to avoid naming conflicts with GUI application

Build System: Implemented Cargo workspace resolver v2 for improved dependency management

Technical Details
Stack: Rust 2021 Edition + Tauri v2 + React 18 + TypeScript 5 + Vite 7

Key Addition: Tauri v2 provides native desktop application capabilities with web technologies

Architecture: Dual-crate workspace enables independent development and deployment of CLI and GUI

[0.1.0] - 2025-12-16
"The Ignition" - Initial Proof of Concept
This release establishes the core high-performance scanning engine. It proves that a Rust-based, parallel architecture can outperform traditional walkers, laying the foundation for the "Trojan Horse" strategy.

Added
Core Engine: Implemented multi-threaded directory walker using jwalk for maximum disk I/O saturation.

Analytics: Added "Extension Profiling" (grouping files by type) and "Heavy Hitters" (identifying top N largest files).

Memory Efficiency: Implemented a Min-Heap (BinaryHeap) algorithm to track top files with O(1) memory overhead, ensuring stability on massive drives.

CLI Interface: Built a dual-mode interface:

Human Mode: Pretty-printed summaries with readable units (GB/MB).

Agent Mode (--json): Structured JSON output for downstream processing/pipelines.

Architecture: Established the "Spectra" vision and "Trojan Horse" architectural blueprint (ARCHITECTURE.md).

Technical Details
Stack: Rust (Nightly/Stable 2021).

Key Crates: jwalk (parallelism), serde (serialization), clap (CLI parsing), humansize (formatting).

Performance: Sub-second scanning capability on standard development directories (~400 files/0.04s).

"Start by mapping the backyard, end by mapping the world."