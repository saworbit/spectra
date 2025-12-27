# Phase 4 Quick Start Guide: The Lens

> **"From Numbers to Vision"** - Interactive Risk Visualization

This guide walks you through launching and using Spectra Vision, the Phase 4 visualization interface that transforms entropy metrics into intuitive risk treemaps.

---

## 🎯 What is Phase 4?

Phase 4 introduces **The Lens** - a visual interface that displays your file system as an interactive treemap where:
- **Size** = Area of rectangles (larger files = bigger blocks)
- **Color** = Risk level based on entropy:
  - 🟩 **Green**: Safe (0-3.0) - Text, code, configs
  - 🟨 **Yellow**: Medium (3.0-6.0) - Binaries, media
  - 🟧 **Orange**: High (6.0-7.5) - Compressed data
  - 🟥 **Red**: Critical (7.5-8.0) - Encrypted/obfuscated data

---

## 🚀 Quick Start (30 Seconds)

### Option 1: One-Click Launch (Recommended)

**Windows:**
```bash
# From the project root
launch-vision.bat
```

**Unix/Linux/macOS:**
```bash
# From the project root
./launch-vision.sh
```

The script will:
1. ✓ Check for Node.js and Cargo
2. ✓ Install dependencies (if needed)
3. ✓ Launch the Tauri application

### Option 2: Manual Launch

```bash
cd app
npm install          # First time only
npm run tauri dev
```

---

## 📊 Using the Interface

### 1. Launch the Application
Run one of the launch scripts above. The Spectra Vision window will open.

### 2. Select a Target Directory
In the path input field, enter the directory you want to scan:
- `./` - Current directory
- `C:\Users\YourName\Documents` - Absolute Windows path
- `/home/user/projects` - Absolute Unix path

### 3. Analyze the Topology
Click the **"Analyze Topology"** button. The scan will:
- Traverse the directory tree (up to 3 levels deep)
- Calculate mock entropy for each file
- Build a hierarchical tree structure
- Render the risk treemap

### 4. Interact with the Visualization
- **Hover** over rectangles to see:
  - File/directory name
  - Total size (formatted, e.g., "1.2M")
  - Entropy score (0.0 to 8.0)
- **Click** on rectangles to:
  - Log details to the browser console
  - (Future: drill down into subdirectories)

### 5. Interpret the Colors
The color encoding reveals risk at a glance:
- **Green dominance**: Mostly text/code - low risk
- **Yellow blocks**: Binary executables or media - normal
- **Orange patches**: Compressed archives - moderate concern
- **Red regions**: Encrypted files or random data - investigate immediately

---

## 🔧 Configuration

### Adjusting Scan Depth

The default scan depth is 3 levels (for UI responsiveness). To change it:

**File:** `app/src-tauri/src/lib.rs`

```rust
scan_directory_recursive(root, 0, 3) // Change 3 to desired depth
```

### Customizing Color Thresholds

**File:** `app/src/components/RiskTreemap.tsx`

```typescript
if (entropy < 3.0) return '#4caf50'; // Green - adjust threshold
if (entropy < 6.0) return '#ffeb3b'; // Yellow
if (entropy < 7.5) return '#ff9800'; // Orange
return '#f44336'; // Red
```

---

## 🧪 Example Scenarios

### Scenario 1: Find Encrypted Backups
**Goal:** Locate large encrypted files consuming space.

**Steps:**
1. Scan your backup directory: `C:\Backups`
2. Look for large red rectangles (high entropy)
3. Hover to see entropy scores > 7.5
4. Decide: Keep, archive, or delete

### Scenario 2: Detect Data Sprawl
**Goal:** Identify which subdirectory has the most data.

**Steps:**
1. Scan a parent directory: `/home/user`
2. Visually compare rectangle sizes
3. Largest rectangles = heaviest subdirectories
4. Drill down to investigate (future feature)

### Scenario 3: Security Audit
**Goal:** Find potentially sensitive files by risk level.

**Steps:**
1. Scan project directory: `./my-project`
2. Note any red/orange blocks in unexpected locations
3. Hover to identify files
4. Investigate files with names like `secrets.enc`, `backup.zip`

---

## 🏗️ Architecture Overview

