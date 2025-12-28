# Time-Travel Analytics

**Version:** 0.5.0
**Status:** Phase 3.5 Implementation
**Last Updated:** 2025-12-28

---

## Overview

Spectra's Time-Travel Analytics enables **temporal intelligence** over filesystem metadata. Instead of asking "what exists now?", you can ask:

- 📈 **"How fast is the data growing?"**
- 🔍 **"Who caused the spike last Tuesday?"**
- ⚡ **"What's the current velocity of log file accumulation?"**

This feature transforms Spectra from a snapshot tool into a **time-series analytics engine** capable of detecting trends, anomalies, and growth patterns.

---

## Architecture

### Data Flow

```
┌─────────────┐
│ Spectra CLI │  Scans filesystem
│   (Agent)   │  Generates snapshot with timestamp
└──────┬──────┘
       │ POST /api/v1/ingest
       │ (JSON payload: AgentSnapshot)
       ▼
┌─────────────────┐
│ Spectra Server  │  Stores in SurrealDB time-series table
│   (The Brain)   │  Indexed by agent_id + timestamp
└──────┬──────────┘
       │
       ├─ GET /api/v1/history/:agent_id
       │  Returns: [timestamp1, timestamp2, ...]
       │
       └─ GET /api/v1/velocity/:agent_id?start=T0&end=T1
          Returns: VelocityReport (Δ bytes, Δ files, bytes/sec)

┌─────────────────┐
│ Spectra Vision  │  Time Slider + Velocity Card
│   (The Lens)    │  Interactive timeline visualization
└─────────────────┘
```

### Key Components

1. **Backend (Rust)**
   - SurrealDB for persistent time-series storage
   - Axum HTTP endpoints for ingestion and querying
   - Velocity calculation engine with extension-level deltas

2. **Frontend (React + TypeScript)**
   - TimeSlider component for selecting time ranges
   - VelocityCard component for displaying growth metrics
   - Real-time API client for fetching analytics

3. **Agent Integration**
   - CLI automatically uploads snapshots when `--server` flag is used
   - Timestamps are Unix epoch (seconds) for universal compatibility

---

## API Specification

### 1. Ingest Snapshot

**Endpoint:** `POST /api/v1/ingest`

**Purpose:** Store a filesystem snapshot for historical analysis

**Request Body:**
```json
{
  "agent_id": "agent_laptop_001",
  "timestamp": 1735401600,
  "hostname": "dev-machine",
  "total_size_bytes": 1500000000,
  "file_count": 5500,
  "top_extensions": [
    ["log", 500000000, 250],
    ["jpg", 400000000, 800],
    ["mp4", 300000000, 15]
  ]
}
```

**Response:**
```json
"Snapshot stored"
```

---

### 2. Get Agent History

**Endpoint:** `GET /api/v1/history/:agent_id`

**Purpose:** Retrieve all available snapshot timestamps for an agent

**Example:**
```bash
curl http://localhost:3000/api/v1/history/agent_laptop_001
```

**Response:**
```json
[1735401600, 1735315200, 1735228800, 1735142400]
```

---

### 3. Calculate Velocity

**Endpoint:** `GET /api/v1/velocity/:agent_id?start=<timestamp>&end=<timestamp>`

**Purpose:** Compute data growth rate between two points in time

**Example:**
```bash
curl "http://localhost:3000/api/v1/velocity/agent_laptop_001?start=1735142400&end=1735401600"
```

**Response:**
```json
{
  "agent_id": "agent_laptop_001",
  "t_start": 1735142400,
  "t_end": 1735401600,
  "duration_seconds": 259200,
  "growth_bytes": 500000000,
  "growth_files": 500,
  "bytes_per_second": 1929.01,
  "extension_deltas": [
    {
      "extension": "log",
      "size_delta": 300000000,
      "count_delta": 150
    },
    {
      "extension": "mp4",
      "size_delta": 200000000,
      "count_delta": 5
    }
  ]
}
```

