@echo off
REM Cross-platform UPLC file creator for Windows
REM Usage: create_uplc.bat filename "uplc_content"

setlocal enabledelayedexpansion

if "%~1"=="" (
    echo Usage: %0 ^<filename^> ^<uplc_content^>
    echo.
    echo Examples:
    echo   %0 validator.uplc "(program 1.0.0 (lam x x))"
    echo   %0 test.uplc "(program 1.0.0 (con integer 42))"
    exit /b 1
)

set FILENAME=%1
set CONTENT=%2

REM Create file with proper UTF-8 encoding using PowerShell
powershell -Command "[System.IO.File]::WriteAllText('%FILENAME%', '%CONTENT%', [System.Text.UTF8Encoding]::new($false))"

if %ERRORLEVEL% equ 0 (
    echo Created UPLC file: %FILENAME%
    echo Content: %CONTENT%
) else (
    echo Failed to create file
    exit /b 1
)

endlocal