```
User Input (Path)
      ↓
[App.tsx] - React State Management
      ↓
invoke("get_scan_tree", { path })
      ↓
[Tauri IPC Bridge]
      ↓
[lib.rs] - Rust Backend
      ├── scan_directory_recursive() - Walk filesystem
      ├── calculate_mock_entropy() - Analyze files
      └── Return TreeNode JSON
      ↓
[RiskTreemap.tsx] - Nivo Visualization
      └── ResponsiveTreeMap (D3-powered)
```

### Data Flow

1. **User** enters path and clicks "Analyze"
2. **Frontend** calls Tauri command via IPC
3. **Backend** scans filesystem recursively
4. **Backend** calculates entropy for each file
5. **Backend** aggregates directory entropy (average of children)
6. **Backend** returns `TreeNode` structure as JSON
7. **Frontend** receives data and updates state
8. **RiskTreemap** component renders visualization
9. **User** interacts with treemap

---

## 📦 Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Desktop Shell** | Tauri 2.0 | Native window, IPC, system access |
| **Backend** | Rust (std::fs) | High-performance filesystem scanning |
| **Frontend** | React 19 + TypeScript | UI state management and interactivity |
| **Visualization** | Nivo 0.84.0 | D3-powered treemap rendering |
| **Styling** | CSS + Emotion | Dark-themed interface |
| **Build Tool** | Vite 7 | Fast development with HMR |

---

## 🔮 Future Enhancements

### Temporal Navigation (Planned)
- **Snapshot History**: Store multiple scans over time
- **Time Slider**: Scrub through historical data
- **Growth Visualization**: Animate changes between T₀ and T₁
- **Velocity Metrics**: Show which directories are growing fastest

### Real Entropy Integration (Planned)
- Replace mock entropy with Phase 2 logic
- Actual Shannon entropy calculation on file headers
- Real-time risk detection during scan

### Advanced Interactions (Planned)
- **Drill-Down**: Click to zoom into subdirectories
- **Filtering**: Show only high-risk files
- **Search**: Find files by name or pattern
- **Export**: Save visualization as PNG or report

---

## 🐛 Troubleshooting

### Issue: "Failed to scan path"

**Cause:** Path doesn't exist or permission denied.

**Fix:**
- Verify the path exists
- Try a different directory
- Check file system permissions

### Issue: "Cargo not found"

**Cause:** Rust toolchain not installed.

**Fix:**
```bash
# Install Rust from https://rustup.rs/
# Windows: Run rustup-init.exe
# Unix: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Issue: "npm install fails"

**Cause:** Node.js not installed or network issue.

**Fix:**
- Install Node.js from https://nodejs.org/
- Clear npm cache: `npm cache clean --force`
- Delete `node_modules` and `package-lock.json`, then retry

### Issue: Application window is blank

**Cause:** Frontend build error or hot reload issue.

**Fix:**
```bash
cd app
npm run build     # Force rebuild
npm run tauri dev # Relaunch
```

---

## 🎓 Best Practices

### 1. Start Small
Begin with a known directory (e.g., `./app`) to understand the visualization before scanning large trees.

### 2. Use Absolute Paths
Relative paths (`./, ../`) work but absolute paths are clearer: `C:\MyData` or `/home/user/docs`.

### 3. Watch the Depth
Deeper scans = more data = slower rendering. Stick to 3-4 levels for interactive performance.

### 4. Color Patterns to Watch
- **All green**: Likely source code or documentation
- **Mixed yellow/green**: Normal project (binaries + text)
- **Red clusters**: Investigate - encrypted backups, obfuscated code, or malware

### 5. Combine with CLI
Use the CLI agent (Phase 1-2) for detailed analysis:
```bash
spectra-cli --path ./suspicious-dir --analyze --json > report.json
```

---

## 📚 Related Documentation

- **[app/README.md](../app/README.md)**: Detailed GUI component documentation
- **[ARCHITECTURE.md](ARCHITECTURE.md)**: System design and rationale
- **[PHASE3_GUIDE.md](PHASE3_GUIDE.md)**: Federation and governance guide
- **[README.md](../README.md)**: Main project overview

---

## 🆘 Need Help?

- **GitHub Issues**: [Report bugs or request features](https://github.com/YOUR_USERNAME/spectra/issues)
- **Architecture Doc**: Deep dive into design decisions
- **Source Code**: `app/src/` (Frontend), `app/src-tauri/src/` (Backend)

---

*"The dark matter becomes visible."*
