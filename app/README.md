# Spectra GUI - The Visual Lens

A high-fidelity visualization interface for Spectra's entropy analysis system. Phase 4 implementation featuring Risk Treemaps and laying groundwork for Temporal Navigation.

## Overview

Spectra Vision transforms file system data into interactive, hierarchical density visualizations where **color represents risk** (entropy) and **size represents data volume**.

## Components

### 1. Risk Treemap (`RiskTreemap.tsx`)

Visualizes the "Data Gravity" (file size) and "Data Toxicity" (entropy) using an interactive treemap.

**Visual Encoding:**
- **Size**: Represented by the area of each rectangle (file/directory size)
- **Color** (Risk Levels):
  - 🟩 **Green** (0-3.0): Low Entropy - Text, Code, Configs
  - 🟨 **Yellow** (3.0-6.0): Medium - Binaries, Media
  - 🟧 **Orange** (6.0-7.5): High - Compressed data
  - 🟥 **Red** (7.5-8.0): Critical - Encryption, High Randomness

**Interactivity:**
- Click on nodes to inspect details
- Hover for tooltip showing name, size, and entropy score
- Hierarchical navigation through directory structures

### 2. Temporal Slider (Coming Soon)

Will allow traversing historical snapshots to visualize data growth and entropy changes over time.

## Architecture

### Backend (Rust)
- **File**: [src-tauri/src/lib.rs](src-tauri/src/lib.rs)
- **Command**: `get_scan_tree` - Scans directory and calculates entropy
- **Data Model**: `TreeNode` structure with entropy and risk scoring
- **Entropy Calculation**: Mock implementation based on file extensions (production will integrate full Phase 2 logic)

### Frontend (React + TypeScript)
- **Main App**: [src/App.tsx](src/App.tsx) - Controls and state management
- **Visualization**: [src/components/RiskTreemap.tsx](src/components/RiskTreemap.tsx) - Nivo treemap rendering
- **Styling**: [src/App.css](src/App.css) - Dark-themed visualization interface

## Technology Stack

- **Framework**: Tauri 2.0 (Rust backend + Web frontend)
- **UI Library**: React 19 with TypeScript
- **Visualization**: Nivo (D3-powered React components)
- **UI Components**: Material-UI 5 (for future controls)
- **Build Tool**: Vite

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
│   │   └── RiskTreemap.tsx      # Treemap visualization component
│   ├── App.tsx                  # Main application component
│   ├── App.css                  # Styles
│   └── main.tsx                 # Entry point
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs               # Rust backend logic
│   │   └── main.rs              # Tauri entry point
│   └── Cargo.toml               # Rust dependencies
└── package.json                 # Node dependencies
```

## Usage

1. **Launch the application**
   ```bash
   npm run tauri dev
   ```

2. **Enter a path** in the input field (e.g., `./`, `/path/to/scan`, `C:\Users\...`)

3. **Click "Analyze Topology"** to scan and visualize

4. **Interact with the treemap**:
   - Hover over rectangles to see details
   - Click to inspect specific files/directories
   - Observe color-coded risk levels

## Future Enhancements (Phase 4 Complete)

- **Temporal Navigation**: Slider to traverse historical snapshots
- **Real Entropy Integration**: Replace mock calculation with full Phase 2 entropy engine
- **Advanced Filtering**: Filter by risk level, file type, or size
- **Export Capabilities**: Save visualizations as images or data exports
- **Comparative Views**: Side-by-side comparisons of different directories or time periods

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Testing

Tests can be run using Vitest (setup required):
```bash
npm test
```

See [src/components/__tests__/](src/components/__tests__/) for component tests.
