@echo off
rem bm.bat — wrapper for bmrk.exe
rem
rem Place this file and bmrk.exe in the same directory on your PATH.
rem The TUI renders to stderr (visible in terminal); stdout carries the path.
rem If stdout is a valid directory path — cd there.
rem Otherwise print stdout as-is (help, version, bookmark list, etc.).

rem Run bmrk and capture stdout to a temp file.
set "TMPFILE=%TEMP%\bmrk_%RANDOM%.tmp"
bmrk.exe %* 1>"%TMPFILE%"
set "EXIT_CODE=%ERRORLEVEL%"

rem Read first (and normally only) line of output.
set "NAVDIR="
set /p NAVDIR= < "%TMPFILE%"

rem Empty output (Esc pressed) — do nothing.
if "%NAVDIR%"=="" (
    del "%TMPFILE%" 2>nul
    exit /b %EXIT_CODE%
)

rem Valid directory path — navigate there.
if exist "%NAVDIR%\" (
    del "%TMPFILE%" 2>nul
    cd /d "%NAVDIR%"
    exit /b 0
)

rem Other output (help, version, list, confirmation) — print it.
type "%TMPFILE%"
del "%TMPFILE%" 2>nul
exit /b %EXIT_CODE%
