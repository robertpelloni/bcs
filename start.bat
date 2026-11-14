@echo off
setlocal
title BTK
cd /d "%~dp0"

echo [BTK] Starting Go port entry point (ports\go)...
where go >nul 2>nul
if errorlevel 1 (
    echo [BTK] go not found. Please install it.
    pause
    exit /b 1
)

pushd ports\go
go run .
set "rc=%errorlevel%"
popd

if not "%rc%"=="0" (
    echo [BTK] Exited with error code %rc%.
    pause
    exit /b %rc%
)
endlocal
