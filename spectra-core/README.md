# spectra-core

Core scanning primitives for SPECTRA - file system traversal and basic statistics.

## Overview

`spectra-core` is a lightweight, high-performance library for scanning file systems and collecting basic statistics. It provides the foundation for the SPECTRA ecosystem without any analysis, governance, or federation features.

## Features

- **Parallel file system scanning** using `jwalk`
- **BinaryHeap-based top files tracking** (O(n log k) complexity)
- **Extension-based file statistics aggregation**
- **Minimal dependencies** (jwalk, serde, anyhow)
- **Platform-agnostic** (Windows, Linux, macOS)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
spectra-core = { path = "../spectra-core" }
```

Basic example:

```rust
use spectra_core::Scanner;
use std::path::PathBuf;

fn main() {
    let scanner = Scanner::new(PathBuf::from("./"), 10);
    let stats = scanner.scan().expect("Scan failed");

    println!("Total files: {}", stats.total_files);
    println!("Total size: {} bytes", stats.total_size_bytes);

    for file in &stats.top_files {
        println!("{}: {} bytes", file.path, file.size_bytes);
    }
}
```

## Architecture

### Data Model

- **FileRecord**: Simplified file information (path, size only)
- **ExtensionStat**: Extension-based statistics (count, total size)
- **ScanStats**: Complete scan results

### Scanner

The `Scanner` struct performs parallel directory traversal:

1. Uses `jwalk` for multi-threaded file system walking
2. Maintains a BinaryHeap for efficient top-K file tracking
3. Aggregates extension statistics in a HashMap
4. Returns complete `ScanStats` with timing information

## Design Principles

- **Speed First**: Minimal overhead, parallel by default
- **Simple Scope**: Only basic scanning, no analysis
- **Zero Analysis**: No entropy, risk, or semantic features
- **Workspace Integration**: Designed for SPECTRA workspace

## Testing

Run tests:

```bash
cargo test -p spectra-core
```

## License

Same as parent SPECTRA project.
