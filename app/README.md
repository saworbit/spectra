# Spectra GUI - The Visual Lens

A high-fidelity visualization interface for Spectra's analytics system. Features Risk Treemaps, Sunburst charts, Time-Travel Analytics, and progressive scanning with real-time feedback.

## Overview

Spectra Vision transforms file system data into interactive visualizations. The interface provides two modes:

1. **Local Scan** - Directory analysis with treemap and sunburst visualizations, progressive scan feedback, and session tracking
2. **Time-Travel Analytics** - Historical timeline navigation with velocity metrics and extension deltas

## Components

### 1. Risk Treemap (`RiskTreemap.tsx`)

Visualizes "Data Gravity" (file size) and "Data Toxicity" (entropy) using an interactive treemap.

**Visual Encoding:**
- **Size**: Area of each rectangle (file/directory size)
- **Color** (Risk Levels):
  - Green (0-3.0): Low Entropy - Text, Code, Configs
  - Yellow (3.0-6.0): Medium - Binaries, Media
  - Orange (6.0-7.5): High - Compressed data
  - Red (7.5-8.0): Critical - Encryption, High Randomness

### 2. Sunburst Chart (`SunburstChart.tsx`)

Alternative extension visualization using `@nivo/sunburst`:
- Shows top 12 extensions by size with "other" bucket
- Toggle between Table and Sunburst views in the Top Extensions card
- Interactive hover tooltips with size and count details

### 3. TimeSlider (`TimeSlider.tsx`)

Interactive timeline navigation for Time-Travel Analytics:
- Dual-range sliders for start/end timestamp selection
- Auto-selects first and last snapshots
- Human-readable date formatting

### 4. VelocityCard (`VelocityCard.tsx`)

Growth metrics visualization:
- Total growth/shrinkage indicators with color coding
- Velocity rate with adaptive units (B/s through GB/s)
- Top 10 contributing extensions sorted by impact

### 5. Progressive Scan UI

Real-time feedback during directory scanning:
- Live counters: files scanned, folders scanned, bytes processed
- Animated progress bar with pulse effect
- Receives streaming events from Tauri backend

### 6. Session Space-Freed Counter

Tracks governance actions during the session:
- Green banner shows total bytes and files freed
- Persists across multiple scans within the session
- Exposed via `window.__spectraTrackDeletion` for Tauri commands

## Architecture

### Backend (Rust)
- **File**: [src-tauri/src/lib.rs](src-tauri/src/lib.rs)
- **Commands**:
  - `get_scan_tree` - Hierarchical tree with entropy for treemap
  - `scan_directory` - Statistics scan with progressive events
- **Events**: `scan-progress` emitted during scanning with file/folder/byte counts

### Frontend (React + TypeScript)
- **Main App**: [src/App.tsx](src/App.tsx) - Dual-mode interface with tab navigation
- **Visualization**: [src/components/RiskTreemap.tsx](src/components/RiskTreemap.tsx), [src/components/SunburstChart.tsx](src/components/SunburstChart.tsx)
- **Time-Travel**: [src/TimeSlider.tsx](src/TimeSlider.tsx), [src/VelocityCard.tsx](src/VelocityCard.tsx)
- **API Client**: [src/api.ts](src/api.ts) - Server communication (history, velocity, snapshots, aggregates)
- **Types**: [src/types.ts](src/types.ts) - ScanStats, ScanProgress, VelocityReport, TimeSeriesBucket

## Technology Stack

- **Framework**: Tauri 2.0 (Rust backend + Web frontend)
- **UI Library**: React 19 with TypeScript
- **Visualization**: Nivo (treemap + sunburst), D3-powered
- **UI Components**: Material-UI 5
- **Build Tool**: Vite 7

## Development

### Prerequisites
```bash
# Install Node.js dependencies
# Note: Use --legacy-peer-deps due to React 19 compatibility
npm install --legacy-peer-deps

# Rust toolchain should be installed via Tauri setup
```

### Running the Application
```bash
# Development mode with hot reload
npm run tauri dev

# Build for production
npm run tauri build
```

### Project Structure
```
app/
├── src/
│   ├── components/
│   │   ├── RiskTreemap.tsx       # Treemap visualization
│   │   ├── SunburstChart.tsx     # Sunburst extension chart
│   │   └── __tests__/
│   ├── App.tsx                   # Main app (tabs, scan, space-freed)
│   ├── App.css                   # Styles (progress bar, viz toggle, banner)
│   ├── TimeSlider.tsx            # Time range selector
│   ├── VelocityCard.tsx          # Growth metrics display
│   ├── api.ts                    # Server API client
│   ├── types.ts                  # TypeScript interfaces
│   └── main.tsx                  # Entry point
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs                # Rust backend (scan commands + events)
│   │   └── main.rs               # Tauri entry point
│   └── Cargo.toml
└── package.json
```

## Usage

1. **Launch the application**: `npm run tauri dev`
2. **Local Scan tab**:
   - Enter a path and click "Deep Scan"
   - Watch progressive scan counters
   - Toggle between Table and Sunburst views for extensions
   - Review heavy hitters (top largest files)
3. **Time-Travel Analytics tab**:
   - Enter an Agent ID
   - Use time sliders to select a range
   - View velocity metrics and extension deltas

## Testing

```bash
npm test           # Single run
npm run test:watch # Watch mode
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
