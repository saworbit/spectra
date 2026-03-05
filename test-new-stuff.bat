@echo off
setlocal enabledelayedexpansion

for %%I in ("%~dp0.") do set "ROOT=%%~fI"
set "SERVER=http://127.0.0.1:3000"
set "HEADLESS=0"

if /i "%~1"=="--headless" set "HEADLESS=1"

echo === Spectra Local Test Harness ===
echo Repo: %ROOT%
echo Server: %SERVER%
echo.

echo [1/5] Starting server...
if "%HEADLESS%"=="1" (
  if not exist "%ROOT%\\tmp" mkdir "%ROOT%\\tmp"
  powershell -NoProfile -Command "Start-Process -FilePath 'cargo' -ArgumentList @('run','-p','spectra-server') -WorkingDirectory '%ROOT%' -RedirectStandardOutput '%ROOT%\\tmp\\server.out.log' -RedirectStandardError '%ROOT%\\tmp\\server.err.log' | Out-Null"
) else (
  start "Spectra Server" cmd /k "cd /d %ROOT% && cargo run -p spectra-server"
)

echo [2/5] Waiting for server to boot...
for /l %%i in (1,1,60) do (
  powershell -NoProfile -Command "$c = New-Object System.Net.Sockets.TcpClient; try { $c.Connect('127.0.0.1',3000); $c.Close(); exit 0 } catch { exit 1 }"
  if not errorlevel 1 goto :server_ready
  ping -n 2 127.0.0.1 >nul
)
echo ERROR: Server did not become ready on port 3000.
if "%HEADLESS%"=="1" (
  if exist "%ROOT%\\tmp\\server.out.log" (
    echo --- Server stdout tail ---
    powershell -NoProfile -Command "Get-Content -Tail 60 '%ROOT%\\tmp\\server.out.log'"
  )
  if exist "%ROOT%\\tmp\\server.err.log" (
    echo --- Server stderr tail ---
    powershell -NoProfile -Command "Get-Content -Tail 60 '%ROOT%\\tmp\\server.err.log'"
  )
)
exit /b 1
:server_ready

for /f %%t in ('powershell -NoProfile -Command "[int][double](Get-Date -UFormat %%s)"') do set NOW=%%t
set /a T1=NOW-7200
set /a T2=NOW-3600

echo [3/5] Ingesting two snapshots for local_test_agent...
powershell -NoProfile -Command "$body=@{agent_id='local_test_agent'; timestamp=%T1%; hostname='local'; total_size_bytes=1000; file_count=10; top_extensions=@(@('txt',600,6),@('log',400,4))} | ConvertTo-Json -Compress; Invoke-RestMethod -Method Post -Uri '%SERVER%/api/v1/ingest' -ContentType 'application/json' -Body $body | Out-Host"
powershell -NoProfile -Command "$body=@{agent_id='local_test_agent'; timestamp=%T2%; hostname='local'; total_size_bytes=1500; file_count=12; top_extensions=@(@('txt',800,7),@('log',700,5))} | ConvertTo-Json -Compress; Invoke-RestMethod -Method Post -Uri '%SERVER%/api/v1/ingest' -ContentType 'application/json' -Body $body | Out-Host"

echo [4/5] Hitting time-travel endpoints...
powershell -NoProfile -Command "Invoke-RestMethod -Uri '%SERVER%/api/v1/history/local_test_agent' | ConvertTo-Json"
powershell -NoProfile -Command "Invoke-RestMethod -Uri '%SERVER%/api/v1/snapshot/local_test_agent?timestamp=%T2%' | ConvertTo-Json -Depth 5"
powershell -NoProfile -Command "Invoke-RestMethod -Uri '%SERVER%/api/v1/aggregate/local_test_agent?start=%T1%&end=%NOW%&bucket_seconds=3600' | ConvertTo-Json -Depth 5"

echo [5/5] Running CLI scan + analysis...
cd /d %ROOT%
cargo run -p spectra-cli -- --path "%ROOT%" --limit 10 --analyze

echo.
echo Optional manual checks:
echo - Watch mode: cd /d "%ROOT%" ^&^& cargo run -p spectra-cli -- --path "%ROOT%" --watch
echo - Tauri UI:  cd /d "%ROOT%\\app" ^&^& npm install ^&^& npm run tauri dev
echo.
if "%HEADLESS%"=="1" (
  taskkill /im spectra-server.exe /f >nul 2>nul
  if exist "%ROOT%\\tmp" rmdir /s /q "%ROOT%\\tmp"
  echo Headless run complete.
) else (
  pause
)
endlocal
