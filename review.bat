@echo off
echo 🔍 MIL-SPEC Security Review
echo ==========================

echo.
echo Recent changes:
git diff --name-only HEAD~1 HEAD 2>nul || git diff --name-only --cached

echo.
echo Security scan results:
echo ----------------------

REM Show any potential issues
echo Checking for secrets...
findstr /R "password" *.py *.js *.rs 2>nul >nul && echo SECRETS FOUND || echo Secrets: CLEAN
findstr /R "secret" *.py *.js *.rs 2>nul >nul && echo SECRETS FOUND || echo Secrets: CLEAN
findstr /R "api_key" *.py *.js *.rs 2>nul >nul && echo SECRETS FOUND || echo Secrets: CLEAN
echo Checking for XSS risks...
findstr /R "eval(" *.js 2>nul >nul && echo XSS RISKS FOUND || echo XSS: CLEAN
findstr /R "innerHTML" *.js 2>nul >nul && echo XSS RISKS FOUND || echo XSS: CLEAN
echo Checking for hardcoded paths...
findstr /R "C:\\" *.py *.js 2>nul >nul && echo HARDCODED PATHS FOUND || echo Paths: CLEAN

echo.
echo Build status:
cargo check >nul 2>&1 && echo Build: PASSED || echo Build: FAILED

echo.
echo Files to be pushed:
git status --porcelain 2>nul || echo No staged changes

echo.
set /p confirm="Review complete? Type 'APPROVED' to unlock: "
if "%confirm%"=="APPROVED" (
    echo REVIEWED > .push_token
    echo ✅ Push unlocked - run 'git push origin main'
    echo.
    echo 🔓 MIL-SPEC security lock removed
    echo ⏰ Token expires in 1 hour
) else (
    echo ❌ Review not approved - push remains blocked
)