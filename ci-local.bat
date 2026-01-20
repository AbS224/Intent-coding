@echo off
echo 🛡️ Crucible Engine Local CI Pipeline (Podman)
echo ==============================================

echo 📦 Running CI in WSL with Podman...
wsl bash -c "cd /mnt/l/FinalTry && ./ci-local.sh"

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ✅ Local CI pipeline completed successfully!
    echo 🔒 You can now proceed with MIL-SPEC review process
    echo.
    echo Next steps:
    echo 1. Run: review.bat
    echo 2. Type 'APPROVED' to unlock
    echo 3. Run: git push origin main
) else (
    echo.
    echo ❌ CI pipeline failed - fix issues before pushing
)