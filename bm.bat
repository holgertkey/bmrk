@echo off
setlocal enabledelayedexpansion

rem bm.bat — wrapper for bmrk.exe
rem
rem Place this file and bmrk.exe in the same directory on your PATH.
rem The TUI renders to stderr (visible in terminal); stdout carries the path.
rem This wrapper captures stdout, and if it is a single directory path, cd's to it.
rem All other output (help, version, bookmark list, confirmations) is printed normally.

rem Run bmrk, redirect stdout to temp file; TUI on stderr is unaffected.
set "TMPFILE=%TEMP%\bmrk_%RANDOM%.tmp"
bmrk.exe %* 1>"%TMPFILE%"
set "EXIT_CODE=%ERRORLEVEL%"

rem Read output: track first line and total line count.
set "RESULT="
set "LINE_COUNT=0"
for /f "tokens=* usebackq" %%A in ("%TMPFILE%") do (
    if !LINE_COUNT! EQU 0 set "RESULT=%%A"
    set /a LINE_COUNT+=1
)

rem Single-line output that is a valid directory -> navigate there.
if defined RESULT (
    if !LINE_COUNT! EQU 1 (
        if exist "!RESULT!\" (
            del "%TMPFILE%" 2>nul
            endlocal & cd /d "%RESULT%"
            exit /b 0
        )
    )
)

rem Otherwise print the captured output.
if !LINE_COUNT! GTR 0 type "%TMPFILE%"
del "%TMPFILE%" 2>nul
endlocal
exit /b %EXIT_CODE%
