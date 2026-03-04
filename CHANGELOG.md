# Changelog

All notable changes to the Spectra project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### v0.6.0 - "The Living Engine" - Performance, Visualization & Intelligence

**10 improvements inspired by open-source ecosystem research (Spacedrive, fclones, diskonaut, Filelight, notify-rs, and others).**

#### Core Library (`spectra-core` v0.2.0)

**Device-Aware I/O Tuning:**
- Auto-detects storage type (SSD vs HDD) using `sysinfo` crate
- Thread count auto-tuned: SSDs get full CPU parallelism, HDDs get 1-2 threads
- `DeviceType` enum and `detect_device_type()` / `recommended_threads()` public API
- Builder pattern: `Scanner::new(path, limit).with_threads(n)`

**Progressive Scan Results:**
- `Scanner::with_progress(callback)` builder method for streaming scan updates
- `ScanProgress` struct emits files_scanned, folders_scanned, bytes_scanned
- Progress callback fires every 1000 items for low overhead
- Tauri integration emits `scan-progress` events to the frontend

**Hash/Entropy Caching:**
- New `cache.rs` module with `ScanCache` struct
- Metadata-keyed cache: `mtime + size` invalidation (no re-read needed)
- Persists as JSON in `~/.spectra/cache/scan_{hash}.json`
- API: `load()`, `get_entropy()`, `put_entropy()`, `save()`
- 2 unit tests for cache operations

**Path Prefix Compression:**
- New `path_pool.rs` module with `PathPool` struct
- Interns common directory prefixes to reduce memory on large scans
- API: `intern()` returns `CompactPath`, `resolve()` reconstructs full path
- `estimated_savings()` reports memory saved
- 3 unit tests

**Transport Abstraction:**
- New `transport.rs` module with unified command interface
- `SpectraCommand` enum: Scan, GetHistory, GetVelocity, GetSnapshot
- `SpectraResponse` enum: ScanResult, History, Velocity, Snapshot, Error
- `Transport` trait with `execute()` method
- `DirectExecutor` for in-process scan commands
- Enables future CLI, Tauri IPC, and HTTP transport sharing

**New Dependencies:**
- `sysinfo = "0.32"` (device detection)
- `serde_json = "1.0"` (cache serialization)

#### CLI Enhancements (`spectra-cli`)

**IQR-Based Entropy Outlier Detection:**
- New `analysis/outliers.rs` module
- Statistical IQR method: Q1, Q3, IQR, lower/upper fences
- Identifies anomalous entropy values relative to peers (not fixed thresholds)
- Outlier files flagged with `entropy_outlier: true` in output
- Console output: quartile stats and outlier warnings
- 5 unit tests

**Filesystem Watching:**
- New `watch.rs` module using `notify` crate v6
- `--watch` CLI flag for real-time monitoring after scan
- `FileSystemWatcher` with `WatchEvent` struct and `WatchEventKind` enum
- Uses `ReadDirectoryChangesW` on Windows, inotify on Linux, FSEvents on macOS
- Event loop with configurable poll timeout

**Cache Integration:**
- Entropy results cached between runs (via `spectra-core` ScanCache)
- Cache hit counter displayed in output
- Automatic invalidation when file metadata changes

**New Dependencies:**
- `notify = "6"` (filesystem watching)

#### Server Enhancements (`spectra-server`)

**New API Endpoints:**
- `GET /api/v1/snapshot/:agent_id?timestamp=<ts>` - Point-in-time snapshot retrieval
- `GET /api/v1/aggregate/:agent_id?start=&end=&bucket_seconds=` - Time-series bucketing with configurable intervals

**Database Indexes:**
- `idx_snapshots_agent` on `agent_id` column
- `idx_snapshots_agent_time` on `agent_id + timestamp` compound index

#### Frontend Enhancements (`app`)

**Sunburst Visualization:**
- New `SunburstChart.tsx` component using `@nivo/sunburst`
- Table/Sunburst toggle in Top Extensions card
- Shows top 12 extensions + "other" bucket
- Interactive hover tooltips with size and count

**Progressive Scan UI:**
- Real-time progress display during scanning (files, folders, bytes)
- Animated progress bar with pulse animation
- Listens for Tauri `scan-progress` events

**Session Space-Freed Counter:**
- Green banner shows total bytes/files freed during session
- Tracks deletions via `window.__spectraTrackDeletion` (callable from Tauri commands)
- Persists across scans within same session

**Device Info Display:**
- Overview card shows detected device type and thread count
- Reflects device-aware I/O tuning from core

**New Dependencies:**
- `@nivo/sunburst ^0.84.0`

