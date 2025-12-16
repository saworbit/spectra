#!/bin/bash
# ============================================
# Spectra Vision - Launch Script (Unix/Linux/macOS)
# Phase 4: The Visualization Layer
# ============================================

set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo ""
echo "╔═══════════════════════════════════════════╗"
echo "║     Spectra Vision - The Lens             ║"
echo "║     Phase 4: Visualization Layer          ║"
echo "╚═══════════════════════════════════════════╝"
echo ""

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
    echo -e "${BLUE}[1/3] Installing dependencies...${NC}"
    echo ""
    npm install
    echo ""
    echo -e "${GREEN}✓ Dependencies installed successfully${NC}"
    echo ""
else
    echo -e "${GREEN}[1/3] Dependencies already installed ✓${NC}"
    echo ""
fi

# Check if Rust/Cargo is available
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo not found. Please install Rust from https://rustup.rs/${NC}"
    exit 1
fi
echo -e "${GREEN}[2/3] Rust toolchain detected ✓${NC}"
echo ""

# Launch Tauri Dev Server
echo -e "${BLUE}[3/3] Launching Spectra Vision...${NC}"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  The application will open in a new window"
echo "  Press Ctrl+C to stop the server"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

npm run tauri dev
