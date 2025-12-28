# Frequently Asked Questions (FAQ)

## General Questions

### What is Spectra?

Spectra (Scalable Platform for Enterprise Content Topology & Resource Analytics) is a high-performance data cartography and governance platform. It starts as a fast storage profiler and evolves into a distributed semantic governance brain for enterprise data management.

### Why choose Spectra over WizTree/ncdu/TreeSize?

While traditional tools focus on size and location, Spectra goes deeper:
- **Semantic understanding**: Analyzes file content, not just size
- **Risk detection**: Identifies sensitive files (passwords, keys, secrets)
- **Entropy analysis**: Detects encrypted/compressed files
- **Federation**: Distributed agent coordination for enterprise scale
- **Governance**: Policy-based automated file management
- **Time-travel**: Historical analytics to track data growth

### Is Spectra production-ready?

**No.** Spectra is currently in **pre-alpha** status. The API is unstable and subject to change. Use it for testing and evaluation, but not in production environments.

### What does "pre-alpha" mean?

Pre-alpha means:
- Core features are implemented but not fully tested at scale
- APIs may change without notice
- Documentation may be incomplete
- Cross-platform testing is ongoing
- Performance benchmarks are preliminary

## Installation & Setup

### What are the system requirements?

**Minimum**:
- OS: Windows 10+, Linux (kernel 4.4+), macOS 10.15+
- RAM: 100MB (can scan 1M files with ~20MB RAM)
- Disk: 50MB for CLI, 100MB for GUI
- CPU: Any modern processor (benefits from multi-core)

**For semantic features**:
- Additional RAM: 2GB+ (for rust-bert models)
- Additional Disk: 500MB+ (for LibTorch)

### How do I install Spectra?

Currently, install from source:

```bash
git clone https://github.com/saworbit/spectra.git
cd spectra
cargo build --release
```

Pre-built binaries will be available in future releases.

### Do I need to install PyTorch for semantic features?

