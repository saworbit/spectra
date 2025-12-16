@echo off
REM Launch script for testing SPECTRA with semantic analysis
REM Usage: test-analyze.bat [path]

SET TARGET_PATH=%1
IF "%TARGET_PATH%"=="" SET TARGET_PATH=.

echo ===================================
echo SPECTRA Phase 2 - Semantic Analysis
echo ===================================
echo.
echo Target: %TARGET_PATH%
echo.

cargo run -p spectra-cli -- --path "%TARGET_PATH%" --analyze --limit 10

echo.
echo ===================================
