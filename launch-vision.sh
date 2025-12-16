#!/bin/bash
# ============================================
# Spectra Vision - Root Launcher (Unix/Linux/macOS)
# ============================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/app"
./launch-spectra-vision.sh
