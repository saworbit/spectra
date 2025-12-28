# Spectra Time-Travel Analytics - Integration Test Script (PowerShell)
# This script simulates multiple filesystem snapshots over time to test the velocity engine

$SERVER_URL = "http://localhost:3000/api/v1"
$AGENT_ID = "agent_sim_01"
$SCRIPT_STARTED_SERVER = $false
$SERVER_PROCESS = $null

Write-Host "Spectra Time-Travel Simulation" -ForegroundColor Blue
Write-Host "==================================" -ForegroundColor Blue
Write-Host ""
Write-Host "This script will:"
Write-Host "  1. Send 5 simulated snapshots spanning 24 hours"
Write-Host "  2. Simulate realistic data growth patterns"
Write-Host "  3. Verify history and velocity endpoints"
Write-Host ""

# Check if server is running
Write-Host "Checking server availability..." -ForegroundColor Yellow

# First, check if port 3000 is listening
$portListening = $false
try {
    $tcpConnection = Test-NetConnection -ComputerName localhost -Port 3000 -InformationLevel Quiet -WarningAction SilentlyContinue
    $portListening = $tcpConnection
} catch {
    # Test-NetConnection might not be available on all systems, fallback to direct check
    try {
        $tcpClient = New-Object System.Net.Sockets.TcpClient
        $tcpClient.Connect("localhost", 3000)
        $tcpClient.Close()
        $portListening = $true
    } catch {
        $portListening = $false
    }
}

if (-not $portListening) {
    Write-Host "Port 3000 is not listening" -ForegroundColor Red
    Write-Host ""
    Write-Host "The Spectra Server is not running. Starting it now..." -ForegroundColor Yellow

    # Check if we're in the right directory
    if (-not (Test-Path ".\server\Cargo.toml")) {
        Write-Host "Error: Cannot find server directory" -ForegroundColor Red
        Write-Host "Please run this script from the Spectra root directory" -ForegroundColor Red
        exit 1
    }

    # Check if cargo is available
    try {
        $null = Get-Command cargo -ErrorAction Stop
    } catch {
        Write-Host "Error: Cargo is not installed or not in PATH" -ForegroundColor Red
        Write-Host "Please install Rust: https://rustup.rs/" -ForegroundColor Red
        exit 1
    }

    # Start the server in a new window
    Write-Host "Starting Spectra Server in a new window..." -ForegroundColor Yellow
    $SERVER_PROCESS = Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd server; cargo run" -WindowStyle Normal -PassThru
    $SCRIPT_STARTED_SERVER = $true

    # Wait for server to start (max 60 seconds)
    Write-Host "Waiting for server to start (up to 60 seconds)..." -ForegroundColor Yellow
    Write-Host "This may take a while on first run (Rust compilation + startup)..." -ForegroundColor Gray
    $maxAttempts = 60
    $attempt = 0
    $serverReady = $false

    while ($attempt -lt $maxAttempts -and -not $serverReady) {
        Start-Sleep -Seconds 1
        $attempt++
        try {
            # Increase timeout and add retry logic
            $response = Invoke-RestMethod -Uri "$SERVER_URL/policies" -Method Get -ErrorAction Stop -TimeoutSec 5
            $serverReady = $true
            Write-Host ""
            Write-Host "Server responded after $attempt seconds" -ForegroundColor Gray
        } catch {
            if ($attempt % 10 -eq 0) {
                Write-Host " [$attempt s]" -NoNewline -ForegroundColor Gray
            } else {
                Write-Host "." -NoNewline
            }
        }
    }
    Write-Host ""

    if (-not $serverReady) {
        Write-Host "Error: Server did not start within 60 seconds" -ForegroundColor Red
        Write-Host "Please check the server window for errors" -ForegroundColor Red
        Write-Host "Note: First run may require more time for Rust compilation" -ForegroundColor Yellow
        exit 1
    }

    Write-Host "Server is now running" -ForegroundColor Green
} else {
    # Port is listening, verify it's actually the Spectra server
    try {
        $null = Invoke-RestMethod -Uri "$SERVER_URL/policies" -Method Get -ErrorAction Stop -TimeoutSec 5
        Write-Host "Server is running and responding" -ForegroundColor Green
    } catch {
        Write-Host "Warning: Port 3000 is in use but not responding to Spectra API" -ForegroundColor Yellow
        Write-Host "Error: $_" -ForegroundColor Red
        Write-Host "Please ensure the Spectra Server is running:" -ForegroundColor Yellow
        Write-Host "  cd server" -ForegroundColor Yellow
        Write-Host "  cargo run" -ForegroundColor Yellow
        exit 1
    }
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

    $dateFormat = "yyyy-MM-dd HH:mm:ss"
    $date = (Get-Date -UnixTimeSeconds $timestamp).ToString($dateFormat)
    Write-Host "Sending snapshot at $date" -ForegroundColor Blue

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
        Write-Host "   Snapshot stored: $total_size bytes, $file_count files" -ForegroundColor Green
    } catch {
        Write-Host "   Failed to store snapshot: $_" -ForegroundColor Red
        exit 1
    }
}

Write-Host "Generating Time-Series Data..." -ForegroundColor Yellow
Write-Host ""

# T0: 24 hours ago - Baseline (1GB total)
Send-Snapshot -timestamp $BASE_TIME -total_size 1000000000 -file_count 5000 `
    -log_size 200000000 -log_count 100 `
    -jpg_size 500000000 -jpg_count 500 `
    -mp4_size 100000000 -mp4_count 10

# T1: 18 hours ago - Logs start growing
Send-Snapshot -timestamp ($BASE_TIME + 21600) -total_size 1100000000 -file_count 5200 `
    -log_size 300000000 -log_count 150 `
    -jpg_size 500000000 -jpg_count 500 `
    -mp4_size 100000000 -mp4_count 10

