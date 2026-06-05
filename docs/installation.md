# Installation

## Step 1 — Install the `bmrk` binary

### From crates.io

```bash
cargo install bmrk
```

### From source

```bash
git clone https://github.com/holgertkey/bmrk.git
cd bmrk
cargo build --release
```

The binary is at:
- `target/release/bmrk` (Linux/macOS)
- `target\release\bmrk.exe` (Windows)

---

## Step 2 — Set up the `bm` wrapper

The `bm` wrapper is **required** for `cd` to work. A process cannot change its parent shell's
directory directly — the wrapper captures `bmrk`'s stdout and calls `cd` in the current shell.
This is the same approach used by `fzf`, `zoxide`, and `autojump`.

---

### Windows — PowerShell

1. Copy `bmrk.exe` to a directory on your PATH (e.g. `C:\Users\<YourName>\bin\`).

2. Open your PowerShell profile for editing:
   ```powershell
   notepad $PROFILE
   ```
   If the file does not exist yet, create it first:
   ```powershell
   New-Item -ItemType File -Force $PROFILE
   ```

3. Add the wrapper to your profile — choose one option:

   **Option A — source the provided `bm.ps1` file (recommended):**
   ```powershell
   . "C:\path\to\bmrk\bm.ps1"
   ```

   **Option B — inline function (no extra file needed):**
   ```powershell
   function bm {
       if ($args.Count -eq 1 -and $args[0] -eq '-') {
           if ($env:BMRK_PREV_DIR -and (Test-Path $env:BMRK_PREV_DIR -PathType Container)) {
               $prev = $env:BMRK_PREV_DIR
               $env:BMRK_PREV_DIR = $PWD.Path
               Set-Location $prev
           } else { Write-Error 'bm: no previous directory' }
           return
       }
       if ($args.Count -ge 1 -and $args[0] -in '-h','--help','-v','--version','-l','--list','-a','--add','-d','--del') {
           & bmrk.exe @args; return
       }
       $t = [IO.Path]::GetTempFileName()
       try {
           & bmrk.exe @args > $t
           if ($LASTEXITCODE -eq 0) {
               $r = (Get-Content $t -Raw)?.Trim()
               if ($r -and (Test-Path $r -PathType Container)) {
                   $env:BMRK_PREV_DIR = $PWD.Path; Set-Location $r
               } elseif ($r) { Write-Output $r }
           }
       } finally { Remove-Item $t -ErrorAction SilentlyContinue }
   }
   ```

4. Reload the profile:
   ```powershell
   . $PROFILE
   ```

5. Test:
   ```powershell
   bm --version
   bm
   ```

---

### Windows — CMD

1. Copy `bmrk.exe` and `bm.bat` to the same directory on your PATH
   (e.g. `C:\Users\<YourName>\bin\`).

2. If that directory is not on your PATH yet:
   ```
   setx PATH "%PATH%;C:\Users\<YourName>\bin"
   ```
   Then open a new CMD window.

3. Test:
   ```
   bm --version
   bm
   ```

---

### Linux / macOS — bash

1. Copy the binary to your PATH:
   ```bash
   cp target/release/bmrk ~/.local/bin/
   # or: sudo cp target/release/bmrk /usr/local/bin/
   ```

2. Add the wrapper to `~/.bashrc` — choose one option:

   **Option A — source the provided file:**
   ```bash
   echo '. /path/to/bmrk/bm.sh' >> ~/.bashrc
   ```

   **Option B — inline one-liner:**
   ```bash
   echo 'bm() { local r; r=$(bmrk "$@"); [ -d "$r" ] && cd "$r" || { [ -n "$r" ] && echo "$r"; }; }' >> ~/.bashrc
   ```

3. Reload:
   ```bash
   source ~/.bashrc
   ```

4. Test:
   ```bash
   bm --version
   bm
   ```

---

### Linux / macOS — zsh

Same as bash, but edit `~/.zshrc` instead of `~/.bashrc`.

---

### Linux / macOS — fish

1. Copy the binary to your PATH:
   ```bash
   cp target/release/bmrk ~/.local/bin/
   ```

2. Create `~/.config/fish/functions/bm.fish`:
   ```fish
   function bm
       set r (bmrk $argv)
       if test -d "$r"
           cd $r
       else if test -n "$r"
           echo $r
       end
   end
   ```

3. Fish reloads functions automatically. Test:
   ```bash
   bm --version
   bm
   ```

---

## Verification

After installation, verify everything works:

```bash
bm -v               # Should print: bmrk x.y.z
bm -l               # Should list bookmarks (empty on first run)
bm                  # Should open the interactive TUI
```

---

## Troubleshooting

### `bmrk: command not found`

Check that `bmrk.exe` / `bmrk` is on your PATH:
```bash
# Linux/macOS
which bmrk
echo $PATH

# PowerShell
(Get-Command bmrk).Source
$env:PATH
```

### `bm` opens but directory doesn't change after selecting

You are likely running `bm.bat` from PowerShell. Use the PowerShell wrapper (`bm.ps1` or the
inline function above) instead — `.bat` files run as subprocesses in PowerShell and cannot change
the parent session's directory.

### Bookmarks file location

Bookmarks are stored as JSON and created automatically on first use:

- **Windows**: `%APPDATA%\bmrk\bookmarks.json`
- **Linux/macOS**: `~/.config/bmrk/bookmarks.json`
