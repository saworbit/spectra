@echo off
REM Launch script for basic SPECTRA scan (no analysis)
REM Usage: test-basic.bat [path]

SET TARGET_PATH=%1
IF "%TARGET_PATH%"=="" SET TARGET_PATH=.

echo ===================================
echo SPECTRA Phase 1 - Basic Scan
echo ===================================
echo.
echo Target: %TARGET_PATH%
echo.

cargo run -p spectra-cli -- --path "%TARGET_PATH%" --limit 10

echo.
echo ===================================
