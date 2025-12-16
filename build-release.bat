@echo off
title Spectra - Release Build
color 0E
echo.
echo ========================================
echo   SPECTRA - Release Build
echo ========================================
echo.
echo Building optimized release binaries...
echo This may take several minutes...
echo.

echo [1/3] Building CLI Agent...
cargo build --release -p spectra-cli
if %ERRORLEVEL% NEQ 0 goto error

echo.
echo [2/3] Building Server...
cargo build --release -p spectra-server
if %ERRORLEVEL% NEQ 0 goto error

echo.
echo [3/3] Building GUI App...
cd app
call npm run tauri build
if %ERRORLEVEL% NEQ 0 goto error
cd ..

echo.
color 0A
echo ========================================
echo   BUILD SUCCESSFUL
echo ========================================
echo.
echo Release binaries available at:
echo   CLI:    target\release\spectra-cli.exe
echo   Server: target\release\spectra-server.exe
echo   GUI:    app\src-tauri\target\release\app.exe
echo.
goto end

:error
color 0C
echo.
echo ========================================
echo   BUILD FAILED
echo ========================================
echo.

:end
pause