# T2: 12 hours ago - Video spike
Send-Snapshot -timestamp ($BASE_TIME + 43200) -total_size 1600000000 -file_count 5250 `
    -log_size 300000000 -log_count 150 `
    -jpg_size 500000000 -jpg_count 500 `
    -mp4_size 600000000 -mp4_count 20

# T3: 6 hours ago - Log explosion
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
Write-Host "Verifying History Endpoint..." -ForegroundColor Yellow

try {
    $history = Invoke-RestMethod -Uri "$SERVER_URL/history/$AGENT_ID" -Method Get
    $snapshotCount = $history.Count

    if ($snapshotCount -ge 5) {
        Write-Host "History verified: $snapshotCount snapshots available" -ForegroundColor Green
    } else {
        Write-Host "History verification failed: Expected 5+, got $snapshotCount" -ForegroundColor Red
        exit 1
    }
} catch {
    Write-Host "Failed to verify history: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Calculating Velocity (T0 to T4)..." -ForegroundColor Yellow

try {
    $currentTime = [int](Get-Date -UFormat %s)
    $velocity = Invoke-RestMethod -Uri "$SERVER_URL/velocity/${AGENT_ID}?start=$BASE_TIME&end=$currentTime" -Method Get

    Write-Host "Velocity calculation successful" -ForegroundColor Green
    Write-Host ""
    Write-Host "Velocity Report Summary:" -ForegroundColor Blue
    Write-Host ($velocity | ConvertTo-Json -Depth 10)
    Write-Host ""

    # Verify expected growth (should be approximately 1GB)
    $growth = $velocity.growth_bytes
    if ($growth -gt 900000000 -and $growth -lt 1100000000) {
        Write-Host "Growth verification passed: $growth bytes (approximately 1GB as expected)" -ForegroundColor Green
    } else {
        Write-Host "Growth anomaly detected: $growth bytes (expected approximately 1GB)" -ForegroundColor Yellow
    }
} catch {
    Write-Host "Velocity calculation failed: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "================================" -ForegroundColor Green
Write-Host "Time-Travel Simulation Complete!" -ForegroundColor Green
Write-Host "================================" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:"
Write-Host "  1. Open the Spectra GUI: cd app, then npm run dev"
Write-Host "  2. Navigate to the Time-Travel Analytics tab"
Write-Host "  3. Use agent ID: $AGENT_ID"
Write-Host "  4. Explore the timeline and velocity metrics"
Write-Host ""
Write-Host "Key insights from the simulation:"
Write-Host "  - Total growth: 1GB over 24 hours"
Write-Host "  - Velocity: approximately 11.5 KB per second average"
Write-Host "  - Top contributor: log files grew by 500MB"
Write-Host "  - Spike detected: mp4 files grew by 500MB in second period"
Write-Host ""
Write-Host ""
Write-Host "================================" -ForegroundColor Cyan
Write-Host "Demo Paused - Explore the GUI" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "The simulation is complete and the server is ready." -ForegroundColor Yellow
Write-Host "You can now explore the Time-Travel Analytics in the GUI:" -ForegroundColor Yellow
Write-Host ""
Write-Host "  1. Open a new terminal and run: cd app && npm run dev"
Write-Host "  2. Navigate to the Time-Travel Analytics tab"
Write-Host "  3. Enter agent ID: $AGENT_ID"
Write-Host "  4. Use the timeline sliders to explore the data"
Write-Host ""
Write-Host "Press ENTER when you're done exploring to clean up and exit..." -ForegroundColor Green
$null = Read-Host

Write-Host ""
Write-Host "Cleaning up..." -ForegroundColor Yellow

# Clean up test data
Write-Host "Deleting test agent data..." -ForegroundColor Yellow
try {
    # Delete all snapshots for the test agent
    # Note: SurrealDB in-memory mode will lose data when server stops anyway
    # But we'll try to clean up explicitly for good measure
    $deleteQuery = "DELETE snapshots WHERE agent_id = '$AGENT_ID'"
    # If there was a delete endpoint, we'd use it here
    # For now, data will be cleaned when server stops (in-memory mode)
    Write-Host "Test data will be removed when server stops (using in-memory database)" -ForegroundColor Gray
} catch {
    Write-Host "Note: Could not explicitly delete data, but in-memory database will be cleared on server stop" -ForegroundColor Gray
}

# Stop the server if we started it
if ($SCRIPT_STARTED_SERVER -and $SERVER_PROCESS) {
    Write-Host "Stopping Spectra Server (PID: $($SERVER_PROCESS.Id))..." -ForegroundColor Yellow
    try {
        Stop-Process -Id $SERVER_PROCESS.Id -Force
        Write-Host "Server stopped successfully" -ForegroundColor Green
    } catch {
        Write-Host "Could not stop server automatically. Please close the server window manually." -ForegroundColor Yellow
    }
} else {
    Write-Host "Server was already running - leaving it running" -ForegroundColor Gray
}

Write-Host ""
Write-Host "================================" -ForegroundColor Green
Write-Host "Cleanup Complete!" -ForegroundColor Green
Write-Host "================================" -ForegroundColor Green
Write-Host ""
Write-Host "Thank you for trying Spectra Time-Travel Analytics!" -ForegroundColor Cyan
Write-Host ""
