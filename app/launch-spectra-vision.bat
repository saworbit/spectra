@echo off
REM ============================================
REM Spectra Vision - Launch Script (Windows)
REM Phase 4: The Visualization Layer
REM ============================================

echo.
echo ╔═══════════════════════════════════════════╗
echo ║     Spectra Vision - The Lens             ║
echo ║     Phase 4: Visualization Layer          ║
echo ╚═══════════════════════════════════════════╝
echo.

REM Check if node_modules exists
if not exist "node_modules\" (
    echo [1/3] Installing dependencies...
    echo.
    call npm install
    if errorlevel 1 (
        echo.
        echo ❌ Failed to install dependencies.
        echo Please ensure Node.js and npm are installed.
        pause
        exit /b 1
    )
    echo.
    echo ✓ Dependencies installed successfully
    echo.
) else (
    echo [1/3] Dependencies already installed ✓
    echo.
)

REM Check if Rust/Cargo is available
where cargo >nul 2>nul
if errorlevel 1 (
    echo ❌ Cargo not found. Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)
echo [2/3] Rust toolchain detected ✓
echo.

REM Launch Tauri Dev Server
echo [3/3] Launching Spectra Vision...
echo.
echo ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
echo   The application will open in a new window
echo   Press Ctrl+C to stop the server
echo ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
echo.

call npm run tauri dev

if errorlevel 1 (
    echo.
    echo ❌ Failed to launch Spectra Vision.
    echo Check the error messages above for details.
    pause
    exit /b 1
)

pause
