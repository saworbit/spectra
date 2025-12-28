# Spectra Time-Travel Analytics - Integration Test Script (PowerShell)
# This script simulates multiple filesystem snapshots over time to test the velocity engine

$SERVER_URL = "http://localhost:3000/api/v1"
$AGENT_ID = "agent_sim_01"

Write-Host "🧪 Spectra Time-Travel Simulation" -ForegroundColor Blue
Write-Host "==================================" -ForegroundColor Blue
Write-Host ""
Write-Host "This script will:"
Write-Host "  1. Send 5 simulated snapshots spanning 24 hours"
Write-Host "  2. Simulate realistic data growth patterns"
Write-Host "  3. Verify history and velocity endpoints"
Write-Host ""

# Check if server is running
Write-Host "📡 Checking server availability..." -ForegroundColor Yellow
try {
    $null = Invoke-RestMethod -Uri "$SERVER_URL/policies" -Method Get -ErrorAction Stop
    Write-Host "✅ Server is running" -ForegroundColor Green
} catch {
    Write-Host "❌ Error: Server is not running at $SERVER_URL" -ForegroundColor Red
    Write-Host "Please start the server first:"
    Write-Host "  cd server; cargo run"
    exit 1
}
Write-Host ""

# Base timestamp (24 hours ago)
$BASE_TIME = [int](Get-Date -UFormat %s) - 86400

# Function to send snapshot
function Send-Snapshot {
    param(
        [int]$timestamp,
        [long]$total_size,
        [int]$file_count,
        [long]$log_size,
        [int]$log_count,
        [long]$jpg_size,
        [int]$jpg_count,
        [long]$mp4_size,
        [int]$mp4_count
    )

    $date = (Get-Date -UnixTimeSeconds $timestamp).ToString("yyyy-MM-dd HH:mm:ss")
    Write-Host "📤 Sending snapshot @ $date" -ForegroundColor Blue

    $body = @{
        agent_id = $AGENT_ID
        timestamp = $timestamp
        hostname = "sim-host-01"
        total_size_bytes = $total_size
        file_count = $file_count
        top_extensions = @(
            @("log", $log_size, $log_count),
            @("jpg", $jpg_size, $jpg_count),
            @("mp4", $mp4_size, $mp4_count),
            @("txt", 50000000, 200),
            @("pdf", 100000000, 50)
        )
    } | ConvertTo-Json -Compress

    try {
        $null = Invoke-RestMethod -Uri "$SERVER_URL/ingest" -Method Post -Body $body -ContentType "application/json"
        Write-Host "   ✅ Snapshot stored ($total_size bytes, $file_count files)" -ForegroundColor Green
    } catch {
        Write-Host "   ❌ Failed to store snapshot: $_" -ForegroundColor Red
        exit 1
    }
}

Write-Host "📊 Generating Time-Series Data..." -ForegroundColor Yellow
Write-Host ""

# T0: 24 hours ago - Baseline (1GB total)
Send-Snapshot -timestamp $BASE_TIME -total_size 1000000000 -file_count 5000 `
    -log_size 200000000 -log_count 100 `
    -jpg_size 500000000 -jpg_count 500 `
    -mp4_size 100000000 -mp4_count 10

# T1: 18 hours ago - Logs start growing (+100MB logs)
Send-Snapshot -timestamp ($BASE_TIME + 21600) -total_size 1100000000 -file_count 5200 `
    -log_size 300000000 -log_count 150 `
    -jpg_size 500000000 -jpg_count 500 `
    -mp4_size 100000000 -mp4_count 10

# T2: 12 hours ago - Video spike! (+500MB mp4)
Send-Snapshot -timestamp ($BASE_TIME + 43200) -total_size 1600000000 -file_count 5250 `
    -log_size 300000000 -log_count 150 `
    -jpg_size 500000000 -jpg_count 500 `
    -mp4_size 600000000 -mp4_count 20

# T3: 6 hours ago - Log explosion (+300MB logs)
Send-Snapshot -timestamp ($BASE_TIME + 64800) -total_size 1900000000 -file_count 5500 `
    -log_size 600000000 -log_count 300 `
    -jpg_size 500000000 -jpg_count 500 `
    -mp4_size 600000000 -mp4_count 20

# T4: Now - Steady state
Send-Snapshot -timestamp ([int](Get-Date -UFormat %s)) -total_size 2000000000 -file_count 5600 `
    -log_size 700000000 -log_count 350 `
    -jpg_size 500000000 -jpg_count 500 `
    -mp4_size 600000000 -mp4_count 20

Write-Host ""
Write-Host "🔍 Verifying History Endpoint..." -ForegroundColor Yellow

try {
    $history = Invoke-RestMethod -Uri "$SERVER_URL/history/$AGENT_ID" -Method Get
    $snapshotCount = $history.Count

    if ($snapshotCount -ge 5) {
        Write-Host "✅ History verified: $snapshotCount snapshots available" -ForegroundColor Green
    } else {
        Write-Host "❌ History verification failed: Expected 5+, got $snapshotCount" -ForegroundColor Red
        exit 1
    }
} catch {
    Write-Host "❌ Failed to verify history: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "⚡ Calculating Velocity (T0 → T4)..." -ForegroundColor Yellow

try {
    $currentTime = [int](Get-Date -UFormat %s)
    $velocity = Invoke-RestMethod -Uri "$SERVER_URL/velocity/${AGENT_ID}?start=$BASE_TIME&end=$currentTime" -Method Get

    Write-Host "✅ Velocity calculation successful" -ForegroundColor Green
    Write-Host ""
    Write-Host "📈 Velocity Report Summary:" -ForegroundColor Blue
    Write-Host ($velocity | ConvertTo-Json -Depth 10)
    Write-Host ""

    # Verify expected growth (should be ~1GB = 1,000,000,000 bytes)
    $growth = $velocity.growth_bytes
    if ($growth -gt 900000000 -and $growth -lt 1100000000) {
        Write-Host "✅ Growth verification passed: $growth bytes (~1GB as expected)" -ForegroundColor Green
    } else {
        Write-Host "⚠️  Growth anomaly detected: $growth bytes (expected ~1GB)" -ForegroundColor Yellow
    }
} catch {
    Write-Host "❌ Velocity calculation failed: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "================================" -ForegroundColor Green
Write-Host "✅ Time-Travel Simulation Complete!" -ForegroundColor Green
Write-Host "================================" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:"
Write-Host "  1. Open the Spectra GUI (npm run dev in app/)"
Write-Host "  2. Navigate to '⏳ Time-Travel Analytics' tab"
Write-Host "  3. Use agent ID: $AGENT_ID"
Write-Host "  4. Explore the timeline and velocity metrics"
Write-Host ""
Write-Host "Key insights from the simulation:"
Write-Host "  - Total growth: +1GB over 24 hours"
Write-Host "  - Velocity: ~11.5 KB/s average"
Write-Host "  - Top contributor: .log files (+500MB)"
Write-Host "  - Spike detected: .mp4 files (+500MB at T2)"
Write-Host ""
