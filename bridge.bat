@echo off
setlocal enabledelayedexpansion
echo Bridge Manager
echo ===============

if "%1"=="reset" (
    echo Resetting WSL bridge...
    wsl --shutdown
    timeout /t 3 /nobreak >nul
    echo WSL bridge reset complete
    goto :eof
)

if "%1"=="status" (
    echo Checking WSL status...
    wsl --list --verbose
    echo.
    echo Checking project access...
    wsl -d Ubuntu -- ls -la /mnt/l/FinalTry
    goto :eof
)

if "%1"=="diagnose" (
    echo Running diagnostics...
    wsl -d Ubuntu -- /mnt/l/FinalTry/bridge-check.sh
    goto :eof
)

echo Usage: bridge.bat [command]
echo.
echo Commands:
echo   reset     - Reset WSL bridge
echo   status    - Check WSL status
echo   diagnose  - Run diagnostics