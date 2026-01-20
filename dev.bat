@echo off
REM Crucible Engine Development Helper - Windows
REM "Correct by Design, Not by Debugging"

if "%1"=="" goto usage

if "%1"=="setup" (
    echo 🔥 Setting up Crucible Engine development environment...
    echo.
    echo Installing Rust toolchain...
    rustup update stable
    rustup component add rustfmt clippy
    echo.
    echo Installing Node.js dependencies...
    npm install
    echo.
    echo Building Rust workspace...
    cargo build
    echo.
    echo ✅ Setup complete! Run './dev.bat status' to verify.
    goto :eof
)

if "%1"=="validate" (
    echo 🛡️ Running MIL-SPEC validation with Podman CI...
    call ci-local.bat
    goto :eof
)

if "%1"=="review" (
    echo 🔍 Starting MIL-SPEC security review...
    call review.bat
    goto :eof
)

if "%1"=="push" (
    echo 🚀 Attempting secure push with full CI validation...
    call ci-local.bat
    if %ERRORLEVEL% EQU 0 (
        echo ✅ CI passed - starting security review...
        call review.bat
        echo After review approval, run: git push origin main
    ) else (
        echo ❌ CI failed - fix issues before pushing
    )
    goto :eof
)

if "%1"=="build" (
    echo 🔨 Building Crucible Engine...
    cargo build
    goto :eof
)

if "%1"=="test" (
    echo 🧪 Running tests...
    cargo test
    goto :eof
)

if "%1"=="demo" (
    echo 🚀 Running demo...
    cargo run --bin crucible-demo
    goto :eof
)

if "%1"=="milspec" (
    echo 📋 Running MIL-SPEC generator...
    python milspec_generator.py %2
    goto :eof
)

if "%1"=="status" (
    echo 📊 Crucible Engine Development Status
    echo =====================================
    echo.
    echo Environment:
    rustc --version 2>nul || echo   Rust: Not installed
    node --version 2>nul || echo   Node.js: Not installed
    python --version 2>nul || echo   Python: Not installed
    echo.
    echo Project Status:
    if exist "Cargo.toml" (echo ✅ Rust workspace configured) else echo ❌ Missing Cargo.toml
    if exist "BUILD_CHECKLIST.md" (echo ✅ MIL-SPEC documentation present) else echo ❌ Missing MIL-SPEC docs
    if exist ".git\hooks\pre-push" (echo ✅ Security hooks installed) else echo ❌ Missing security hooks
    echo.
    echo Security:
    if exist ".security.env" (echo ✅ Security configuration present) else echo ❌ Missing security config
    if exist "review.bat" (echo ✅ Review system ready) else echo ❌ Missing review system
    goto :eof
)

if "%1"=="clean" (
    echo 🧹 Cleaning build artifacts...
    cargo clean
    if exist ".milspec-unlock" del ".milspec-unlock"
    echo ✅ Clean complete
    goto :eof
)

:usage
echo Crucible Engine Development Helper
echo ==================================
echo.
echo Usage: dev.bat [command]
echo.
echo Commands:
echo   setup     - Initial development environment setup
echo   validate  - Run Podman CI pipeline validation
echo   review    - Start security review process
echo   push      - Full CI + security review for push
echo   build     - Build the project
echo   test      - Run tests
echo   demo      - Run the demo
echo   milspec   - Generate MIL-SPEC documentation
echo   status    - Show development environment status
echo   clean     - Clean build artifacts and unlock tokens
echo.
echo Examples:
echo   dev.bat setup
echo   dev.bat validate
echo   dev.bat push