@echo off
REM Quick setup for Macro Bot GUI on Windows

echo Building Macro Bot GUI...
echo.
echo First build takes 5-10 minutes (compiling GUI framework)
echo Subsequent builds are fast!
echo.

cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo Build successful!
    echo.
    echo To run:
    echo   .\target\release\macro-bot-gui.exe
    echo.
    echo A window will open with all the controls!
    echo No freezing or blocking issues!
    echo.
) else (
    echo.
    echo Build failed
    exit /b 1
)
