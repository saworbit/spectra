# SPECTRA CLI Usage Guide

## Quick Reference

```bash
# Basic scan (Phase 1 - topology only)
spectra-cli --path /path/to/scan

# Semantic analysis (Phase 2 - entropy + risk)
spectra-cli --path /path/to/scan --analyze

# Full AI classification (requires 'semantic' feature)
spectra-cli --path /path/to/scan --semantic

# JSON output
spectra-cli --path /path/to/scan --analyze --json

# Limit results
spectra-cli --path /path/to/scan --analyze --limit 20
```

## Installation

### Standard Build (Fast, ~2-5MB binary)
```bash
cargo build --release -p spectra-cli
./target/release/spectra-cli --help
```

### With Semantic Features (~500MB with LibTorch)
```bash
# May require LIBTORCH environment variables set
cargo build --release -p spectra-cli --features semantic
./target/release/spectra-cli --help
```

## Command-Line Options

### Required Arguments
- `--path <PATH>` or `-p <PATH>`: Directory to scan (default: current directory `.`)

### Optional Flags
- `--json` or `-j`: Output results as JSON instead of human-readable format
- `--limit <N>` or `-l <N>`: Number of top files to track (default: 10)
- `--analyze`: Enable Phase 2 semantic analysis (entropy + risk scoring)
- `--semantic`: Enable AI-based content classification (requires `semantic` feature)

## Usage Examples

### Example 1: Quick Disk Usage Check
```bash
# Scan current directory, show top 10 files
spectra-cli

# Scan specific directory
spectra-cli --path /var/log
```

**Output:**
```
🚀 SPECTRA: Profiling topology of '/var/log'...
------------------------------------------------
✅ Scan Complete in 0.12s
------------------------------------------------
📂 Location : /var/log
📄 Files    : 1,234
💾 Total Size: 2.4 GB
------------------------------------------------
📊 Top Extensions by Volume:
   .log   :    1.2 GB (456)
   .gz    :  800 MB (234)
   .txt   :  400 MB (544)
...
```

### Example 2: Security Audit (Find Sensitive Files)
```bash
# Scan with risk analysis
spectra-cli --path ~/Documents --analyze --limit 50
```

**Output:**
```
🐳 Top Largest Files:
   10 MB | Entropy:2.6 | 🔴 Critical    ~/Documents/passwords.txt
   5 MB | Entropy:7.8 | 🟠 High         ~/Documents/.env
   2 MB | Entropy:3.2 | 🟡 Medium       ~/Documents/backup.zip
   1 MB | Entropy:4.1                   ~/Documents/report.pdf
```

### Example 3: Find Encrypted/Compressed Files
```bash
# High entropy files are likely encrypted or compressed
spectra-cli --path . --analyze --json | jq '.top_files[] | select(.entropy > 7.0)'
```

**Output:**
```json
{
  "path": "./data/encrypted.db",
  "size_bytes": 104857600,
  "entropy": 7.94,
  "risk_level": null
}
```

### Example 4: Content Classification (AI-powered)
```bash
# Requires semantic feature
cargo build --release -p spectra-cli --features semantic
./target/release/spectra-cli --path ~/Documents --semantic
```

**Output:**
```
🐳 Top Largest Files:
   5 MB | Entropy:4.2 | [legal contract]      contract.pdf
   3 MB | Entropy:3.8 | [source code]          main.rs
   2 MB | Entropy:5.1 | [financial invoice]    invoice.pdf
```

### Example 5: JSON Export for Analysis
```bash
# Export full analysis to JSON
spectra-cli --path . --analyze --json > scan_results.json

# Query with jq
cat scan_results.json | jq '.top_files[] | select(.risk_level == "Critical")'
```

**Output:**
```json
{
  "path": "./.ssh/id_rsa",
  "size_bytes": 3247,
  "entropy": 6.2,
  "risk_level": "Critical"
}
```

## Understanding Analysis Results

