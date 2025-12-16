# Phase 3: The Enterprise Mesh - Quick Start Guide

## Overview

Phase 3 transforms Spectra from a local-only tool into a federated, enterprise-ready platform with:

- **Central Control Plane**: The Hub server aggregates telemetry from distributed agents
- **Time-Travel Analytics**: Historical snapshots to track data growth over time
- **Active Governance**: Policy-based file management with safety-first design

## Architecture

```
┌─────────────────┐
│  Spectra Server │  ← Central Hub (Port 3000)
│   (The Brain)   │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
┌───▼───┐ ┌──▼────┐
│Agent 1│ │Agent 2│  ← Distributed Spokes
└───────┘ └───────┘
```

## Quick Start

### 1. Start the Server (The Hub)

```bash
# Option 1: Use the convenience script
run-server.bat

# Option 2: Run directly
cd server
cargo run
```

The server will start listening on `http://0.0.0.0:3000`

### 2. Run an Agent (The Spoke)

```bash
# Option 1: Use the convenience script
run-agent.bat

# Option 2: Run directly
cargo run -p spectra-cli -- --path . --server http://localhost:3000
```

### 3. Observe the Federation

**On the Server Console:**
You'll see incoming telemetry:
```
📡 Received Telemetry from Agent: agent_1734394726
   📊 Size: 524288000 bytes
```

**On the Agent Console:**
You'll see policy distribution:
```
🌐 Fetching governance policies from http://localhost:3000...
📋 Loaded 1 policies
⚠️  Running in DRY-RUN mode. Use --enforce to execute actions.
```

## Features

### 1. Time-Travel Analytics

Agents send periodic snapshots to the server. Each snapshot includes:
- Agent ID and hostname
- Timestamp
- Total size and file count
- Top 10 file extensions by volume

**Future capabilities:**
- Compare snapshots over time: "How much did we grow this month?"
- Data velocity tracking: "Which teams are accumulating data fastest?"
- Growth trend predictions

### 2. Active Governance

The server distributes policies to agents. Each policy includes:
- **Rules**: Criteria for matching files (extension, size, age)
- **Actions**: What to do with matches (Report, Delete, Archive)

**Example Policy (from server):**
```json
{
  "name": "Cleanup Old Logs",
  "rules": ["extension == 'log'", "days_since_modified > 90"],
  "action": "DELETE"
}
```

**Safety Features:**
- **Dry-Run by Default**: Policies report matches but don't modify files
- **Explicit Enforcement**: Requires `--enforce` flag for destructive actions
- **Double-Check**: Confirms file metadata before deletion
- **Comprehensive Tests**: Full test coverage for governance engine

### 3. Federation Protocol

**Agent → Server Communication:**
- `POST /api/v1/ingest` - Upload snapshot
- `GET /api/v1/policies` - Download policies

**Data Format:** JSON over HTTP/2 (MessagePack support planned)

## Usage Examples

### Scan and Report Only (Dry-Run)
```bash
cargo run -p spectra-cli -- --path . --server http://localhost:3000
```

### Scan with Active Governance (Execute Policies)
```bash
cargo run -p spectra-cli -- --path . --server http://localhost:3000 --enforce
```

⚠️ **WARNING**: `--enforce` enables destructive actions (file deletion). Use with caution!

### Scan with Analysis and Governance
```bash
cargo run -p spectra-cli -- --path . --server http://localhost:3000 --analyze --enforce
```

### Offline Mode (No Server)
```bash
cargo run -p spectra-cli -- --path . --analyze
```

Agents work perfectly fine without a server connection.

## Testing

### Run Unit Tests
```bash
# Test CLI (includes governance tests)
cargo test -p spectra-cli

# Test server
cargo test -p spectra-server
```

### Test Governance Safely
The governance module includes comprehensive tests:
- Extension matching
- Size threshold evaluation
- Age-based filtering
- Dry-run mode verification

All tests pass and use temporary directories to avoid any real file modifications.

## What's Next

Phase 3 is now **implemented** with the following status:

✅ **Completed:**
- Server scaffolding with Axum
- Agent snapshot ingestion API
- Policy distribution API
- Governance engine with rule evaluation
- Safety mechanisms (dry-run mode)
- CLI integration with `--server` and `--enforce` flags
- Comprehensive test suite

🚧 **Future Enhancements:**
- SurrealDB integration for persistent storage
- Time-travel query interface (UI)
- Agent authentication and authorization
- Policy management UI
- Growth rate analytics and alerting
- Archive functionality implementation
- Custom policy configuration files
- Docker deployment
- Kubernetes manifests

## Security Considerations

1. **No Raw Data Upload**: Only metadata summaries are transmitted
2. **Dry-Run by Default**: Governance defaults to read-only reporting
3. **Local Autonomy**: Agents continue operating if server is unreachable
4. **Audit Trail**: All policy executions are logged (planned)
5. **Authentication**: Agent authentication coming in future releases

## Troubleshooting

**Server won't start:**
- Check if port 3000 is already in use
- Run with different port: Modify `server/src/main.rs` and change the bind address

**Agent can't connect to server:**
- Ensure server is running
- Check firewall settings
- Verify the URL: `http://localhost:3000`

**Policies not executing:**
- Remember to use `--enforce` flag for active governance
- Check policy evaluation logic in console output

## Files Created

```
spectra/
├── server/                     ← New Phase 3 Hub
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── cli/
│   └── src/
│       └── governance/         ← New Phase 3 Module
│           ├── mod.rs
│           ├── engine.rs
│           └── tests.rs
├── run-server.bat             ← Convenience script
├── run-agent.bat              ← Convenience script
└── PHASE3_GUIDE.md            ← This file
```

---

**Phase 3: Complete ✅**

*"We have given Spectra memory and hands. It can now remember the past and act on the present."*
