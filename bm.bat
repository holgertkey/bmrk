@echo off
REM bm.bat - Cmd.exe wrapper for bmrk
REM Provides cd integration for the shell
REM
REM Usage:
REM   bm              - Open interactive TUI
REM   bm <name|path>  - Navigate to bookmark or directory
REM   bm -            - Return to previous directory
REM   bm -v           - Show version
REM   bm -l           - List bookmarks
REM   bm -a <name>    - Add bookmark
REM   bm -d <name>    - Delete bookmark

REM Handle bm - (return to previous directory)
if "%~1"=="-" (
    if not defined BMRK_PREV_DIR (
        echo bm: no previous directory >&2
        exit /b 1
    )
    if not exist "%BMRK_PREV_DIR%" (
        echo bm: previous directory does not exist >&2
        exit /b 1
    )
    set "BMRK_TMP=%BMRK_PREV_DIR%"
    set "BMRK_PREV_DIR=%CD%"
    cd /d "%BMRK_TMP%"
    set "BMRK_TMP="
    exit /b 0
)

REM Flags that should run bmrk directly without cd
if "%~1"=="-h"       goto :passthrough
if "%~1"=="--help"   goto :passthrough
if "%~1"=="-v"       goto :passthrough
if "%~1"=="--version" goto :passthrough
if "%~1"=="-l"       goto :passthrough
if "%~1"=="--list"   goto :passthrough
if "%~1"=="-a"       goto :passthrough
if "%~1"=="--add"    goto :passthrough
if "%~1"=="-d"       goto :passthrough
if "%~1"=="--del"    goto :passthrough

REM Run bmrk and capture stdout to a temp file.
REM TUI renders to stderr (visible in terminal); stdout carries the path.
set "BMRK_TMP=%TEMP%\bmrk_%RANDOM%.tmp"
bmrk.exe %* 1>"%BMRK_TMP%"
set "BMRK_EXIT=%ERRORLEVEL%"

REM Read first line of output.
set "BMRK_DIR="
set /p BMRK_DIR= <"%BMRK_TMP%"

REM Empty output (Esc pressed or error) — do nothing.
if "%BMRK_DIR%"=="" (
    del "%BMRK_TMP%" 2>nul
    exit /b %BMRK_EXIT%
)

REM Valid directory path — navigate there.
if exist "%BMRK_DIR%\" (
    del "%BMRK_TMP%" 2>nul
    set "BMRK_PREV_DIR=%CD%"
    cd /d "%BMRK_DIR%"
    exit /b 0
)

REM Other output (help, version, list, error) — print it.
type "%BMRK_TMP%"
del "%BMRK_TMP%" 2>nul
exit /b %BMRK_EXIT%

:passthrough
bmrk.exe %*
exit /b %ERRORLEVEL%
