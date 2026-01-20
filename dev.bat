@echo off
echo 🔥 Crucible Engine - Development Setup
echo.

if "%1"=="demo" (
    echo Running Crucible Engine demo...
    cargo run --bin crucible-demo
    goto :eof
)

if "%1"=="build" (
    echo Building Crucible Engine...
    cargo build
    goto :eof
)

if "%1"=="test" (
    echo Running tests...
    cargo test
    goto :eof
)

if "%1"=="clean" (
    echo Cleaning build artifacts...
    cargo clean
    goto :eof
)

echo Usage: dev.bat [demo^|build^|test^|clean]
echo.
echo Commands:
echo   demo   - Run the core demo
echo   build  - Build the project
echo   test   - Run tests
echo   clean  - Clean build artifacts