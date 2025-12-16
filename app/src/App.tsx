import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import RiskTreemap from './components/RiskTreemap';
import './App.css';

function App() {
  const [treeData, setTreeData] = useState(null);
  const [scanPath, setScanPath] = useState("./"); // Default scan current dir
  const [isScanning, setIsScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function startScan() {
    try {
      setIsScanning(true);
      setError(null);
      console.log("Scanning...", scanPath);
      // Call the Rust command defined in lib.rs
      const result = await invoke("get_scan_tree", { path: scanPath });
      setTreeData(result as any);
    } catch (error) {
      console.error("Scan failed:", error);
      setError(error as string);
    } finally {
      setIsScanning(false);
    }
  }

  return (
    <div className="container">
      <h1>Spectra Vision</h1>

      <div className="controls">
        <input
          type="text"
          value={scanPath}
          onChange={(e) => setScanPath(e.target.value)}
          placeholder="/path/to/scan"
          disabled={isScanning}
        />
        <button onClick={startScan} disabled={isScanning}>
          {isScanning ? "Analyzing..." : "Analyze Topology"}
        </button>
      </div>

      {error && (
        <div className="error">
          Error: {error}
        </div>
      )}

      <div className="visualization-container">
        {treeData ? (
          <RiskTreemap data={treeData} />
        ) : (
          <p className="placeholder">Select a target to map the dark matter.</p>
        )}
      </div>
    </div>
  );
}

export default App;
