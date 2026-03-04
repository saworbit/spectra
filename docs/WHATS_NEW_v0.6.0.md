# What's New in Spectra v0.6.0 - The Living Engine

**Release Date:** 2026-03-04
**Code Name:** "The Living Engine"

---

## Overview

v0.6.0 delivers **10 improvements** inspired by open-source ecosystem research. These enhancements make Spectra smarter, faster, and more adaptive - from device-aware I/O tuning to real-time filesystem watching.

### Research Sources

Improvements were inspired by studying: Spacedrive (transport abstraction), fclones (device-aware I/O, caching), diskonaut (session tracking), Filelight/DaisyDisk (sunburst visualization), notify-rs (filesystem watching), and entropyscan-rs (outlier detection).

---

## New Features

### 1. Device-Aware I/O Tuning

Spectra now auto-detects your storage type and optimizes parallelism:

- **SSD detected**: Full CPU core parallelism for maximum throughput
- **HDD detected**: 1-2 threads to avoid seek thrashing
- **Unknown**: Half CPU cores as a safe default

```rust
// Automatic - no configuration needed
let scanner = Scanner::new(path, 10);
// Thread count is auto-tuned based on device type

// Or override manually
let scanner = Scanner::new(path, 10).with_threads(4);
```

The Overview card in the GUI now shows the detected device type and thread count.

### 2. Progressive Scan Results

No more waiting for the full scan to complete. The UI now shows real-time progress:

- Live counters: files scanned, folders scanned, bytes processed
- Animated progress bar with pulse effect
- Streamed from Rust backend via Tauri events (fires every 1000 items)

### 3. Sunburst Visualization

A new alternative to the table view for extension breakdown:

- Toggle between **Table** and **Sunburst** views in the Top Extensions card
- Shows top 12 extensions by size with remaining grouped as "other"
- Interactive hover tooltips with size and file count
- Built with `@nivo/sunburst`

### 4. IQR Entropy Outlier Detection

Instead of fixed thresholds, Spectra now uses statistical methods:

- Calculates Q1, Q3, and IQR (Interquartile Range) across scanned files
- Files with entropy outside 1.5x IQR from quartiles are flagged as outliers
- Console output shows quartile statistics and outlier warnings
- Identifies anomalies relative to peers, not absolute values

### 5. Hash/Entropy Caching

Skip redundant computation on unchanged files:

- Cache keyed by file metadata (mtime + size)
- Automatic invalidation when file changes
- Persisted as JSON in `~/.spectra/cache/`
- Cache hit counter shown in CLI output

### 6. Filesystem Watching

Real-time monitoring after initial scan:

```bash
spectra-cli --path /data --watch
```

- Uses `notify` crate v6 (ReadDirectoryChangesW on Windows, inotify on Linux, FSEvents on macOS)
- Reports create, modify, and delete events
- Ctrl+C to stop watching

### 7. Session Space-Freed Counter

When governance actions free disk space, a green banner appears in the GUI showing:

- Total bytes freed this session
- Total files freed this session
- Persists across multiple scans within the same session

### 8. Path Prefix Compression

Reduces memory footprint on large scans:

- `PathPool` interns common directory prefixes
- Only stores unique prefixes once, references them by ID
- `estimated_savings()` reports how many bytes were saved

### 9. Transport Abstraction

Unified command/response model for any execution context:

- `SpectraCommand` enum: Scan, GetHistory, GetVelocity, GetSnapshot
- `SpectraResponse` enum: ScanResult, History, Velocity, Snapshot, Error
- `Transport` trait with `execute()` method
- Enables sharing logic across CLI, Tauri IPC, and HTTP

### 10. Time-Series Aggregation

Two new server endpoints for advanced time-series queries:

- `GET /api/v1/snapshot/:agent_id?timestamp=<ts>` - Point-in-time snapshot
- `GET /api/v1/aggregate/:agent_id?start=&end=&bucket_seconds=` - Time-series bucketing

Plus optimized database indexes for query performance.

---

## Test Results

```
spectra-core:   9 tests passed (was 1)
spectra-cli:   17 tests passed (was 12)
workspace:     26 tests total
cargo clippy:   0 warnings
tsc --noEmit:   Clean
```

---

## New Files

```
spectra-core/src/cache.rs       # Entropy/hash caching
spectra-core/src/path_pool.rs   # Path prefix compression
spectra-core/src/transport.rs   # Transport abstraction
cli/src/analysis/outliers.rs    # IQR outlier detection
cli/src/watch.rs                # Filesystem watching
app/src/components/SunburstChart.tsx  # Sunburst chart component
```

## New Dependencies

| Crate/Package | Version | Purpose |
|---------------|---------|---------|
| `sysinfo` | 0.32 | Device type detection (SSD/HDD) |
| `notify` | 6 | Filesystem event watching |
| `@nivo/sunburst` | ^0.84.0 | Sunburst visualization |

---

## Getting Started

### For Existing Users

1. **Pull latest**: `git pull origin main`
2. **Rebuild**: `cargo build --workspace`
3. **Try device detection**: `cargo run -p spectra-cli -- --path ./ --analyze`
4. **Try watch mode**: `cargo run -p spectra-cli -- --path ./ --watch`
5. **Try sunburst**: Launch GUI, scan a directory, click "Sunburst" toggle

### For New Users

See [README.md](../README.md) for installation and quick start.

---

## What's Next?

- ML-powered anomaly detection
- Predictive storage forecasting
- Cost attribution analytics
- Alert webhooks for velocity thresholds
- Multi-agent comparison dashboard

---

## Related Documentation

- [CHANGELOG.md](../CHANGELOG.md) - Full version history
- [ARCHITECTURE.md](ARCHITECTURE.md) - System design
- [TIME_TRAVEL_ANALYTICS.md](TIME_TRAVEL_ANALYTICS.md) - Time-series guide
- [PHASE4_GUIDE.md](PHASE4_GUIDE.md) - Visualization guide
- [FAQ.md](FAQ.md) - Common questions

---

**"The engine is alive. It sees, it remembers, it adapts."**
