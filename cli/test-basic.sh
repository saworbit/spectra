#!/bin/bash
# Launch script for basic SPECTRA scan (no analysis)
# Usage: ./test-basic.sh [path]

TARGET_PATH=${1:-.}

echo "==================================="
echo "SPECTRA Phase 1 - Basic Scan"
echo "==================================="
echo ""
echo "Target: $TARGET_PATH"
echo ""

cargo run -p spectra-cli -- --path "$TARGET_PATH" --limit 10

echo ""
echo "==================================="
