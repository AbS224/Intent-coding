@echo off
echo 📋 Crucible Engine MIL-SPEC Documentation Manager
echo.

if "%1"=="generate" (
    echo Generating all MIL-SPEC documents...
    python milspec_generator.py generate
    goto :eof
)

if "%1"=="check" (
    echo Checking MIL-SPEC compliance...
    python milspec_generator.py check
    goto :eof
)

if "%1"=="srd" (
    echo Generating System Requirements Document...
    python milspec_generator.py srd
    goto :eof
)

if "%1"=="sdd" (
    echo Generating System Design Document...
    python milspec_generator.py sdd
    goto :eof
)

if "%1"=="secd" (
    echo Generating Security Design Document...
    python milspec_generator.py secd
    goto :eof
)

if "%1"=="tpd" (
    echo Generating Test Plan Document...
    python milspec_generator.py tpd
    goto :eof
)

if "%1"=="review" (
    echo Opening documents for review...
    if exist "docs\active\" (
        explorer "docs\active"
    ) else (
        echo No documents found. Run 'milspec.bat generate' first.
    )
    goto :eof
)

echo Usage: milspec.bat [command]
echo.
echo Commands:
echo   generate  - Generate all MIL-SPEC documents
echo   check     - Check compliance status
echo   srd       - Generate System Requirements Document
echo   sdd       - Generate System Design Document  
echo   secd      - Generate Security Design Document
echo   tpd       - Generate Test Plan Document
echo   review    - Open documents folder for review
echo.
echo Examples:
echo   milspec.bat generate
echo   milspec.bat check
echo   milspec.bat review