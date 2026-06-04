# bm.ps1 - PowerShell wrapper for bmrk
#
# INSTALLATION
# ------------
# Add ONE of these lines to your PowerShell profile ($PROFILE):
#
#   . "C:\path\to\bm.ps1"
#
#   -- or inline (no extra file needed):
#
#   function bm { $t=[IO.Path]::GetTempFileName(); try { & bmrk.exe @args > $t; if ($LASTEXITCODE -eq 0) { $r=(Get-Content $t -Raw)?.Trim(); if ($r -and (Test-Path $r -PathType Container)) { $env:BMRK_PREV_DIR=$PWD.Path; Set-Location $r } elseif ($r) { Write-Output $r } } } finally { Remove-Item $t -EA 0 } }
#
# To open your profile for editing:
#   notepad $PROFILE
#
# After editing, reload:
#   . $PROFILE

function bm {
    # Return to previous directory
    if ($args.Count -eq 1 -and $args[0] -eq '-') {
        if ($env:BMRK_PREV_DIR -and (Test-Path $env:BMRK_PREV_DIR -PathType Container)) {
            $prev = $env:BMRK_PREV_DIR
            $env:BMRK_PREV_DIR = $PWD.Path
            Set-Location $prev
        } else {
            Write-Error 'bm: no previous directory'
        }
        return
    }

    # Flags that should run bmrk directly without cd
    if ($args.Count -ge 1 -and $args[0] -in '-h', '--help', '--version', '-bm', '--bm') {
        & bmrk.exe @args
        return
    }

    # Run bmrk, capture stdout to temp file.
    # TUI renders to stderr (visible in terminal); stdout carries the path.
    $tmpFile = [IO.Path]::GetTempFileName()
    try {
        & bmrk.exe @args > $tmpFile
        $exitCode = $LASTEXITCODE

        if ($exitCode -ne 0) { return }

        $result = (Get-Content $tmpFile -Raw)?.Trim()

        if ($result -and (Test-Path $result -PathType Container)) {
            $env:BMRK_PREV_DIR = $PWD.Path
            Set-Location $result
        } elseif ($result) {
            Write-Output $result
        }
    } finally {
        Remove-Item $tmpFile -ErrorAction SilentlyContinue
    }
}
