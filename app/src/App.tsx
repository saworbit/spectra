import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { ScanStats } from "./types";

// Helper to format bytes into human-readable strings
const formatBytes = (bytes: number, decimals = 2) => {
  if (!+bytes) return "0 Bytes";
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
};

function App() {
  const [path, setPath] = useState("");
  const [stats, setStats] = useState<ScanStats | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function scan() {
    if (!path) return;
    setLoading(true);
    setError(null);
    setStats(null);

    try {
      // Invoke the Rust command 'scan_directory'
      // limit: number of top files to return
      const result = await invoke<ScanStats>("scan_directory", {
        path,
        limit: 10
      });
      setStats(result);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className="container">
      <h1>Spectra Dashboard</h1>
      <p className="subtitle">Enterprise Content Topology</p>

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

      {error && <div className="error-banner">❌ Error: {error}</div>}

      {stats && (
        <div className="dashboard-grid">
          {/* Summary Card */}
          <div className="card summary">
            <h2>Overview</h2>
            <div className="stat-row">
              <span>📂 Location:</span> <strong>{stats.root_path}</strong>
            </div>
            <div className="stat-row">
              <span>⏱️ Duration:</span> <strong>{(stats.scan_duration_ms / 1000).toFixed(2)}s</strong>
            </div>
            <div className="stat-row">
              <span>📄 Total Files:</span> <strong>{stats.total_files.toLocaleString()}</strong>
            </div>
            <div className="stat-row">
              <span>💾 Total Size:</span> <strong>{formatBytes(stats.total_size_bytes)}</strong>
            </div>
          </div>

          {/* Top Extensions */}
          <div className="card extensions">
            <h2>Top Extensions</h2>
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
          </div>

          {/* Heavy Hitters */}
          <div className="card files full-width">
            <h2>🐳 Heavy Hitters (Top Files)</h2>
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
    </main>
  );
}

export default App;
