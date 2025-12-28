# What's New in Spectra v0.5.0 - Time-Travel Analytics

**Release Date:** 2025-12-28
**Code Name:** "The Temporal Lens"

---

## 🎉 Major Feature: Time-Travel Analytics

Spectra can now **remember the past** and answer questions like:
- "How fast is my data growing?"
- "What caused the spike last Tuesday?"
- "Which file types are accumulating fastest?"

### Why This Matters

Before v0.5.0, Spectra could tell you **what exists now**. With Time-Travel Analytics, you can:
- **Track trends** over days, weeks, or months
- **Detect anomalies** (unexpected spikes or drops)
- **Attribute changes** (which extensions are growing fastest)
- **Forecast needs** based on historical velocity

### Quick Demo (5 Minutes)

```bash
# 1. Start the server
cd server
cargo run

# 2. Generate test data (simulates 24 hours of growth)
.\test-time-travel.ps1   # Windows
./test-time-travel.sh    # Linux/macOS

# 3. Open the GUI
cd app
npm run dev

# 4. Navigate to "⏳ Time-Travel Analytics" tab
# Agent ID: agent_sim_01
# Use the sliders to explore!
```

---

## ✨ New Features

### Backend (Spectra Server)

**SurrealDB Integration:**
- ✅ Persistent time-series storage
- ✅ In-memory mode for instant development
- ✅ RocksDB backend for production persistence
- ✅ O(log n) indexed queries

**New API Endpoints:**
- `GET /api/v1/history/:agent_id` - Get available snapshots
- `GET /api/v1/velocity/:agent_id?start=<ts>&end=<ts>` - Calculate growth velocity

**Enhanced Existing Endpoints:**
- `POST /api/v1/ingest` - Now stores snapshots in SurrealDB

### Frontend (Spectra Vision)

**TimeSlider Component:**
- Interactive dual-range timeline
- Human-readable timestamps
- Auto-selection of first/last snapshots
- Real-time validation

**VelocityCard Component:**
- Growth/shrinkage metrics (📈 or 📉)
- Velocity rate (bytes/second)
- Top 10 contributing extensions
- Color-coded deltas (green = growth, red = shrinkage)

**Tab Navigation:**
- "📂 Local Scan" - Existing directory analysis
- "⏳ Time-Travel Analytics" - NEW: Historical trends

### Testing Tools

**Simulation Scripts:**
- `test-time-travel.ps1` (Windows PowerShell)
- `test-time-travel.sh` (Linux/macOS Bash)

Both scripts:
- Inject 5 snapshots over 24 hours
- Simulate realistic growth patterns
- Verify all endpoints automatically
- Provide detailed metrics output

---

## 📊 Example Use Cases

### Use Case 1: Log File Accumulation

**Before v0.5.0:**
- "We have 500GB of log files."
- Manual cleanup every few weeks.

**With Time-Travel Analytics:**
- "Log files grew by 2GB/day last week (velocity: 23 KB/s)."
- "Spike detected: 10GB increase on Tuesday (deploy event?)."
- "Action: Set up automated archival for logs >30 days old."

### Use Case 2: Cleanup Verification

**Before v0.5.0:**
- "I deleted some files. Hope it helped."

**With Time-Travel Analytics:**
- "Before cleanup: 1.5TB total."
- "After cleanup: 1.2TB total."
- "Net reduction: -300GB in .mp4 files."
- "Verification: Cleanup was successful!"

### Use Case 3: Capacity Planning

**Before v0.5.0:**
- "We're running out of disk space."
- Reactive purchasing decisions.

**With Time-Travel Analytics:**
- "Current growth rate: 50GB/week."
- "At this velocity, disk will be full in 10 weeks."
- "Proactive: Order new storage now, not when it's critical."

---

## 🔧 Technical Highlights

### Storage Efficiency
- **~2KB per snapshot** (metadata only)
- Hourly snapshots for 1 year: 17.5MB per agent
- 100 agents, daily snapshots, 5 years: 365MB total

### Query Performance
- History lookup: O(log n)
- Velocity calculation: O(1) for two-point comparison
- Extension deltas: O(n) where n = number of extensions (~10-50)

### Privacy Preserving
- **Zero file content storage**
- Only aggregated statistics
- No PII collected
- Localhost-first design

---

## 📚 Documentation Updates

**New Documents:**
- [docs/TIME_TRAVEL_ANALYTICS.md](TIME_TRAVEL_ANALYTICS.md) - Comprehensive guide
  - Architecture
  - API specification
  - Usage examples
  - Troubleshooting
  - FAQ

**Updated Documents:**
- [README.md](../README.md) - Time-Travel Quick Start
- [CHANGELOG.md](../CHANGELOG.md) - Full v0.5.0 changes
- [FAQ.md](FAQ.md) - 10 new Q&A about Time-Travel
- [ARCHITECTURE.md](ARCHITECTURE.md) - Time-series flow diagrams

---

## 🚀 Getting Started

### For First-Time Users

If you're new to Spectra, start with the [main README](../README.md) and follow:
1. Installation
2. Local Scan demo
3. Time-Travel Analytics demo

### For Existing Users

If you've used Spectra before:
1. **Pull the latest code**: `git pull origin main`
2. **Rebuild dependencies**: `cd server && cargo build`
3. **Run the simulation**: `.\test-time-travel.ps1`
4. **Launch the GUI**: `cd app && npm run dev`
5. **Explore the new tab**: Click "⏳ Time-Travel Analytics"

---

## 🔮 What's Next?

Phase 4 Enhancements (Planned):
- [ ] Anomaly detection (auto-flag unusual spikes)
- [ ] Predictive analytics (forecast storage needs)
- [ ] Alert webhooks (notifications on thresholds)
- [ ] CSV/PDF export (velocity reports)
- [ ] Multi-agent comparison (side-by-side metrics)

Phase 5 Vision:
- [ ] ML-powered insights ("This pattern is abnormal")
- [ ] File-level diffing (track specific additions/deletions)
- [ ] Cost attribution (map velocity to cloud costs)
- [ ] Geographic distribution (data movement tracking)

---

## 🙏 Feedback Welcome

This is a **pre-alpha** feature. We'd love to hear:
- What works well?
- What's confusing?
- What features would you like next?

**Report issues:** [GitHub Issues](https://github.com/saworbit/spectra/issues)

---

## 🎓 Learn More

- [Complete Time-Travel Guide](TIME_TRAVEL_ANALYTICS.md)
- [API Documentation](TIME_TRAVEL_ANALYTICS.md#api-specification)
- [FAQ - Time-Travel Section](FAQ.md#time-travel-analytics-phase-35)
- [Main README](../README.md)

---

**"We have given the machine memory. Now it can learn from its past."**
