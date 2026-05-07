# Spectra Roadmap

A living list of planned and in-flight work. Items are loosely ordered by priority within each section, not by dependency. Status legend: ✅ done · 🚧 in progress · 🔜 planned · 💭 idea.

---

## Recently Shipped

- ✅ **CLI progress spinner** (`indicatif`) — live files/folders/bytes feedback during scans, suppressed in `--json` mode. Core emits progress every 1000 items or every 250ms (whichever first), so small scans stay visible too.
- ✅ Device-aware I/O (SSD vs HDD thread tuning).
- ✅ Time-travel snapshot/aggregate endpoints with truncation flag.
- ✅ Real-time `--watch` mode (notify-rs).
- ✅ Entropy + IQR outlier detection.

---

## Onboarding & Distribution

Goal: turn "git clone + cargo build" into "one command."

- 🔜 **Publish `spectra-cli` to crates.io** so users can `cargo install spectra-cli`.
- 🔜 **`cargo-dist` + GitHub Actions releases** — pre-built binaries for Linux/macOS/Windows on every tag, including Tauri GUI installers (`.msi`, `.dmg`, `.AppImage`).
- 🔜 **Unified `spectra` launcher** — single entrypoint that detects CLI vs GUI intent.
- 💭 Homebrew formula and `winget` manifest once binaries are published.

## Build & Repo Hygiene

Goal: kill the `.bat`/`.sh` script sprawl.

- 🔜 **Consolidate scripts into a `justfile`** (or `cargo xtask`) — replace `launch-vision.bat`, `run-server.bat`, `test-time-travel.*`, `validate-refactor.bat`, `build-release.bat` with one cross-platform task runner.
- 🔜 Document `semantic` feature build cost (rust-bert + LibTorch ~500MB) prominently; default to "light" build.
- 🔜 Polish the first-run experience: `cargo run -p spectra-cli -- --help` should feel inviting (clean grouping, examples, colored output).
- 💭 Move release packaging logic into Rust (`xtask`) so it's tested.

## Robustness & Observability

Goal: stop swallowing errors silently.

- 🔜 **Adopt the `tracing` crate** across `spectra-core`, `spectra-cli`, `spectra-server` — replace `flatten()` and ignored `metadata()` errors with structured spans/events.
- 🔜 `--verbose` / `--log-level` flag on CLI; structured JSON logs in server.
- 🔜 Permission-denied, symlink, and UNC/long-path edge cases tested with fixtures.
- 💭 Optional error-summary report at end of scan (`N files skipped due to permissions`).

## CLI Enhancements

- 🔜 **Config file support** — `~/.config/spectra/config.toml` and `.spectra.toml` in scanned dir for default `--threads`, `--limit`, `--server`, etc.
- 🔜 More output formats: CSV, Parquet, Prometheus metrics export.
- 💭 Interactive mode (`--interactive`) for governance confirmations.
- 💭 Better colored output and end-of-scan summary panel.

## Analysis & Intelligence

- 🔜 Stronger filename-risk regex set (more secret patterns, better entropy edge cases).
- 🔜 **Duplicate detection** via content hashing (gated behind a flag — only hash candidates of equal size).
- 💭 Lighter semantic alternative: local ONNX models or optional Ollama/llama.cpp integration so users don't need rust-bert/LibTorch.
- 💭 "Data rot" scoring (age × access pattern × redundancy).

## Time-Travel & Forecasting

The standout feature. Make it predictive.

- 🔜 **Capacity forecasting** — "at current velocity, you'll hit capacity in X days." Simple linear regression on snapshot history is enough to start.
- 🔜 Side-by-side snapshot comparison in the GUI Time-Travel tab.
- 🔜 Export growth reports (CSV/PDF).
- 🔜 Better aggregation queries — uniform-sample bucketing (replace earliest-N truncation), percentile bands, anomaly detection on velocity.
- 💭 Animated timeline playback (play/pause) in the sunburst view.
- 💭 Per-extension and per-folder velocity breakouts.

## GUI (The Lens)

- 🔜 Filters and search in Heavy Hitters / Extensions tables.
- 🔜 Treemap view (alongside the existing sunburst).
- 🔜 Export buttons (PNG/SVG/CSV) on every chart.
- 💭 Responsive layout polish for narrow windows.

## Server & Federation

- 🔜 **Optional local SQLite mode** — agents already work fully offline; lean into that. Server should be optional, not required.
- 🔜 WebSocket support for real-time agent updates (replace polling in the GUI).
- 🔜 Improved auth: JWT or rotatable API keys.
- 💭 Server-side dedup of snapshots (content-addressed storage).

## Testing & Quality

- 🔜 Property-based testing (`proptest`) for the scanner.
- 🔜 Integration tests with real directory fixtures (large + edge cases).
- 🔜 `cargo audit` + dependency updates in CI.
- 🔜 Criterion benchmark suite for scanning performance, tracked over time.
- 🔜 **Cross-compilation in CI** — matrix builds for Linux/macOS/Windows on every PR (catches platform breakage before release tags).
- 💭 Frontend test coverage with vitest + @testing-library/react.

## Security & Governance

- 🔜 Audit the policy engine, especially `Delete`/`Archive` actions.
- 🔜 Audit logging for all enforcement actions (append-only log, optionally signed).
- 🔜 Input sanitization on server endpoints.
- 💭 Confirmation prompts for destructive actions in `--enforce` mode.

## Documentation & Community

- 🔜 "Getting Started in 60 seconds" section in the README.
- 🔜 Troubleshooting guide (Windows long paths, UNC paths, permission errors).
- 🔜 Architecture diagrams (Mermaid) embedded in [ARCHITECTURE.md](ARCHITECTURE.md).
- 🔜 Real-world use-case examples ("find duplicate video libraries", "audit shared drives for stale data").
- 🔜 Polish [CONTRIBUTING.md](CONTRIBUTING.md): "your first PR" walkthrough, local validation steps, code-style expectations, where to ask questions.
- 🔜 Issue and PR templates in `.github/` (bug report, feature request, PR checklist).
- 💭 Contributor onboarding video.

---

## Out of Scope (For Now)

These are deliberately deferred to keep focus:

- Cloud-hosted SaaS offering — Spectra stays local-first.
- Mobile apps.
- Real-time content scanning (Spectra is metadata-only by design).
- Built-in ML training pipelines — we ship pretrained models or integrate with external runtimes.

---

Have an idea or want to pick something up? See [CONTRIBUTING.md](CONTRIBUTING.md).
