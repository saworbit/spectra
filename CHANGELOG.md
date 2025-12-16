# Changelog

All notable changes to the S.P.E.C.T.R.A. project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned Features
- **Persistence Layer**: Integration of DuckDB or Rusqlite for queryable local history
- **SurrealDB Integration**: Full graph database support for topology storage
- **Policy Management UI**: Web interface for creating and managing governance policies
- **Advanced Visuals**: WebGL/Wasm Treemap visualization in GUI application
- **Agent Authentication**: Secure authentication and authorization for federated agents
- **Growth Analytics**: Data velocity tracking and trend prediction

---

## [0.3.0] - 2024-12-16

### "The Enterprise Mesh" - Phase 3 Federation & Governance

**This release transforms Spectra from a local-only tool into a federated enterprise platform with central coordination, historical analytics, and active governance capabilities.**

#### Added

**Hub & Spoke Architecture:**
- **Spectra Server (`spectra-server`)**: New central control plane crate
  - Axum-based HTTP/2 REST API server listening on port 3000
  - `/api/v1/ingest` endpoint for receiving agent snapshots
  - `/api/v1/policies` endpoint for distributing governance rules
  - Ready for SurrealDB integration

**Time-Travel Analytics:**
- Agent snapshot ingestion with metadata summaries (no raw file data)
- Historical topology tracking with timestamps
- Top extensions tracking across distributed fleet
- Data velocity calculation framework (T₀ vs T₁)

**Active Governance Engine (`cli/src/governance/`):**
- `engine.rs`: Policy evaluation and execution engine
- `tests.rs`: Comprehensive safety test suite (12 tests passing)
- Rule-based policy evaluation (extension, size, age)
- Three action types: Report, Delete, Archive
- **Safety-First Design**: Dry-run by default, requires `--enforce` flag

**Federation Protocol:**
- Beacon capabilities in CLI agent (push snapshots, pull policies)
- REST over HTTP/2 for deployment ease
- Offline-capable agents with graceful degradation

**CLI Enhancements:**
- `--server <URL>`: Connect to Spectra Server
- `--enforce`: Enable active policy execution (⚠️ CAUTION)
- Real-time governance checks during scan

**Scripts & Tools:**
- `run-server.bat`: Start the Hub server
- `run-agent.bat`: Run federated agent (dry-run)
- `build-release.bat`: Build all release binaries
- `test-all.bat`: Run full test suite

**Dependencies:**
- CLI: `reqwest`, `chrono`, `config`
- Server: `axum`, `tokio`, `serde_json`, `tracing`, `surrealdb`

#### Changed
- Workspace now includes three crates: `cli`, `app/src-tauri`, `server`
- Phase renumbering: Visual Interface moved from Phase 3 to Phase 4

#### Documentation
- Added `PHASE3_GUIDE.md` with quick start guide
- Updated `ARCHITECTURE.md` with comprehensive Phase 3 section
- Updated `README.md` with federation examples
- Enhanced batch scripts with color-coded output

#### Technical Details
- **Performance**: No impact on scan speed (governance is opt-in)
- **Test Coverage**: 12 governance tests (100% passing)
- **Binary Size**: Server ~15MB; CLI remains <20MB
- **Security**: No raw data upload, dry-run by default, local autonomy maintained

---

## [0.2.0] - 2024-12-16

### "The Semantic Bridge" - Phase 2 Intelligence Layer

**This release introduces intelligent content analysis to Spectra, transitioning from pure topology (size/location) to typography (meaning/risk). The system now possesses "Sight" beyond mere file dimensions.**

#### Added

**Analysis Module (`cli/src/analysis/`):**
  - entropy.rs: Shannon entropy calculation on file headers (first 8KB)
  - heuristics.rs: Pattern-based risk scoring for sensitive files
  - semantic.rs: Optional AI content classification using rust-bert
  - mod.rs: Public API for analysis capabilities

**Entropy Profiling:**
- Calculates Shannon entropy (0.0 to 8.0 scale)
- Detects encrypted, compressed, or obfuscated files
- Microsecond-level performance
- Read-only on first 8KB of files

**Risk Detection System:**
- 5-level classification: None, Low, Medium, High, Critical
- Detects: passwords, secrets, keys, tokens, certificates, SSH keys
- Path-aware detection (e.g., `.ssh/id_rsa` flagged as Critical)
- Zero file reads required - filename/path pattern matching only

**AI Content Classification (Optional):**
- rust-bert DistilBERT zero-shot classification
- Categories: legal contract, source code, financial invoice, log file
- Feature-gated to keep base binary small
- Only analyzes text files (low entropy)

**CLI Enhancements:**
- `--analyze`: Enable entropy + risk analysis
- `--semantic`: Enable AI classification (requires 'semantic' feature)
- Enhanced output with risk icons (🔴 Critical, 🟠 High, 🟡 Medium, 🟢 Low)
- JSON output includes all analysis metadata

**Dependencies:**
- `regex`, `lazy_static`, `rust-bert` (optional), `tempfile`

#### Changed
- Version: 0.1.0 → 0.2.0
- FileRecord structure: Added `entropy`, `risk_level`, and `semantic_tag` fields
- Main scan loop: Refactored to use `.flatten()` for cleaner error handling

#### Documentation
- Updated `ARCHITECTURE.md` with Phase 2 implementation details
- Updated `README.md` with new capabilities and usage examples

#### Technical Details
- **Performance**: Post-scan analysis on top N files only
- **Privacy**: All analysis is local; no data leaves the machine
- **Test Coverage**: 8 unit tests (100% passing)
- **Binary Size**: Base ~15MB; AI features require LibTorch (~500MB)

---

## [0.1.0] - 2024-12-14

### "The Ignition" - Initial Proof of Concept

**This release establishes the core high-performance scanning engine, proving that a Rust-based parallel architecture can outperform traditional walkers.**

#### Added

**Core Scanning Engine:**
- Multi-threaded directory walker using `jwalk`
- BinaryHeap algorithm for tracking top N largest files (O(1) memory)
- Extension profiling (grouping files by type)
- Heavy Hitters identification (top largest files)

**CLI Interface:**
- **Human Mode**: Pretty-printed summaries with readable units
- **Agent Mode** (`--json`): Structured JSON output for pipelines
- Configurable path and file limit

**Architecture:**
- Dual-crate workspace: CLI + GUI separation
- Established "Trojan Horse" architectural blueprint

**Dependencies:**
- `jwalk`, `serde`, `clap`, `humansize`, `anyhow`

#### Technical Details
- **Performance**: Sub-second scanning (<1s for 100K files on NVMe)
- **Memory**: <20MB RAM for 1M files
- **Binary Size**: ~15MB (CLI)

---

*"Start by mapping the backyard, end by mapping the world."*