#!/bin/bash
# Launch script for testing SPECTRA with semantic analysis
# Usage: ./test-analyze.sh [path]

TARGET_PATH=${1:-.}

echo "==================================="
echo "SPECTRA Phase 2 - Semantic Analysis"
echo "==================================="
echo ""
echo "Target: $TARGET_PATH"
echo ""

cargo run -p spectra-cli -- --path "$TARGET_PATH" --analyze --limit 10

echo ""
echo "==================================="
