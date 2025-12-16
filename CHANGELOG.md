Changelog
All notable changes to the S.P.E.C.T.R.A. project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

[Unreleased]
Planned
Persistence Layer: Integration of DuckDB or Rusqlite to dump scan results to a queryable local file.

Enterprise Mesh: Federated governance and centralized data intelligence platform.

Advanced Visuals: WebGL/Wasm Treemap visualization in GUI application.

[0.2.0] - 2025-12-16
"The Semantic Bridge" - Phase 2 Intelligence Layer
This release introduces intelligent content analysis to Spectra, transitioning from pure topology (size/location) to typography (meaning/risk). The system now possesses "Sight" beyond mere file dimensions.

Added
Analysis Module (cli/src/analysis/):
  - entropy.rs: Shannon entropy calculation on file headers (first 8KB)
  - heuristics.rs: Pattern-based risk scoring for sensitive files
  - semantic.rs: Optional AI content classification using rust-bert
  - mod.rs: Public API for analysis capabilities

Entropy Profiling:
  - Calculates Shannon entropy (0.0 to 8.0 scale)
  - Detects encrypted, compressed, or obfuscated files
  - Microsecond-level performance
  - Read-only on first 8KB of files

Risk Detection System:
  - 5-level classification: None, Low, Medium, High, Critical
  - Detects: passwords, secrets, keys, tokens, certificates, SSH keys, wallets, credentials
  - Path-aware detection (e.g., .ssh/id_rsa flagged as Critical)
  - Zero file reads required - filename/path pattern matching only

AI Content Classification (Optional):
  - rust-bert DistilBERT zero-shot classification
  - Categories: legal contract, source code, financial invoice, log file, documentation
  - Feature-gated to keep base binary small
  - Only analyzes text files (low entropy) with confidence thresholds

CLI Enhancements:
  - --analyze flag: Enable entropy + risk analysis
  - --semantic flag: Enable AI classification (requires 'semantic' feature)
  - Enhanced human output with risk icons (🔴 Critical, 🟠 High, 🟡 Medium, 🟢 Low)
  - JSON output includes all analysis metadata

Convenience Scripts:
  - test-basic.sh / test-basic.bat: Quick basic scan testing
  - test-analyze.sh / test-analyze.bat: Quick semantic analysis testing

Dependencies:
  - regex 1.10: Pattern matching
  - lazy_static 1.4: Static regex compilation
  - rust-bert 0.21: AI classification (optional, feature-gated)
  - tempfile 3.8: Testing utilities

Changed
Version: 0.1.0 → 0.2.0

FileRecord Structure: Added entropy, risk_level, and semantic_tag fields

Main Scan Loop: Refactored to use .flatten() for cleaner error handling

Code Quality:
  - All clippy warnings resolved
  - Formatted with rustfmt
  - 8 unit tests, all passing

Documentation:
  - Updated ARCHITECTURE.md with Phase 2 implementation details
  - Updated README.md with new capabilities and usage examples
  - Comprehensive inline documentation

Technical Details
Performance: Post-scan analysis on top N files only (configurable via --limit)

Privacy: All analysis is local; no data leaves the machine

Safety: Read-only operations on first 8KB of files

Binary Size: Base ~2-5MB; AI features require LibTorch (~500MB) when enabled

Tiered Architecture:
  - Tier 0: Metadata (nanoseconds)
  - Tier 1: Heuristics (microseconds)
  - Tier 2: Semantic/AI (milliseconds, optional)

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