#### Test Results

```
spectra-core:  9 tests passed (was 1)
spectra-cli:  17 tests passed (was 12)
workspace:    26 tests total
clippy:        0 warnings
tsc --noEmit:  Clean
```

### Security Hardening & Dependency Audit (v0.5.1)

**Security Fixes:**
- **Content Security Policy**: Enabled proper CSP in Tauri configuration (was `null`/disabled)
  - `default-src 'self'`, restricted `connect-src` to `localhost:3000`
  - `'unsafe-inline'` for `style-src` (required by MUI/Emotion CSS-in-JS)
- **CORS Restriction**: Replaced `CorsLayer::permissive()` with an explicit origin allowlist
  - Default origins: `http://localhost:1420`, `tauri://localhost`, `https://tauri.localhost`
  - Configurable via `SPECTRA_CORS_ORIGINS` environment variable (comma-separated)
  - Only allows `GET`/`POST` methods and `Content-Type`/`X-API-Key` headers
- **API Key Authentication**: Added middleware to Spectra Server
  - Set `SPECTRA_API_KEY` environment variable to enable
  - All requests must include `X-API-Key` header when enabled
  - Graceful fallback: unauthenticated access in development mode when unset
- **React Error Boundary**: Added crash-recovery UI wrapping `<App />`
  - Prevents white-screen crashes from unhandled React errors
  - Displays error message with "Try again" button

**Dependency Upgrades:**
- Replaced deprecated `lazy_static` with `std::sync::OnceLock` (stable since Rust 1.70)
- Upgraded `reqwest` from 0.11 to 0.12 (eliminates 4 transitive vulnerabilities)
- Reduced `tokio` features from `"full"` to `["rt-multi-thread", "macros", "net"]`
- Updated `bytes` 1.11.0 → 1.11.1 (fixes integer overflow in `BytesMut::reserve`)
- Updated `rkyv` 0.7.45 → 0.7.46 (fixes undefined behavior on OOM)
- Updated `time` 0.3.44 → 0.3.47 (fixes DoS via stack exhaustion)

**Frontend Fixes:**
- Fixed memory leak in `TimeSlider` useEffect (added cancelled-flag cleanup pattern)
- Fixed mutating `.sort()` → `[...history].sort()` to avoid mutating input arrays
- Added test dependencies to `package.json`: `vitest`, `@testing-library/react`, `@testing-library/jest-dom`, `jsdom`
- Added `test` and `test:watch` npm scripts

**Audit Results (Post-Fix):**
- Resolved: 7 → 4 vulnerabilities (remaining 4 are transitive via `surrealdb` and `rust-bert`)
- `cargo fmt`: Clean
- `cargo clippy -D warnings`: Zero warnings across all 4 crates
- `tsc --noEmit`: Clean
- All 13 Rust tests passing

### Added - Phase 3.5: Time-Travel Analytics (v0.5.0)

**⏳ Time-Series Intelligence Engine:**
- **SurrealDB Integration**: Persistent time-series storage for filesystem snapshots
  - In-memory mode for development (instant startup)
  - RocksDB backend support for production (persistent storage)
  - Indexed queries on `agent_id` and `timestamp` for O(log n) lookups
- **Velocity Calculation**: Data growth rate analytics
  - Bytes per second with adaptive units (B/s, KB/s, MB/s, GB/s)
  - Per-extension delta breakdown (what caused the change)
  - Positive/negative growth detection (expansion vs. cleanup)
  - Duration-based analytics (seconds to days)
- **Historical Queries**: Ask temporal questions
  - "How fast is data growing?"
  - "What caused the spike last Tuesday?"
  - "Which file types are accumulating fastest?"

**Backend (Spectra Server):**
- `POST /api/v1/ingest` - Enhanced snapshot ingestion with time-series storage
- `GET /api/v1/history/:agent_id` - Retrieve available snapshot timestamps
- `GET /api/v1/velocity/:agent_id?start=<ts>&end=<ts>` - Calculate growth velocity
- Comprehensive error handling and structured logging (tracing)
- CORS enabled for React frontend connectivity
- Smart snapshot comparison with extension-level attribution
- Fallback handling for missing or incomplete data

**Frontend (Spectra Vision):**
- **TimeSlider Component**: Interactive timeline navigation
  - Dual-range sliders for start/end timestamp selection
  - Auto-selects first and last snapshots by default
  - Human-readable date formatting
  - Smart boundary validation (prevents invalid ranges)
  - Snapshot counter and period display
