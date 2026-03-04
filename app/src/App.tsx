import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";
import { ScanStats, ScanProgress, VelocityReport } from "./types";
import { TimeSlider } from "./TimeSlider";
import { VelocityCard } from "./VelocityCard";
import { SunburstChart } from "./components/SunburstChart";
import { fetchVelocity } from "./api";

// Helper to format bytes into human-readable strings
const formatBytes = (bytes: number, decimals = 2) => {
  if (!+bytes) return "0 Bytes";
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
};

type AppMode = "scan" | "timetravel";
type VizMode = "table" | "sunburst";

function App() {
  // Read URL parameters
  const urlParams = new URLSearchParams(window.location.search);
  const urlAgentId = urlParams.get('agentId');

  // Tab Management
  const [mode, setMode] = useState<AppMode>(urlAgentId ? "timetravel" : "scan");

  // Local Scan State
  const [path, setPath] = useState("");
  const [stats, setStats] = useState<ScanStats | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Progressive scan progress (#1)
  const [scanProgress, setScanProgress] = useState<ScanProgress | null>(null);

  // Visualization mode (#3 - Sunburst)
  const [vizMode, setVizMode] = useState<VizMode>("table");

  // Session "space freed" counter (#7)
  const [spaceFreed, setSpaceFreed] = useState(0);
  const [filesFreed, setFilesFreed] = useState(0);

  // Time-Travel State
  const [agentId, setAgentId] = useState(urlAgentId || "agent_01");
  const [velocityReport, setVelocityReport] = useState<VelocityReport | null>(null);
  const [velocityLoading, setVelocityLoading] = useState(false);

  // Listen for progressive scan events (#1)
  useEffect(() => {
    let cleanup: (() => void) | null = null;

    listen<ScanProgress>('scan-progress', (event) => {
      setScanProgress(event.payload);
    }).then((unlisten) => {
      cleanup = unlisten;
    }).catch(() => {
      // Not in Tauri environment — ignore
    });

    return () => {
      if (cleanup) cleanup();
    };
  }, []);

  async function scan() {
    if (!path) return;
    setLoading(true);
    setError(null);
    setStats(null);
    setScanProgress(null);

    try {
      if (typeof invoke === 'undefined') {
        throw new Error('Tauri runtime not available. Please run the app using: npm run tauri dev');
      }

      const result = await invoke<ScanStats>("scan_directory", {
        path,
        limit: 10
      });
      setStats(result);
      setScanProgress(null);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  const handleRangeSelect = useCallback(async (startTime: number, endTime: number) => {
    setVelocityLoading(true);
    setVelocityReport(null);

    try {
      const report = await fetchVelocity(agentId, startTime, endTime);
      setVelocityReport(report);
    } catch (e) {
      console.error("Failed to fetch velocity:", e);
    } finally {
      setVelocityLoading(false);
    }
  }, [agentId]);

  // Track space freed from governance actions (#7)
  // Exposed on window for Tauri commands to call when files are deleted
  useEffect(() => {
    (window as unknown as Record<string, unknown>).__spectraTrackDeletion = (sizeBytes: number) => {
      setSpaceFreed(prev => prev + sizeBytes);
      setFilesFreed(prev => prev + 1);
    };
    return () => {
      delete (window as unknown as Record<string, unknown>).__spectraTrackDeletion;
    };
  }, []);

  return (
    <main className="container">
      <h1>Spectra Dashboard</h1>
      <p className="subtitle">Enterprise Content Topology & Time-Travel Analytics</p>

      {/* Session Space Freed Banner (#7) */}
      {spaceFreed > 0 && (
        <div className="space-freed-banner">
          Freed {formatBytes(spaceFreed)} across {filesFreed} file{filesFreed !== 1 ? 's' : ''} this session
        </div>
      )}

      {/* Tab Navigation */}
      <div className="tab-nav">
        <button
          className={`tab-button ${mode === "scan" ? "active" : ""}`}
          onClick={() => setMode("scan")}
        >
          Local Scan
        </button>
        <button
          className={`tab-button ${mode === "timetravel" ? "active" : ""}`}
          onClick={() => setMode("timetravel")}
        >
          Time-Travel Analytics
        </button>
      </div>

      {/* Local Scan Mode */}
      {mode === "scan" && (
        <>
          <form
            className="row"
            onSubmit={(e) => {
              e.preventDefault();
              scan();
            }}
          >
            <input
              id="path-input"
              onChange={(e) => setPath(e.currentTarget.value)}
              placeholder="/path/to/scan"
              value={path}
              disabled={loading}
            />
            <button type="submit" disabled={loading || !path}>
              {loading ? "Scanning..." : "Deep Scan"}
            </button>
          </form>

          {/* Progressive scan progress (#1) */}
          {loading && scanProgress && (
            <div className="scan-progress">
              <div className="progress-stats">
                <span>{scanProgress.files_scanned.toLocaleString()} files</span>
                <span>{scanProgress.folders_scanned.toLocaleString()} folders</span>
                <span>{formatBytes(scanProgress.bytes_scanned)}</span>
              </div>
              <div className="progress-bar">
                <div className="progress-bar-fill" />
              </div>
            </div>
          )}

          {error && <div className="error-banner">Error: {error}</div>}

          {stats && (
            <div className="dashboard-grid">
              {/* Summary Card */}
              <div className="card summary">
                <h2>Overview</h2>
                <div className="stat-row">
                  <span>Location:</span> <strong>{stats.root_path}</strong>
                </div>
                <div className="stat-row">
                  <span>Duration:</span> <strong>{(stats.scan_duration_ms / 1000).toFixed(2)}s</strong>
                </div>
                <div className="stat-row">
                  <span>Total Files:</span> <strong>{stats.total_files.toLocaleString()}</strong>
                </div>
                <div className="stat-row">
                  <span>Total Size:</span> <strong>{formatBytes(stats.total_size_bytes)}</strong>
                </div>
                {stats.device_type && (
                  <div className="stat-row">
                    <span>Device:</span>
                    <strong>{stats.device_type} ({stats.threads_used} threads)</strong>
                  </div>
                )}
              </div>

              {/* Extensions with viz toggle (#3) */}
              <div className="card extensions">
                <div className="card-header-row">
                  <h2>Top Extensions</h2>
                  <div className="viz-toggle">
                    <button
                      className={`viz-btn ${vizMode === "table" ? "active" : ""}`}
                      onClick={() => setVizMode("table")}
                      title="Table view"
                    >
                      Table
                    </button>
                    <button
                      className={`viz-btn ${vizMode === "sunburst" ? "active" : ""}`}
                      onClick={() => setVizMode("sunburst")}
                      title="Sunburst chart"
                    >
                      Sunburst
                    </button>
                  </div>
                </div>

                {vizMode === "table" && (
                  <ul>
                    {Object.entries(stats.extensions)
                      .sort(([, a], [, b]) => b.size - a.size)
                      .slice(0, 5)
                      .map(([ext, data]) => (
                        <li key={ext} className="list-item">
                          <span className="badge">.{ext}</span>
                          <span className="spacer"></span>
                          <span>{data.count} files</span>
                          <span className="size">{formatBytes(data.size)}</span>
                        </li>
                      ))}
                  </ul>
                )}

                {vizMode === "sunburst" && (
                  <SunburstChart extensions={stats.extensions} />
                )}
              </div>

              {/* Heavy Hitters */}
              <div className="card files full-width">
                <h2>Heavy Hitters (Top Files)</h2>
                <table>
                  <thead>
                    <tr>
                      <th>Size</th>
                      <th>Path</th>
                    </tr>
                  </thead>
                  <tbody>
                    {stats.top_files.map((file, idx) => (
                      <tr key={idx}>
                        <td className="whitespace-nowrap">{formatBytes(file.size_bytes)}</td>
                        <td className="path-cell" title={file.path}>{file.path}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}
        </>
      )}

      {/* Time-Travel Analytics Mode */}
      {mode === "timetravel" && (
        <>
          <form
            className="row"
            onSubmit={(e) => {
              e.preventDefault();
            }}
            style={{ marginBottom: "2rem" }}
          >
            <input
              id="agent-input"
              onChange={(e) => setAgentId(e.currentTarget.value)}
              placeholder="agent_01"
              value={agentId}
            />
            <button type="button" disabled style={{ opacity: 0.5 }}>
              Agent ID
            </button>
          </form>

          <div className="dashboard-grid">
            {/* Time Slider */}
            <TimeSlider agentId={agentId} onRangeSelect={handleRangeSelect} />

            {/* Velocity Report */}
            {velocityLoading && (
              <div className="card full-width">
                <div className="loading">Calculating velocity...</div>
              </div>
            )}

            {!velocityLoading && velocityReport && (
              <VelocityCard report={velocityReport} />
            )}

            {!velocityLoading && !velocityReport && (
              <div className="card full-width">
                <div className="no-data">
                  Select a time range to view velocity analytics
                  <br />
                  <br />
                  <strong>Quick Start:</strong>
                  <ol style={{ textAlign: "left", maxWidth: "600px", margin: "1rem auto" }}>
                    <li>Start the Spectra Server: <code>cd server && cargo run</code></li>
                    <li>Run the CLI agent with telemetry enabled: <code>spectra-cli --path /your/path --server http://localhost:3000</code></li>
                    <li>Wait a few minutes and run the agent again to generate multiple snapshots</li>
                    <li>Use the time slider above to select a range</li>
                  </ol>
                </div>
              </div>
            )}
          </div>
        </>
      )}
    </main>
  );
}

export default App;