### Entropy Values
- **0.0 - 2.0**: Very low entropy - Highly repetitive data (zeros, simple patterns)
- **2.0 - 5.0**: Low to medium entropy - Text files, source code, XML/JSON
- **5.0 - 7.0**: Medium to high entropy - Compressed files, efficient binary formats
- **7.0 - 8.0**: High entropy - Encrypted files, random data, strong compression

### Risk Levels
- **🔴 Critical**: Private keys (.pem, .p12), passwords, secrets, SSH keys, wallets
- **🟠 High**: Credentials, tokens, KeePass databases (.kdbx), .env files
- **🟡 Medium**: Backup files, database dumps, configuration files with potential secrets
- **🟢 Low**: Other sensitive patterns detected
- **None**: No sensitive patterns detected

### Semantic Categories (with --semantic)
- **legal contract**: Legal documents, agreements, contracts
- **source code**: Programming language source files
- **financial invoice**: Invoices, receipts, financial documents
- **personal letter**: Personal correspondence, emails
- **log file**: Application or system logs
- **configuration file**: Config files, settings
- **documentation**: Readme files, documentation

## Performance Tips

1. **Use --limit wisely**: Analysis only runs on top N files
   ```bash
   spectra-cli --analyze --limit 100  # Analyze top 100 files
   ```

2. **Skip AI for speed**: Use --analyze without --semantic for fast heuristic analysis
   ```bash
   spectra-cli --analyze  # Fast (microseconds per file)
   # vs
   spectra-cli --semantic  # Slower (milliseconds per file)
   ```

3. **Pipe to jq for complex queries**:
   ```bash
   spectra-cli --analyze --json | jq '.top_files[] | select(.entropy > 7.5 and .size_bytes > 1000000)'
   ```

## Convenience Scripts

Located in `cli/` directory:

### Windows
```batch
REM Basic scan
test-basic.bat

REM Scan with analysis
test-analyze.bat C:\Users\YourName\Documents
```

### Unix/Linux/macOS
```bash
# Basic scan
./test-basic.sh

# Scan with analysis
./test-analyze.sh ~/Documents
```

## Integration Examples

### Find All Critical Risk Files
```bash
spectra-cli --path / --analyze --limit 1000 --json | \
  jq -r '.top_files[] | select(.risk_level == "Critical") | .path'
```

### Generate Security Report
```bash
echo "Security Scan Report - $(date)" > report.txt
spectra-cli --path /home --analyze --limit 500 >> report.txt
```

### Monitor for High-Entropy Files (Ransomware Detection)
```bash
# High entropy in typically low-entropy directories may indicate encryption
spectra-cli --path ~/Documents --analyze --json | \
  jq '.top_files[] | select(.entropy > 7.5) | {path, entropy, size_bytes}'
```

## Troubleshooting

### LibTorch Not Found (for --semantic)
```bash
# Set LIBTORCH environment variable
export LIBTORCH=/path/to/libtorch
export LD_LIBRARY_PATH=$LIBTORCH/lib:$LD_LIBRARY_PATH
```

### Permission Denied Errors
```bash
# Run with appropriate permissions or skip protected directories
sudo spectra-cli --path /root --analyze
```

### Large Directories Are Slow
```bash
# Reduce --limit or skip --semantic flag
spectra-cli --path /massive/dir --analyze --limit 50  # Fast heuristics only
```

## Technical Details

### What Files Are Read?
- **Phase 1 (basic scan)**: Only filesystem metadata (no file content read)
- **Phase 2 with --analyze**: First 8KB of top N files for entropy calculation
- **Phase 2 with --semantic**: First 2KB of low-entropy files for AI classification

### Privacy & Security
- All analysis is performed **locally**
- No data is transmitted over the network
- No telemetry or analytics
- Read-only operations
- Safe to run on sensitive data

### Memory Usage
- Base scan: ~20MB RAM for 1M files
- With analysis: +1-2MB per 100 files analyzed
- With semantic: +500MB model loading overhead

## See Also

- [ARCHITECTURE.md](../ARCHITECTURE.md) - Technical architecture details
- [CHANGELOG.md](../CHANGELOG.md) - Version history and changes
- [README.md](../README.md) - Project overview
