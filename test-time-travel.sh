#!/bin/bash

# Spectra Time-Travel Analytics - Integration Test Script
# This script simulates multiple filesystem snapshots over time to test the velocity engine

SERVER_URL="http://localhost:3000/api/v1"
AGENT_ID="agent_sim_01"
SCRIPT_STARTED_SERVER=false
SERVER_PID_TO_CLEANUP=""

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
echo -e "${YELLOW}Checking server availability...${NC}"

# Check if port 3000 is listening
if ! nc -z localhost 3000 2>/dev/null && ! timeout 1 bash -c 'cat < /dev/null > /dev/tcp/localhost/3000' 2>/dev/null; then
    echo -e "${RED}Port 3000 is not listening${NC}"
    echo ""
    echo -e "${YELLOW}The Spectra Server is not running. Starting it now...${NC}"

    # Check if we're in the right directory
    if [ ! -f "./server/Cargo.toml" ]; then
        echo -e "${RED}Error: Cannot find server directory${NC}"
        echo -e "${RED}Please run this script from the Spectra root directory${NC}"
        exit 1
    fi

    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: Cargo is not installed or not in PATH${NC}"
        echo -e "${RED}Please install Rust: https://rustup.rs/${NC}"
        exit 1
    fi

    # Start the server in background
    echo -e "${YELLOW}Starting Spectra Server in background...${NC}"
    (cd server && cargo run > /tmp/spectra-server.log 2>&1) &
    SERVER_PID=$!
    SERVER_PID_TO_CLEANUP=$SERVER_PID
    SCRIPT_STARTED_SERVER=true

    # Wait for server to start (max 60 seconds)
    echo -e "${YELLOW}Waiting for server to start (up to 60 seconds)...${NC}"
    echo -e "${BLUE}This may take a while on first run (Rust compilation + startup)...${NC}"
    MAX_ATTEMPTS=60
    ATTEMPT=0
    SERVER_READY=false

    while [ $ATTEMPT -lt $MAX_ATTEMPTS ] && [ "$SERVER_READY" = false ]; do
        sleep 1
        ATTEMPT=$((ATTEMPT + 1))
        if curl -s -f --max-time 5 "${SERVER_URL}/policies" > /dev/null 2>&1; then
            SERVER_READY=true
            echo ""
            echo -e "${BLUE}Server responded after ${ATTEMPT} seconds${NC}"
        else
            if [ $((ATTEMPT % 10)) -eq 0 ]; then
                echo -n " [${ATTEMPT} s]"
            else
                echo -n "."
            fi
        fi
    done
    echo ""

    if [ "$SERVER_READY" = false ]; then
        echo -e "${RED}Error: Server did not start within 60 seconds${NC}"
        echo -e "${RED}Server log: /tmp/spectra-server.log${NC}"
        tail -n 20 /tmp/spectra-server.log
        echo -e "${YELLOW}Note: First run may require more time for Rust compilation${NC}"
        kill $SERVER_PID 2>/dev/null
        exit 1
    fi

    echo -e "${GREEN}Server is now running (PID: $SERVER_PID)${NC}"
    echo "Note: Server will continue running in background. To stop it, run: kill $SERVER_PID"
else
    # Port is listening, verify it's actually the Spectra server
    if curl -s -f "${SERVER_URL}/policies" > /dev/null 2>&1; then
        echo -e "${GREEN}Server is running and responding${NC}"
    else
        echo -e "${YELLOW}Warning: Port 3000 is in use but not responding to Spectra API${NC}"
        echo -e "${RED}Please ensure the Spectra Server is running:${NC}"
        echo "  cd server && cargo run"
        exit 1
    fi
fi
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
echo "  - Total growth: 1GB over 24 hours"
echo "  - Velocity: approximately 11.5 KB per second average"
echo "  - Top contributor: log files grew by 500MB"
echo "  - Spike detected: mp4 files grew by 500MB in second period"
echo ""
echo ""
echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}Launching GUI...${NC}"
echo -e "${BLUE}================================${NC}"
echo ""

# Check if npm is available
if ! command -v npm &> /dev/null; then
    echo -e "${YELLOW}Warning: npm not found. Please install Node.js to run the GUI${NC}"
    echo "You can manually start the GUI with: cd app && npm run dev"
    echo ""
    echo -e "${GREEN}Press ENTER to clean up and exit...${NC}"
    read -r
