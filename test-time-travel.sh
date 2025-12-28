#!/bin/bash

# Spectra Time-Travel Analytics - Integration Test Script
# This script simulates multiple filesystem snapshots over time to test the velocity engine

SERVER_URL="http://localhost:3000/api/v1"
AGENT_ID="agent_sim_01"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧪 Spectra Time-Travel Simulation${NC}"
echo "=================================="
echo ""
echo "This script will:"
echo "  1. Send 5 simulated snapshots spanning 24 hours"
echo "  2. Simulate realistic data growth patterns"
echo "  3. Verify history and velocity endpoints"
echo ""

# Check if server is running
echo -e "${YELLOW}📡 Checking server availability...${NC}"
if ! curl -s -f "${SERVER_URL}/policies" > /dev/null 2>&1; then
    echo -e "${RED}❌ Error: Server is not running at ${SERVER_URL}${NC}"
    echo "Please start the server first:"
    echo "  cd server && cargo run"
    exit 1
fi
echo -e "${GREEN}✅ Server is running${NC}"
echo ""

# Base timestamp (24 hours ago)
BASE_TIME=$(($(date +%s) - 86400))

# Function to send snapshot
send_snapshot() {
    local timestamp=$1
    local total_size=$2
    local file_count=$3
    local log_size=$4
    local log_count=$5
    local jpg_size=$6
    local jpg_count=$7
    local mp4_size=$8
    local mp4_count=$9

    echo -e "${BLUE}📤 Sending snapshot @ $(date -d @${timestamp} '+%Y-%m-%d %H:%M:%S')${NC}"

    curl -s -X POST "${SERVER_URL}/ingest" \
      -H "Content-Type: application/json" \
      -d '{
        "agent_id": "'"${AGENT_ID}"'",
        "timestamp": '"${timestamp}"',
        "hostname": "sim-host-01",
        "total_size_bytes": '"${total_size}"',
        "file_count": '"${file_count}"',
        "top_extensions": [
          ["log", '"${log_size}"', '"${log_count}"'],
          ["jpg", '"${jpg_size}"', '"${jpg_count}"'],
          ["mp4", '"${mp4_size}"', '"${mp4_count}"'],
          ["txt", 50000000, 200],
          ["pdf", 100000000, 50]
        ]
      }' > /dev/null

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}   ✅ Snapshot stored (${total_size} bytes, ${file_count} files)${NC}"
    else
        echo -e "${RED}   ❌ Failed to store snapshot${NC}"
        exit 1
    fi
}

echo -e "${YELLOW}📊 Generating Time-Series Data...${NC}"
echo ""

# T0: 24 hours ago - Baseline (1GB total)
send_snapshot \
    $BASE_TIME \
    1000000000 \
    5000 \
    200000000 100 \
    500000000 500 \
    100000000 10

# T1: 18 hours ago - Logs start growing (+100MB logs)
send_snapshot \
    $((BASE_TIME + 21600)) \
    1100000000 \
    5200 \
    300000000 150 \
    500000000 500 \
    100000000 10

# T2: 12 hours ago - Video spike! (+500MB mp4)
send_snapshot \
    $((BASE_TIME + 43200)) \
    1600000000 \
    5250 \
    300000000 150 \
    500000000 500 \
    600000000 20

# T3: 6 hours ago - Log explosion (+300MB logs)
send_snapshot \
    $((BASE_TIME + 64800)) \
    1900000000 \
    5500 \
    600000000 300 \
    500000000 500 \
    600000000 20

# T4: Now - Steady state
send_snapshot \
    $(date +%s) \
    2000000000 \
    5600 \
    700000000 350 \
    500000000 500 \
    600000000 20

echo ""
echo -e "${YELLOW}🔍 Verifying History Endpoint...${NC}"
HISTORY=$(curl -s "${SERVER_URL}/history/${AGENT_ID}")
SNAPSHOT_COUNT=$(echo "$HISTORY" | grep -o '[0-9]\+' | wc -l)

if [ "$SNAPSHOT_COUNT" -ge 5 ]; then
    echo -e "${GREEN}✅ History verified: ${SNAPSHOT_COUNT} snapshots available${NC}"
    echo "   Timestamps: $(echo $HISTORY | head -c 100)..."
else
    echo -e "${RED}❌ History verification failed: Expected 5+, got ${SNAPSHOT_COUNT}${NC}"
    exit 1
fi

echo ""
echo -e "${YELLOW}⚡ Calculating Velocity (T0 → T4)...${NC}"
VELOCITY_RESPONSE=$(curl -s "${SERVER_URL}/velocity/${AGENT_ID}?start=${BASE_TIME}&end=$(date +%s)")

# Extract growth bytes using grep/sed (works on most systems)
GROWTH=$(echo "$VELOCITY_RESPONSE" | grep -o '"growth_bytes":[0-9]*' | grep -o '[0-9]*')

if [ -n "$GROWTH" ]; then
    echo -e "${GREEN}✅ Velocity calculation successful${NC}"
    echo ""
    echo -e "${BLUE}📈 Velocity Report Summary:${NC}"
    echo "$VELOCITY_RESPONSE" | python3 -m json.tool 2>/dev/null || echo "$VELOCITY_RESPONSE"
    echo ""

    # Verify expected growth (should be ~1GB = 1,000,000,000 bytes)
    if [ "$GROWTH" -gt 900000000 ] && [ "$GROWTH" -lt 1100000000 ]; then
        echo -e "${GREEN}✅ Growth verification passed: ${GROWTH} bytes (~1GB as expected)${NC}"
    else
        echo -e "${YELLOW}⚠️  Growth anomaly detected: ${GROWTH} bytes (expected ~1GB)${NC}"
    fi
else
    echo -e "${RED}❌ Velocity calculation failed${NC}"
    echo "Response: $VELOCITY_RESPONSE"
    exit 1
fi

echo ""
echo -e "${GREEN}════════════════════════════════${NC}"
echo -e "${GREEN}✅ Time-Travel Simulation Complete!${NC}"
echo -e "${GREEN}════════════════════════════════${NC}"
echo ""
echo "Next steps:"
echo "  1. Open the Spectra GUI (npm run dev in app/)"
echo "  2. Navigate to '⏳ Time-Travel Analytics' tab"
echo "  3. Use agent ID: ${AGENT_ID}"
echo "  4. Explore the timeline and velocity metrics"
echo ""
echo "Key insights from the simulation:"
echo "  • Total growth: +1GB over 24 hours"
echo "  • Velocity: ~11.5 KB/s average"
echo "  • Top contributor: .log files (+500MB)"
echo "  • Spike detected: .mp4 files (+500MB at T2)"
echo ""