- **VelocityCard Component**: Growth metrics visualization
  - Large growth/shrinkage indicators with color coding (green/red)
  - Velocity rate display with dynamic formatting
  - Top 10 contributing extensions sorted by impact
  - Per-extension file count deltas
  - Duration breakdown (days, hours, minutes, seconds)
- **Tab Navigation**: Dual-mode interface
  - "📂 Local Scan" tab (existing functionality)
  - "⏳ Time-Travel Analytics" tab (new)
  - Clean separation of concerns
- **API Client** (`app/src/api.ts`):
  - `fetchAgentHistory()` - Get snapshot timestamps
  - `fetchVelocity()` - Calculate velocity between two points
  - Utility formatters for bytes, timestamps, and velocity rates
- **TypeScript Types** (`app/src/types.ts`):
  - `AgentSnapshot` - Snapshot data structure
  - `VelocityReport` - Growth analytics report
  - `ExtensionDelta` - Per-extension change metrics
- **Dark Theme Styling**: Consistent with existing enterprise UI
  - Gradient backgrounds for cards
  - Animated range sliders with hover effects
  - Color-coded deltas (green = growth, red = shrinkage)
  - Responsive grid layout

**Testing & Validation:**
- `test-time-travel.sh` (Linux/macOS): Bash simulation script
- `test-time-travel.ps1` (Windows): PowerShell simulation script
- Both scripts inject 5 realistic snapshots spanning 24 hours:
  - Baseline: 1GB total data
  - Log accumulation: +500MB over time
  - Video spike: +500MB at midpoint
  - Total velocity: ~11.5 KB/s average
- Automated verification of all endpoints
- Comprehensive test output with metrics validation

**Documentation:**
- `docs/TIME_TRAVEL_ANALYTICS.md`: Complete feature guide
  - Architecture diagrams and data flow
  - API specification with examples
  - Usage guide (Quick Start, Production Deployment)
  - GUI features walkthrough
  - Database schema documentation
  - Performance considerations and scaling recommendations
  - Troubleshooting section
  - FAQ and roadmap
- Updated `README.md` with Time-Travel Quick Start
- Updated all documentation references

**Dependencies:**
- Added `tower-http 0.5` (CORS support)
- Added `anyhow 1.0` (error handling)
- Updated `surrealdb 1.0` with `kv-mem` feature
- Zero breaking changes to existing dependencies

**Performance Characteristics:**
- **Storage**: ~2KB per snapshot (metadata only, no file content)
- **Query Speed**: O(log n) for history, O(1) for two-point velocity
- **Scalability**: Tested with 100+ snapshots per agent
- **Memory**: In-memory mode uses <50MB for typical workloads
- **Bandwidth**: <2MB/month per agent with hourly snapshots

**Privacy & Security:**
- Zero file content storage (metadata only)
- Only aggregated statistics transmitted
- No PII or sensitive data collected
- Localhost-first design (network optional)

### Added - CI/CD & Quality Improvements

**Continuous Integration:**
- **GitHub Actions Workflows**: Automated CI pipelines for all pushes and pull requests
  - `rust-ci.yml`: Multi-platform Rust testing (Ubuntu, Windows, macOS)
    - Code formatting check (`cargo fmt`)
    - Clippy linting with warnings as errors
    - Compilation check (`cargo check`)
    - Test suite execution on all platforms
  - `frontend-ci.yml`: Frontend build and type checking
    - TypeScript type checking with strict mode
    - Production build validation
    - Node.js 20 LTS support
- **Quality Gates**: All checks must pass before merge
- **Rust Caching**: Swatinem/rust-cache for faster build times
- **Dependency Management**: Legacy peer deps support for React 19 compatibility

**Code Quality Fixes:**
- Fixed unused variable warning in `RiskTreemap.tsx` (replaced with array destructuring)
- Excluded test files from TypeScript compilation to avoid missing test dependency errors
- Updated workflows to skip optional `semantic` feature (rust-bert) which requires libtorch
- All local CI checks passing before push to GitHub

### Added - Modular Refactoring (Pre-Alpha Status)

#### 🏗️ **BREAKING CHANGE**: New Modular Architecture

**Core Library (`spectra-core` v0.1.0):**
- Extracted core scanning logic into standalone, reusable library
- New `Scanner` struct with parallel `jwalk` traversal and BinaryHeap optimization
- Simplified `FileRecord` data model (path, size_bytes only - NO analysis fields)
- `ExtensionStat` for file type aggregation
- `ScanStats` result structure with comprehensive metrics
- Platform-agnostic design (Windows, Linux, macOS)
- **Minimal dependencies**: jwalk 0.8, serde 1.0, anyhow 1.0
- O(n log k) complexity for top files tracking
- Comprehensive integration tests
- Dedicated `README.md` with usage guide

