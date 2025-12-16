@echo off
setlocal enabledelayedexpansion

echo.
echo ================================================
echo  S.P.E.C.T.R.A. Modular Refactoring Validation
echo ================================================
echo  Pre-Alpha Quality Assurance Suite
echo ================================================
echo.

REM Color setup for better output
set "PASS=[92m✓[0m"
set "FAIL=[91m✗[0m"
set "INFO=[94mℹ[0m"

REM Step counter
set /a STEP=0

REM ==========================
REM CODE QUALITY
REM ==========================

echo [Step %STEP%] Code Formatting...
set /a STEP+=1
cargo fmt --all -- --check
if %errorlevel% neq 0 (
    echo %FAIL% Code formatting check failed. Run: cargo fmt --all
    exit /b 1
)
echo %PASS% Code properly formatted
echo.

echo [Step %STEP%] Linting with Clippy...
set /a STEP+=1
cargo clippy --workspace --all-targets -- -D warnings
if %errorlevel% neq 0 (
    echo %FAIL% Clippy linting failed
    exit /b 1
)
echo %PASS% No clippy warnings
echo.

REM ==========================
REM BUILD VALIDATION
REM ==========================

echo [Step %STEP%] Building spectra-core...
set /a STEP+=1
cargo build -p spectra-core --release
if %errorlevel% neq 0 (
    echo %FAIL% Core library build failed
    exit /b 1
)
echo %PASS% Core library built successfully
echo.

echo [Step %STEP%] Testing spectra-core...
set /a STEP+=1
cargo test -p spectra-core
if %errorlevel% neq 0 (
    echo %FAIL% Core tests failed
    exit /b 1
)
echo %PASS% Core tests passed
echo.

echo [Step %STEP%] Building spectra-cli...
set /a STEP+=1
cargo build -p spectra-cli --release
if %errorlevel% neq 0 (
    echo %FAIL% CLI build failed
    exit /b 1
)
echo %PASS% CLI built successfully
echo.

echo [Step %STEP%] Testing spectra-cli...
set /a STEP+=1
cargo test -p spectra-cli
if %errorlevel% neq 0 (
    echo %FAIL% CLI tests failed
    exit /b 1
)
echo %PASS% CLI tests passed
echo.

echo [Step %STEP%] Integration test: CLI basic scan...
set /a STEP+=1
cargo run -p spectra-cli --release -- --path ./spectra-core --limit 5 > nul
if %errorlevel% neq 0 (
    echo %FAIL% CLI basic scan failed
    exit /b 1
)
echo %PASS% CLI basic scan works
echo.

echo [Step %STEP%] Integration test: CLI with analysis...
set /a STEP+=1
cargo run -p spectra-cli --release -- --path ./spectra-core --analyze --limit 3 --json > nul
if %errorlevel% neq 0 (
    echo %FAIL% CLI analysis scan failed
    exit /b 1
)
echo %PASS% CLI analysis works
echo.

echo [Step %STEP%] Building Tauri app...
set /a STEP+=1
cargo build -p app --release
if %errorlevel% neq 0 (
    echo %FAIL% Tauri app build failed
    exit /b 1
)
echo %PASS% Tauri app built successfully
echo.

echo [Step %STEP%] Building server...
set /a STEP+=1
cargo build -p spectra-server --release
if %errorlevel% neq 0 (
    echo %FAIL% Server build failed
    exit /b 1
)
echo %PASS% Server built successfully
echo.

REM ==========================
REM WORKSPACE VALIDATION
REM ==========================

echo [Step %STEP%] Full workspace test...
set /a STEP+=1
cargo test --workspace
if %errorlevel% neq 0 (
    echo %FAIL% Workspace tests failed
    exit /b 1
)
echo %PASS% All workspace tests passed
echo.

REM ==========================
REM SUCCESS SUMMARY
REM ==========================

echo.
echo ================================================
echo  ✅ ALL VALIDATION CHECKS PASSED
echo ================================================
echo.
echo  Modular Architecture Status:
echo    [92m●[0m spectra-core  : Core scanning library
echo    [92m●[0m spectra-cli   : CLI with analysis + governance
echo    [92m●[0m app           : Tauri visualization app
echo    [92m●[0m spectra-server: Federation server
echo.
echo  Quality Gates:
echo    %PASS% Code formatting
echo    %PASS% Linting (clippy)
echo    %PASS% Unit tests
echo    %PASS% Integration tests
echo    %PASS% Release builds
echo.
echo  Pre-Alpha Status: READY FOR TESTING
echo ================================================
echo.

endlocal
