@echo off
title Spectra Server - Phase 3 Hub
color 0A
echo.
echo ========================================
echo   SPECTRA SERVER - The Enterprise Hub
echo ========================================
echo.
echo Starting server on http://0.0.0.0:3000
echo Press Ctrl+C to stop the server
echo.
cargo run -p spectra-server
if %ERRORLEVEL% NEQ 0 (
    color 0C
    echo.
    echo ERROR: Server failed to start
    echo.
)
pause