**CLI Refactoring (`spectra-cli` v0.2.0):**
- **BREAKING**: Now depends on `spectra-core` for basic scanning
- New `AnalyzedFileRecord` wrapping core `FileRecord` + analysis fields (entropy, risk, semantic)
- `CliScanStats` wrapper for type-safe conversions
- **Backward Compatible**: All features preserved:
  - ✅ Phase 2: Entropy analysis, risk scoring, semantic classification
  - ✅ Phase 3: Governance policies, federation
- Reduced code duplication (~50 lines removed from main.rs)
- Clean separation: Core handles scanning, CLI adds intelligence layers

**Tauri App Enhancement (`app` v0.1.0):**
- Added `spectra-core` dependency for native scanning
- **NEW COMMAND**: `scan_directory(path, limit)` using core Scanner for statistics
- **PRESERVED**: `get_scan_tree()` for TreeNode visualization (backward compatible)
- Dual command architecture supports both analytics and visualization use cases
- Updated mock entropy comment to reference future core integration

**Workspace Updates:**
- Added `spectra-core` to workspace members (now 4 crates)
- Updated `Cargo.toml` with clean dependency graph:
  ```
  spectra-server ← (independent)
  spectra-cli    ← spectra-core
  app            ← spectra-core
  ```
- No circular dependencies, clean layered architecture

### Changed

**Quality Assurance:**
- **NEW**: Comprehensive validation script `validate-refactor.bat`
- Automated quality gates:
  - ✓ Code formatting check (cargo fmt --check)
  - ✓ Linting with clippy (-D warnings)
  - ✓ Unit tests (all crates)
  - ✓ Integration tests (CLI basic + analysis)
  - ✓ Release builds (all crates)
- Color-coded output for better visibility
- Step-by-step progress tracking
- Pre-alpha quality gate system

**Code Quality:**
- Fixed clippy warning: `manual_flatten` in core scanner loop
- Applied `.flatten()` pattern for cleaner iteration
- All code formatted with `cargo fmt`
- Zero clippy warnings across workspace

**Architecture:**
- **MAJOR**: Transitioned from monolithic to modular design
- Scanning logic now shared between CLI and GUI
- Clear separation of concerns:
  - **Core**: Basic scanning primitives (Phase 1)
  - **CLI**: Analysis + Governance + Federation (Phases 2-3)
  - **App**: Visualization (Phase 4)
  - **Server**: Control plane (Phase 3)

### Technical Details

**New File Structure:**
```
spectra-core/
├── Cargo.toml           # Core library manifest
├── README.md            # Usage guide
└── src/
    └── lib.rs           # Scanner, FileRecord, ScanStats (all-in-one)
```

**Modified Files:**
- `Cargo.toml` (root): Added spectra-core to workspace.members
- `cli/Cargo.toml`: Added `spectra-core = { path = "../spectra-core" }`
- `cli/src/main.rs`:
  - Lines 1-10: Import from spectra_core
  - Lines 52-103: New type conversions (AnalyzedFileRecord, CliScanStats)
  - Lines 201-206: Use Scanner instead of inline BinaryHeap logic
- `app/src-tauri/Cargo.toml`: Added `spectra-core = { path = "../../spectra-core" }`
- `app/src-tauri/src/lib.rs`:
  - Lines 5-6: Import Scanner, ScanStats
  - Lines 116-138: New scan_directory command

**Dependencies:**
- No new external dependencies added
- Internal dependency tree:
  ```
  spectra-core (new)
    ├─> jwalk 0.8
    ├─> serde 1.0
    └─> anyhow 1.0

  spectra-cli
    ├─> spectra-core (new)
    └─> (existing dependencies unchanged)

  app
    ├─> spectra-core (new)
    └─> (existing dependencies unchanged)
  ```

**Test Results:**
```
✓ spectra-core: 1 test passed
✓ spectra-cli:  12 tests passed
✓ workspace:    13 tests total
✓ clippy:       0 warnings
✓ fmt:          All code formatted
```

**Performance:**
- No regression: Same parallel scanning algorithm
- Memory usage: Unchanged (~20MB for 1M files)
- Scan speed: Unchanged (sub-second for 100K files on NVMe)

### Migration Guide

**For CLI Users:**
- ✅ No breaking changes in CLI interface
- ✅ All commands work identically
- ✅ JSON output format unchanged
- ✅ Performance unchanged

