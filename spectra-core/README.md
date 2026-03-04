# spectra-core

Core scanning primitives for SPECTRA - file system traversal, device-aware I/O, caching, and transport abstractions.

## Overview

`spectra-core` is a high-performance library for scanning file systems and collecting statistics. It provides the foundation for the SPECTRA ecosystem with device-aware parallelism, entropy caching, path compression, and a unified transport layer.

## Features

- **Parallel file system scanning** using `jwalk` with device-aware thread tuning
- **Device detection** (SSD vs HDD) for automatic I/O optimization
- **Progressive scan callbacks** for real-time UI updates
- **BinaryHeap-based top files tracking** (O(n log k) complexity)
- **Extension-based file statistics aggregation**
- **Entropy/hash caching** with metadata-based invalidation
- **Path prefix compression** for reduced memory on large scans
- **Transport abstraction** for unified CLI/Tauri/HTTP command interface
- **Platform-agnostic** (Windows, Linux, macOS)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
spectra-core = { path = "../spectra-core" }
```

### Basic scan:

```rust
use spectra_core::Scanner;
use std::path::PathBuf;

fn main() {
    let scanner = Scanner::new(PathBuf::from("./"), 10);
    let stats = scanner.scan().expect("Scan failed");

    println!("Total files: {}", stats.total_files);
    println!("Total size: {} bytes", stats.total_size_bytes);
    println!("Device: {:?} ({} threads)", stats.device_type, stats.threads_used.unwrap_or(0));

    for file in &stats.top_files {
        println!("{}: {} bytes", file.path, file.size_bytes);
    }
}
```

### With progress callback:

```rust
use spectra_core::Scanner;
use std::path::PathBuf;

let scanner = Scanner::new(PathBuf::from("./"), 10)
    .with_progress(|progress| {
        println!("{} files, {} bytes scanned", progress.files_scanned, progress.bytes_scanned);
    });
let stats = scanner.scan().expect("Scan failed");
```

### With custom thread count:

```rust
use spectra_core::Scanner;
use std::path::PathBuf;

let scanner = Scanner::new(PathBuf::from("./"), 10)
    .with_threads(4);
let stats = scanner.scan().expect("Scan failed");
```

### Entropy caching:

```rust
use spectra_core::ScanCache;
use std::path::PathBuf;

let root = PathBuf::from("./");
let mut cache = ScanCache::load(&root);

// Check cache before computing
if let Some(entropy) = cache.get_entropy(&PathBuf::from("file.rs"), 1024) {
    println!("Cached entropy: {}", entropy);
} else {
    let entropy = 3.2; // compute entropy...
    cache.put_entropy(&PathBuf::from("file.rs"), 1024, entropy);
}

cache.save().expect("Failed to save cache");
```

### Path prefix compression:

```rust
use spectra_core::path_pool::PathPool;

let mut pool = PathPool::new();
let compact = pool.intern("/home/user/documents/report.pdf");
let full_path = pool.resolve(&compact);
println!("Savings: {} bytes", pool.estimated_savings(1000));
```

## Architecture

### Modules

- **`lib.rs`** - Scanner, FileRecord, ScanStats, DeviceType, ScanProgress
- **`cache.rs`** - ScanCache for persistent entropy/hash caching
- **`path_pool.rs`** - PathPool for path prefix compression
- **`transport.rs`** - Transport trait, SpectraCommand/SpectraResponse enums

### Scanner

The `Scanner` struct performs parallel directory traversal:

1. Detects storage device type (SSD/HDD) via `sysinfo`
2. Auto-tunes thread count based on device (SSDs get full parallelism, HDDs get 1-2 threads)
3. Uses `jwalk` for multi-threaded file system walking
4. Maintains a BinaryHeap for efficient top-K file tracking
5. Aggregates extension statistics in a HashMap
6. Emits progress callbacks every 1000 items
7. Returns complete `ScanStats` with timing and device info

### Device Detection

```rust
use spectra_core::{detect_device_type, recommended_threads, DeviceType};
use std::path::Path;

let device = detect_device_type(Path::new("/data"));
let threads = recommended_threads(device);
// SSD -> all CPUs, HDD -> 2, Unknown -> CPUs/2
```

## Design Principles

- **Speed First**: Device-aware parallelism, minimal overhead
- **Smart Caching**: Avoid recomputation when files haven't changed
- **Memory Efficient**: Path compression reduces footprint on million-file scans
- **Workspace Integration**: Designed for SPECTRA workspace (CLI, GUI, Server)
- **Extensible Transport**: Unified command/response model for any execution context

## Testing

Run tests:

```bash
cargo test -p spectra-core
# 9 tests: scanner, cache (2), path_pool (3), transport (3)
```

## License

Dual-licensed under MIT and Apache 2.0.
