@echo off
setlocal enabledelayedexpansion

set ROOT=%~dp0
set SERVER=http://127.0.0.1:3000

echo === Spectra Local Test Harness ===
echo Repo: %ROOT%
echo Server: %SERVER%
echo.

echo [1/5] Starting server in a new window...
start "Spectra Server" cmd /k "cd /d %ROOT% && cargo run -p spectra-server"

echo [2/5] Waiting for server to boot...
timeout /t 3 >nul

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
echo - Watch mode: cd /d %ROOT% ^&^& cargo run -p spectra-cli -- --path "%ROOT%" --watch
echo - Tauri UI:  cd /d %ROOT%\\app ^&^& npm install ^&^& npm run tauri dev
echo.
pause
endlocal