Yes, the optional semantic classification feature requires LibTorch (PyTorch C++ library). See the [rust-bert documentation](https://github.com/guillaume-be/rust-bert) for installation instructions.

Most users won't need this feature and can use the default build without LibTorch.

## Usage Questions

### How fast is Spectra?

Performance depends on your storage and system:
- **NVMe SSD**: ~100,000 files/second
- **SATA SSD**: ~50,000 files/second
- **HDD**: ~10,000 files/second
- **Network drives**: Varies significantly

Example: Scanning 1 million files on NVMe takes ~10 seconds.

### Does Spectra work on network drives?

Yes, but performance will be limited by network latency. For best results on network shares:
- Use the federated agent architecture (run locally, report centrally)
- Scan during off-peak hours
- Consider increasing the file limit to reduce overhead

### Can I scan cloud storage (S3, Azure Blob, etc.)?

Not directly. Spectra scans local file systems. For cloud storage:
- Mount the storage locally (S3FS, Azure File Sync, etc.)
- Use cloud-native tools for inventory
- Wait for future Spectra cloud connectors (planned)

### What file systems are supported?

Spectra works with any file system accessible through standard OS APIs:
- **Windows**: NTFS, FAT32, exFAT, ReFS
- **Linux**: ext4, XFS, Btrfs, ZFS, etc.
- **macOS**: APFS, HFS+

### How much memory does Spectra use?

Memory usage is O(k) where k = number of top files tracked (default: 100):
- **Typical usage**: <20MB for 1M files
- **Heavy analysis**: +10-50MB depending on file content
- **Semantic mode**: +500MB-2GB for AI models

### Can Spectra delete files?

Yes, but with strict safety measures:
- Governance policies can specify delete actions
- **Dry-run by default**: Reports what would be deleted without actually deleting
- **Requires --enforce flag**: Must explicitly enable destructive operations
- **Audit logging**: All actions are logged
- **Offline by default**: Agents work locally without server

**CAUTION**: Use `--enforce` flag carefully in production!

## Technical Questions

### What programming languages are used?

- **Backend**: Rust 2021 edition
- **Frontend**: TypeScript 5.x + React 19
- **GUI Framework**: Tauri 2.0
- **Server**: Axum (async Rust web framework)

### Why Rust?

Rust provides:
- **Performance**: Zero-cost abstractions, no garbage collection
- **Safety**: Memory safety without runtime overhead
- **Concurrency**: Fearless parallelism with send/sync traits
- **Ecosystem**: Excellent crates for file I/O, serialization, and web

### How does parallel scanning work?

Spectra uses [jwalk](https://github.com/Byron/jwalk) for parallel directory traversal:
- Multiple threads scan different branches of the directory tree
- Lock-free BinaryHeap for top-N file tracking
- Zero-copy file metadata access where possible
- Automatic load balancing across cores

### What is entropy analysis?

Entropy measures randomness in data (Shannon entropy, 0-8 scale):
- **Low (0-3)**: Text, source code, configs
- **Medium (3-6)**: Binaries, images, audio
- **High (6-7.5)**: Compressed archives, video
- **Critical (7.5-8)**: Encrypted files, cryptographic keys

Spectra reads only the first 8KB of each file for entropy calculation.

### How does risk scoring work?

Risk scoring uses pattern matching:
- **File names**: password.txt, id_rsa, secret.key
- **Extensions**: .pem, .key, .p12, .pfx
- **Paths**: .ssh/, .aws/, .gnupg/
- **Content patterns**: BEGIN PRIVATE KEY, API key formats

No machine learning is used for basic risk detection (fast heuristics only).

### What is the semantic classification feature?

Optional AI-powered content classification using DistilBERT:
- Categories: legal contract, source code, financial invoice, log file
- Only analyzes text files (low entropy)
- Requires `--features semantic` build flag
- Adds ~500MB dependency (LibTorch)
- Slower than heuristics (~100ms per file)

## Architecture Questions

### How does federation work?

Spectra uses a Hub & Spoke model:
- **Agents**: Scan local file systems, push snapshots to hub
- **Hub (Server)**: Aggregates telemetry, distributes policies
- **Communication**: REST over HTTP/2
- **Offline capable**: Agents work without server connection

### What database does Spectra use?

Spectra server uses [SurrealDB](https://surrealdb.com/) for time-series storage:
- Multi-model database (document, graph, key-value)
- Built-in time-travel queries
- Rust-native client with zero-cost FFI
- **Status**: ✅ Fully integrated (Phase 3.5)
- In-memory mode for development, RocksDB for production

For local storage, future versions may add DuckDB or SQLite for offline velocity queries.

### Is there a web UI?

Currently, Spectra has:
- CLI (terminal interface)
- Desktop GUI (Tauri app)
- Server REST API

A web dashboard is planned for Phase 5.

### How extensible is Spectra?

Spectra is designed for extensibility:
- **Core library**: `spectra-core` for basic scanning
- **Analysis plugins**: Add custom analysis modules
- **Governance policies**: JSON/YAML policy files
- **API integration**: REST API for external tools
- **Custom visualizations**: React components for GUI

## Security & Privacy

### Does Spectra upload file content?

**No.** Spectra never uploads raw file content. Only metadata is transmitted:
- File paths
- Sizes
- Entropy scores
- Risk classifications
- Extension statistics

All analysis happens locally.

### How is sensitive data handled?

- **Local-first**: All processing happens on the agent machine
- **Metadata only**: Only aggregated stats sent to server
- **Configurable**: You control what metadata is shared
- **No telemetry**: No usage tracking or analytics
- **Open source**: Audit the code yourself

### Can I use Spectra in air-gapped environments?

Yes! Spectra agents work completely offline:
- No internet connection required
- No external dependencies at runtime
- All features work locally
- Federation is optional

### What about GDPR/compliance?

Spectra is designed with privacy in mind:
- No PII collection by default
- No cloud dependencies
- Full data sovereignty
- Audit trail for governance actions

However, YOU are responsible for:
- Configuring appropriate policies
- Ensuring compliant handling of scan results
- Managing access controls
- Following your organization's data policies

## Troubleshooting

### Spectra crashes on large directories

Possible solutions:
1. Increase the file limit: `--limit 100`
2. Scan subdirectories separately
3. Check available RAM
4. Report the issue with details

### Permission denied errors

Normal behavior. Spectra logs these and continues:
- Check if you have read permissions
- Run with appropriate privileges if needed
- Review error logs for problematic paths

### Slow scanning on network drives

Expected due to network latency. Optimize with:
- Run agents on network hosts directly
- Use federation mode
- Increase parallelism carefully (may saturate network)

### GUI doesn't start

Common issues:
1. **Node.js not installed**: Install Node.js 20 LTS
2. **Dependencies missing**: Run `npm ci --legacy-peer-deps` in `app/`
3. **Rust toolchain missing**: Install Rust stable
4. **Port conflict**: Check if port 1420 (default Tauri dev port) is available

### Build fails with "libtorch not found"

You're building with the `semantic` feature:
```bash
# Build without semantic features
cargo build --release -p spectra-cli

# Or install LibTorch (see rust-bert docs)
```

### Frontend CI fails with peer dependency errors

Known issue with React 19 and older packages:
- Workflows use `--legacy-peer-deps` flag
- Update your Node.js to 20 LTS
- Run `npm ci --legacy-peer-deps` instead of `npm ci`

## Contributing

### How can I contribute?

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines. Quick start:
1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Run CI checks locally
5. Submit a pull request

### What should I work on?

Check the [GitHub Issues](https://github.com/saworbit/spectra/issues) for:
- Issues labeled `good first issue`
- Issues labeled `help wanted`
- Feature requests

### I found a bug. What should I do?

1. Check if it's already reported in [Issues](https://github.com/saworbit/spectra/issues)
2. If not, create a new issue with:
   - Clear description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version, etc.)
   - Relevant logs or error messages

## Roadmap

### When will v1.0 be released?

No timeline yet. v1.0 will be released when:
- All Phase 1-4 features are stable
- APIs are finalized
- Comprehensive testing is complete
- Documentation is thorough
- Production deployments are successful

### What features are planned?

See [CHANGELOG.md](../CHANGELOG.md) for "Planned Features" section. Highlights:
- SurrealDB integration for graph queries
- Web dashboard for server management
- Real-time scanning with file system watchers
- Advanced policy management UI
- Cloud storage connectors
- Enhanced visualizations and reporting

### Can I request a feature?

Yes! Open a GitHub Issue with:
- Clear use case description
- Why existing features don't solve it
- Proposed implementation (if you have ideas)
- Whether you're willing to contribute code

## Time-Travel Analytics (Phase 3.5)

### What is Time-Travel Analytics?

Time-Travel Analytics is Spectra's time-series intelligence feature that tracks filesystem changes over time. Instead of just "what exists now?", you can ask:
- "How fast is data growing?"
- "What caused the spike last Tuesday?"
- "Which file types are accumulating fastest?"

### Does this store my actual files?

**No.** Only metadata is stored:
- Total size (bytes)
- File count
- Top extension statistics (extension, size, count)
- Timestamps

Your file content is never transmitted or stored. Privacy is preserved.

### How much storage does the history database use?

Each snapshot is approximately **2KB** of metadata. Examples:
- **Hourly snapshots for 1 year**: ~17.5MB per agent
- **Daily snapshots for 5 years**: ~3.6MB per agent
- **100 agents, daily snapshots, 5 years**: ~365MB total

Storage requirements are minimal.

### Can I see which specific files were deleted?

Not in the current version (v0.5.0). Time-Travel Analytics tracks **aggregate changes** (e.g., "MP4 files decreased by 2GB") but not individual file additions/deletions.

File-level diffing is planned for Phase 5 but requires significantly more storage and bandwidth.

### Why SurrealDB instead of PostgreSQL/MySQL?

SurrealDB was chosen for:
- **Time-series optimization**: Native support for temporal queries
- **Embedded deployment**: Runs in-process (single binary) or distributed
- **Schema flexibility**: Handles evolving metadata without migrations
- **Rust-native**: Zero-cost FFI, no serialization overhead
- **Graph capabilities**: Future roadmap includes relationship queries

### How do I test Time-Travel Analytics?

Run the simulation script to generate test data:

**Windows PowerShell:**
```powershell
.\test-time-travel.ps1
```

**Linux/macOS:**
```bash
chmod +x test-time-travel.sh
./test-time-travel.sh
```

This creates 5 snapshots spanning 24 hours with realistic growth patterns.

### What happens if the server is down during a scan?

The CLI agent will:
1. Log the connection failure
2. Continue scanning locally
3. Display results as normal

**Future enhancement**: Local buffering and retry logic for automatic upload when the server comes back online.

### Can I export velocity reports?

Not yet. Currently, velocity reports are only available via:
- REST API (`GET /api/v1/velocity/:agent_id`)
- GUI visualization (Time-Travel Analytics tab)

CSV/PDF export is planned for Phase 4 enhancements.

### How accurate is the velocity calculation?

Velocity is calculated using **exact snapshot data**:
- Growth bytes = `Snapshot_End.total_size - Snapshot_Start.total_size`
- Velocity = `Growth_bytes / Duration_seconds`

Accuracy depends on:
- **Snapshot frequency**: More snapshots = better trend resolution
- **Timing**: Snapshots should be taken at consistent intervals
- **Agent reliability**: Ensure agents complete scans successfully

### Can I compare velocity across multiple machines?

Yes! Each agent has a unique `agent_id`. The GUI allows you to:
1. Switch between different agent IDs
2. View velocity for each agent independently

**Planned feature**: Multi-agent comparison dashboard showing side-by-side velocity metrics.

### Does Time-Travel Analytics work offline?

Partially:
- **Agent scanning**: Works completely offline
- **Snapshot storage**: Requires server connection for persistence
- **Velocity calculation**: Requires server connection (server-side computation)

**Future enhancement**: Local SQLite storage for offline velocity queries.

## Comparison with Other Tools

### Spectra vs WizTree

| Feature | Spectra | WizTree |
|---------|---------|---------|
| Scan Speed | Fast (parallel) | Very Fast (MFT) |
| Semantic Analysis | ✅ | ❌ |
| Risk Detection | ✅ | ❌ |
| Time-Travel Analytics | ✅ | ❌ |
| Federation | ✅ | ❌ |
| Governance | ✅ | ❌ |
| Cross-platform | ✅ | Windows only |
| GUI | ✅ (Tauri) | ✅ (Native) |

**Choose WizTree if**: You only need size visualization on Windows
**Choose Spectra if**: You need governance, risk analysis, time-series analytics, or cross-platform support

---

## Still Have Questions?

- **Documentation**: Check [README.md](../README.md), [ARCHITECTURE.md](ARCHITECTURE.md)
- **Discussions**: GitHub Discussions for general questions
- **Issues**: GitHub Issues for bugs and feature requests
- **Community**: Join our Discord (link coming soon)

*This FAQ is updated regularly. Last updated: 2025-12-28*
