@echo off
title Spectra - Test Suite
color 0D
echo.
echo ========================================
echo   SPECTRA - Running Test Suite
echo ========================================
echo.

echo Running all workspace tests...
echo.
cargo test --workspace
if %ERRORLEVEL% NEQ 0 (
    color 0C
    echo.
    echo ========================================
    echo   TESTS FAILED
    echo ========================================
    echo.
) else (
    color 0A
    echo.
    echo ========================================
    echo   ALL TESTS PASSED
    echo ========================================
    echo.
)

pause