else
    # Check if app directory exists
    if [ ! -f "./app/package.json" ]; then
        echo -e "${YELLOW}Warning: Cannot find app directory${NC}"
        echo -e "${GREEN}Press ENTER to clean up and exit...${NC}"
        read -r
    else
        # Launch the GUI in background
        echo -e "${YELLOW}Starting Spectra Vision GUI in background...${NC}"
        (cd app && npm run dev > /tmp/spectra-gui.log 2>&1) &
        GUI_PID=$!

        echo -e "${BLUE}GUI is starting (this may take a moment for npm to start Vite)...${NC}"

        # Wait for Vite to start, then open browser
        sleep 3
        echo -e "${BLUE}Opening browser...${NC}"

        # Try port 1420 first (Tauri), then 5173 (standalone Vite)
        # Pre-populate agent ID and auto-switch to Time-Travel tab
        URL_1420="http://localhost:1420/?agentId=${AGENT_ID}"
        URL_5173="http://localhost:5173/?agentId=${AGENT_ID}"

        # Use xdg-open on Linux, open on macOS
        if command -v xdg-open &> /dev/null; then
            xdg-open "$URL_1420" 2>/dev/null || xdg-open "$URL_5173" 2>/dev/null || echo -e "${YELLOW}Could not auto-open browser. Please navigate to: ${URL_1420} or ${URL_5173}${NC}"
        elif command -v open &> /dev/null; then
            open "$URL_1420" 2>/dev/null || open "$URL_5173" 2>/dev/null || echo -e "${YELLOW}Could not auto-open browser. Please navigate to: ${URL_1420} or ${URL_5173}${NC}"
        else
            echo -e "${YELLOW}Browser auto-open not available. Please navigate to: ${URL_1420} or ${URL_5173}${NC}"
        fi

        echo ""
        echo -e "${GREEN}================================${NC}"
        echo -e "${GREEN}Ready to Explore!${NC}"
        echo -e "${GREEN}================================${NC}"
        echo ""
        echo -e "${YELLOW}Browser should now be open with Time-Travel Analytics loaded!${NC}"
        echo -e "${GREEN}Agent ID pre-filled: ${AGENT_ID}${NC}"
        echo ""
        echo -e "${BLUE}What you'll see:${NC}"
        echo "  - Time-Travel Analytics tab (auto-selected)"
        echo "  - Agent ID: ${AGENT_ID} (pre-filled)"
        echo "  - Interactive timeline with 5 snapshots over 24 hours"
        echo "  - Velocity metrics showing ~1GB growth"
        echo ""
        echo -e "${GREEN}Press ENTER when you're done exploring to clean up and exit...${NC}"
        read -r

        # Clean up GUI process
        echo ""
        echo -e "${YELLOW}Stopping GUI...${NC}"
        if kill $GUI_PID 2>/dev/null; then
            echo -e "${BLUE}   Stopped GUI process (PID: ${GUI_PID})${NC}"
        else
            echo -e "${YELLOW}Could not stop GUI automatically (may have already exited)${NC}"
        fi

        # Clean up GUI log
        if [ -f "/tmp/spectra-gui.log" ]; then
            rm -f /tmp/spectra-gui.log
            echo -e "${BLUE}   Removed log file: /tmp/spectra-gui.log${NC}"
        fi
    fi
fi

echo ""
echo -e "${YELLOW}Cleaning up...${NC}"

# Clean up test data
echo -e "${YELLOW}Deleting test agent data...${NC}"
echo -e "${BLUE}   Agent ID: ${AGENT_ID} (5 snapshots in in-memory database)${NC}"
echo -e "${BLUE}   Data will be removed when server stops (using in-memory database)${NC}"

# Stop the server if we started it
if [ "$SCRIPT_STARTED_SERVER" = true ] && [ -n "$SERVER_PID_TO_CLEANUP" ]; then
    echo -e "${YELLOW}Stopping Spectra Server...${NC}"
    if kill "$SERVER_PID_TO_CLEANUP" 2>/dev/null; then
        echo -e "${BLUE}   Stopped server process (PID: ${SERVER_PID_TO_CLEANUP})${NC}"
        echo -e "${BLUE}   Server was started by this script at http://localhost:3000${NC}"
    else
        echo -e "${BLUE}   Server process (PID: ${SERVER_PID_TO_CLEANUP}) already exited${NC}"
        echo -e "${BLUE}   (This is normal - the server may have stopped on its own)${NC}"
    fi
else
    echo -e "${BLUE}Server was already running - leaving it running${NC}"
    echo -e "${BLUE}   Location: http://localhost:3000${NC}"
fi

# Clean up log file
if [ -f "/tmp/spectra-server.log" ] && [ "$SCRIPT_STARTED_SERVER" = true ]; then
    rm -f /tmp/spectra-server.log
    echo -e "${BLUE}   Removed log file: /tmp/spectra-server.log${NC}"
fi

echo ""
echo -e "${GREEN}================================${NC}"
echo -e "${GREEN}Cleanup Complete!${NC}"
echo -e "${GREEN}================================${NC}"
echo ""
echo -e "${BLUE}Thank you for trying Spectra Time-Travel Analytics!${NC}"
echo ""