**For Tauri App Users:**
- ✅ Existing `get_scan_tree` still works
- 🆕 New `scan_directory(path, limit)` available for statistics
- Frontend integration optional

**For Developers:**
```rust
// NEW: Import core scanning
use spectra_core::{Scanner, ScanStats};

// Basic scan in 3 lines
let scanner = Scanner::new("/path/to/scan", 10);
let stats = scanner.scan()?;
println!("Found {} files", stats.total_files);
```

### Known Limitations (Pre-Alpha)

- ⚠️ Core library API unstable (may change before v1.0)
- ⚠️ Documentation incomplete
- ⚠️ Production deployment NOT recommended
- ⚠️ Cross-platform testing needed (currently Windows-focused)
- ⚠️ Performance benchmarks pending

### Planned Features
- **Temporal Navigation**: Time-slider for traversing historical snapshots in GUI
- **Persistence Layer**: Integration of DuckDB or Rusqlite for queryable local history
- **SurrealDB Integration**: Full graph database support for topology storage
- **Policy Management UI**: Web interface for creating and managing governance policies
- **Real Entropy Integration**: Replace mock calculation with full Phase 2 entropy engine in GUI
- **Agent Authentication**: Secure authentication and authorization for federated agents
- **Advanced Filtering**: Filter treemap by risk level, file type, or size
- **Export Capabilities**: Save visualizations as images or data exports
- **Comparative Views**: Side-by-side comparisons of different directories or time periods

---

## [0.4.0] - 2025-12-16

### "The Lens" - Phase 4 Visualization Layer

**This release brings Spectra to life with an interactive visual interface. The system now has "Eyes" - transforming raw entropy metrics into intuitive risk treemaps where danger is visible at a glance.**

#### Added

**Risk Treemap Visualization:**
- Interactive hierarchical treemap where color = entropy risk, size = data volume
- **Visual Risk Encoding:**
  - 🟩 Green (0-3.0): Low entropy - Text, code, configs
  - 🟨 Yellow (3.0-6.0): Medium - Binaries, media
  - 🟧 Orange (6.0-7.5): High - Compressed data
  - 🟥 Red (7.5-8.0): Critical - Encryption, high randomness
- Hover tooltips with detailed entropy scores and file sizes
- Click inspection for drill-down into file/directory details
- Responsive design with dark-themed interface optimized for data density

**GUI Application Components:**
- **Frontend** (`app/src/`):
  - `RiskTreemap.tsx`: Nivo-powered D3 treemap visualization component
  - `App.tsx`: Main application with scan controls and state management
  - `App.css`: Dark-themed styling with visualization container
  - `__tests__/RiskTreemap.test.tsx`: Component test suite
- **Backend** (`app/src-tauri/src/lib.rs`):
  - `TreeNode` data structure for hierarchical file system representation
  - `get_scan_tree`: Tauri command for directory scanning
  - `calculate_mock_entropy`: Extension-based entropy simulation
  - Recursive scanning with configurable depth limit (default: 3 levels)

**Launch Scripts:**
- `launch-vision.bat`: Windows one-click launcher (root level)
- `launch-vision.sh`: Unix/Linux/macOS one-click launcher (root level)
- `app/launch-spectra-vision.bat`: Sophisticated Windows launcher with dependency checks
- `app/launch-spectra-vision.sh`: Sophisticated Unix launcher with color-coded output
- Auto-detection of missing dependencies (Node.js, Cargo)
- Progress indicators and user-friendly error messages

**Dependencies:**
- Frontend: `@nivo/treemap`, `@nivo/core`, `@mui/material`, `@emotion/react`, `@emotion/styled`, `d3-scale-chromatic`, `clsx`
- No changes to backend Rust dependencies (serde and serde_json already present)

#### Changed
- GUI application fully functional with Phase 4 features
- Enhanced project structure with components directory
- Improved user experience with loading states and error handling

#### Documentation
- Created comprehensive `app/README.md` with component documentation
- Updated main `README.md` with Phase 4 quick start and visual encoding guide
- Added usage instructions and development guidelines
- Documented future enhancements (temporal navigation, real entropy integration)

#### Technical Details
- **Framework**: Tauri 2.0 (Rust backend + React 19 frontend)
- **Build Tool**: Vite 7 with hot module replacement
- **Visualization**: Nivo 0.84.0 (React-native D3 bindings)
- **Performance**: Depth-limited scanning (3 levels) for responsive UI
- **Bundle Size**: ~50MB (includes visualization libraries)
- **Test Coverage**: 4 component tests for RiskTreemap

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
- `regex`, `rust-bert` (optional), `tempfile`

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