**Interpretation:**
- Over 3 days, data grew by **500MB** (+500 files)
- Average velocity: **1.93 KB/s**
- Top contributor: `.log` files (+300MB)
- Spike detected: `.mp4` files (+200MB)

---

## Usage Guide

### Quick Start (Local Testing)

1. **Start the Spectra Server**
   ```bash
   cd server
   cargo run
   ```

2. **Run the Simulation Script**

   **Linux/macOS:**
   ```bash
   chmod +x test-time-travel.sh
   ./test-time-travel.sh
   ```

   **Windows PowerShell:**
   ```powershell
   .\test-time-travel.ps1
   ```

   This will inject 5 simulated snapshots spanning 24 hours with realistic growth patterns.

3. **Launch the GUI**
   ```bash
   cd app
   npm install
   npm run dev
   ```

4. **Explore the Timeline**
   - Navigate to the **⏳ Time-Travel Analytics** tab
   - Enter Agent ID: `agent_sim_01`
   - Use the time sliders to select a range
   - View velocity metrics and extension deltas

---

### Production Deployment

#### Enable Persistent Storage

By default, the server uses **in-memory** SurrealDB. For production, switch to **RocksDB**:

**Edit `server/src/main.rs`:**
```rust
// Change from:
let db = Surreal::new::<Mem>(()).await?;

// To:
use surrealdb::engine::local::RocksDb;
let db = Surreal::new::<RocksDb>("data/spectra.db").await?;
```

**Update `server/Cargo.toml`:**
```toml
surrealdb = { version = "1.0", features = ["kv-rocksdb"] }
```

#### Configure Agents for Continuous Telemetry

**Option 1: Cron Job (Linux/macOS)**
```bash
# Run scan every hour and upload to server
0 * * * * /usr/local/bin/spectra-cli --path /data --server http://spectra-hub:3000
```

**Option 2: Windows Task Scheduler**
```powershell
$action = New-ScheduledTaskAction -Execute "spectra-cli.exe" -Argument "--path C:\Data --server http://spectra-hub:3000"
$trigger = New-ScheduledTaskTrigger -Once -At (Get-Date) -RepetitionInterval (New-TimeSpan -Hours 1)
Register-ScheduledTask -TaskName "SpectraTelemetry" -Action $action -Trigger $trigger
```

---

## GUI Features

### Time Slider Component

The time slider provides interactive timeline navigation:

- **Dual Range Sliders:** Select start and end timestamps independently
- **Auto-Selection:** Automatically selects first and last snapshots by default
- **Timestamp Display:** Shows human-readable dates (e.g., "2025-12-28 14:30:00")
- **Snapshot Counter:** Displays total available snapshots and selected range
- **Smart Boundaries:** Prevents invalid ranges (start must be before end)

### Velocity Card Component

Displays comprehensive growth analytics:

- **Growth Metrics:**
  - Total data change (bytes and file count)
  - Direction indicator (📈 growth or 📉 shrinkage)
  - Color-coded positive/negative changes

- **Velocity Rate:**
  - Bytes per second with adaptive units (B/s, KB/s, MB/s, GB/s)
  - Duration breakdown (days, hours, minutes)

- **Extension Breakdown:**
  - Sorted by absolute impact (largest changes first)
  - Color-coded deltas (green for additions, red for deletions)
  - Per-extension file count changes

---

## Database Schema

### Snapshots Table

SurrealDB stores snapshots with the following structure:

```surrealql
-- Table: snapshots
-- Fields:
--   id: auto-generated
--   agent_id: string (indexed)
--   timestamp: i64 (indexed)
--   hostname: string
--   total_size_bytes: u64
--   file_count: u64
--   top_extensions: array<[string, u64, u64]>

-- Query example:
SELECT * FROM snapshots
WHERE agent_id = "agent_01"
ORDER BY timestamp DESC
LIMIT 10;
```

### Indexing Strategy

For optimal performance with large datasets:

```surrealql
DEFINE INDEX idx_agent_timestamp ON TABLE snapshots COLUMNS agent_id, timestamp;
```

---

## Performance Considerations

### Storage Requirements

- **Per Snapshot:** ~2KB (metadata only, no file content)
- **Hourly Snapshots for 1 Year:** ~17.5MB per agent
- **100 Agents, Daily Snapshots, 5 Years:** ~365MB total

### Query Performance

- **History Lookup:** O(log n) with indexed `agent_id`
- **Velocity Calculation:** O(1) for two-point comparison
- **Extension Deltas:** O(n) where n = number of extensions (~10-50 typically)

### Scaling Recommendations

- **< 1000 agents:** In-memory SurrealDB is sufficient
- **1000-10,000 agents:** Use RocksDB with SSD storage
- **> 10,000 agents:** Consider distributed SurrealDB or time-series partitioning

---

## Troubleshooting

### No Snapshots Available

**Symptom:** GUI shows "No historical data available"

**Solutions:**
1. Verify server is running: `curl http://localhost:3000/api/v1/policies`
2. Check agent is uploading: Look for "📡 Ingested Snapshot" in server logs
3. Ensure `--server` flag is used: `spectra-cli --path /data --server http://localhost:3000`
4. Run the test script to inject sample data

### Velocity Calculation Returns Zero

**Symptom:** All deltas show 0 bytes/sec

**Possible Causes:**
1. **Same Snapshot Selected:** Ensure T_start ≠ T_end
2. **Insufficient Time Separation:** Try selecting snapshots hours apart, not seconds
3. **Database Issue:** Check server logs for SurrealDB errors

### Time Slider Shows Only One Snapshot

**Symptom:** Cannot select a range

**Solution:**
- Run the agent multiple times with different data states
- Use `test-time-travel.sh` to generate synthetic history
- Wait for scheduled scans to accumulate (if using cron/scheduler)

---

## Roadmap

### Phase 4 Enhancements (Planned)

- [ ] **Anomaly Detection:** Auto-flag unusual growth spikes
- [ ] **Predictive Analytics:** Forecast storage needs based on velocity trends
- [ ] **Alerts:** Webhook notifications when velocity exceeds threshold
- [ ] **File-Level Diffing:** Track specific file additions/deletions (requires agent enhancement)
- [ ] **Multi-Agent Comparison:** Compare velocity across different machines
- [ ] **Export:** Download velocity reports as CSV/PDF

### Phase 5 Vision

- [ ] **ML-Powered Insights:** "This .log growth pattern is abnormal for Tuesdays"
- [ ] **Retention Policies:** Auto-archive snapshots older than N days
- [ ] **Geographic Distribution:** Track data movement across regions
- [ ] **Cost Attribution:** Map velocity to cloud storage costs

---

## FAQ

**Q: Does this store my actual files?**
A: No. Only metadata (total size, counts, top extensions) is stored. Your content privacy is preserved.

**Q: How much bandwidth does telemetry use?**
A: ~2KB per snapshot. Even hourly uploads consume <2MB/month per agent.

**Q: Can I see which specific files were deleted?**
A: Not in this version. File-level tracking requires Phase 5 enhancements.

**Q: Why SurrealDB instead of PostgreSQL?**
A: SurrealDB excels at time-series data, runs embedded (single binary), and handles schema-less JSON natively.

**Q: What happens if the server is down during a scan?**
A: The CLI will log the failure but continue. Future versions will support local buffering and retry logic.

---

## Contributing

Found a bug or have a feature request? Open an issue at:
**[github.com/saworbit/spectra/issues](https://github.com/saworbit/spectra/issues)**

---

## License

Dual-licensed under **MIT** or **Apache 2.0**.
See [LICENSE-MIT](../LICENSE-MIT) and [LICENSE-APACHE](../LICENSE-APACHE) for details.

---

**"We have given the machine memory. Now it can learn from its past."**
