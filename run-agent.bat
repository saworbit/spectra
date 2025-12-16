@echo off
title Spectra Agent - Federated Mode
color 0B
echo.
echo ========================================
echo   SPECTRA AGENT - Federated Mode
echo ========================================
echo.
echo NOTE: Ensure the server is running at:
echo       http://localhost:3000
echo.
echo Mode: DRY-RUN (reports only)
echo Use --enforce flag for active governance
echo.
echo Scanning current directory...
echo.
cargo run -p spectra-cli -- --path . --server http://localhost:3000 --analyze
if %ERRORLEVEL% NEQ 0 (
    color 0C
    echo.
    echo ERROR: Agent scan failed
    echo.
)
pause
